                    )));
                }

                // Mint-absent guard: when this fold takes its anchor from a compartment
                // (coverage present), the minted boundary must be a block id that exists in
                // the live input THIS pass — the anchor is the last covered block, which the
                // producer always sends (trimming happens in our OUTPUT, and a producer-side
                // coverage trim keeps ordinals >= coverage_ordinal, so the boundary block
                // itself is never trimmed away). An empty or absent mint means either the
                // compartment's end_message_id is empty or in the wrong vocabulary (it must
                // be the flat block id `<mid>#<index>`, not a bare message id), or the store
                // still covers messages a revert removed and has not been re-cut. Committing
                // such an anchor makes presence impossible on every later pass, so reconcile
                // can never clear and every pass re-materializes — an unbounded phantom-HARD
                // loop serving summaries of content that may no longer exist. Fail loud
                // instead, on EVERY hard including a reconcile-rematerialize: a rematerialize
                // that cannot mint a presentable anchor has no path to clearing reconcile
                // either. If a hard pass cannot create a live terminal anchor, keep returning an
                // error rather than committing an anchor that cannot be presented. A later pass
                // can commit when the anchor returns; a permanently trimmed anchor requires the
                // store to be re-cut. Clearing all compartments is a valid revert, yields None
                // coverage, and mints the reserved empty anchor without entering this guard.
                if let Some(coverage_end) = comp.coverage_ordinal {
                    let minted = comp.boundary_id.as_str();
                    validate_live_boundary_ordinal(minted, coverage_end, &live)?;
                    if minted.is_empty()
                        || !boundary_available(
                            minted,
                            &live,
                            &boundary_state,
                            req.declared_trim.as_ref(),
                        )
                    {
                        if loaded.core.reconcile_pending {
                            let compartments = store.load_compartments(&req.session_id)?;
                            let keep_through_seq =
                                surviving_revert_prefix_seq(&compartments, &live);
                            let outcome = store.truncate_compartments_for_revert(
                                &req.session_id,
                                keep_through_seq,
                                commit_expected,
                            )?;
                            commit_expected = Some(outcome.row_version);
                            meta.revert_epoch = outcome.revert_epoch;
                            meta.last_recut = outcome.last_recut;
                            m1_signal = m1_revision_signal_parts_for_pass_timed(
                                store,
                                ctx.project_path,
                                ctx.note_project_path,
                                &req.session_id,
                                loaded.meta.user_profile_version,
                                ctx.memory_enabled,
                                m1_visibility_cutoff_ms,
                                Some(&mut m1_revision_read_timings),
                            )?;
                            current_m1_digest = m1_signal.revision;
                            let recut_compartments = store.load_compartments(&req.session_id)?;
                            let recut_coverage_bounds =
                                coverage_bounds_from_compartments(&recut_compartments)?;
                            let recut_covered_system_messages =
                                covered_system_messages_for_coverage(
                                    req,
                                    recut_coverage_bounds.map(|(_, end)| end),
                                    recut_coverage_bounds.map(|(start, _)| start),
                                    serializer_profile,
                                );
                            comp = crate::m0_compose::compose_m0_from_store_timed(
                                store,
                                &crate::m0_compose::M0ComposeInputs {
                                    session_id: &req.session_id,
                                    project_path: ctx.project_path,
                                    project_directory: ctx.project_directory,
                                    now_ms: ctx.now_ms,
                                    history_budget_tokens: ctx.history_budget_tokens,
                                    covered_system_messages: &recut_covered_system_messages,
                                    memory_enabled: ctx.memory_enabled,
                                    memory_budget_tokens: ctx.memory_budget_tokens,
                                    user_profile_budget_tokens: ctx.user_profile_budget_tokens,
                                    inject_docs: ctx.inject_docs,
                                    temporal_awareness: ctx.temporal_awareness,
                                    mural: m0_mural_input(req, serializer_profile),
                                },
                                estimate_tokens,
                                incremental_history,
                                &mut timings.compose,
                            )?;
                            meta.last_execute_ordinal = meta
                                .last_execute_ordinal
                                .min(comp.coverage_ordinal.unwrap_or(0));

                            if let Some(stray) = first_uncovered_live_block(
                                &recut_compartments,
                                &live,
                                comp.coverage_ordinal,
                            ) {
                                return Err(TransformError::CoverageGap(format!(
                                    "coverage gap after re-cut: live item {} (ordinal {}) is below coverage end {:?} but uncovered",
                                    stray.id(),
                                    stray.ordinal(),
                                    comp.coverage_ordinal
                                )));
                            }

                            if let Some(coverage_end) = comp.coverage_ordinal {
                                let reminted = comp.boundary_id.as_str();
                                validate_live_boundary_ordinal(reminted, coverage_end, &live)?;
                                if reminted.is_empty()
                                    || !boundary_available(
                                        reminted,
                                        &live,
                                        &boundary_state,
                                        req.declared_trim.as_ref(),
                                    )
                                {
                                    return Err(TransformError::BoundaryNotPresent(format!(
                                        "re-cut kept compartments through sequence {keep_through_seq}, \
                                     but the fold still minted absent anchor {reminted:?}; \
                                     the publisher must write flat end_message_id block ids"
                                    )));
                                }
                            }
                        } else {
                            return Err(TransformError::BoundaryNotPresent(format!(
                                "fold minted anchor {minted:?} from the folded compartment's \
                             end_message_id, but no live block carries that id; the anchor \
                             must be the flat block id (`<mid>#<index>`) of the last covered \
                             block; check the publisher's end_message_id"
                            )));
                        }
                    }
                }

                // Keep reductions whose targets remain in the new tail; discard reductions covered
                // by the new m0 summary or orphaned by a revert. Because apply_units cannot remove
                // those obsolete units in place, rebuild the frozen unit set.
                let effective = effective_reductions(
                    &core,
                    &selected_reductions,
                    suppress_bootstrap_reduction_tag_overlay,
                );
                let survivors = surviving_red_units(&effective, &live, comp.coverage_ordinal);
    compartments: &[StoredCompartment],
    live: &[&'a FlatBlock],
    coverage: Option<u64>,
) -> Option<&'a FlatBlock> {
    let coverage = coverage?;
    live.iter()
        .copied()
        .filter(|block| block.role != "system" && block.ordinal() <= coverage)
        .filter(|block| {
            !compartments
                .iter()
                .any(|compartment| stored_compartment_covers_ordinal(compartment, block.ordinal()))
        })
        .min_by_key(|block| block.ordinal())
}

