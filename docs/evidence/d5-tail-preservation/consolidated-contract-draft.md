---
title: "D5 tail preservation: seal → reply → redeem contract (MC module + store; shared wire contract with the Claude Code gateway)"
date: 2026-09-12
status: draft
# rigor_proposed: r2
---

## intent

The executed D5 defect proof shows a successor with real history ending at ordinal 1798, an empty continuation boundary at 1940, and no automatic conversation recovery for the nonempty predecessor tail 1799–1939. Before a destructive placeholder can replace the native transcript, every predecessor tail block must be either covered by validated durable history or preserved as a durable, reduction-respecting projection that the successor serves and can expand. Magic Context owns projection, archival preservation, transactional fencing, successor custody and recovery; the Claude Code gateway owns native capture, positional MID allocation, ingress admission, budget evidence and the placeholder send gate. The predecessor's in-flight historian is not the preservation carrier, and its epoch rejection remains intact. Explicitly saved memory and copied notes can survive independently, so the defect is a conversation-continuity hole, not proof that every fact or token in the tail disappeared.

## constraints

1. **Vocabulary.** P is the predecessor key, S its successor and T the ordered uncovered tail. Source means gateway-supplied transport-normalized native blocks, excluding recognized compaction additions by provenance. A is MC's durable applied state: `red:*`, skeleton, strip and caveman units, tags, pending drops and ledger. V is `project(source, A)` for every manifest block, including never-served blocks. Source authorizes material; A authorizes permissions. Gateway clauses are target semantics, not claims about existing APIs. MUST denotes a requirement and MAY an option within it.

2. **Preservation boundary.** No destructive placeholder without matching SEALED and durable send permission; no successor mint without that receipt and intact fence. Every unfolded block is represented in carry under A. An empty lineage boundary proves no fold or transcript coverage. After seal, recovery uses the receipt, archive and durable attempt identity, never re-prepare from discarded source. P's in-flight publisher is not a carrier. Fenced P blocks normal requests; it neither replays V nor ignores appended input. Raw forwarding and transformed-body replay are forbidden.

3. **Wire types.** `schema_version: 1` is a new shared contract, not today's wire. Signatures are exact at field and variant level; transport framing may wrap them. Ordered arrays, explicit empty values and unknown required semantics must not be lost. `Option<T>` is none or some(T), never a zero sentinel. ID and key types are opaque distinct identities; ordinal, position, index, generation, version, sequence, token and byte types are nonnegative checked integers. Digest is SHA256 over canonical versioned bytes. BootId identifies a boot, not envelope validity after restart.

4. **Common wire identities and evidence.** Request types:

   ```text
   MaterialFingerprint = {
     digest: Digest,
     normalization_version: integer,
     excluded_additions: ordered list<ProvenanceTag>
   }
   AttemptKey = {
     predecessor_key: SessionKey, agent: AgentId,
     F: MaterialFingerprint, attempt_id: AttemptId,
     incarnation: Incarnation
   }
   AdmissionTicket = {
     resolve_generation: ResolveGeneration,
     P: SessionKey, agent: AgentId, incarnation: Incarnation
   }
   IngressEvidence = {
     boot_id: BootId, watermark: IngressSequence,
     sequence_seen: IngressSequence, ownership_id: opaque identity
   }
   NativeMessage = {
     position: MessagePosition, ordinal: Ordinal, mid: Mid,
     role: Role, blocks: ordered list<NativeBlock>
   }
   NativeBlock = {
     index: BlockIndex, kind: BlockKind, bytes: bytes,
     provenance: ProvenanceTag, tool_links: explicit tool-arc metadata
   }
   SourceSegment = {
     normalization_version: integer,
     messages: ordered list<NativeMessage>,
     excluded_additions: ordered list<ProvenanceTag>
   }
   NormalProjectionIdentities = {
     model: ModelId, profile: ProfileId,
     tool_surface: Digest, guidance_surface: Digest,
     system_surface: Digest
   }
   ```

   Before dispatch every message has a gateway MID and separate block indexes. Preserve order, grouping, roles and tool links. F binds normalized source and exclusions, not retry markers. NormalProjectionIdentities describe MC's intended successor normal surface, never summary max_tokens or summary tools.

5. **Exact operation signatures.** The contract exposes the following MC operations, with the request fields defined in this document:

   ```text
   prepare(
     P: SessionKey, agent: AgentId, F: MaterialFingerprint,
     source_segment: SourceSegment, attempt_id: AttemptId,
     incarnation: Incarnation, admission_ticket: AdmissionTicket,
     ingress: IngressEvidence, budget: PrepareBudgetEvidence
   ) -> PrepareResult

   attempt.resolve(
     P: SessionKey, agent: AgentId, F: MaterialFingerprint,
     attempt_id: AttemptId, incarnation: Incarnation,
     admission_ticket: AdmissionTicket
   ) -> ResolveResult

   redeem(
     edge: LineageEdge, receipt_id: ReceiptId,
     incarnation: Incarnation
   ) -> RedeemResult

   release(
     receipt_id: ReceiptId, incarnation: Incarnation,
     never_send_proof: NeverSendProof
   ) -> ReleaseResult

   cancel(
     receipt_id: ReceiptId, incarnation: Incarnation,
     assertion: CancelAssertion
   ) -> CancelResult
   ```

   MC checks consistent attempt, ticket, ingress, source, envelope lineage and current incarnation. Retries retain identity. Old incarnations refuse; transport timeouts remain UNKNOWN.

6. **Typed results and terminal replay.** The result algebra is:

   ```text
   PFence = none
          | SEALED { receipt_id: ReceiptId }
          | REDEEMED { successor_key: SessionKey }
   FenceSnapshot = {
     p_fence: PFence, fence_generation: FenceGeneration
   }
   Refusal = {
     reason: RefusalReason,
     receipt_id: Option<ReceiptId>,
     details: typed reason-specific evidence
   }
   AttemptOutcome = SEALED { receipt: ReceiptV1 }
                  | REFUSED { refusal: Refusal, negative: NegativeProof }
                  | REDEEMED {
                      receipt_id: ReceiptId, successor_key: SessionKey
                    }
                  | RELEASED { receipt_id: ReceiptId }
   PrepareResult = {
     attempt_outcome: AttemptOutcome,
     p_fence: PFence, fence_generation: FenceGeneration
   }
   ResolveResult = {
     attempt_outcome: AttemptOutcome,
     p_fence: PFence, fence_generation: FenceGeneration,
     resolve_generation: ResolveGeneration
   }
   RedeemResult = REDEEMED {
                    receipt_id: ReceiptId, successor_key: SessionKey,
                    existing: boolean, fence_generation: FenceGeneration
                  }
                | REFUSED { refusal: Refusal }
                | lineage_corrupt { receipt_id: ReceiptId }
   ReleaseResult = RELEASED {
                     receipt_id: ReceiptId, already_released: boolean,
                     fence_generation: FenceGeneration
                   }
                 | REFUSED { refusal: Refusal }
   CancelResult = RECORDED {
                    receipt_id: ReceiptId, receipt_state: ReceiptState
                  }
                | REFUSED { refusal: Refusal }
   ReceiptState = SEALED | REFUSED | REDEEMED | RELEASED
   NegativeProof = tombstone {
                     attempt_id: AttemptId, incarnation: Incarnation
                   }
                 | generation_fence {
                     incarnation: Incarnation,
                     invalidated_ticket_generation: ResolveGeneration,
                     current_resolve_generation: ResolveGeneration
                   }
   ```

   Fresh prepare success is SEALED; retries may replay terminal outcomes. `existing` and `already_released` mark idempotent success, never new send permission. Negative proof is mandatory; its generation-only encoding remains open. Neither absent nor PREPARING grants admission.

7. **Refusals.** Required reasons are `budget_unknown`, `budget_model_mismatch`, `budget_evidence_mismatch`, `p_already_sealed`, `seal_material_mismatch`, `resolved_absent`, `seal_after_tombstone`, `seal_after_resolve`, `attempt_quota`, `sealed_unredeemed` and `lineage_corrupt`. Stale incarnation, freshness conflict, deadline, token cap, byte cap and invalid terminal operation are typed failures too. Budget refusals name unknown fields or failed caps with numbers. REFUSED describes its attempt, not P's global fence. A diagnostic winning receipt ID never authorizes different material. Refusal cannot enter lineage_protocol_passthrough; following ordinary input requires current-fence admission and normal transform.

