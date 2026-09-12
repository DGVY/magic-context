        trigger_timer.timings.token_cache_hits = trigger_timer
            .timings
            .token_cache_hits
            .saturating_add(token_cache_hits);
        trigger_timer.timings.tokenized_blocks = trigger_timer
            .timings
            .tokenized_blocks
            .saturating_add(tokenized_blocks);
        let compartment_end_result = store.max_compartment_end_ordinal(&parsed.session_id);
        let last_compartment_end_ordinal = match compartment_end_result {
            Ok(ordinal) if ordinal > 0 => Some(ordinal as u64),
            Ok(_) | Err(_) if loaded.meta.ordinal_continuation_base.is_some() => {
                let detail = "continued_ordinal_offset_missing";
                eprintln!(
                    "mc-module: aborting historian trigger for {}: {detail}",
                    parsed.session_id
                );
                let diagnostics = historian_no_fire_diagnostics(NoFireDiagnosticsInput {
                    no_fire: detail.into(),
                    detail_kind: detail,
                    cause: HistorianNoFireCause::ContinuedOrdinalOffsetMissing,
                    extra: None,
                    reason: None,
                    state,
                    progress: None,
                    last_failure,
                });
                self.record_no_fire(
                    &store,
                    &parsed.session_id,
                    &loaded,
                    &diagnostics,
                    &decision_context,
                );
                return PreparedHistorianAction::Complete(diagnostics);
            }
            Ok(_) | Err(_) => None,
        };
        let serializer_profile = SerializerProfile::parse(&parsed.serializer_profile)
            .expect("serializer_profile validated upstream");
        let fold_is_only_reclaim = !tail_reclaim(serializer_profile);
            Err(error) => tool_error_result(format!("Error: {error}")),
        }
    }

    async fn handle_ctx_expand_facade(&self, channel: u16, request: &Value) -> HandlerOutcome {
        let Some(args) = facade_arguments(request, &["message", "start"]) else {
            return invalid_params_error("ctx_expand arguments must be an object");
        };
        let args = &args;
        let facade_scope = match self
            .resolve_facade_scope(channel, Some(args), "memories", false)
            .await
        {
            Ok(scope) => scope,
            Err(outcome) => return outcome,
        };
        let store = match self.store.get() {
            Some(store) => store,
            None => return store_unavailable_error(),
        };
        let session_id = facade_scope.conversation_key.as_str();
        if args.get("message").is_some() {
            // Ordinal 0 is a real message on the Claude Code leg (its chunk
            // transcripts store 0-based ordinals), so the domain is non-negative.
            let Some(message) = i64_arg(args, "message").filter(|value| *value >= 0) else {
                return tool_error_result("Error: message must be a non-negative integer.");
            };
            if let Some(raw_message) =
                self.cached_expand_messages(session_id)
                    .and_then(|messages| {
                        messages.into_iter().find(|candidate| {
                            i64::try_from(candidate.ordinal).ok() == Some(message)
                        })
                    })
            {
                return mcp_text_result(render_cached_message_expand(&raw_message), false);
            }
            return match store.load_chunk_transcript_for_message(session_id, message) {
                Ok(Some(row)) => {
                    if let Some(raw_message) = durable_expand_messages(std::slice::from_ref(&row))
                        .into_iter()
                        .find(|candidate| i64::try_from(candidate.ordinal).ok() == Some(message))
                    {
                        mcp_text_result(render_cached_message_expand(&raw_message), false)
                    } else {
                        mcp_text_result(render_message_expand(row, message), false)
                    }
                }
                Ok(None) => mcp_text_result(
                    format!(
                        "Message {message} is no longer recoverable from persisted chunk transcripts. The span was evicted or was compacted before transcript capture."
                    ),
                    false,
                ),
                Err(error) => tool_error_result(format!("Error: {error}")),
            };
        }
        let Some(start) = i64_arg(args, "start") else {
            return tool_error_result(
                "Error: provide either message=<ordinal>, or start and end (non-negative integers, start <= end).",
            );
        };
        let Some(end) = i64_arg(args, "end") else {
            return tool_error_result(
                "Error: provide either message=<ordinal>, or start and end (non-negative integers, start <= end).",
            );
        };
        if start < 0 || end < start {
            return tool_error_result(
                "Error: provide either message=<ordinal>, or start and end (non-negative integers, start <= end).",
            );
        }
        let last_compacted_ordinal = match store.last_compacted_ordinal(session_id) {
            Ok(ordinal) => ordinal,
            Err(error) => return tool_error_result(format!("Error: {error}")),
        };
        if last_compacted_ordinal < start {
            return mcp_text_result(
                format!(
                    "No compacted compartments found in range {start}-{end}. The range may be live tail, outside this session's history, or compacted before transcript capture."
                ),
                false,
            );
        }
        let bounded_end = end
            .min(last_compacted_ordinal)
            .min(start.saturating_add(CTX_EXPAND_MAX_ORDINAL_SPAN - 1));
        let compartments = match store.load_compartments_for_range(
            session_id,
            start,
            bounded_end,
            CTX_EXPAND_MAX_ROWS,
        ) {
            Ok(compartments) => compartments,
            Err(error) => return tool_error_result(format!("Error: {error}")),
        };
        let transcripts = match store.load_chunk_transcripts_for_range_bounded(
            session_id,
            start,
            bounded_end,
            CTX_EXPAND_MAX_ROWS,
        ) {
            Ok(transcripts) => transcripts,
            Err(error) => return tool_error_result(format!("Error: {error}")),
        };
        if args.get("verbose").and_then(Value::as_bool) == Some(true) {
            let durable_messages = durable_expand_messages(&transcripts);
            let rendered = self
                .cached_expand_messages(session_id)
                .or_else(|| (!durable_messages.is_empty()).then_some(durable_messages))
                .map(|messages| render_verbose_range_expand(&messages, start, bounded_end))
