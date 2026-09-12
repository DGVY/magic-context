            })
            .map_err(Into::into)
    }

    /// Delete every row whose ownership is expressed by an exact `session_id` column.
    /// Project memories and smart notes survive because their ownership is project-scoped;
    /// session notes and every cache/overlay/producer ledger row are removed atomically.
    pub fn delete_session(
        &self,
        session_id: &str,
        project_path: &str,
    ) -> Result<usize, McStoreError> {
        self.with_note_conn_fenced(project_path, |tx| {
            let tables = {
                let mut stmt = tx.prepare(
                    "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
                )?;
                let rows = stmt
                    .query_map([], |row| row.get::<_, String>(0))?
                    .collect::<Result<Vec<_>, _>>()?;
                rows
            };
            let mut deleted = 0usize;
            for table in tables {
                let quoted = format!("\"{}\"", table.replace('"', "\"\""));
                let has_session_id = {
                    let mut stmt = tx.prepare(&format!("PRAGMA table_info({quoted})"))?;
                    let columns = stmt
                        .query_map([], |row| row.get::<_, String>(1))?
                        .collect::<Result<Vec<_>, _>>()?;
                    columns.into_iter().any(|column| column == "session_id")
                };
                if has_session_id {
                    deleted = deleted.saturating_add(if table == "mc_notes" {
                        tx.execute(
                            &format!(
                                "DELETE FROM {quoted} WHERE session_id = ?1 AND project_path = ?2 AND type = 'session'"
                            ),
                            params![session_id, project_path],
                        )?
                    } else {
                        tx.execute(
                            &format!("DELETE FROM {quoted} WHERE session_id = ?1"),
                            params![session_id],
                        )?
                    });
                }
            }
            Ok(deleted)
        })
    }
        request: HistorianPublishRequest<'_>,
    ) -> Result<HistorianPublishResult, HistorianPublishError> {
        let session_id = request.session_id;
        let expected_row_version = request.expected_row_version;
        let predicate = request.predicate;
        let outcome = self.inner.with_conn_fenced(|tx| {
            let row = tx
                .query_row(
                    "SELECT row_version, meta FROM mc_cache_state WHERE session_id = ?1",
                    params![session_id],
                    |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)),
                )
                .optional()?;

            let Some((current, meta_json)) = row else {
                return Ok(PublishTxnOutcome::InvalidState("missing".to_string()));
            };

            let cas_ok = match expected_row_version {
                Some(v) => current == v as i64,
                None => current == NO_ROW,
            };
            if !cas_ok {
                return Ok(PublishTxnOutcome::CasConflict {
                    found: current.max(0) as u64,
                    reason: None,
                });
            }

            let mut meta: ModuleMeta = match serde_json::from_str(&meta_json) {
                Ok(meta) => meta,
                Err(e) => return Ok(PublishTxnOutcome::Serde(e.to_string())),
            };

            if !matches!(
                meta.historian.state,
                HistorianPhase::Publishing | HistorianPhase::AwaitingProducer
            ) {
                return Ok(PublishTxnOutcome::InvalidState(
                    meta.historian.state.as_str().to_string(),
                ));
            }

            let predicate_matches = meta.historian.firing_seq == predicate.firing_seq
                && meta.historian.producer_run_id.as_deref()
                    == Some(predicate.producer_run_id.as_str())
                && meta.historian.chunk_fingerprint == predicate.chunk_fingerprint
                && meta.historian.selected_range_identities == predicate.selected_range_identities
                && meta.historian.compartment_set_generation
                    == predicate.compartment_set_generation;
            if !predicate_matches {
                return Ok(PublishTxnOutcome::StateMismatch(Box::new(meta.historian)));
            }

            // `chunk_fingerprint` remains a readable structural diagnostic; exact
            // content freshness is verified using the durable block identities. An empty
            // vector means the firing predates selected-range identity persistence, so it
            // cannot establish that the selected content is still current.
            if predicate.selected_range_identities.is_empty() {
                return Ok(PublishTxnOutcome::FenceRejected(
                    "historian firing has no selected-range content identities".to_string(),
                ));
            }
            if let Some(changed) = predicate.selected_range_identities.iter().find(|selected| {
                meta.block_identity_by_mid.get(&selected.mid) != Some(&selected.block_identities)
            }) {
                return Ok(PublishTxnOutcome::FenceRejected(format!(
                    "selected historian message {} changed after firing",
                    changed.mid
                )));
            }

            if meta.revert_epoch != request.expected_revert_epoch {
                return Ok(PublishTxnOutcome::CasConflict {
                    found: current.max(0) as u64,
                    reason: Some(
                        "revert epoch mismatch (session was re-cut mid-firing)".to_string(),
                    ),
                });
            }

            let current_compartment_set_generation = tx.query_row(
                "SELECT COALESCE(MAX(sequence), 0), COUNT(*)
                 FROM mc_compartments WHERE session_id = ?1",
                params![session_id],
                |row| {
                    Ok(CompartmentSetGeneration {
                        max_sequence: row.get(0)?,
                        count: row.get(1)?,
                    })
                },
            )?;
            if current_compartment_set_generation != predicate.compartment_set_generation {
                return Ok(PublishTxnOutcome::FenceRejected(format!(
                    "compartment set changed after firing (expected max sequence {} with {} rows, found {} with {} rows)",
                    predicate.compartment_set_generation.max_sequence,
                    predicate.compartment_set_generation.count,
                    current_compartment_set_generation.max_sequence,
                    current_compartment_set_generation.count,
                )));
            }

            let first_appended_sequence = next_compartment_sequence_tx(tx, session_id)?;
            match append_compartments_tx(tx, session_id, request.compartments)? {
                AppendCompartmentsTxnOutcome::Appended => {}
                AppendCompartmentsTxnOutcome::Overlap {
                    existing_sequence,
                    incoming_start_message,
                    incoming_end_message,
                } => {
                    return Ok(PublishTxnOutcome::CompartmentOverlap {
                        existing_sequence,
                        incoming_start_message,
                        incoming_end_message,
                    });
                }
            }
            if request.chunk_transcript.is_some() || request.raw_chunk_messages.is_some() {
                insert_chunk_transcripts_tx(
                    tx,
                    session_id,
                    first_appended_sequence,
                    request.compartments,
                    request.chunk_transcript,
                    request.raw_chunk_messages,
                )?;
            }
            let promoted_refs = if request.promote_facts {
                promote_facts_tx(tx, request.project_path, request.facts)?
            } else {
                Vec::new()
    )?;
    Ok(())
}