fn validate_live_boundary_ordinal(
    boundary_id: &str,
    expected_ordinal: u64,
    live: &[&FlatBlock],
) -> Result<(), TransformError> {
    let Some(block) = live.iter().find(|block| block.id() == boundary_id) else {
        return Ok(());
    };
    if block.ordinal() == expected_ordinal {
        return Ok(());
    }
    Err(TransformError::BoundaryOrdinalMismatch(format!(
        "anchor {boundary_id:?} lives at ordinal {}, but the tail compartment claims {expected_ordinal}",
        block.ordinal(),
    )))
}

fn boundary_available(
    id: &str,
    live: &[&FlatBlock],
    boundary_state: &BoundaryState,
    declared: Option<&DeclaredTrim>,
) -> bool {
    live.iter().any(|block| block.id() == id)
        || matches!(boundary_state, BoundaryState::DeclaredTrimValidated)
            && declared.is_some_and(|declared| declared.flat_boundary_id == id)
}

fn resolve_boundary_state(
    store: &McStore,
    req: &TransformRequest,
    core: &CoreState,
    meta: &ModuleMeta,
    live: &[&FlatBlock],
) -> Result<(BoundaryState, Option<TrimMismatch>), TransformError> {
    if !core.boundary_id.is_empty() && live.iter().any(|block| block.id() == core.boundary_id) {
        if let Some(expected_ordinal) = meta.coverage_ordinal {
            validate_live_boundary_ordinal(core.boundary_id.as_str(), expected_ordinal, live)?;
        }
        return Ok((BoundaryState::LivePresent, None));
    }

    let Some(declared) = req.declared_trim.as_ref() else {
        return Ok((BoundaryState::Absent, None));
    };

    if declared.flat_boundary_id != core.boundary_id {
        return Ok((
            BoundaryState::Absent,
            Some(trim_mismatch(
                "boundary_identity",
                format!(
                    "declared boundary {:?} did not match durable boundary {:?}",
                    declared.flat_boundary_id, core.boundary_id
                ),
            )),
        ));
    }

    if meta.coverage_ordinal != Some(declared.boundary_absolute_ordinal) {
        return Ok((
            BoundaryState::Absent,
            Some(trim_mismatch(
                "coverage_ordinal",
                format!(
                    "declared boundary ordinal {} did not match durable coverage {:?}",
                    declared.boundary_absolute_ordinal, meta.coverage_ordinal
                ),
            )),
        ));
    }

    let compartments = store.load_compartments(&req.session_id)?;
    let tail = compartments
        .iter()
        .max_by_key(|compartment| compartment.sequence);
    match tail {
        Some(tail)
            if tail.end_message_id == declared.flat_boundary_id
                && tail.end_message == declared.boundary_absolute_ordinal as i64
                && split_block_id(&tail.end_message_id)
                    .map(|(mid, _)| mid == declared.boundary_bare_message_id)
                    .unwrap_or(false) => {}
        Some(tail) => {
            return Ok((
                BoundaryState::Absent,
                Some(trim_mismatch(
                    "tail_compartment",
                    format!(
                        "tail compartment ended at id {:?} ordinal {}, not declared id {:?} bare {:?} ordinal {}",
                        tail.end_message_id,
                        tail.end_message,
                        declared.flat_boundary_id,
                        declared.boundary_bare_message_id,
                        declared.boundary_absolute_ordinal
                    ),
                )),
            ));
        }
        None => {
            return Ok((
                BoundaryState::Absent,
                Some(trim_mismatch(
                    "tail_compartment",
                    "declared trim had no durable tail compartment".to_string(),
                )),
            ));
        }
    }

    let first_live_non_system = req
        .messages
        .iter()
        .filter(|message| !message.ck.meta.synthetic && message.ck.role != "system")
        .map(|message| message.ordinal)
        .min();
    if first_live_non_system != Some(declared.next_absolute_ordinal) {
        return Ok((
            BoundaryState::Absent,
            Some(trim_mismatch(
                "continuity",
                format!(
                    "first non-system live ordinal {:?} did not match declared next ordinal {}",
                    first_live_non_system, declared.next_absolute_ordinal
                ),
            )),
        ));
    }

    Ok((BoundaryState::DeclaredTrimValidated, None))
}

fn trim_mismatch(predicate: &'static str, detail: String) -> TrimMismatch {
    TrimMismatch { predicate, detail }
}

fn surviving_revert_prefix_seq(compartments: &[StoredCompartment], live: &[&FlatBlock]) -> i64 {
    let live_ids: BTreeSet<&str> = live.iter().map(|block| block.id()).collect();
    compartments
        .iter()
        .take_while(|compartment| live_ids.contains(compartment.end_message_id.as_str()))
        .map(|compartment| compartment.sequence)
        .last()
        .unwrap_or(-1)
}

fn has_durable_lineage(core: &CoreState, meta: &ModuleMeta, has_compartments: bool) -> bool {
    has_compartments || !core.boundary_id.is_empty() || meta.coverage_ordinal.is_some()
}

fn absent_shape_fingerprint(live: &[&FlatBlock]) -> String {
    let mut hasher = Sha256::new();
    for block in live {
        hasher.update(block.id.as_bytes());
        hasher.update([0]);