8. **Gateway attempt record before dispatch.** Before dispatching prepare, the gateway MUST durably persist the attempt identity, ingress ownership, positional MID allocations or their durable references, F, P, agent, incarnation and the admission ticket. The record has `receipt_id: none` until a response or authoritative resolution binds it, and `MAY_HAVE_REPLIED: false` initially. Persistence only before the first reply byte is insufficient: a crash after MC seals but before a gateway attempt row would otherwise misclassify P. If durable attempt creation fails, no prepare is dispatched and no placeholder is emitted.

   A target record shape is:

   ```text
   GatewayAttempt = {
     attempt_id: AttemptId, P: SessionKey, agent: AgentId,
     lineage_id: LineageId, incarnation: Incarnation,
     F: MaterialFingerprint, admission_ticket: AdmissionTicket,
     ingress: IngressEvidence, started_at: Timestamp,
     receipt_id: Option<ReceiptId>, aliases: set<AttemptId>,
     outcome: UNKNOWN | AttemptOutcome,
     MAY_HAVE_REPLIED: boolean,
     send_state: OPEN | RELEASE_INTENT | NEVER_SEND,
     retries: durable set<RetryIdentity>,
     allocation_ref: positional MID allocation identity
   }
   ```

   Before writing any placeholder byte the gateway durably flips the monotonic MAY_HAVE_REPLIED bit and acquires a send right serialized with terminal revocation. The bit means possible delivery, not confirmed delivery; a crash between the flip and the first byte is conservatively treated as possibly delivered. Recovery uses this record and MC's current active receipt, never the presence of F in the successor's native transcript. Every retry and alias participates in the same delivery authority.

9. **Tickets and generations.** The gateway acquires `{resolve_generation, P, agent, incarnation}` from MC and persists it BEFORE prepare dispatch. MC checks it at admission and seal CAS. A delayed handler cannot read a fresh generation to escape an earlier resolve. Same-attempt retries NEVER refresh tickets; a genuinely new attempt gets a new ID and ticket. resolve_generation closes potential seals; fence_generation protects actual ordinary admission. Neither is a timeout or delivery counter. Incarnation fencing across deletion and recreation prevents numeric counter reuse from making an old ticket current.

10. **Ingress.** Before prepare, durable exclusive per-session and per-agent ingress ownership establishes a boot and sequence watermark. Lower or equal requests, including their MC mutations, drain or abort before dispatch; abort returns failure. Higher requests queue at most D or receive 503 without losing appended input. Post-resolution in-memory cohort tracking is not the fence. SEALED blocks normal P; REDEEMED routes to current S; RELEASED resumes P subject to fresh admission. Restart queries P's active receipt even with no local attempt row. Admission either holds exclusive ownership through resolve and admit, or rechecks fence_generation at the actual admission step. A cached none fence is never standing permission.

11. **Projection and identity.** Before seal, project every block under A: existing units supply placeholders, strips, skeletons or caveman forms; other blocks use ordinary defer serialization. Never process the summary instruction as a turn, create reductions or bind deferred commands at seal. Never-served blocks still have defined V. Gateway MID allocation is positional `(attempt_id, P, ordered native MESSAGE position)`; block index is separate. Identical content at distinct positions gets distinct MIDs; retries reuse allocations. MC reserves deterministic ordered tag identities from `(mid,index)` with A and the seal CAS, not a separate allocating transaction. Identity-only allocation commits iff the receipt does.

12. **Archive authority.** Archive V, ordered manifest and A are durable and content-addressed as `sha256(manifest || V || A)` under canonical versioned encoding. Source hashes identify material without retaining dropped raw bodies: A wins, and the archive stores no alternate dropped-body recovery source. Preserve all block kinds, grouping, roles, order and tool arcs, not just tagged text. Projection has no clock, host or unrelated row-version input. Incomplete or nondeterministic encoding refuses. Staging is not SEALED; external staging, if used, requires orphan cleanup and atomic durable reachability from the committed receipt before send permission.

13. **Atomic seal and freshness.** All awaits, projection, estimation, optional grace and encoding precede one synchronous transaction with no wait inside. It commits archive references, SEALED receipt, identities, P.sealed_for, revert_epoch bump and fence-generation change atomically. CAS checks incarnation, pre-dispatch ticket resolve_generation, tombstone absence, P.state_version, A hash, coverage frontiers, pending-drop count and every projection dependency. Concurrent publisher or command changes require recompute and rebudget within D or refusal. A stale SQLite snapshot retries the WHOLE read and CAS, not only the write. A caller timeout near commit remains UNKNOWN; a pre-transaction expiry check cannot prove no late commit.

14. **Prepare identity.** Concurrent active same-(P,agent,F) prepares may compute different candidates, but the P-state CAS chooses one receipt; every coalesced observer gets the winning manifest and a durable alias. Refused attempts never revive by ticket refresh. If F1 is SEALED, F2 returns `REFUSED{p_already_sealed, receipt_id:R1}` without changing R1 or treating it as F2's seal; losing input remains queued or refused. Pin both winner orderings. After RELEASED a new attempt with the same F may create a new receipt and reuse an identical content-addressed blob. New A and attempt-scoped identities need not equal historical candidates.

15. **Attempt resolution, including negative-row saturation.** `attempt.resolve` executes as one durable store transaction. For an existing attempt or alias it returns SEALED, REFUSED, REDEEMED with the successor key, or RELEASED, together with P's current fence and fence_generation from that transaction. With no attempt receipt it establishes an authoritative no-future-seal guarantee. When capacity permits, it writes `REFUSED{reason: resolved_absent, attempt_id}` as an incarnation-scoped tombstone. Seal checks that tombstone in its own transaction and fails `seal_after_tombstone` if resolution won.

    When the negative-row bound is saturated by current-incarnation rows, resolution still closes the attempt: it CAS-bumps the session's resolve_generation in the same transaction, invalidating the ticket carried by the old prepare even if no new tombstone row can be written. The result carries logical REFUSED and generation-fence evidence rather than failing open. A prepare delayed before handler arrival must fail admission or seal with `seal_after_resolve` on its original ticket. A new attempt with a newly acquired ticket may proceed. Tombstones support per-attempt reporting; they are not the sole safety fence at capacity. Existing SEALED or REDEEMED outcomes remain explicit and are not erased by a generation bump.

16. **UNKNOWN and current fence.** Timeout, elapsed D, missing gateway row, read-only MC absence and false delivery bit do not prove P unfenced. Next touch resolves authoritatively. A tombstone closes only its attempt; another attempt may hold SEALED or REDEEMED, which p_fence must report separately. Seal-first resolution returns existing state; resolve-first blocks the old seal. Writer serialization or failed snapshot upgrade forces a full CAS retry. Before resolution, physical absence and gateway UNKNOWN are legitimate transients, not successful refusal or observable PREPARING permission. Network loss during resolve keeps the barrier.

17. **Redeem edge and atomic successor mint.** `LineageEdge` contains its stable edge ID, predecessor key, agent, F, lineage ID and the declared native continuation identity. Redeem verifies the edge material against the receipt and requires SEALED plus intact MC fences. A mismatch returns `seal_material_mismatch` without deleting the archive or discarding queued material. First redeem binds the edge once and atomically mints S, copies the real compartments and ordinary chunk transcripts, preserves tags and applicable A, installs the inherited archive membership, transfers owned receipt and archive references, transfers carried pending commands, establishes the separate frontiers and identities, and marks the receipt REDEEMED.

    A crash before commit leaves SEALED and no partial successor; a crash after commit leaves a complete successor and REDEEMED. A retry of a committed redeem returns the existing successor key, with no second lineage row, edge binding, tag mint or MID remint. If a REDEEMED receipt references a missing successor, MC returns `lineage_corrupt{receipt_id}`, exposes corruption in health and refuses to remint. REFUSED and RELEASED are not redeemable. The successor need not recover anything from the harness's discarded source or a predecessor in-memory snapshot.

18. **Terminal send revocation.** NeverSendProof binds receipt, incarnation, every alias and retry, and durable revocation identity. Under the same authority as reply rights, retry creation and bit flips, gateway transitions RELEASE_INTENT to NEVER_SEND only with all MAY_HAVE_REPLIED bits false. Any true bit forbids safe release despite socket failure. Revoke before remote release; cached SEALED workers recheck immediately before acquiring send rights. No registration or bit flip follows revocation. Crash after revocation leaves retained SEALED and a retryable release, not permission to send. A later legitimate prepare uses a new attempt and ticket.

19. **Release and cancel.** MC release CASes SEALED to RELEASED against the same row as redeem: exactly one wins. Repeat RELEASED returns idempotent success; other states refuse. In that transaction merge every frozen pending and post-seal command back to P with provenance, ledger and `(command_id,target)` dedup, then clear the admission fence atomically. Never roll the epoch backward to admit old publishers. CancelAssertion records cancelled_before_delivery as a hint only; it neither releases fences, starts retention clocks nor removes archives. Cancel cannot undo a terminal state. Without valid release, SEALED remains possibly delivered and retained.

20. **Lifecycle and terminal custody.** The durable lifecycle is:

    ```text
    absent --prepare commit--> SEALED --redeem commit--> REDEEMED
       |                         |
       |                         +--release commit--> RELEASED
       +--refusal or resolve--> REFUSED (tombstone or generation-backed negative)
    ```

    REFUSED has no newly committed archive or admission fence for that attempt. SEALED has archive and both fences; REDEEMED has a complete successor and transferred custody; RELEASED has P command custody restored and a positive release timestamp. Terminal attempts do not transition back to SEALED; a later legitimate attempt is a new identity. At the negative cap, physical absence of a receipt row can coexist with a logical REFUSED outcome protected by a generation fence. This is not an unfenced absent attempt. Cancel does not appear as a state edge. Explicit forced deletion is a separately authorized destructive exception, not a fifth successful preservation state.

