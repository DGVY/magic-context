            recent_decision: self.recent_decision.clone(),
            completion_now_ms: crate::now_ms,
            publication_fence: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssembleHistorianFiringOutcome {
    Fire(Box<AssembledHistorianFiring>),
    NoFire(HistorianNoFireReason),
}

pub fn assemble_historian_firing(
    store: &McStore,
    messages: &[CkIngressMessage],
    live: &[FlatBlock],
    block_identities_by_mid: &BTreeMap<String, Vec<BlockIdentity>>,
    config: HistorianAssemblerConfig,
    now_ms: i64,
) -> Result<AssembleHistorianFiringOutcome, mc_store::McStoreError> {
    if config.model_chain.is_empty() {
        return Ok(AssembleHistorianFiringOutcome::NoFire(
            HistorianNoFireReason::NoModels,
        ));
    }
    let snapshot = store.load_historian_assembly_snapshot(&config.session_id)?;
    let compartments = snapshot.compartments;
    let expected_revert_epoch = snapshot.revert_epoch;
    let compartment_set_generation = snapshot.compartment_set_generation;
    let eligible_end = config.boundary.eligible_head.end;
    let chunk_start =
        if let Some(last_end) = compartments.iter().map(|c| c.end_message as u64).max() {
            let Some(next_present) = messages
                .iter()
                .filter(|message| !message.ck.meta.synthetic)
                .map(|message| message.ordinal)
                .filter(|ordinal| *ordinal > last_end && *ordinal < eligible_end)
                .min()
            else {
                return Ok(AssembleHistorianFiringOutcome::NoFire(
                    HistorianNoFireReason::EmptyChunk,
                ));
            };
            next_present
        } else {
            let Some(first_live_eligible) = messages
                .iter()
                .filter(|message| !message.ck.meta.synthetic)
                .filter(|message| message.ck.role != "system")
                .map(|message| message.ordinal)
                .filter(|ordinal| *ordinal < eligible_end)
                .min()
            else {
                return Ok(AssembleHistorianFiringOutcome::NoFire(
                    HistorianNoFireReason::EmptyChunk,
                ));
            };
            first_live_eligible
        };
    if chunk_start >= eligible_end {
        return Ok(AssembleHistorianFiringOutcome::NoFire(
            HistorianNoFireReason::EmptyEligibleRange {
                start_ordinal: chunk_start,
                eligible_end_ordinal: eligible_end,
            },
        ));
    }
    let chunk = build_historian_chunk(
        messages,
        live,
        chunk_start,
        config.token_budget,
        eligible_end,
    );
    if chunk.text.is_empty() || chunk.chunk.lines.is_empty() {
        // An empty producer input is not necessarily an empty read. Persist only
        // complete observed ranges so absent raw messages cannot be declared noise.
        let rows: Vec<_> = messages
            .iter()
            .filter(|m| {
                !m.ck.meta.synthetic && m.ordinal >= chunk_start && m.ordinal < eligible_end
            })
            .collect();
        if chunk.text.is_empty()
            && chunk.chunk.lines.is_empty()
            && rows.len() as u64 == eligible_end - chunk_start
            && rows.iter().enumerate().all(|(i, m)| {
                m.ordinal == chunk_start + i as u64
                    && live.iter().filter(|b| b.mid == m.mid).count() == m.ck.content.len()
            })
        {
            let endpoint = |m: &CkIngressMessage| {
                live.iter()
                    .rev()
                    .find(|b| b.ordinal == m.ordinal)
                    .map(|b| b.id.clone())
                    .unwrap_or_else(|| m.mid.clone())
            };
            let marker = StoredCompartment {
                start_message: chunk_start as i64,
