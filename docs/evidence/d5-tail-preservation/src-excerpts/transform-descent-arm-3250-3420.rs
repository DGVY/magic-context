    req: &TransformRequest,
    ctx: &ProducerContext<'_>,
    estimate_tokens: impl Fn(&str) -> usize + Copy,
    output_cache: Option<&Mutex<SerializedOutputCache>>,
    projection_cache: Option<&ProjectionCacheInput>,
    boundary_divergence_retry: bool,
    boundary_divergence_detected: &mut bool,
    incremental_history: bool,
) -> Result<TransformWithProjection, TransformError> {
    *boundary_divergence_detected = false;
    if !ctx.compaction_enabled {
        return apply_additive_only(store, req, ctx, estimate_tokens);
    }
    let total_started_at = Instant::now();
    let mut timings = TransformTimings::default();
    let mut m1_revision_read_timings = M1RevisionReadTimings::default();
    // OpenCode transports the frozen todo pair as one marked tool part. Older adapters did not
    // copy that marker into CK metadata, so recognize the reserved call-id namespace here too.
    // Normalizing before projection keeps the replayed pair out of selection, coverage, and output.
    let projection_started_at = Instant::now();
    let normalized_req = normalize_synthetic_todo_ingress(req);
    let ingress_req = normalized_req.as_ref().unwrap_or(req);
    // Recognition uses the canonical block projection before overlays, field stripping, or
    // ordinal rewriting. Hash only the matched continuation block so changes to sibling blocks
    // cannot invalidate the persisted anchor.
    let reusable_projection = projection_cache.filter(|cache| {
        !ingress_req.lineage_switched
            && cache.replace_from <= ingress_req.messages.len()
            && cache.replace_from <= cache.projection.message_count()
    });
    let initial_projection = if let Some(cache) = reusable_projection {
        project_messages_incremental(&ingress_req.messages, &cache.projection, cache.replace_from)?
    } else {
        project_messages(&ingress_req.messages)?
    };
    let trusted_projection_prefix = reusable_projection.and_then(|cache| {
        cache
            .projection
            .prefix_block_count(cache.replace_from)
            .map(|blocks| (cache.prior_fingerprint.as_str(), blocks))
    });
    timings.projection = elapsed_ms(projection_started_at);
    timings.projection_reused_messages = reusable_projection.map_or(0, |cache| cache.replace_from);
    timings.projection_projected_messages = ingress_req
        .messages
        .len()
        .saturating_sub(timings.projection_reused_messages);
    if reusable_projection.is_some() && prefix_projection_differential_enabled() {
        assert_prefix_projection_equivalent(&initial_projection, &ingress_req.messages)?;
    }
    if ingress_req.lineage_switched && ingress_req.is_subagent {
        return Ok(lineage_protocol_passthrough(
            ingress_req,
            initial_projection,
        ));
    }
    let mut lineage_state = LineagePassState::default();
    let mut rebased_req = None;
    if ingress_req.lineage_switched && !ingress_req.is_subagent {
        if ingress_req.descent_edge_id == 0
            || ingress_req.prior_conversation_key.is_empty()
            || ingress_req.constituents.len() > 5
            || ingress_req.session_id
                != ingress_req
                    .constituents
                    .last()
                    .map(|(_, new_key, _)| new_key.as_str())
                    .unwrap_or(ingress_req.session_id.as_str())
        {
            eprintln!(
                "mc-module: lineage protocol error for {}: malformed edge {} or target mismatch",
                ingress_req.session_id, ingress_req.descent_edge_id
            );
            return Ok(lineage_protocol_passthrough(
                ingress_req,
                initial_projection,
            ));
        }
        let initial_state = store.load(&ingress_req.session_id)?;
        let anchor = continuation_summary_anchor(ingress_req, &initial_projection);
        let constituents = ingress_req
            .constituents
            .iter()
            .map(|(prior_key, new_key, epoch)| LineageConstituent {
                prior_key: prior_key.clone(),
                new_key: new_key.clone(),
                epoch: *epoch,
            })
            .collect::<Vec<_>>();
        let outcome = store.descend_lineage(LineageDescentRequest {
            target_key: &ingress_req.session_id,
            expected_target_row_version: initial_state.row_version,
            edge_id: ingress_req.descent_edge_id,
            prior_key: &ingress_req.prior_conversation_key,
            prior_epoch: ingress_req.prior_epoch,
            new_epoch: ingress_req.new_epoch,
            constituents: &constituents,
            compaction_observed: ingress_req.compaction_observed,
            anchor: anchor.as_ref(),
            now_ms: ctx.now_ms,
        })?;
        if outcome.disposition == LineageDescentDisposition::PendingBuildSkew {
            eprintln!(
                "mc-module: lineage descent pending build-skew for target {} edge {}",
                ingress_req.session_id, ingress_req.descent_edge_id
            );
            return Ok(lineage_protocol_passthrough(
                ingress_req,
                initial_projection,
            ));
        }
        lineage_state.acknowledge_edge = outcome.acknowledge.then_some(ingress_req.descent_edge_id);
        lineage_state.disposition = Some(outcome.disposition.as_str());
        lineage_state.ordinal_base = outcome.prior_last_ordinal;
        lineage_state.force_hard = outcome.materialization_required;
        if let Some(base) = lineage_state.ordinal_base {
            rebased_req = rebase_descent_ordinals(ingress_req, base)?;
        }
    }
    let req = rebased_req.as_ref().unwrap_or(ingress_req);
    // Snapshot the store-derived request before any renderer mutation can add sentinels,
    // canonical blanks, or synthetic composition artifacts.
    let trailing_blank_source_decisions = snapshot_trailing_blank_source_decisions(req);

    // --- ingress: CK messages -> flat blocks, then strip synthetic before cache logic ---
    let projection = if rebased_req.is_some() {
        let rebase_projection_started_at = Instant::now();
        let projection = project_messages(&req.messages)?;
        timings.projection += elapsed_ms(rebase_projection_started_at);
        projection
    } else {
        initial_projection
    };
    timings.projection_blocks = projection.blocks.len();
    if let Some(id) = duplicate_ids(&projection.blocks) {
        return Err(TransformError::DuplicateBlockId(id));
    }
    let live: Vec<&FlatBlock> = projection
        .blocks
        .iter()
        .filter(|i| !i.synthetic())
        .collect();
    for item in &live {
        if item.id().starts_with(RESERVED_ID_PREFIX) {
            return Err(TransformError::ReservedId);
        }
    }
    let mut prev: Option<u64> = None;
    for msg in req.messages.iter().filter(|m| !m.ck.meta.synthetic) {
        if let Some(p) = prev {
            if msg.ordinal <= p {
                return Err(TransformError::OrdinalViolation);
            }
        }
        prev = Some(msg.ordinal);
    }

    let serializer_profile = SerializerProfile::parse(&req.serializer_profile);
    let mutation_exempt_mid = latest_assistant_message_mutation_exempt_mid(
        &req.messages,
        serializer_profile,
        req.mid_turn,
    );
    let cc_u1_active = crate::cc_u1_active(serializer_profile, req.tool_present);
    let tagging_surface_requested =
        crate::tagging_surface_active(serializer_profile, req.tool_present);
    let seed_or_sync_started_at = Instant::now();
    let transform_snapshot = store.load_transform_snapshot(&req.session_id)?;
    timings.seed_or_sync = elapsed_ms(seed_or_sync_started_at);
    timings.store_cache_state = transform_snapshot.timings.cache_state_ms;
    let tag_hydration_started_at = Instant::now();