21. **Crash and publication ordering matrix.** Each state-relevant cut has the following required result. Independent events commute only if they cannot change source, A, fence, command custody, edge binding or send authority.

    | Named ordering | Required resolution |
    |---|---|
    | Crash during pre-seal projection or staging | No committed archive or fence; gateway stays UNKNOWN until resolve closes or finds the attempt. |
    | Crash after archive writes but before seal transaction commit | All seal rows roll back together; no placeholder permission. |
    | Seal commits, reply to prepare is lost | SEALED archive and fences survive; resolve returns the receipt. |
    | Gateway crash before attempt persistence | Prepare was not dispatched; recovery still checks P's active fence before normal admission. |
    | Gateway crash after prepare dispatch | Durable identity and ticket exist; resolve rather than infer refusal. |
    | SEALED response received, crash before delivery bit | Retain SEALED; release requires terminal exclusion of every future reply writer. |
    | Delivery bit persisted, crash before or during bytes | Possibly delivered; never release on a socket-based guess. |
    | Placeholder delivered, crash before redeem | Retain SEALED; next matching edge redeems from archive. |
    | Redeem commit followed by lost response | Retry returns the same S; no remint. |
    | P publisher commits before seal CAS | Changed A or coverage causes recompute and rebudget or refusal within D. |
    | P publisher assembled before seal lands after seal | Existing revert_epoch guard rejects before compartment or transcript append. |
    | Seal CAS read races resolver write | Only one serialized commit wins; losing snapshot retries the whole CAS. |
    | Old prepare arrives after capped resolve generation bump | Old pre-dispatch ticket refuses; it cannot read a fresh generation to escape closure. |
    | Resolve reads no fence, concurrent prepare seals before ordinary admission | Admission generation consumption rejects stale permission. |
    | Old REFUSED alias resolves while another attempt sealed P | Return both the old outcome and the current SEALED fence; no ordinary P admission. |
    | Same-F prepares overlap | One receipt; all observed coalesced attempts become durable aliases. |
    | Different-F prepares overlap | Winning material seals; other material receives p_already_sealed and stays queued or refused. |
    | Release competes with reply permission | Terminal NEVER_SEND or possible-send authority wins atomically; both cannot succeed. |
    | Release competes with redeem | Exactly one receipt CAS wins; loser observes terminal state without changing custody. |
    | Cancel occurs without release | No state discharge, fence removal or archive clock. |
    | Delete and recreate races a delayed operation | Old incarnation refuses independently of tombstone presence. |

22. **Commands at every cut.** Freeze pending T drops in A with provenance. After seal, one transactional lineage lookup routes SEALED targets to the receipt, REDEEMED targets to the CURRENT descendant queue if carried or ledger covered if folded or retired if reduced, and RELEASED targets to P. Dedup by `(command_id,target)`. Races with redeem or release either join the transferred queue or see the new owner; no copy-delivery gap. Protection and queued-until-aged rules persist on S. Targets folded before seal receive covered, not fake successful reduction. Banked migration #2732 enabling ledger partial and covered dispositions is a hard prerequisite.

23. **Every-pass carry.** Serve outstanding carry after m0 and m1 and before continuation on every forwarded pass: initial HARD, later DEFER, growth and restart. Only validated folds or legitimate block reductions retire it. Partial fold retires only its interval and causes normal SOFT; remaining carry persists. Inherited blocks participate before HARD unit-retention and pruning, not as an output-only splice. Absent decisions, ordered membership and bytes stay identical. New native suffix input is preserved independently. Neither an archive hash nor an abandoned P publisher authorizes ignoring current applied state.

24. **Digest classes.** archive_id and manifest_digest identify immutable snapshots. Every unreduced block on every pass must match frozen served length and sha256; unrelated row-version, tag, host or serializer changes are no authorization. Check exact ordered membership, including missing and duplicate members. Legitimately changed blocks validate against committed unit bytes and `projection_digest{unit,row_version}` for that pass, not the old frozen hash. Bind the ordered unit set to committed state: a new digest alone cannot authorize reduction. Expansion and rerender use current A and never resurrect archived original bodies.

25. **Frontiers and identities on S.** S stores separate fields:

    ```text
    SuccessorFrontiers = {
      folded_frontier: Option<Ordinal>,
      first_inherited_ordinal: Option<Ordinal>,
      source_frontier: Option<Ordinal>,
      lineage_anchor: Ordinal,
      coverage_identity: Option<BlockIdentity>,
      continuation_identity: BlockIdentity
    }
    BlockIdentity = { mid: Mid, index: BlockIndex, ordinal: Ordinal }
    ```

    `folded_frontier` is the end of the last real compartment, never the empty lineage boundary. `source_frontier` is the end of the preserved carried source; `first_inherited_ordinal` is its first manifest ordinal when carry exists. `lineage_anchor` is the native continuation coordinate, kept separate from the offset used to rebase subsequent ordinals. For the specimen, folded is 1798, inherited starts 1799, source ends 1939, the native anchor is 1940, and the continuation base used by the native ordinal mapping is 1939. Names and wire fields must not conflate that base with the anchor or with actual coverage.

    `coverage_identity` is the last real compartment's endpoint, such as `mid1798#index`; `continuation_identity` is the native anchor at 1940. Coverage identity advances with valid folds. Continuation identity never moves after redeem, including a crossing fold. The initial empty boundary row advances only continuation metadata. An optional field with no real fold is absent, not ordinal zero. Ordinal zero is a real Claude Code ordinal.

26. **Eligibility.** With a real fold, eligible head is folded_frontier plus one using checked arithmetic. Without one and with carry, use first_inherited_ordinal; do not fabricate zero or trigger continued_ordinal_offset_missing. Ordinal zero is real and differs from absence. Historian assembly reads inherited archive messages plus genuinely new native input; merely lowering MAX(end) while selecting only native request messages still loses carry. The specimen's first trigger exposes positive inherited eligible tokens without changing scheduling floors.

27. **Boundary proofs.** Coverage derivation, m0, m1, historian and expansion use coverage_identity and folded_frontier. Native rebase, host marker, reconcile and trim use continuation_identity. Live guards prove continuation against native input and coverage against receipt manifest or real compartment. Never substitute folded1798 numerically beside unexplained native mid1940, globally bypass mint or revert guards, or truncate inherited history because its folded endpoint is absent natively. resolve_boundary_state separates coverage from declared native trim. Non-D5 native proof paths stay unchanged.

28. **Absorb crossing anchors.** Filter `episode_type='lineage_boundary'` before coverage range and sequence validation; exempt only that D5 non-covering type from publication overlap. Partial fold1799–1905 leaves anchor1940 and remaining carry. Crossing fold1799–1950 removes the anchor row and preserves continuation_identity and native anchor MID on the new compartment in the SAME publish transaction. Continuation never moves; real coverage advances. Assembly snapshots, generation checks and append sequence must agree on this layout. Repeated and nested folds preserve metadata without phantom coverage or duplicate anchors. Epoch rejection remains independent and intact.

29. **Expansion.** Inherited membership selects a new ctx_expand arm BEFORE same-key cache or raw transcript fallback. Initially return archive V; after successor decisions return its currently permitted reduced form, never original dropped bodies. Existing non-inherited chunk semantics stay unchanged. Range mode resolves carry before last_compacted cutoff and merges ordered archive and ordinary transcript ranges with existing bounds and dedup. Restart must work from durable custody. Validated folds keep appropriate covered expansion available without resurrection. Tags, ancestor caches and transcript-file paths are not implementations of this archive resolver.

30. **Nested carry.** prepare on S with outstanding carry obtains it from S's own archive: gateway native source excludes injected carry. New T is the ordered union with genuinely new uncovered S-native tail, bounded by the existing merge-collapse cap and budgeted in full. Dedup by originating receipt and block identity, never content hash; equal bytes at different positions survive, duplicate references to one origin collapse. Apply CURRENT S A, re-project inherited blocks and re-mint frozen hashes at the new seal. Preserve origin provenance through collapse with self-contained data or protecting references, so ancestor deletion cannot break S2 expansion.

