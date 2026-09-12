                .reused_items
                > 0
        );

        store
            .append_pending_agent_drops(
                "duplicate-tool-use",
                &["live-call#0".to_string(), "live-result#0".to_string()],
                1,
            )
            .unwrap();
        let selection_request = with_usage(replay_request, 70, 100);
        let mut selection_context = smart_pctx();
        selection_context.injected_reductions = with_reductions(vec![
            reduce("live-call#0", "skeleton", "read [dropped]"),
            reduce("live-result#0", "drop", "[dropped]"),
        ]);
        let selected = apply_once_with_estimator(
            &store,
            &selection_request,
            &selection_context,
            estimate,
            Some(&cache),
        )
        .unwrap();

        assert_eq!(selected.response.action, "SOFT");
        assert_eq!(
            selected.response.materialize_reason.as_deref(),
            Some("selection")
        );
        assert!(store
            .load_pending_agent_drops("duplicate-tool-use")
            .unwrap()
            .is_empty());
        assert_no_duplicate_tool_use_ids(selected.response.messages());
        let ids = selected
            .response
            .messages()
            .iter()
            .flat_map(|message| message.content.iter())
            .filter_map(|block| match &block.kind {
                ck_wire::CkKind::ToolCall { id, .. } => Some(id.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            ids.iter()
                .filter(|id| **id == pair.call_id.as_str())
                .count(),
            1,
            "the frozen synthetic owner is served exactly once"
        );
    }

    #[test]
    #[ignore = "run manually to profile a production-sized module pass"]
    fn full_module_pass_timing_fixture() {
        const MESSAGE_COUNT: usize = 2_500;
        const FROZEN_UNIT_COUNT: usize = 47_075;
        const NEW_TAG_COUNT: usize = 5;
        const PAYLOAD_BYTES: usize = 4_096;
        let messages = (0..MESSAGE_COUNT)
            .map(|index| {
                item(
                    &format!("m{index}"),
                    index as u64 + 1,
                    &format!("tail payload {index} {}", "x".repeat(PAYLOAD_BYTES)),
                )
            })
            .collect::<Vec<_>>();
        let mut request = req("perf-full-module-pass", "cfg0", messages);
        request.serializer_profile = "opencode-aisdk".to_string();
        request.serve_native = true;
        request.full_array_fingerprint = Some("perf-full-module-pass-0".to_string());
        let projection_started_at = Instant::now();
        let projection = project_messages(&request.messages).unwrap();
        let full_projection_ms = elapsed_ms(projection_started_at);
        let mut changed_messages = request.messages.clone();
        let changed = changed_messages
            .last_mut()