fn append_compartments_tx(
    tx: &rusqlite::Transaction<'_>,
    session_id: &str,
    compartments: &[StoredCompartment],
) -> rusqlite::Result<AppendCompartmentsTxnOutcome> {
    if compartments.is_empty() {
        return Ok(AppendCompartmentsTxnOutcome::Appended);
    }

    let next_sequence = next_compartment_sequence_tx(tx, session_id)?;
    let mut statement = tx.prepare(
        "SELECT sequence, start_message, end_message
         FROM mc_compartments WHERE session_id = ?1",
    )?;
    let mut ranges = statement
        .query_map(params![session_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    drop(statement);

    // Validate the whole append before writing its first row. This keeps a rejected
    // batch atomic and makes ordinal-overlap corruption impossible even if a caller
    // bypassed the historian's optimistic publish fence.
    for (index, compartment) in compartments.iter().enumerate() {
        if let Some((existing_sequence, _, _)) = ranges.iter().find(|(_, start, end)| {
            compartment.start_message <= *end && *start <= compartment.end_message
        }) {
            return Ok(AppendCompartmentsTxnOutcome::Overlap {
                existing_sequence: *existing_sequence,
                incoming_start_message: compartment.start_message,
                incoming_end_message: compartment.end_message,
            });
        }
        ranges.push((
            next_sequence + index as i64,
            compartment.start_message,
            compartment.end_message,
        ));
    }

    for (index, compartment) in compartments.iter().enumerate() {
        insert_compartment_tx(tx, session_id, next_sequence + index as i64, compartment)?;