31. **Receipt schema.** A positive receipt and its associated state use the following versioned record. A negative receipt carries the same attempt, incarnation and reporting identity but no invented archive or positive manifest; state-dependent optional fields are absent rather than zero-filled.

    ```text
    ReceiptV1 = {
      schema_version: 1,
      receipt_id: ReceiptId, attempt_id: AttemptId,
      predecessor_key: SessionKey, successor_key: Option<SessionKey>,
      session_id: SessionId, lineage_id: LineageId, owner_key: SessionKey,
      agent: AgentId, incarnation: Incarnation,
      aliases: ordered set<AttemptId>, F: MaterialFingerprint,
      admission_ticket: AdmissionTicket,
      sealed_state: Option<{
        P_state_version: StateVersion, projection_key: opaque identity,
        applied_state_hash: Digest,
        epoch_before: integer, epoch_after: integer,
        fence_generation: FenceGeneration,
        resolve_generation: ResolveGeneration
      }>,
      ingress: IngressEvidence,
      frontiers: Option<SuccessorFrontiers>,
      representation: carry | refused,
      budget: BudgetRecord,
      manifest: Option<ManifestV1>,
      refs: Option<{
        archive_id: ArchiveId, manifest_digest: Digest,
        applied_state_hash: Digest
      }>,
      pending_drops: ordered list<CommandTarget>,
      post_seal_drops: ordered list<CommandTarget>,
      state: ReceiptState, refusal: Option<Refusal>,
      edge: Option<LineageEdge>,
      sealed_at: Option<Timestamp>, released_at: Option<Timestamp>,
      cancelled_before_delivery: Option<CancelAssertion>
    }
    CommandTarget = {
      command_id: CommandId, target: BlockIdentity,
      state: queued | pending | partial | covered | retired,
      provenance: command provenance, protection: durable protection state
    }
    ```

    Snapshot bytes are immutable; lifecycle, owner and command custody may change. Owner transfer does not change archive_id. No speculative successor_key before redeem. REFUSED diagnostics mark unmeasured fields unknown. Schema evolution preserves terminal replay and refuses incompatible semantics.

32. **Archive, manifest and response schema.** The persistent archive and per-response proof are distinct:

    ```text
    ArchiveV1 = {
      schema_version: 1, archive_id: ArchiveId,
      encoding_version: integer,
      manifest: ManifestV1, V: ordered projected message bytes,
      A: AppliedStateSnapshot
    }
    ManifestV1 = {
      schema_version: 1, normalization_version: integer,
      encoding_version: integer,
      messages: ordered list<{
        ordinal: Ordinal, mid: Mid, native_position: Option<MessagePosition>,
        role: Role, blocks: ordered list<{
          index: BlockIndex, kind: BlockKind,
          identity: BlockIdentity,
          provenance: native {
            attempt_id: AttemptId, predecessor_key: SessionKey,
            message_position: MessagePosition
          } | inherited_from {
            receipt_id: ReceiptId, origin_identity: BlockIdentity
          },
          source: { len: ByteCount, sha256: Digest },
          served: { len: ByteCount, sha256: Digest },
          applied_unit: Option<UnitKey>, tool_links: tool-arc metadata
        }>
      }>
    }
    CarryProjectionV1 = {
      schema_version: 1,
      receipt_id: ReceiptId, archive_id: ArchiveId,
      manifest_digest: Digest, row_version: RowVersion,
      coverage_identity: Option<BlockIdentity>,
      continuation_identity: BlockIdentity,
      members: ordered list<{
        identity: BlockIdentity,
        validation: frozen { served_sha256: Digest }
                  | projection_digest {
                      sha256: Digest, unit: UnitKey, row_version: RowVersion
                    }
      }>,
      projection_digest: {
        sha256: Digest, row_version: RowVersion,
        units: ordered list<UnitKey>
      },
      coverage_proof: receipt-backed manifest or real-compartment proof
    }
    ```

    Transfer full manifest once; later passes use resolved references and ordered proofs. Missing reference is not empty membership. Unreduced members still use frozen served hashes despite aggregate digest. Provenance survives ancestor deletion. Owner-scoped references carry predecessor_key, successor_key, lineage_id, owner_key and incarnation; shared blobs have multiple owners through references.

33. **Prepare budget-evidence struct.** Numeric system and tool token splits are not demanded from the gateway. Prepare takes this explicit new evidence:

    ```text
    GeometryV1 = {
      usable_soft: TokenCount, usable_hard: TokenCount,
      absolute_wall: Option<TokenCount>, derivation: descriptive text,
      reserve_accounting: once_carved | none_declared
    }
    EnvelopeRecordV1 = {
      schema_version: 1, envelope_id: EnvelopeId,
      request_id: RequestId, request_sha256: Digest,
      captured_at: Timestamp, model: ModelId,
      system_bytes: KnownBytes, tools_bytes: KnownBytes,
      geometry: GeometryV1, profile: ProfileId,
      guidance_surface: Digest, tool_surface: Digest,
      system_surface: Digest,
      session_id: SessionId, agent: AgentId, lineage_id: LineageId,
      predecessor_key: SessionKey, incarnation: Incarnation,
      gateway_boot_id: BootId, ingress_sequence: IngressSequence,
      reserve_accounting: once_carved | none_declared
    }
    KnownBytes = present { bytes: bytes, sha256: Digest }
               | unknown { reason: text }
    EnvelopeSlot = available { record: EnvelopeRecordV1 }
                 | absent { reason: text }
                 | newer_unusable {
                     boot_id: BootId, sequence: IngressSequence, reason: text
                   }
    PrepareBudgetEvidence = {
      schema_version: 1, geometry: GeometryV1,
      envelope: EnvelopeSlot,
      intended_normal_projection: NormalProjectionIdentities
    }
    ```

    MC resolves its own model output reserve and measured fixed anchor reserve, estimates the raw segments, its m0 and m1, and the full projected carry, and records provenance. Today's TransformInput raw body and geometry do not already contain output_reserve, anchor_reserve, system_tokens or tools_tokens. The summary request's max_tokens is not the successor's output reserve. Explicit known-empty system or tools is valid evidence of an empty segment; missing evidence is not an empty segment and refuses `budget_unknown`. Conflicting duplicated reserve_accounting values are evidence mismatch, not an opportunity to pick the more permissive one.

34. **Per-source reserve accounting and independent hard test.** Let X be the real scheduler estimator over system bytes, tools bytes, m0, m1, the full projected carry and the measured continuation wrapper represented by anchor_reserve. Let R be `output_reserve_mc.tokens`. Both are known token estimates with model and estimator provenance. The structured `reserve_accounting` flag, not the human-readable derivation string, determines soft conversion:

    ```text
    soft_declared = geometry.usable_soft
    hard_declared = geometry.usable_hard
    soft_bounded = min(soft_declared, hard_declared)
    fit_soft = soft_bounded                    if once_carved
             = soft_bounded - R                if none_declared
    soft_ok = X <= fit_soft
    hard_ok = X + R <= hard_declared
    token_fit = soft_ok AND hard_ok
    ```

    Checked subtraction with R greater than soft_bounded is a typed insufficient-budget refusal, not wraparound or zero substitution. The hard test is computed independently and recorded even when soft already rejects. The clamp exists only inside the fit calculation; reported supplied geometry remains untouched. Over-hard candidates are refused with their original values, not silently rewritten into a successful request. A named additional safety margin, if separately configured, is not mislabeled as a second use of the scheduler output reserve.

    Claude Code gateway geometry carries no declared output-reserve subtraction and declares `none_declared`: its default scheduling soft value is 167,000 with hard 200,000, while context-1m models may have equal soft and hard. Existing MC OpenCode and Pi geometry carves reserve into usable_soft and declares `once_carved` where this evidence format is used. That source distinction does not activate D5 on those profiles. A 200,000 window with 20,000 reserve and already-carved 180,000 soft accepts X=170,000; adding the reserve to the soft side again would be wrong. Default 167,000 and 200,000 pins once-only accounting, not a purported fits-soft-but-not-hard counterexample that is impossible under the stated nonnegative reserve. An above-hard soft override supplies the candidate for the independent hard-rejection pin; the internal clamp and hard predicate both remain observable.

35. **Budget record and refusal diagnostics.** Each receipt's budget block records what was measured and what could not be resolved:

    ```text
    ReserveRecord = {
      model: ModelId, tokens: TokenCount,
      source: window-geometry | config, units: tokens
    }
    BudgetRecord = {
      schema_version: 1,
      geometry_wire: GeometryV1,
      reserve_accounting: once_carved | none_declared,
      output_reserve_mc: Known<ReserveRecord>,
      anchor_reserve: Known<{
        tokens: TokenCount, source: measured_mc_wrapper_constant,
        units: tokens, wrapper_sha256: Digest, measurement_version: integer
      }>,
      estimator: Known<{
        identity: text, version: text, model: ModelId, units: tokens
      }>,
      envelope_id: Option<EnvelopeId>, request_sha256: Option<Digest>,
      budget_evidence: fresh | aged { n: integer }
                     | mismatch { field: text } | unknown { reason: text },
      estimates: {
        system: Known<TokenCount>, tools: Known<TokenCount>,
        m0: Known<TokenCount>, m1: Known<TokenCount>,
        carry: Known<TokenCount>, anchor: Known<TokenCount>,
        total_input_X: Known<TokenCount>
      },
      soft_declared: Known<TokenCount>, hard_declared: Known<TokenCount>,
      soft_bounded: Known<TokenCount>, fit_soft: Known<TokenCount>,
      hard_required: Known<TokenCount>, clamp_applied: Known<boolean>,
      soft_ok: Known<boolean>, hard_ok: Known<boolean>,
      encoded_bytes: Known<{
        archive: ByteCount, carry_frame: ByteCount,
        gateway_limit: ByteCount, mc_frame_page_limit: ByteCount,
        configured_archive_limit: ByteCount
      }>,
      failed_caps: ordered list<typed cap result>,
      unknown_inputs: ordered list<field and reason>
    }
    Known<T> = known { value: T } | unknown { reason: text }
    ```

    Unknown geometry, raw segments, model reserve, estimator result, wrapper measurement, byte capacity or identity evidence causes refusal; unknown is never converted to zero, false success or an omitted cap. Output reserve records model, token units and whether it came from window-geometry or configuration. The gateway supplies evidence bytes, not diagnostic capture token guesses. MC's own m0 and m1 estimates and the projected union are authoritative for the MC contribution. The receipt keeps envelope identity and measurement provenance, not an independent MC cache of the whole normal envelope after prepare.

