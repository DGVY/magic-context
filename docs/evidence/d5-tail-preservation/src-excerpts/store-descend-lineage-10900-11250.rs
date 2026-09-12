                    |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()?;
            let Some((source_core_json, source_meta_json)) = source_row else {
                return Ok(LineageDescentTxnOutcome::Invalid(
                    "selected lineage source disappeared inside the fenced transaction".to_string(),
                ));
            };
            let source_core: CoreState = match serde_json::from_str(&source_core_json) {
                Ok(core) => core,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };
            let source_meta: ModuleMeta = match serde_json::from_str(&source_meta_json) {
                Ok(meta) => meta,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };
            let prior_last = source_meta.newest_live_ordinal;
            if prior_last == 0 {
                target_meta.lineage_descent_target_key = request.target_key.to_string();
                target_meta.lineage_descent_edge_id = request.edge_id;
                target_meta.lineage_descent_counters.compaction_seen = target_meta
                    .lineage_descent_counters
                    .compaction_seen
                    .saturating_add(1);
                target_meta
                    .lineage_descent_counters
                    .pending_build_skew = target_meta
                    .lineage_descent_counters
                    .pending_build_skew
                    .saturating_add(1);
                let next_version = current_target_version.max(0) as u64 + 1;
                let core_json = match serde_json::to_string(&target_core) {
                    Ok(json) => json,
                    Err(error) => {
                        return Ok(LineageDescentTxnOutcome::Serde(error.to_string()))
                    }
                };
                let meta_json = match serde_json::to_string(&target_meta) {
                    Ok(json) => json,
                    Err(error) => {
                        return Ok(LineageDescentTxnOutcome::Serde(error.to_string()))
                    }
                };
                tx.execute(
                    "INSERT INTO mc_cache_state (session_id, row_version, core_state, meta, last_activity_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)
                     ON CONFLICT(session_id) DO UPDATE SET
                         row_version = excluded.row_version,
                         core_state = excluded.core_state,
                         meta = excluded.meta,
                         last_activity_at = excluded.last_activity_at",
                    params![
                        request.target_key,
                        next_version as i64,
                        core_json,
                        meta_json,
                        request.now_ms
                    ],
                )?;
                return Ok(LineageDescentTxnOutcome::Committed(LineageDescentOutcome {
                    loaded: LoadedState {
                        core: target_core,
                        meta: target_meta,
                        row_version: Some(next_version),
                    },
                    disposition: LineageDescentDisposition::PendingBuildSkew,
                    source_key: Some(source_key),
                    prior_last_ordinal: None,
                    materialization_required: false,
                    acknowledge: false,
                }));
            }

            let mut statement = tx.prepare(
                "SELECT sequence, start_message, end_message
                   FROM mc_compartments
                  WHERE session_id = ?1
                  ORDER BY sequence ASC",
            )?;
            let ranges = statement
                .query_map(params![source_key], |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, i64>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()?;
            drop(statement);
            let prior_last_i64 = match i64::try_from(prior_last) {
                Ok(value) => value,
                Err(_) => {
                    return Ok(LineageDescentTxnOutcome::Invalid(
                        "prior lineage ordinal exceeds SQLite range".to_string(),
                    ))
                }
            };
            let mut previous_end = None;
            for (_, start, end) in &ranges {
                if *start > *end
                    || *end > prior_last_i64
                    || previous_end.is_some_and(|previous| *start <= previous)
                {
                    return Ok(LineageDescentTxnOutcome::Invalid(format!(
                        "descent range validation failed: [{start},{end}] after {previous_end:?}, prior_last={prior_last}"
                    )));
                }
                previous_end = Some(*end);
            }
            let anchor = request.anchor.expect("eligible descent has an anchor");
            let placeholder_ordinal = match prior_last.checked_add(1) {
                Some(value) => value,
                None => {
                    return Ok(LineageDescentTxnOutcome::Invalid(
                        "prior lineage ordinal overflow".to_string(),
                    ))
                }
            };
            // Fresh-lineage anchors sit at the assigner's origin, and both live
            // origin bases are legitimate: 1-based (Pi-style) and 0-based (the
            // CC-leg assigner, whose first message is ordinal 0 on every pass
            // the module already accepts). Continued lineages anchor at the
            // placeholder. Anything else is a mid-space anchor and refuses.
            if anchor.ordinal > 1 && anchor.ordinal != placeholder_ordinal {
                return Ok(LineageDescentTxnOutcome::Invalid(format!(
                    "descent anchor {} has ordinal {}, expected a fresh origin (0 or 1) or continued ordinal {}",
                    anchor.block_id, anchor.ordinal, placeholder_ordinal
                )));
            }
            let placeholder_ordinal_i64 = match i64::try_from(placeholder_ordinal) {
                Ok(value) => value,
                Err(_) => {
                    return Ok(LineageDescentTxnOutcome::Invalid(
                        "placeholder ordinal exceeds SQLite range".to_string(),
                    ))
                }
            };
            let placeholder_sequence = ranges
                .last()
                .map_or(1, |(sequence, _, _)| sequence.saturating_add(1));

            // Prepare every fallible blob mutation before the first row copy. From this point
            // onward any SQL failure unwinds the fenced transaction instead of returning a
            // success-shaped validation outcome that could commit a partial adoption.
            let prior_row = tx
                .query_row(
                    "SELECT row_version, meta FROM mc_cache_state WHERE session_id = ?1",
                    params![request.prior_key],
                    |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
                )
                .optional()?;
            let Some((prior_version, prior_meta_json)) = prior_row else {
                return Ok(LineageDescentTxnOutcome::Invalid(
                    "root prior lineage has no cache state for the publish fence".to_string(),
                ));
            };
            let mut prior_meta: ModuleMeta = match serde_json::from_str(&prior_meta_json) {
                Ok(meta) => meta,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };
            prior_meta.revert_epoch = prior_meta.revert_epoch.saturating_add(1);
            prior_meta.last_recut = Some(format!(
                "lineage descent edge {} fenced prior key at epoch {}",
                request.edge_id, prior_meta.revert_epoch
            ));
            let prior_meta_json = match serde_json::to_string(&prior_meta) {
                Ok(json) => json,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };

            target_core = source_core;
            // A descent changes the live message set. Tail reasoning decisions cannot
            // authorize changes in the new lineage, even if a harness reuses an id.
            target_core.frozen_units.retain(|unit| {
                !unit.key.starts_with("strip:reasoning_clear:")
                    && !unit.key.starts_with("strip:native_reasoning_keep:")
            });
            target_core.boundary_id = anchor.block_id.clone();
            target_core.reconcile_pending = false;
            target_meta = source_meta;
            target_meta.coverage_ordinal = Some(placeholder_ordinal);
            target_meta.coverage_compartment_seq = Some(placeholder_sequence);
            target_meta.newest_live_ordinal = prior_last;
            target_meta.historian = HistorianDurableState::default();
            target_meta.pending_rewrite = None;
            target_meta.pending_rewrite_trip_count = 0;
            target_meta.pending_rewrite_ambiguous = false;
            target_meta.pending_rewrite_last_failure = None;
            target_meta.served_output_fingerprint.clear();
            target_meta.served_output_generation = None;
            target_meta.reasoning_replay_evidence = None;
            target_meta.reasoning_clear_initialized = false;
            target_meta.anchor_block_id = Some(anchor.block_id.clone());
            target_meta.anchor_content_hash = Some(anchor.content_hash.clone());
            target_meta.ordinal_continuation_base = Some(prior_last);
            target_meta.lineage_descent_materialized = false;
            record_lineage_disposition(
                &mut target_meta,
                request.target_key,
                request.edge_id,
                &LineageDescentDisposition::Descended,
                Some(&source_key),
                Some(prior_last),
                true,
            );
            let next_target_version = current_target_version.max(0) as u64 + 1;
            let target_core_json = match serde_json::to_string(&target_core) {
                Ok(json) => json,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };
            let target_meta_json = match serde_json::to_string(&target_meta) {
                Ok(json) => json,
                Err(error) => return Ok(LineageDescentTxnOutcome::Serde(error.to_string())),
            };

            for table in [
                "mc_chunk_transcripts",
                "mc_compartments",
                "mc_tags",
                "mc_temporal_marks",
                "mc_user_hints",
                "mc_channel1_appends",
                "mc_overlay_frontiers",
            ] {
                tx.execute(
                    &format!("DELETE FROM {table} WHERE session_id = ?1"),
                    params![request.target_key],
                )?;
            }
            // Session notes follow the descended conversation key. Do not copy smart notes:
            // their project-wide visibility is independent of one lineage's retained history.
            let note_projects = {
                let mut statement = tx.prepare(
                    "SELECT DISTINCT project_path FROM mc_notes
                      WHERE session_id = ?1 AND type = 'session'",
                )?;
                let projects = statement
                    .query_map(params![source_key], |row| row.get::<_, String>(0))?
                    .collect::<Result<Vec<_>, _>>()?;
                projects
            };
            for note_project in note_projects {
                let previous_project = note_caller_project
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .replace(note_project.clone());
                let copy_result = (|| -> rusqlite::Result<()> {
                    tx.execute(
                        "DELETE FROM mc_notes
                          WHERE session_id = ?1 AND project_path = ?2 AND type = 'session'",
                        params![request.target_key, note_project],
                    )?;
                    tx.execute(
                        &format!(
                             "INSERT INTO mc_notes ({NOTE_INSERT_COLUMNS})
                              SELECT type, project_path, ?1, content, status, surface_condition,
                                     compiled_provider, compiled_config, compiled_at, compile_status,
                                     ready_at, ready_reason, manifest_json, compiled_check, check_hash,
                                    check_cron, check_failure_count, check_network_failure_count,
                                    check_quarantined_until, check_next_due_at, check_compiled_at,
                                    check_false_since_at, check_last_liveness_at, last_checked_at,
                                    check_status, check_version, policy_version, harness,
                                    anchor_block_id, anchor_ordinal, dismissed_at, dismissal_resolution,
                                    status_version, created_at_ms, updated_at_ms, NULL, NULL
                               FROM mc_notes
                              WHERE session_id = ?2 AND project_path = ?3 AND type = 'session'"
                        ),
                        params![request.target_key, source_key, note_project],
                    )?;
                    Ok(())
                })();
                *note_caller_project
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner()) = previous_project;
                copy_result?;
            }
            tx.execute(
                "INSERT INTO mc_compartments (
                     session_id, sequence, start_message, end_message, start_message_id,
                     end_message_id, start_date, end_date, title, content, p1, p2, p3, p4,
                     importance, episode_type, legacy, created_at
                 )
                 SELECT ?1, sequence, start_message, end_message, start_message_id,
                        end_message_id, start_date, end_date, title, content, p1, p2, p3, p4,
                        importance, episode_type, legacy, created_at
                   FROM mc_compartments WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_chunk_transcripts (
                     session_id, compartment_seq, start_ordinal, end_ordinal,
                     transcript_deflate, raw_messages_deflate, created_at_ms
                 )
                 SELECT ?1, compartment_seq, start_ordinal, end_ordinal,
                        transcript_deflate, raw_messages_deflate, created_at_ms
                   FROM mc_chunk_transcripts WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_tags (
                     session_id, tag_number, block_id, kind, token_count, created_at_ms, source_bytes
                 )
                 SELECT ?1, tag_number, block_id, kind, token_count, created_at_ms, source_bytes
                   FROM mc_tags WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_temporal_marks (session_id, block_id, marker_text, created_at)
                 SELECT ?1, block_id, marker_text, created_at
                   FROM mc_temporal_marks WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_user_hints (session_id, block_id, hint_text, created_at)
                 SELECT ?1, block_id, hint_text, created_at
                   FROM mc_user_hints WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_channel1_appends (session_id, block_id, reminder_text, fired_at_ms)
                 SELECT ?1, block_id, reminder_text, fired_at_ms
                   FROM mc_channel1_appends WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_overlay_frontiers (session_id, max_seen_ordinal)
                 SELECT ?1, max_seen_ordinal
                   FROM mc_overlay_frontiers WHERE session_id = ?2",
                params![request.target_key, source_key],
            )?;
            tx.execute(
                "INSERT INTO mc_compartments (
                     session_id, sequence, start_message, end_message, start_message_id,
                     end_message_id, start_date, end_date, title, content, p1, p2, p3, p4,
                     importance, episode_type, legacy, created_at
                 ) VALUES (?1, ?2, ?3, ?3, ?4, ?5, NULL, NULL, '', '', '', '', '', '', 100,
                           'lineage_boundary', 0, ?6)",
                params![
                    request.target_key,
                    placeholder_sequence,
                    placeholder_ordinal_i64,
                    anchor.message_id,
                    anchor.block_id,
                    request.now_ms
                ],
            )?;
            let placeholder_valid = tx
                .query_row(
                    "SELECT start_message, end_message, end_message_id
                       FROM mc_compartments
                      WHERE session_id = ?1 AND sequence = ?2",