36. **Resource caps and specimen sizing.** Independently of tokens, encoded archive, carry frame, manifest, A and native envelope overhead must meet configured byte ceilings. Gateway ingress is 64 MiB; the smaller applicable gateway and MC transform frame or page capacity governs. Archive ceiling is explicit configuration, not a substance policy. Estimate the FULL projected specimen union including untagged tool inputs, reasoning, system, m0, m1 and anchor. The reported 83 tags and 24,073 stored tokens are fragment mass, neither full carry nor exact billed loss. A conservative soft refusal below physical hard capacity is a stated safety-liveness choice. Every successor render still passes actual outgoing token and byte gates.

37. **Envelope selection.** Retain one whole normal-turn envelope slot per session and agent, scoped to lineage and incarnation. Select the latest managed normal request committed to send, including a successfully forwarded harness retry; exclude summary, rejected and unmanaged attempts. Write-then-swap all segments and identities atomically. New model, tools or system produce the newer record, not indefinite stale evidence. Pass the record by value; MC retains its identity and estimator evidence in the receipt, not a mutable envelope cache. First-turn absence, persistence failure and declared degraded restart remain explicit unknowns.

38. **Envelope constraint one: causal order.** Select at committed-send time, not response completion. Within a boot only strictly newer ingress sequence replaces; across boots durable causal ordering, not lexical BootId ordering, establishes newer. Session, agent, lineage and incarnation must match. Older overlapping completion never overwrites. A newer forwarded request whose record cannot persist marks newer_unusable with sequence; never reuse readable older bytes as latest. If the marker itself cannot persist, recovery must conservatively invalidate the slot rather than claim durable freshness.

39. **Envelope constraint two: write-ahead persistence.** Persist the complete record before provider send. Crash between persist and send leaves valid envelope-description evidence of a possibly unsent request. Non-persisting mode is declared degradation and yields budget_unknown after restart. Ordinary BootId change does NOT invalidate durable matching-incarnation evidence. Missing segment, disk failure or unusable marker refuses honestly; diagnostic traces cannot substitute. Persist segments and identities together, never combine old tools with newer model or geometry.

40. **Envelope constraint three: normal identity.** Envelope evidence includes header-derived geometry, profile, guidance, tool and system surface hashes. Validate against intended NORMAL successor identities resolved by MC, not raw summary tools, omitted fields or max_tokens. Model mismatch refuses budget_model_mismatch; other surface mismatches refuse budget_evidence_mismatch naming the field. Matching evidence records fresh or aged{sequence distance}; age is not proof or a new TTL. Undecidable comparison refuses budget_unknown. Successful evidence never skips first-pass actual outgoing geometry.

41. **Envelope constraint four: bounded witness.** MC store.db incarnation independently detects an older-incarnation gateway restore; treat that envelope as absent-by-rollback. Foreign or inconsistent incarnation is not a match. BootId is causal ordering, not restart invalidation. Equal incarnation does NOT detect within-incarnation rollback, and joint MC-plus-gateway restore rolls back the witness too. In those explicit limits, intended normal-projection identity agreement is the remaining evidence; undecidable freshness yields UNKNOWN or refusal, never universal rollback assurance from IDs in restored databases.

42. **Envelope constraint five: refusal.** Missing usable evidence returns budget_unknown before seal, with no placeholder. Compaction has FAILED at the gateway; no provider-summary fallback exists. Calibrated AUTO resumes ordinary old-history input; manual exposes error. UNKNOWN still resolves and ordinary admission still uses current p_fence. Prepare fit is evidence for the retained envelope only. First S actual outgoing overflow cannot send raw, drop carry, undo REDEEMED or remint. Immediate refusal versus an existing authorized degradation lane remains the narrow open decision; either preserves custody and current reduction authority.

43. **Deadline and calibration citation.** D is fixed at 120 seconds as server policy, independent of fill. It bounds ingress drain, projection, archive work, optional in-flight grace of at most 20 seconds, and sealing. Work exceeding the budget before a seal is committed is refused rather than silently continuing toward a placeholder. A caller-side timeout remains UNKNOWN because the caller cannot prove whether a store commit crossed its deadline. The deadline does not authorize deleting a SEALED receipt, clearing P's fence, treating absent rows as refusal or admitting a queued ordinary request without generation checks.

    Calibration citation: CURRENT-CC native compaction 503 and delay probe, Claude Code 2.1.258, exact-version network-disabled container with an isolated fake provider, validated from raw request hashes and native boundaries; calibration report digest `25145bcfaba48dfeedc0753063494a9d484af8ca9956b8d43f4ba3b7192e7693`, sanitized manifest digest `79cc7ac955ed220fcc9b371597fd414e7c47280c4f2038fa47dad9844746a2ad`. AUTO 503, 503, 200 recovered and replaced warm-up markers. Persistent AUTO 11 refusals retained warm-ups, emitted no summary or compact boundary and resumed the exact pending ordinary request with 200; the measured follow-up interval was 0.028376 seconds. Manual persistent refusal retained history and exposed a compaction error. Manual and AUTO first responses tolerated 120.003 and 120.004 seconds before a 503 and successful retry. A 125-second success is tolerance evidence, not a measured client maximum, and there is no 150-second extrapolation. Initial host isolation limitations are not promoted to the exact-version container's evidence claim.

44. **Positive-discharge retention.** SEALED is never time-swept; possibly delivered includes lost confirmation. Expose sealed_unredeemed{age} and skip SEALED keys in last_activity GC. Cancel does not discharge. REDEEMED custody lives with S until explicit owner-lineage deletion. RELEASED alone starts 24-hour archive forensics; afterward its reference may be swept, but delete a shared blob only at zero protecting references across all states. REFUSED creates no archive, yet its no-future-seal protection has independent retention: archive absence never justifies premature tombstone deletion.

45. **Ownership and deletion.** Receipt and archive references carry predecessor_key, nullable successor_key, lineage_id, owner_key and incarnation. Redeem transfers P ownership and every protecting reference to S atomically; nested custody transfers hop by hop or creates self-contained successor references. Deleting ancestors never removes a descendant's sole archive. Unforced delete(P) while SEALED-unredeemed refuses sealed_unredeemed; forced destruction is explicit and logged. Add D5 receipt, reference, alias and negative tables to session deletion and orphan inventories, but delete custody by owner_key, not predecessor session_id. Generic deleteSessionScopedRows must not erase transferred protection. Blobs are refcounted across all states.

46. **Negative retention and quota.** Prepare, resolve and seal carry incarnation; delete and recreate advances it so delayed old operations refuse without needing old tombstones. Current-incarnation negatives remain while attempts could seal; stale-incarnation rows retire oldest-first. Per-session attempt_quota refuses non-terminal allocation before growth, and negative rows have a configured bound. At current-incarnation saturation resolve_generation still closes pre-dispatch tickets without allocating a tombstone. Aliases cannot evade bounds. Retire optimization rows only with terminal replay or immutable reconstruction preserved. Bounds never authorize sweeping SEALED or possibly delivered attempts.

47. **Gateway allocation retirement.** Reclaim attempt-ALLOCATION bookkeeping only after terminality, all aliases non-replyable, no receipt, carry or archive reference, and incarnation advance or applicable receipt-retention expiry. No TTL-only deletion of MAY_HAVE_REPLIED, UNKNOWN or non-terminal attempts. Preserve terminal replay or immutable reconstruction. Canonical MID high-water, issued IDs and identities carried across descendants are NEVER reclaimed or reused. New attempts may allocate new IDs without reopening old reply authority. Retry records retain revocation and release evidence; removing an optimization cannot authorize a cached SEALED reply.

48. **Consumer-rebind table and source citation basis.** The following inventory is the union of the independent source-checked seat A Q4 and seat B clause 4 inventories, with the panel's named `resolve_boundary_state` and generation-ordering consumers explicit. Source locations are cited as those reviews read them at MC baseline `711b097eefe1448c5dbeff93f0009d0aeecbf08d`; the executed defect proof used an older source position for the original fake-compaction fixture. Paths in the table are ordinary product-source citations, not runtime dependence on local review artifacts. `mc-store` and `mc-module` paths are under `crates/` unless written in full. Every row is an executable rebind pin, not merely a search suggestion.

    | Consumer and cited site | Required D5 authority and observable result |
    |---|---|
    | `mc-store/src/lib.rs:10916,10997–11007,11080–11095,11230–11244`, descend_lineage | Newest-live and placeholder advance continuation metadata only; real folded frontier remains 1798 in the specimen; the nonempty 1799–1939 tail has a receipt archive. |
    | `mc-module/src/compartment_coverage.rs:180–202`, resolve_coverage | Filter lineage_boundary before coverage ordering and maximum-sequence derivation; coverage end and identity come from the last real compartment. Preserve legitimate sparse non-D5 ranges. |
    | `mc-module/src/transform.rs:7296–7308`, coverage_ordinal_from_compartments | Use the D5-aware real-coverage resolver, not terminal anchor end. |
    | `mc-module/src/transform.rs:4150–4151`, system-content trigger | Derive coverage and trim decisions from real fold evidence rather than placeholder coverage. |
    | `mc-module/src/m0_compose.rs:434–449`, m0 compose | Coverage, boundary identity, first ordinal and folded sequence use real compartments; native marker authority is continuation_identity, separately represented. |
    | `mc-module/src/transform.rs:4744–4751,4753–4789,4779–4790`, system coverage and live gap | Compose true history and receipt-backed carry without declaring it folded or requiring absent predecessor endpoints in native input. |
    | `mc-module/src/transform.rs:4812–4854,4912–4917`, mint availability and reconcile | Validate coverage_identity with receipt proof and continuation_identity with native presence; do not truncate inherited compartments because a folded MID is absent natively. |
    | `mc-module/src/m1_compose.rs:313,328–344`, m1 new_coverage | Exclude anchor-only sequence; advance boundary only for real new summarized coverage. |
    | `mc-module/src/transform.rs:5261–5309`, m1 boundary movement and SOFT trim | A valid partial carry fold creates the normal SOFT change; native trim continues to use continuation identity and proof. |
    | `mc-module/src/lib.rs:5198–5260`, historian trigger | Use Option folded_frontier and first_inherited_ordinal; eligible inherited tokens are not hidden by max anchor end or continued_ordinal_offset_missing. |
    | `mc-module/src/historian_chunk.rs:626–674`, assemble_historian_firing | Select archive-backed inherited messages at the true eligible head plus live native suffix; snapshot epoch and publication generation consistently. |
    | `mc-store/src/lib.rs:12451–12495,16663–16713`, historian publication and append overlap | Exempt only lineage_boundary from covering overlap, absorb it on crossing folds, preserve anchor metadata, and keep generation and append ordering coherent. |
    | `mc-store/src/lib.rs:10445–10473`, SQL MAX(end_message) and last_compacted | Return actual folded end, excluding non-covering D5 anchor; absence remains optional. |
    | `mc-module/src/lib.rs:11907–11935`, ctx_expand message mode | Inherited archive resolver precedes cache or raw chunk fallback and returns current permitted projection. |
    | `mc-module/src/lib.rs:11952–11999`, ctx_expand range mode | Resolve inherited membership before last-compacted cutoff; merge mixed archive and ordinary transcript ranges without false no-compartments result. |
    | `mc-module/src/transform.rs:7843–7864`, first_uncovered_live_block | Use receipt-backed membership for inherited content and native membership for continuation; do not mistake absent native tail for proof of no carry. |
    | `mc-module/src/transform.rs:7866–7904`, validate_live_boundary_ordinal | Compare each identity with its own coordinate and proof; no pairing of folded ordinal and unexplained continuation MID. |
    | `mc-module/src/transform.rs:7894–7995,8002–8009`, resolve_boundary_state, declared trim and surviving revert prefix | Separate folded coverage from native declared trim and surviving native endpoint checks; preserve all mint and revert safeguards. |
    | `mc-module/src/transform.rs:6134–6140`, committed execute response | Export both coverage and native-addressable continuation marker semantics; do not advertise absent inherited MID as a host boundary. |
    | `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:682–712`, host compaction marker | Map continuation_identity to native endMessageId for D5 while preserving separate coverage; no implicit mixed ordinal and MID pair. |
    | `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:3377–3379`, host boundary presence | Assert native continuation presence, not absent predecessor folded endpoint. Existing non-D5 behavior remains unchanged. |
    | `mc-module/src/decay_render.rs:45–53,235–237,328–342,395–398`, decay and empty rendering | Empty payload omission is not coverage proof; retain current rendering behavior and gate coverage exclusion on D5 anchor type rather than all empty rows. |
    | `mc-module/src/transform.rs:37308–37329,37371–37441`, gap-accepting fixture and fake_compaction_descends_materializes_and_write_free_replay_acks | Replace the accepted uncovered 7–10 gap with preserved nonempty tail and the fixed placeholder; assert content, expansion, eligibility and repeated carry instead of coverage11 alone. |
    | `mc-module/src/transform.rs:4922–4939` and `mc-store/src/lib.rs:11070–11076`, applied-unit pruning and reasoning-state copy | Carry participates before HARD pruning; inherited current strip and reduction permissions survive rather than being cleared as absent live input. |

49. **Inventory limits.** The concrete Claude Code gateway marker receiving line is unavailable in the cited inventory; TS host sites are shared consumers, not proof of implemented gateway persistence or NEVER_SEND. The supplementary gateway map was not supplied. The union in clause48, including panel-named resolve_boundary_state, is the known normative inventory with that gap explicit. The panel's gap-test excerpt was not the cited regression; use the independently checked relocated fixture. Every row requires D5 and applicable non-D5 pins: numeric frontier checks alone miss identity, archive precedence and crossing-publication failures.

50. **Parity gate.** Every shared change requires actual D5 receipt or lineage_boundary state. OpenCode and Pi gain no D5 protocol. Current descent is gated by lineage_switched and not-subagent, not serializer equality alone. No-D5 paths retain sparse increasing ordinals, empty filtered-noise semantics, same-key raw expansion and native markers. OpencodeAiSdk and Pi differential goldens stay unchanged, not regenerated. No global contiguity tightening, all-empty exclusion, boundary bypass or floor change. Positive D5 fixtures prove reachability; absent-D5 negatives prove isolation.

51. **Defect evidence.** The executed report found no real compartment or transcript over1799–1939, empty anchor1940, native replacement starting1940, and three probe strings absent from all successor messages on three passes and unavailable through successor expansion. Saved memory14536's 1,091 characters and copied note content survive independently: no total-information-loss claim. The P fire covered only1799–1905 and failed the separate revert_epoch predicate before transcript insertion, not Some(203) formatting. Preserve the remainder through1939 without weakening that guard. Cite findings and digests, not ignored local evidence paths.

52. **Acceptance discipline.** Executable invariants and red-first controls below are requirements, not results of this documentation task. Use real two-connection transactions, provider-visible arrays, durable restart and relevant compile gates. Silent-loss guards need named non-vacuity controls, not unrelated red commands. Rewriting the gap-accepting test changes its preservation claim and must say why: anchor-shape acceptance is replaced by nonempty fixed-placeholder carry evidence. Migration #2732 and versioned wire agreement are hard dependencies. Do not fire this spec, implement product or gateway code, run product tests or deploy within this consolidation.

## acceptance sketch

I1: Fixed placeholder with matching SEALED receipt, missing receipt, REFUSED result and mismatched F fixtures → only the matching SEALED path with durable send permission emits a placeholder; redeem without valid SEALED refuses.

I2: Tail containing previously served blocks, never-served blocks, applied placeholders, untagged tool inputs and two identical native messages at different positions → each block is covered by real history or served as archive V on first S pass; fresh blocks equal defer rendering, dropped bodies never appear, and duplicate positions retain distinct MIDs and exact ordered membership.

I3: Inherited message and mixed inherited-plus-ordinary range expansion before and after successor reduction and restart → inherited membership resolves to current permitted archive projection before cache or raw fallback, with existing ordinary chunk-transcript behavior unchanged.

I4: P historian assembled before seal and forced to publish on each side of the seal transaction → before-seal commit triggers fresh projection or refusal, after-seal publish fails the unchanged revert_epoch guard before any transcript append.

I5: Same-F repeats, concurrent same-F attempts, retry-marker-only deltas, different-F contenders, terminal release and new attempt → one active winning receipt with durable aliases, exact positional allocation reuse within an attempt, p_already_sealed for different material, and new attempt identity only after a legitimate terminal boundary.

I6: Crash at every archive-write, seal, reply-record, redeem and release commit cut → no half-fence, half-archive or half-successor; post-commit recovery returns the committed state, and REDEEMED with missing S is loud lineage_corrupt rather than remint.

I7: Specimen folded1798 with carry1799–1939 and continuation1940, no-real-compartment carry, and a real fold ending at ordinal0 → historian head uses folded successor ordinal or first_inherited_ordinal as appropriate, with inherited eligible_tokens greater than zero without floor changes or continued_ordinal_offset_missing.

I8: First HARD, two DEFER passes, partial inherited fold, block reduction, restart and another DEFER → unreduced carry remains byte-identical to frozen served.sha256 every pass, valid fold causes SOFT, reduced block validates against its committed unit projection and never resurrects original bytes.

I9: Requests below and above the ingress watermark, including outstanding MC mutations and a queued appended ordinary request → lower requests drain or abort before prepare, higher requests queue no longer than D or receive 503, with no drop, transformed-body replay or raw forward.

I10: SEALED age beyond every sweep horizon, cancel without release, positive RELEASED clock and shared archive reuse → SEALED remains fenced and retained with sealed_unredeemed status, only positive release starts 24-hour forensics, and a blob survives until its final protecting reference is removed.

I11: Full projected union under each reserve_accounting source, missing fields, soft exceedance, over-hard override and byte exceedance → receipt reports original geometry, reserve source and units, independent soft and hard outcomes and actual encoded caps; unknown inputs refuse rather than zero-fill.

I12: Existing ten-message gap fixture with real history only through6, nonempty instruction and tool result at7–10 and the fixed placeholder instead of Durable summary alpha → successor preserves the unfolded tail in content, expansion, first-trigger eligibility and repeated passes rather than merely expecting anchor11 coverage.

I13: Real two-connection seal-versus-resolve interleavings paused before transaction entry, after CAS read and at commit → resolve-first produces a durable no-future-seal fence and old seal fails, while seal-first resolves SEALED without a contradictory tombstone or partial fence.

I14: P→S with outstanding carry, a legitimate S reduction, byte-identical distinct-origin blocks, then S→S2 with only new native source supplied by gateway → S2 serves the provenance-deduplicated union projected under current S A with re-minted digests, preserves distinct origins and expands every outstanding inherited block after ancestor deletion.

I15: Table-driven fixtures covering every clause48 consumer with specimen shape, empty real history, ordinal0, partial fold1799–1905, crossing fold1799–1950, mixed expansion and actual wire anchors → real coverage reports1798 before folds, continuation remains1940, coverage proof is receipt-backed, native proof is live, crossing publication absorbs the anchor atomically and non-D5 results are unchanged.

I16: REFUSED attempt A, concurrent SEALED attempt B, coalesced alias resolution and gateway restart with missing local row → attempt.resolve reports A's outcome separately from current p_fence and fence_generation; ordinary P never enters on REFUSED alone.

I17: Resolve returns p_fence none, another prepare seals, then ordinary request attempts admission → current fence generation or held exclusive ingress ownership rejects stale admission without losing appended input.

I18: Cached SEALED retry races RELEASE_INTENT→NEVER_SEND, retry registration and MAY_HAVE_REPLIED flip in both orders → exactly one send-authority outcome, no placeholder after revocation, and release is never claimed safe if any alias may have replied.

I19: Receipt-held commands C-before-release, C-after-release and C racing release, plus commands racing redeem and nested routing → each target reaches current owner exactly once in effect with provenance and ledger intact; release transfers pending and post-seal queues before clearing P's fence.

I20: Release and redeem race on the same receipt, followed by lost-response retries of either terminal operation → one terminal CAS wins, redeem retries return the existing S without remint, release retries do not duplicate queues, and neither terminal state is undone.

I21: Different-F prepares overlap before either seal and repeat after F1 seals → only one material can seal, losing F2 receives p_already_sealed identifying R1 diagnostically, R1 stays unchanged and F2 input is preserved rather than authorized against R1.

I22: Negative-row cap full of current-incarnation tombstones, old prepare delayed before handler arrival, resolve bump, delayed arrival and then new attempt → old ticket fails seal_after_resolve without fence or new archive, resolver remains authoritative without allocating a row, and a new ticket with new attempt can succeed.

I23: Session delete and recreate after a negative refusal, stale delayed prepare and resolve, and current-incarnation attempt → stale incarnation refuses independent of tombstone reclamation and counter values, while current work uses fresh tickets without canonical MID reuse.

I24: P→S→S2 with shared archive references, delete P then S, unforced delete while SEALED and explicit forced deletion → descendant expansion survives ancestor deletion, owner_key governs D5 deletion, unforced SEALED deletion refuses and forced destruction is logged as an exception.

I25: Attempt quota and negative-row bounds under repeated refuse, release and retry, followed by MID bookkeeping retirement → bounded allocation refuses before growth, negative-cap resolution still fences, terminal replay remains reconstructible and canonical issued MIDs or carried identities are never reclaimed or reused.

I26: First-turn compaction, a valid normal envelope, successfully forwarded harness retry, overlapping normal sends and late completion of an older one → unknown first evidence refuses; latest causal committed-send record wins atomically, including retry, without old completion overwriting it.

I27: Persisted envelope then ordinary gateway restart, write failure on newer normal envelope, declared non-persisting restart and old-incarnation gateway restore → durable same-incarnation evidence survives boot change, newer-unusable and degraded absence refuse budget_unknown, and MC rejects the older-incarnation envelope.

I28: Same-incarnation envelope rollback and joint MC-plus-gateway restore with changed or undecidable intended normal surface → no universal rollback-detection claim; identity mismatch or unresolved evidence refuses, while the bounded witness limitation is explicitly observable in fixture expectations.

I29: Normal envelope differs from raw summary tools or max_tokens but agrees with intended successor surface, then separate model, tools, system, guidance and profile mismatches → summary shape is not substituted for normal evidence, valid identities use recorded age, model mismatch and evidence mismatch have typed field-specific refusal.

I30: Prepare fits retained normal envelope, first S pass installs larger tools or guidance exceeding actual geometry → final outgoing gate refuses or uses only an authorized existing reduction path, never sends overflow, drops carry silently, undoes REDEEMED or remints S.

I31: AUTO persistent 503, recovering AUTO 503-503-200, manual persistent refusal and prepare timeout around seal → fixed D120 policy follows calibrated boundaries, authoritative refusal retains old history and normal-transform path, UNKNOWN resolves, and no provider-summary fallback or raw passthrough occurs.

I32: Candidate manifest with omitted block, duplicated block, forged new row_version, invented unit and host-dependent unreduced serialization → validator rejects each illicit membership or authorization change while accepting a genuine successor reduction with committed unit bytes.

I33: Partial fold followed by crossing fold, repeat fold, nested descent and publish against a changed assembly generation → continuation_identity never moves, anchor is absorbed only with atomic real publication, stale snapshots cannot append inconsistent ranges and remaining carry stays eligible.

I34: Full preserved specimen with 141 native messages, actual reductions, untagged tool inputs, reasoning, system, m0, m1 and wrapper → budget sizing uses estimator over the full projected manifest union and reports safety-policy refusal distinctly from physical hard capacity; 24,073 tagged tokens are never used as total carry size.

I35: Keys lacking D5 receipt and lineage boundary under OpencodeAiSdk and Pi, sparse retired ordinals, ordinary empty filtered-noise rows and same-key expansion → differential goldens remain byte-identical, coverage and marker semantics unchanged, and no D5 proof bypass is activated.

I36: Known-empty system and tools versus absent segments, unknown output reserve, unknown wrapper measurement, missing structured reserve flag and unresolved estimator → known emptiness is accepted as evidence, each unknown refuses with named provenance and never becomes zero or a successful cap result.

**Differential and parity pins.** Keep OpencodeAiSdk and Pi goldens unchanged. Pair reachable D5-positive fixtures with no-D5 sparse coverage, filtered-noise, decay, native-boundary and same-key expansion controls. Nested fixtures omit injected carry from native source and must read S's archive.

**Specimen content probe.** Seed the predecessor projected tail with these three distinct strings in unreduced blocks, independently assert their presence in predecessor source, then recursively search every string value in the complete successor served array, including tool inputs, results, reasoning and system blocks. All three must be present on S passes one, two and post-restart, and in inherited expansion; repeat on S2 while those blocks remain outstanding. The second string contains a literal tab after 9226 when materialized in the fixture, not the two characters backslash and t.

- `Your parsed disk assertions are independently verified: setup intact` at predecessor `ccm-1824#0`.
- `9226\t            // A second archived project, never touched by this test's` at predecessor `ccm-1864#0`, decoding `\t` as that literal tab.
- `Take the real follow-up note1274: audit persistence-related test assertions in this repo` at predecessor `ccm-1927#0`.

Use the fixed placeholder `<summary>\nConversation history compacted and preserved by Magic Context. Full context continues to be served automatically.\n</summary>`, embedded in the normal continuation wrapper, not replaced by a real summary of the tail. Preserve the independent positive control of saved memory14536 in m0 and copied note content; those controls cannot satisfy the three transcript probes. After intentionally reducing a probe block, assert its committed reduced representation and absence of the original body instead of falsely requiring verbatim resurrection. Content presence must be tested in provider-visible arrays, not just manifest IDs, tag storage, hashes or predecessor-only archives.

**Required red-first mutations.** These are required implementation controls, not executions by this documentation task. Rows name the tests that must redden. Start from passing named tests, stage the exact live files to be mutated, confirm an empty working diff, apply one temporary change marked `NON-VACUITY BREAK`, capture a non-empty diff stat, run the named gate, restore from that staged live state and touch the restored files, then capture an empty diff stat. Never commit a mutant. Report the exact expected failure and names of any other failed tests; a generic red command or an unrelated build error does not prove the named guard.

| Control | Named test that must fail |
|---|---|
| Bypass the gateway SEALED and matching-F placeholder permission | `d5_placeholder_requires_matching_sealed_receipt` |
| Remove persisted attempt creation before prepare dispatch | `d5_crash_after_dispatch_recovers_durable_attempt` |
| Refresh the ticket when a delayed old prepare reaches MC | `d5_resolve_at_cap_rejects_prearrival_old_ticket` |
| Disable the tombstone predicate while leaving ordinary seals reachable | `d5_resolve_before_seal_prevents_commit` |
| Admit P from attempt REFUSED without consuming current fence generation | `d5_refused_attempt_does_not_admit_fenced_p` |
| Cache a none fence across a concurrent seal without rechecking | `d5_admission_consumes_current_fence_generation` |
| Allow a cached SEALED retry to send after NEVER_SEND | `d5_release_revokes_cached_reply_permission` |
| Remove release transfer of parked commands | `d5_release_restores_pending_and_postseal_commands` |
| Permit both release and redeem updates without the shared receipt CAS | `d5_release_redeem_have_one_terminal_winner` |
| Return a newly minted successor on redeem retry | `d5_redeem_retry_returns_existing_successor` |
| Replace carry with an empty provider-visible slice while keeping metadata | `d5_specimen_tail_content_on_successor_passes` |
| Validate an unreduced block only against a newly claimed mutable digest | `d5_unreduced_carry_rejects_mutated_bytes` |
| Accept missing or duplicated members if their individual hashes match | `d5_carry_requires_exact_ordered_membership` |
| Compare a legitimate reduced block to its frozen original served hash | `d5_legitimate_reduction_uses_unit_projection` |
| Let inherited ctx_expand choose a raw cache entry before current projection | `d5_inherited_expand_never_resurrects_reduced_body` |
| Use S's gateway-native segment alone for nested prepare | `d5_nested_descent_carries_outstanding_union` |
| Reuse ancestor V instead of applying S's current A at nested seal | `d5_nested_descent_reprojects_current_reduction` |
| Deduplicate nested union by content hash | `d5_nested_distinct_origin_equal_bytes_survive` |
| Read terminal anchor end as real folded frontier | `d5_all_coverage_consumers_use_real_folded_frontier` |
| Treat absent folded frontier as zero and reject continued offsets | `d5_no_real_compartment_carry_is_eligible` |
| Validate coverage_identity only against native input | `d5_coverage_receipt_proof_and_native_anchor_are_distinct` |
| Remove lineage_boundary overlap exemption | `d5_crossing_fold_absorbs_anchor_atomically` |
| Move continuation_identity to crossing fold end | `d5_crossing_fold_keeps_original_continuation_identity` |
| Delete D5 protecting rows by predecessor session_id after ownership transfer | `d5_delete_ancestors_preserves_descendant_archive` |
| Sweep SEALED by age or delete a shared blob with a live reference | `d5_sealed_and_shared_archives_survive_sweep` |
| Omit incarnation check after delete and recreate | `d5_stale_incarnation_prepare_never_seals` |
| Reclaim canonical MID identities with terminal allocation bookkeeping | `d5_mid_retirement_never_reuses_issued_identity` |
| Subtract output reserve twice for once_carved evidence | `d5_budget_once_carved_170k_fits_180k_soft` |
| Neutralize hard predicate in the budget evaluator, with its hard-result diagnostic directly asserted | `d5_budget_overhard_override_reports_independent_hard_failure` |
| Replace unknown segment or reserve with zero | `d5_budget_unknown_is_not_zero` |
| Size carry from retained tag-token sum instead of full projection | `d5_budget_sizes_full_projected_specimen_union` |
| Exclude successfully forwarded harness retries from envelope selection | `d5_envelope_forwarded_retry_is_latest_normal` |
| Swap the envelope on completion instead of causal committed-send order | `d5_envelope_older_completion_cannot_overwrite` |
| Reuse older envelope after newer persistence failure | `d5_envelope_newer_unusable_refuses_old_evidence` |
| Compare intended normal identity to raw summary tools | `d5_budget_uses_intended_normal_projection_identity` |
| Skip the first successor actual outbound geometry gate | `d5_successor_first_pass_rechecks_actual_geometry` |
| Apply D5 anchor exclusions globally to non-D5 empty rows | `d5_non_d5_filtered_noise_coverage_unchanged` |
| Route authoritative refusal into raw lineage passthrough | `d5_refusal_never_forwards_raw_overlimit` |

The independent hard test must be observed directly through its recorded hard_ok and hard_required results as well as the combined admission decision: a remaining soft clamp can correctly keep a mutated candidate refused and otherwise hide removal of the hard predicate. Mutations must reach the actual evaluator or gate, not a fixture proxy that computes expected and actual with the same faulty function. A package whose control is undefended must include a reddened reachable control for the same file or target, and the implementation review must resolve the missing defense rather than call the suite green.

53. **Generation-fenced logical REFUSED has no receipt row.** When `attempt.resolve` cannot allocate a negative receipt row (per-session negative-row cap saturated at the current incarnation), it MUST still bump `resolve_generation` in the same transaction and return `REFUSED{reason: resolved_absent, fenced_by: ResolveGeneration}` with no row written. The terminal replay answer for that attempt is reconstructed deterministically from `(AttemptKey, fenced_by)`: any later resolve or prepare for the same AttemptKey observes `ticket.resolve_generation < current resolve_generation` and returns the identical REFUSED result; monotonic generation makes the reconstruction immutable without storage. Reporting surfaces (status, health, sentinel) count generation-fenced refusals under `refused_by_generation` separately from row-backed tombstones.

54. **Successor first-pass overflow is a typed refusal that arms the successor fold.** When the successor's first actual outgoing geometry check fails although prepare accepted a valid retained-envelope estimate, the ordinary transform on S MUST return the typed refusal `successor_overflow{estimated, actual, usable_hard}` (the gateway answers 503 to that ordinary request, preserving its appended input) AND MUST arm the successor's emergency historian fold over the carried range in the same pass so the next ordinary request fits; no degradation lane, no raw forward, no carry discard, no body replay, no redeem undo, no successor remint. The refusal counts in the receipt's `post_redeem` block (`overflow_refusals`), and a second consecutive `successor_overflow` after a completed fold is a defect surfaced as `state=stuck` in health.

55. **anchor_reserve is measured by the gateway and pinned by MC.** The gateway measures the placeholder-plus-wrapper token cost per serving encoding (the bytes are the gateway's) and publishes `{encoding, tokens, measurement_version, measured_at}`; MC stores the constant per encoding, records `anchor_reserve{tokens, measurement_version, source: gateway}` in every receipt's budget block, pins each published value with a fixture that re-encodes the wrapper bytes through MC's estimator (drift beyond 10 percent fails the pin), and refuses `budget_unknown` for an encoding with no published measurement (unknown is not zero).

## non-goals

- Change OpenCode or Pi behavior, activate D5 on those hosts, tighten their sparse coverage rules, or replace their differential golden expectations.
- Implement the Claude Code gateway, its persistence API, send revocation, MID store or provider capture in this documentation work; only their required target semantics are specified here.
- Fall back to a provider-generated summary, forward a refused compaction body raw, replay a transformed predecessor request, or substitute the summary request's max_tokens and tools for normal-turn budget evidence.
- Time-sweep SEALED archives, infer non-delivery from age or socket failure, or reclaim UNKNOWN and MAY_HAVE_REPLIED attempts by TTL.
- Detect compaction mid-turn, change substance floors or scheduler fill policy, make the predecessor's in-flight fire the carrier, or relax the existing revert_epoch guard.
- Restore already reduced source bodies, change ordinary same-key raw chunk expansion semantics, or claim that all predecessor facts vanished despite independently saved memory and notes.
- Solve arbitrary same-incarnation or joint-database rollback, guarantee every future successor envelope fits a prior estimate, or establish a client maximum timeout from the D120 calibration.
- Fire the spec pipeline, alter product code or tests, deploy D5, or commit to master as part of this consolidation.

## open_questions

None. The three questions raised during consolidation (tombstone reporting under the generation fence, successor first-pass overflow handling, anchor_reserve measurement ownership) are resolved in clauses 53 to 55 above.
