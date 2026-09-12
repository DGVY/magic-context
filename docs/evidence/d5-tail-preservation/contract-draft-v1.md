---
title: "D5 tail preservation: seal → reply → redeem contract (MC module + store; shared wire contract with the Claude Code gateway)"
date: 2026-09-12
status: draft
validation_only: true
# rigor_proposed: r2
---

## intent

The executed D5 defect proof shows a successor with real history ending at ordinal 1798, an empty continuation boundary at 1940, and no automatic conversation recovery for the nonempty predecessor tail 1799–1939. Before a destructive placeholder can replace the native transcript, every predecessor tail block is to be either covered by validated durable history or preserved as a durable, reduction-respecting projection that the successor serves and can expand. Magic Context owns projection, archival preservation, transactional fencing, successor custody and recovery; the Claude Code gateway owns native capture, positional MID allocation, ingress admission, budget evidence and the placeholder send gate. The predecessor's in-flight historian is not the preservation carrier, and its epoch rejection remains intact. Explicitly saved memory and copied notes can survive independently, so the defect is a conversation-continuity hole, not proof that every fact or token in the tail disappeared.

Calibration: CURRENT-CC Claude Code 2.1.258, exact-version network-disabled fake-provider container; raw hashes and native boundaries verified. AUTO 503-503-200 recovers; persistent AUTO retains history, no summary or boundary, resumes exact pending ordinary request; manual retains history and errors. Manual 120.003 s and AUTO 120.004 s tolerate 503 then retry. Success at 125 s proves tolerance, not a client maximum or 150 s extrapolation. Initial host isolation limits are not container claims. Digests: report `25145bcfaba48dfeedc0753063494a9d484af8ca9956b8d43f4ba3b7192e7693`; manifest `79cc7ac955ed220fcc9b371597fd414e7c47280c4f2038fa47dad9844746a2ad`.

## constraints

### types

1. **Vocabulary and preservation boundary.** P is predecessor, S successor, T ordered uncovered tail. Source is transport-normalized native blocks, excluding recognized compaction additions by provenance. A is durable MC applied state: red units, skeleton, strip, caveman, tags, pending drops and ledger. V=project(source,A) includes never-served blocks. Source authorizes material; A authorizes permissions. The fixed placeholder is the exact summary-wrapper fixture in acceptance. Gateway clauses specify target semantics, not existing APIs. Every unfolded block MUST have validated real-history coverage or durable carry under A. Empty lineage boundaries prove no coverage. P's in-flight publisher is not the carrier.

2. **Shared wire definitions.** New schema_version: 1 contract: exact fields and variants, preserved order, explicit emptiness and unknowns. Option is none or some, never zero. IDs and keys are distinct opaque types; numeric types are checked nonnegative integers. Digest is SHA-256 of canonical versioned bytes. BootId is not envelope validity. Bytes use base64 in JSON; unions use kind.

```text
MaterialFingerprint = { digest: Digest, normalization_version: integer, excluded_additions: ordered list<ProvenanceTag>
  }
AttemptKey = { predecessor_key: SessionKey, agent: AgentId, F: MaterialFingerprint, attempt_id: AttemptId, incarnation:
  Incarnation }
AdmissionTicket = { resolve_generation: ResolveGeneration, P: SessionKey, agent: AgentId, incarnation: Incarnation }
IngressEvidence = { boot_id: BootId, watermark: IngressSequence, sequence_seen: IngressSequence, ownership_id: opaque
  identity }
LineageEdge = { edge_id: EdgeId, predecessor_key: SessionKey, successor_key: SessionKey, agent: AgentId, F:
  MaterialFingerprint, lineage_id: LineageId, continuation_identity: BlockIdentity }
NativeMessage = { position: MessagePosition, ordinal: Ordinal, mid: Mid, role: Role, blocks: ordered list<NativeBlock> }
NativeBlock = { index: BlockIndex, kind: BlockKind, bytes: bytes, provenance: ProvenanceTag, tool_links: explicit
  tool-arc metadata }
SourceSegment = { normalization_version: integer, messages: ordered list<NativeMessage>, excluded_additions: ordered
  list<ProvenanceTag> }
NormalProjectionIdentities = { model: ModelId, profile: ProfileId, tool_surface: Digest, guidance_surface: Digest,
  system_surface: Digest }
PFence = none | SEALED { receipt_id: ReceiptId } | REDEEMED { successor_key: SessionKey }
FenceSnapshot = { p_fence: PFence, fence_generation: FenceGeneration, resolve_generation: ResolveGeneration }
Refusal = { reason: RefusalReason, receipt_id: Option<ReceiptId>, details: typed reason-specific evidence }
AttemptOutcome = SEALED { receipt: ReceiptV1 } | REFUSED { refusal: Refusal, negative: NegativeProof } | REDEEMED {
  receipt_id: ReceiptId, successor_key: SessionKey } | RELEASED { receipt_id: ReceiptId }
PrepareResult = { attempt_outcome: AttemptOutcome, p_fence: PFence, fence_generation: FenceGeneration }
ResolveResult = { attempt_outcome: Option<AttemptOutcome>, p_fence: PFence, fence_generation: FenceGeneration,
  resolve_generation: ResolveGeneration }
RedeemResult = REDEEMED { receipt_id: ReceiptId, successor_key: SessionKey, existing: boolean, fence_generation:
  FenceGeneration } | REFUSED { refusal: Refusal } | lineage_corrupt { receipt_id: ReceiptId }
ReleaseResult = RELEASED { receipt_id: ReceiptId, already_released: boolean, fence_generation: FenceGeneration } |
  REFUSED { refusal: Refusal }
CancelResult = RECORDED { receipt_id: ReceiptId, receipt_state: ReceiptState } | REFUSED { refusal: Refusal }
ReceiptState = SEALED | REFUSED | REDEEMED | RELEASED
NegativeProof = tombstone_row { attempt_id: AttemptId, incarnation: Incarnation } | generation_fence { incarnation:
  Incarnation, invalidated_ticket_generation: ResolveGeneration, fenced_by: ResolveGeneration }
GatewayAttempt = { attempt_id: AttemptId, P: SessionKey, agent: AgentId, lineage_id: LineageId, incarnation:
  Incarnation, F: MaterialFingerprint, admission_ticket: Option<AdmissionTicket>, ingress: IngressEvidence, started_at:
  Timestamp, receipt_id: Option<ReceiptId>, aliases: set<AttemptId>, outcome: UNKNOWN | AttemptOutcome,
  MAY_HAVE_REPLIED: boolean, send_state: OPEN | RELEASE_INTENT | NEVER_SEND, retries: durable set<RetryIdentity>,
  allocation_ref: positional MID allocation identity }
SuccessorFrontiers = { folded_frontier: Option<Ordinal>, first_inherited_ordinal: Option<Ordinal>, source_frontier:
  Option<Ordinal>, lineage_anchor: Ordinal, rebase_base: Ordinal, coverage_identity: Option<BlockIdentity>,
  continuation_identity: BlockIdentity }
BlockIdentity = { mid: Mid, index: BlockIndex, ordinal: Ordinal }
ReceiptV1 = { schema_version: 1, receipt_id: ReceiptId, attempt_id: AttemptId, predecessor_key: SessionKey,
  successor_key: Option<SessionKey>, lineage_id: LineageId, owner_key: SessionKey, agent: AgentId, incarnation:
  Incarnation, aliases: ordered set<{attempt_id: AttemptId, ticket: AdmissionTicket}>, F: MaterialFingerprint,
  admission_ticket: AdmissionTicket, sealed_state: Option<{ P_state_version: StateVersion, projection_key: opaque identity, applied_state_hash: Digest, epoch_before: integer, epoch_after: integer, fence_generation: FenceGeneration,
  resolve_generation: ResolveGeneration }>, ingress: IngressEvidence, frontiers: Option<SuccessorFrontiers>,
  representation: carry | refused, budget: BudgetRecord, manifest: Option<ManifestV1>, refs: Option<{ archive_id:
  ArchiveId, manifest_digest: Digest, applied_state_hash: Digest }>, pending_drops: ordered list<CommandTarget>,
  post_seal_drops: ordered list<CommandTarget>, post_redeem: Option<PostRedeemRecord>, state: ReceiptState, refusal:
  Option<Refusal>, negative: Option<NegativeProof>, edge: Option<LineageEdge>, sealed_at: Option<Timestamp>,
  released_at: Option<Timestamp>, cancelled_before_delivery: Option<CancelAssertion> }
PostRedeemRecord = { schema_version: 1, overflow_refusals: integer, last_diagnosis: Option<CapacityDiagnosis>,
  preserved_but_blocked: boolean, relief: SuccessorReliefState, overflow_probe: {attempts: integer, next_probe_at:
  Timestamp}, last_overflow: Option<{estimated: TokenCount, actual: TokenCount, actual_source: mc_estimate | provider,
  usable_hard: TokenCount, at: Timestamp}> }
CapacityDiagnosis = { system_tools_tokens: TokenCount, carry_tokens: TokenCount, m0_tokens: TokenCount, m1_tokens:
  TokenCount, usable_hard: TokenCount, reason: system_tools_exceed_hard | no_outstanding_carry | pending_relief |
  below_min_chunk | producer_unavailable | still_over_capacity }
SuccessorOverflow = successor_overflow { estimated: TokenCount, actual: TokenCount, actual_source: mc_estimate |
  provider, usable_hard: TokenCount }
CommandTarget = { command_id: CommandId, target: BlockIdentity, state: queued | pending | partial | covered | retired,
  provenance: command provenance, protection: durable protection state }
ArchiveV1 = { schema_version: 1, archive_id: ArchiveId, encoding_version: integer, manifest: ManifestV1, V: ordered
  projected message bytes, A: AppliedStateSnapshot }
ManifestV1 = { schema_version: 1, normalization_version: integer, encoding_version: integer, messages: ordered list<{
  ordinal: Ordinal, native_mid: Mid, native_position: Option<MessagePosition>, role: Role, blocks: ordered list<{ index:
  BlockIndex, kind: BlockKind, predecessor_identity: BlockIdentity, provenance: native { attempt_id: AttemptId,
  predecessor_key: SessionKey, message_position: MessagePosition } | inherited_from { receipt_id: ReceiptId,
  origin_identity: BlockIdentity }, source: { len: ByteCount, sha256: Digest }, served: { len: ByteCount, sha256: Digest
  }, applied_unit: Option<UnitKey>, tool_links: tool-arc metadata }> }> }
CarryProjectionV1 = { schema_version: 1, receipt_id: ReceiptId, archive_id: ArchiveId, manifest_digest: Digest,
  row_version: RowVersion, coverage_identity: Option<BlockIdentity>, continuation_identity: BlockIdentity, members:
  ordered list<{ identity: BlockIdentity, validation: frozen { served_sha256: Digest } | projection_digest { sha256:
  Digest, unit: UnitKey, row_version: RowVersion } }>, projection_digest: { sha256: Digest, row_version: RowVersion,
  units: ordered list<UnitKey> }, coverage_proof: receipt-backed manifest or real-compartment proof }
GeometryV1 = { usable_soft: TokenCount, usable_hard: TokenCount, absolute_wall: Option<TokenCount>, derivation:
  descriptive text, reserve_accounting: once_carved | none_declared }
EnvelopeRecordV1 = { schema_version: 1, envelope_id: EnvelopeId, request_id: RequestId, request_sha256: Digest,
  captured_at: Timestamp, model: ModelId, system_bytes: KnownBytes, tools_bytes: KnownBytes, geometry: GeometryV1,
  profile: ProfileId, guidance_surface: Digest, tool_surface: Digest, system_surface: Digest, session_id: SessionId,
  agent: AgentId, lineage_id: LineageId, predecessor_key: SessionKey, incarnation: Incarnation, gateway_boot_id: BootId,
  ingress_sequence: IngressSequence, reserve_accounting: once_carved | none_declared }
KnownBytes = present { bytes: bytes, sha256: Digest } | unknown { reason: text }
EnvelopeSlot = available { record: EnvelopeRecordV1 } | absent { reason: text } | newer_unusable { boot_id: BootId,
  sequence: IngressSequence, reason: text }
PrepareBudgetEvidence = { schema_version: 1, geometry: GeometryV1, envelope: EnvelopeSlot, intended_normal_projection:
  NormalProjectionIdentities }
ReserveRecord = { model: ModelId, tokens: TokenCount, source: window-geometry | config, units: tokens }
PolicyReserveRecord = { tokens_estimate: TokenCount, reminder_tokens: TokenCount, estimator: {name: mc-tokenizer,
  version: text}, model: ModelId, profile: ProfileId, fixture_manifest_sha256: Digest, margin: ratio,
  supported_profiles: list<ProfileId>, dynamic_field_limits: list<{field: text, limit: ByteCount}>, source: fixtures }
BudgetRecord = { schema_version: 1, geometry_wire: GeometryV1, reserve_accounting: once_carved | none_declared,
  output_reserve_mc: Known<ReserveRecord>, policy_reserve: Known<PolicyReserveRecord>, estimator: Known<{ identity:
  text, version: text, model: ModelId, units: tokens }>, envelope_id: Option<EnvelopeId>, request_sha256:
  Option<Digest>, budget_evidence: fresh | aged { n: integer } | mismatch { field: text } | unknown { reason: text },
  estimates: { system: Known<TokenCount>, tools: Known<TokenCount>, m0: Known<TokenCount>, m1: Known<TokenCount>, carry:
  Known<TokenCount>, policy: Known<TokenCount>, total_input_X: Known<TokenCount> }, soft_declared: Known<TokenCount>,
  hard_declared: Known<TokenCount>, soft_bounded: Known<TokenCount>, fit_soft: Known<TokenCount>, hard_required:
  Known<TokenCount>, clamp_applied: Known<boolean>, soft_ok: Known<boolean>, hard_ok: Known<boolean>, encoded_bytes:
  Known<{ archive: ByteCount, carry_frame: ByteCount, gateway_limit: ByteCount, mc_frame_page_limit: ByteCount,
  configured_archive_limit: ByteCount }>, failed_caps: ordered list<typed cap result>, unknown_inputs: ordered
  list<field and reason> }
Known<T> = known { value: T } | unknown { reason: text }
SuccessorReliefState = none | armed { range: {first: Ordinal,
  last: Ordinal}, refusal_seq: integer, armed_at: Timestamp } | assembled { firing_id: FiringId, range: {first: Ordinal, last: Ordinal} }
  | published { compartment_seq: Sequence, published_at: Timestamp } | unavailable { reason: no_outstanding_carry |
  below_min_chunk | system_tools_exceed_hard | producer_unavailable }
HealthD5 = { sealed_unredeemed: ordered list<{receipt_id: ReceiptId, age: integer}>, lineage_corrupt: ordered
  list<ReceiptId>, refused_by_generation_responses: integer, refused_by_generation_attempts: integer,
  refused_by_tombstone: integer, d5: {preserved_but_blocked: boolean, overflow_refusals: integer}, successor_relief:
  ordered list<{ successor_key: SessionKey, relief: SuccessorReliefState, overflow_refusals: integer }>, state: ok |
  preserved_but_blocked | lineage_corrupt }
NeverSendProof = { receipt_id: ReceiptId, incarnation: Incarnation, aliases: ordered set<AttemptId>, retries: ordered
  set<RetryIdentity>, revocation_id: opaque durable identity }
CancelAssertion = cancelled_before_delivery
UploadRef = {upload_id: UploadId, digest: Digest, total_bytes: ByteCount}
PrepareSource = inline {segment: SourceSegment} | ref {upload: UploadRef}
EnvelopePending = pending {sequence: IngressSequence}
```

   Negative receipts keep attempt, incarnation and reporting identity, not invented archive or manifest; absent optionals stay absent. Receipt proof is row-backed only. Snapshot bytes and archive_id immutable; lifecycle, owner, commands and post-redeem state mutable. successor_key, edge and post_redeem appear only at redeem. Evolution preserves replay, refuses incompatibility. Transfer full manifest once, then ordered reference proofs; missing is not empty. Protecting references carry predecessor_key, nullable successor_key, lineage_id, owner_key and incarnation; blobs support multiple owners.

### lifecycle ops

3. **Operations and result discipline.** The MC operation surface is:

```text
attempt.ticket(P: SessionKey, agent: AgentId, incarnation: Incarnation) -> AdmissionTicket
prepare(P: SessionKey, agent: AgentId, F: MaterialFingerprint,
  source_segment: PrepareSource, attempt_id: AttemptId, incarnation: Incarnation,
  admission_ticket: AdmissionTicket, ingress: IngressEvidence,
  budget: PrepareBudgetEvidence) -> PrepareResult
attempt.resolve(P: SessionKey, agent: AgentId, incarnation: Incarnation,
  attempt_id: Option<AttemptId>, F: Option<MaterialFingerprint>,
  admission_ticket: Option<AdmissionTicket>) -> ResolveResult
redeem(edge: LineageEdge, receipt_id: ReceiptId, incarnation: Incarnation) -> RedeemResult
release(receipt_id: ReceiptId, release_attempt_id: ReleaseAttemptId,
  incarnation: Incarnation, never_send_proof: NeverSendProof) -> ReleaseResult
cancel(receipt_id: ReceiptId, incarnation: Incarnation,
  assertion: CancelAssertion) -> CancelResult
lineage.begin(attempt: AttemptKey, ticket: AdmissionTicket, kind: source_segment,
  total_bytes: ByteCount, total_chunks: integer, digest: Digest) -> {upload_id: UploadId}
lineage.put(upload_id: UploadId, seq: ChunkSeq, bytes: base64)
  -> {seq: ChunkSeq, chunk_digest: Digest}
lineage.finish(upload_id: UploadId, digest: Digest) -> {upload: UploadRef}
```

   Restart read: attempt_id, F and ticket none → outcome none and current fence snapshot, no writes or bumps. Closure: all three present → some(outcome); partial combinations refuse. MC checks attempt, ticket, ingress, source, envelope lineage and incarnation consistency. Fresh success is SEALED; terminal retries replay. existing and already_released are idempotent success, not send rights. Absence and PREPARING grant no admission; timeout is UNKNOWN.

4. **Stateless tickets and generation fencing.** attempt.ticket only samples the current fence: no MC write, issuance record, ticket ledger, custody, orphan or issuance-idempotency concept. No ticket_id. Lost or unused samples are discardable.  Admission and seal compare generation and incarnation; delayed handlers never refresh.  resolve_generation closes potential seals; fence_generation protects ordinary admission; neither counts delivery or timeouts. resolve is not acquisition.

5. **Projection and positional identity.** Project all blocks before seal: applied units supply permitted reductions; other blocks use ordinary defer serialization. Do not process the summary instruction as a turn, create reductions or bind deferred commands. Preserve message order, grouping, roles, block kinds and tool links. Each native message has a gateway positional native_mid and separate block indexes. Manifest binds native_mid to P's predecessor_identity (mid#index): A and tags use predecessor_identity, serving uses native_mid. Retain issued predecessor identities; allocate deterministic never-served identities with seal. Distinct equal-content positions retain distinct MIDs; retries reuse allocation.

   F binds normalized bytes, order, roles, kinds, provenance and exclusions; excludes MIDs, attempt_id and retry markers. Same-F aliases carry tickets and identical positional seal allocation; mismatch refuses seal_material_mismatch. Losing provisional allocation binds nothing. Tag identities from predecessor (mid,index) commit with A and receipt CAS, never independently. Intended projection identities describe normal successor, not summary tools or max_tokens.

6. **Atomic seal and freshness.** Finish awaits, projection, estimates, encoding and optional grace before one synchronous no-wait transaction committing archive references, SEALED, identities, P.sealed_for, revert_epoch bump and fence_generation change atomically. CAS checks incarnation, original ticket generation, tombstone absence, P.state_version, A hash, frontiers, pending-drop count and all projection dependencies. Publisher or command changes require recompute and rebudget within D or refusal; stale SQLite snapshots retry whole read and CAS. Pre-seal publication changes checked dependencies; pre-seal assembly publishing afterward fails unchanged revert_epoch before any compartment or transcript append. Expiry checks cannot exclude a late commit.

7. **Prepare identity and coalescing.** CAS selects one receipt and manifest for same-(P,agent,F), aliasing every coalesced observer. Refused attempts never revive. Different-F loser gets p_already_sealed with diagnostic winner ID, no authority or winner change; retain losing input queued or refused. Pin both winner orderings. After RELEASED a new same-F attempt may reuse identical content-addressed bytes, but new A and identities need not match historical candidates.

8. **Authoritative attempt resolution.** One store transaction returns existing attempt or alias outcome and current P fence plus both generations. Unrelated generation advances do not erase SEALED or REDEEMED. Absent attempt resolution durably prevents future seal: if negative capacity permits, persist incarnation-scoped resolved_absent tombstone, checked by seal with seal_after_tombstone. At saturated current-incarnation negative bound, allocate no row; CAS-increment resolve_generation only for a still-valid ticket, never re-bump an invalidated one.

   Rowless result is AttemptOutcome::REFUSED: refusal reason resolved_absent, receipt_id none, details resolved_absent{attempt_id,incarnation}; negative generation_fence takes incarnation and invalidated_ticket_generation from the original bound ticket, with fenced_by=ticket.resolve_generation+1 (the fixed minimal invalidating generation). Replay from AttemptKey and original ticket is identical after unrelated resolves; current fence diagnostics advance outside NegativeProof. Delayed prepare returns seal_after_resolve with the same proof, no new archive or fence. New ID and sample may succeed. Recovery never refreshes an old attempt; missing local identity uses read-only restart form.

   NegativeProof has only tombstone_row and generation_fence. Status, health and sentinel separate refused_by_generation_responses, refused_by_generation_attempts and refused_by_tombstone; replay increments responses, not distinct attempts. Counters introduce neither receipt rows nor ticket issuance ledger.

9. **UNKNOWN, interleavings and admission.** Elapsed D, missing gateway row, read-only absence and false delivery bit do not prove P unfenced. Resolve bound attempts with original identity and ticket; state queries use attempt_id none. p_fence independently reports other attempts. Seal-first returns receipt; resolve-first blocks old seal. Physical absence and UNKNOWN are legitimate transients, never refusal or PREPARING permission. Lost resolve reply retains barrier. Admission holds exclusive ingress ownership through read and admit or consumes current fence_generation at admission; cached none is not standing permission.

10. **Redeem and successor custody.** Only after verified placeholder delivery and observed native successor, redeem checks edge material, matching SEALED and intact fences. Mismatch returns seal_material_mismatch without deleting archives or queued input. One transaction binds stable edge once, mints caller-declared successor_key, copies real compartments and ordinary chunk transcripts, preserves tags and applicable A, installs inherited membership, transfers protecting references and commands, sets frontiers and marks REDEEMED. No speculative S before harness replacement.

   Before-commit crash leaves SEALED and no partial S; after-commit crash leaves complete S. Retry returns same S without new lineage row, edge, tags or MIDs; different bound successor_key refuses. Missing S behind REDEEMED is loud lineage_corrupt in result and health, never remint. REFUSED and RELEASED cannot redeem; discarded source and in-memory P snapshots are unnecessary.

11. **Terminal release and cancel.** Release and redeem CAS one receipt, with one terminal winner. Stable (receipt_id,release_attempt_id) survives unknown-response retries: release CASes SEALED to RELEASED or returns existing RELEASED idempotently; REDEEMED refuses already_redeemed, other invalid states refuse. Atomically restore all frozen pending and post-seal commands to P with provenance, ledger and (command_id,target) dedup before clearing fence. Never roll epoch backward. Cancel records cancelled_before_delivery as hint only: no terminal undo, discharge, retention clock or archive removal.

   Lifecycle: absent→SEALED→REDEEMED or RELEASED; alternatively absent→REFUSED with row or generation proof. REFUSED creates no new archive or admission fence; rowless logical refusal is still fenced. Terminal attempt never returns to SEALED. Forced deletion is an explicit destructive exception, not a successful preservation state.

12. **Commands across lifecycle cuts.** Freeze pending T drops in A with provenance. Transactional lineage lookup routes SEALED targets to receipt, REDEEMED to current descendant queue if carried, ledger covered if folded or retired if reduced, RELEASED to P. Dedup (command_id,target); races join transferred queue or see new owner, never copy-delivery gaps. Protection and queued-until-aged rules persist. Already folded targets get covered, not fake successful reduction. Partial and covered depend on prerequisite ledger migration.

### receipt and archive

13. **Archive authority and crash atomicity.** Archive address is sha256(manifest || V || A) under canonical versioned encoding. Source hashes identify material; store no alternate dropped raw bodies. Projection has no clock, host or unrelated row-version input; incomplete or nondeterministic encoding refuses. External staging may be used only with orphan cleanup and atomic durable receipt reachability before send permission; staging is not SEALED.

   Pre-seal crashes, including archive writes before commit, leave no half-archive or fence; UNKNOWN resolves. Lost committed responses recover receipt; crash after placeholder before redeem retains SEALED for matching edge.  Only events independent of source, A, fences, commands, edge and send authority commute.

14. **Positive-discharge retention and deletion.** Never time-sweep SEALED; lost confirmation is possibly delivered. Expose sealed_unredeemed{age}, exclude SEALED from last_activity GC; cancel does not discharge. REDEEMED custody lives with S until owner-lineage deletion. Only positive RELEASED starts24-hour forensics; later sweep its reference, delete blob only at zero protecting references across all states. REFUSED archive absence never permits premature negative-protection removal.

   Redeem atomically transfers every protecting reference and ownership P→S; nested custody transfers hopwise or self-contained. Ancestor deletion cannot remove descendant's sole archive. Unforced SEALED-unredeemed delete refuses; forced destruction is explicit and logged. D5 tables enter deletion and orphan inventories; owner-key deletion follows clause 27.

15. **Negative quotas and identity retirement.** Recreation advances incarnation; old operations refuse without old tombstones. Keep current-incarnation negatives while attempts could seal; retire stale-incarnation rows oldest-first. Configured per-session attempt_quota refuses nonterminal growth and negative rows have a configured bound; aliases cannot evade either. Saturation still fences rowlessly. Retire optimization rows only with terminal replay or immutable reconstruction intact, never sweep possibly delivered or SEALED custody.

   Gateway allocation bookkeeping retires only after terminality, every alias non-replyable, no receipt, carry or archive reference, and incarnation advance or applicable retention expiry. No TTL-only removal of UNKNOWN, MAY_HAVE_REPLIED or nonterminal attempts. Preserve issued canonical IDs, MID high-water and descendant identities forever without reuse; retries retain release and revocation proof.

16. **Budget evidence and unknowns.** Gateway supplies normal raw system and tools bytes plus geometry, not token splits or capture guesses. MC estimates the full budget with provenance and resolves output reserve. Summary max_tokens is not normal output reserve. Known-empty segments are valid; absent segments unknown. Conflicting reserve_accounting duplicates refuse evidence mismatch.

   Unknown geometry, model reserve, segments, estimator, fixtures, byte capacity or identity refuses budget_unknown, never zero, omitted cap or success. Model mismatch is budget_model_mismatch; other evidence mismatch names field. Token, byte, deadline, freshness, incarnation and invalid-operation failures are typed; Report original failed-cap numbers or unknown provenance. REFUSED describes attempt, not global fence or authorization for different material. Never enter lineage_protocol_passthrough; following ordinary input requires fresh-fence admission and normal transform.

17. **Two-variable budget test.** Let X be the real scheduler estimate over system bytes, tools bytes, MC m0 and m1, full projected carry and policy_reserve. Let R be output_reserve_mc.tokens. Both are known estimates with model and estimator provenance; structured reserve_accounting, not derivation prose, controls:

```text
soft_declared = geometry.usable_soft
hard_declared = geometry.usable_hard
soft_bounded = min(soft_declared, hard_declared)
fit_soft = soft_bounded       if once_carved
         = soft_bounded - R   if none_declared
soft_ok = X <= fit_soft
hard_required = X + R
hard_ok = hard_required <= hard_declared
token_fit = soft_ok AND hard_ok
```

   R>soft_bounded refuses insufficient budget, no wrap or zero. Compute hard_ok even if soft fails; clamp fit only, retain original geometry. Name any additional safety margin separately, not a second output reserve.

   Claude Code gateway declares none_declared: default soft 167,000 and hard 200,000; context-1m may have equal soft and hard. Existing OpenCode and Pi evidence declares once_carved where applicable without activating D5. Window200,000, reserve20,000, already-carved soft 180,000 accepts X170,000. Default167,000 and200,000 pin once-only accounting, not an impossible fits-soft-but-not-hard example. An above-hard soft override exercises the independent hard predicate and internal clamp. BudgetRecord records every operand, result, cap, unit, source, provenance and unknown plus envelope identity, not an MC envelope cache.

18. **Policy reserve and drift pin.** Claude Code authors variable wrapper fields (transcript path, instructions, harness version); gateway supplies observed and derived fixtures, not per-encoding measured constants. MC policy_reserve=ceil(max-over-supported-fixtures mc-tokenizer(model,profile)[currentDate reminder+placeholder+wrapper with declared dynamic-field maxima] ×(1+margin)). Record reminder_tokens separately, included once in reserve. Policy numbers are estimates, never provider measurements. Missing fixtures or unsupported profile refuses budget_unknown pre-seal. Re-encode drift beyond 10 percent fails fixture pin and makes evidence unusable; within tolerance current estimate, not old pin, enters X.

   Tested policy=510 estimated tokens:306 B reminder+LF+998 B derived auto-120 fixed-placeholder anchor with transcript path 512 B; input SHA-256 632fa5b75a1e691bbbc9556ef6ab0ae87d5681339261ac03d484ac4366f000c3; estimate 443 ×1.15 rounded up=510. Census 9,910 observations,13 unique paths, max116 B, calibration max291 B supports512 B coverage, not a universal bound; margin and limits are policy. Declare limits for every dynamic field. Evidence: docs/evidence/d5-tail-preservation/policy-reserve-calc-v3.md and docs/evidence/d5-tail-preservation/policy-reserve-inputs-v3/. Unexpected post-seal capacity remains preserved-but-blocked.

19. **Independent byte caps and full specimen sizing.** Independently gate encoded archive, manifest, A, carry frame and native-envelope overhead against configured bytes. Gateway ingress64 MiB; smaller applicable gateway and MC transform frame or page bound governs. Archive limit is explicit config, not substance policy. Estimate full specimen union including untagged tools, reasoning, system, m0, m1 and policy reserve;83 tags and24,073 stored tokens are fragment mass, not total carry or exact billed loss. Soft refusal below physical hard is a safety-liveness choice. Gate actual outgoing tokens and bytes on every successor render.

### successor semantics

20. **Every-pass carry and digest validation.** Serve carry after m0 and m1 before continuation on first HARD, every DEFER, growth and restart. Only validated folds or legitimate block reductions retire it; partial fold retires its interval, causes normal SOFT and leaves remainder. Include inherited blocks before HARD retention and pruning, not output-only splice. Preserve new suffix independently.

    Unreduced members match frozen served length and sha256 every pass, with missing and duplicate checks. Tags, unrelated versions, hosts and serializers authorize nothing. Reduced members validate committed unit bytes and current projection_digest(unit,row_version), not original hash. Bind ordered unit set to committed A; new digest alone is insufficient.

21. **Distinct frontiers and proofs.** folded_frontier is last real compartment end, never anchor. first_inherited_ordinal is first carried ordinal; source_frontier is preserved source end and MAY advance with folds. lineage_anchor and continuation_identity.ordinal are first live native coordinate. Frozen rebase_base set at redeem is the only rebase addend, never source_frontier or continuation+1. Specimen: folded 1798, carry 1799–1939, base 1939, continuation 1940. coverage_identity advances with real folds; continuation never moves, even crossing. No real fold means absent, not ordinal 0.

   Eligible head is checked folded_frontier+1 or, without real fold, first_inherited_ordinal. No fabricated zero or D5 continued_ordinal_offset_missing. Historian selects archive messages plus new native suffix, not native-only after lowering MAX(end); first trigger has positive inherited eligible tokens without floor changes. Coverage, m0, m1, historian and expansion use real coverage identity and frontier; rebase, host marker, reconcile and trim use continuation and base. Coverage proof is receipt or real compartment; continuation proof is live native. resolve_boundary_state separates declared trim from coverage. No mixed ordinal and MID or native-absence truncation of carry.

22. **Anchor absorption and sequence accounting.** Exclude only lineage_boundary from covering-range validation and publication overlap. Coverage end and identity exclude anchor; max_sequence, m0 folded_compartment_seq and m1_compartment_seq include its sequence. partition_by_folded_seq excludes anchor by type regardless of sequence. rendered_m1_coverage uses only real new compartments; empty set advances nothing. Repeated passes render no empty anchor in m1 and retain stable watermarks.

   Partial 1799–1905 leaves anchor 1940 and remainder. Crossing 1799–1950 atomically removes anchor while preserving continuation_identity and native anchor MID on real published compartment; coverage advances, continuation and base stay fixed. Assembly snapshot, publication generation and append order agree. Repeated and nested folds preserve metadata without phantom coverage or duplicate anchors.

23. **Expansion and nested descent.** Inherited ctx_expand arm precedes same-key cache and raw transcript fallback, returning archive V initially and current permitted projection after reduction, never dropped originals. Range mode resolves carry before last_compacted cutoff, merging ordered archive and ordinary ranges with existing bounds and dedup. Folds preserve appropriate covered expansion across restart. Tags, ancestor caches and transcript paths are not archive resolution.

   Nested prepare reads S's outstanding archive because native source excludes injected carry. Union with genuinely new uncovered native tail is fully budgeted and bounded by hop cap 5 with middle-hop collapse. Dedup origin receipt+block identity, not content; equal bytes at different origins survive. Re-project current S A and re-mint frozen hashes. Self-contained data or protecting references preserve provenance across collapse and ancestor deletion for S2 expansion.

24. **Post-seal overflow preserves custody without a fit promise.** Gate every actual S outgoing body despite prepare fit. Overflow returns typed SuccessorOverflow and gateway 503 preserving appended input for harness retry, never transformed replay. actual is outgoing-body MC estimate labelled mc_estimate, or real count for that request labelled provider. No overflow send, raw-forward, carry discard, REDEEMED undo, remint or degradation.

   Atomically persist overflow_refusals, last_diagnosis, last_overflow and preserved_but_blocked; expose d5 blocked flag and count. Bound retries with overflow_probe attempts and next_probe_at. Same pass arms emergency historian over eligible outstanding carry; empty carry or system+tools alone over hard names diagnosis and does not arm. Existing minimum chunk and producer limits remain; below-minimum or unavailable producer retains named blocked diagnosis.

   Durable armed→assembled→published uses ordinary asynchronous historian, unchanged epoch and assembly-generation guards and entitled anchor absorption; remainder stays eligible. All post-redeem state survives restart. Missing producer stays armed or unavailable, still refuses. No next-fit promise, even after publication: repeated overflow remains diagnosed, bounded preserved_but_blocked, not an automatic stuck defect.

### consumer rebind

25. **D5 isolation and accessor fence.** Shared changes require D5 receipt or lineage_boundary. OpenCode and Pi gain no D5 protocol; descent remains lineage_switched and not-subagent gated, not serializer equality alone. Existing max_compartment_end_ordinal and last_compacted_ordinal COALESCE(MAX(end_message),0) accessor and other callers stay unchanged. For non-D5 real ordinal 0 folds retain continued_ordinal_offset_missing, prior_boundary_ordinal and migration_floor_active.

   New D5 Option sibling excludes anchor and rebinds exactly four callers under D5 state: historian trigger, ctx_expand range cutoff, coverage_ordinal_from_compartments and m0 compose. Preserve no-D5 sparse ordinals, filtered-noise empties, same-key raw expansion, native markers and floors. OpencodeAiSdk and Pi goldens stay byte-identical, never regenerated. No global contiguity tightening, empty-row exclusion or guard bypass. Pin reachable D5 and no-D5 controls for every shared change.

26. **Single consumer inventory.** Paths beginning mc-store or mc-module are under crates/. Re-cite every row at dispatch baseline, including unnumbered sites; each needs D5 and applicable non-D5 pins. Gateway marker receiving site is unavailable; TS sites do not establish gateway target implementation.

| Consumer and source site | D5 observable |
|---|---|
| `mc-store/src/lib.rs:10916,10997–11007,11080–11095,11230–11244` | Advance continuation only; real1798 coverage, tail archive. |
| `mc-module/src/compartment_coverage.rs:180–202` | Real coverage; anchor-inclusive sequence (22); sparse parity. |
| `mc-module/src/m1_compose.rs:313,328–344` | Rendered coverage advances only for real new compartments. |
| `mc-module/src/m1_compose.rs`, partition_by_folded_seq and m1_compartment_seq | Exclude anchor from new set by type; include sequence watermark. |
| `mc-module/src/m0_compose.rs:434–449` | D5 Option caller (25); real first ordinal, anchor sequence, native marker. |
| `mc-module/src/transform.rs:7296–7308` | D5 Option caller: real coverage, not anchor end. |
| `mc-module/src/transform.rs:4150–4151` | Real fold controls coverage and trim. |
| `mc-module/src/transform.rs:4744–4751,4753–4789,4779–4790` | True history plus receipt carry (21). |
| `mc-module/src/transform.rs:4812–4854,4912–4917` | Separate receipt and native proofs (21). |
| `mc-module/src/transform.rs:5261–5309` | Partial carry fold causes SOFT; native trim retains continuation proof. |
| `mc-module/src/lib.rs:5198–5260` | D5 Option caller (25), archive eligibility (21). |
| `mc-module/src/historian_chunk.rs:626–674` | Assemble archive carry plus suffix; coherent epoch and generation snapshot. |
| `mc-store/src/lib.rs:12451–12495,16663–16713` | Atomic absorption and publication ordering (22). |
| `mc-store/src/lib.rs:10445–10473` | D5 Option sibling and non-D5 COALESCE isolation (25). |
| `mc-module/src/lib.rs:11907–11935` | Inherited projection before cache or raw fallback. |
| `mc-module/src/lib.rs:11952–11999` | D5 Option caller: inherited membership before cutoff; ordered mixed expansion. |
| `mc-module/src/transform.rs:7843–7864` | Distinct membership proofs (21). |
| `mc-module/src/transform.rs:7866–7904` | Each identity uses its own coordinate and proof. |
| `mc-module/src/transform.rs:7894–7995,8002–8009` | Separate coverage, declared trim, surviving endpoint; guards intact. |
| `mc-module/src/transform.rs:6134–6140` | Export coverage separately from native-addressable marker. |
| `mc-module/src/transform.rs`, continuation_summary_anchor and the NotCompactionShape and ObservedFlagMissingShapePresent dispositions | Fixed wrapper yields descended and valid anchor; other dispositions unchanged. |
| `packages/plugin/src/features/magic-context/compaction-marker.ts` | Fixed-placeholder marker shape agrees with recognizer; non-D5 unchanged. |
| `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:682–712` | continuation_identity maps native endMessageId; coverage remains separate. |
| `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:3377–3379` | Prove native continuation presence, not absent predecessor endpoint; non-D5 parity. |
| `mc-module/src/decay_render.rs:45–53,235–237,328–342,395–398` | Empty rendering unchanged; D5-only coverage exclusion. |
| `mc-module/src/transform.rs:37308–37329,37371–37441` | Nonempty fixed-placeholder tail replaces accepted gap (I12). |
| `mc-module/src/transform.rs:4922–4939` and `mc-store/src/lib.rs:11070–11076` | Pre-prune carry and current reduction permissions (20). |
| `mc-store/src/lib.rs:7444–7490` | D5 tables omit session_id; explicit owner deletion preserves descendant references. |

27. **Owner facts and prerequisite partition.** Owner facts: mc_reduce_command_ledger partial and covered dispositions need an mc-store migration; #2732 is not implemented or banked. Dispatch this D5 prerequisite first; verify migration and ledger row shape before command-custody dependents. Existing lineage descent hop cap 5 with middle-hop merge-collapse was verified live 2026-08-07, drive Leg 5. Reflecting delete_session enumerates PRAGMA table_info and deletes by session_id on every table having it: D5 receipt, archive, manifest and related custody tables instead have owner_key and no session_id; explicit D5 owner deletion governs, not reflection exemptions.

   Disjoint slices in order:1 mc-store receipt, archive, manifest and ledger schemas; ticket, resolve, seal, redeem, release, fences, CAS, delete — one fence-bearing coordinated ck-mc bounce.2 mc-module lifecycle, wire, budget, policy reserve, gateway types.3 module carry, digests, identities, absorption, overlap, accessor, rebind, nested descent.4 module historian, ctx_expand, markers, parity and I-items. Sequence transform.rs and store lib.rs churn by rebase, never parallel overlap. Shared-wire agreement is prerequisite.

28. **Defect and verification.** The failed P firing covered1799–1905 and hit revert_epoch before insertion, not Some(203) formatting. Preserve through 1939 without weakening it. Surviving memory14536 has1,091 characters; three tail probes failed on three passes and expansion independently.

   Implement acceptance with real two-connection transactions, visible arrays, restart and compile gates. Explain replacing anchor-only gap acceptance with nonempty fixed-placeholder preservation. Silent-loss proof: passing test; stage live files and empty diff; temporary NON-VACUITY BREAK and nonempty diff stat; named gate; restore staged state, touch, empty diff stat. Never commit mutant; name expected and other failures. Mutate actual evaluator, not proxy. Direct hard_ok and hard_required plus admission prevent soft rejection masking hard removal. Undefended controls require a reddened same-file or target control and resolved missing defense. Documentation performs no product changes, tests, deploy or spec firing.

### transport

29. **Dedicated route and trusted authority.** ck-mc RouteTarget::InternalService{module_id:"magic-context",service_id:"mc.lineage"} uses decision-only on_bind: admit iff req.principal==Some(Principal::Reserved{module_id:"thalamus"}), daemon-stamped spawn-nonce authority. Never trust BindIdentity. Restrict only this route; ManagementSurface session.status, wrapup, delete, doctor and dashboard callers unchanged. No MCP tool exposure, generic wire change or protocol pin bump.

   Unary Vec<u8> JSON-body frames use JSON framing, not FLAG_BINARY. Body cap 64 MiB; request streaming unavailable (responses only). Preserve full typed SEALED, REFUSED, REDEEMED and RELEASED payloads; ErrorBody{code,message} is solely transport or authority failure, never lifecycle flattening.

30. **Chunked prepare and upload custody.** Gateway MUST use inline prepare when its serialized inline body is at most 1 MiB, ref otherwise; selection applies only to prepare. raw_chunk_bytes≤1 MiB means unencoded put bytes; serialized_body_bytes≤64 MiB means JSON frame. A1 MiB raw chunk is about 1.33 MiB base64 JSON and valid; do not apply inline threshold to put. Digest is SHA-256 over ordered raw bytes. Only kind source_segment is accepted: gateway owns native-by-provenance source; MC generates and returns manifest, never accepts its upload.

   begin idempotency key (AttemptKey,ticket,kind,digest) returns same upload_id after lost reply; differing totals refuse upload_declaration_conflict without rewriting declaration. Identical (upload_id,seq) bytes replay; differing bytes refuse chunk_conflict. finish verifies ordered complete bytes, declaration and digest, replaying same UploadRef. upload_digest_mismatch discards only unreferenced staging; wrong finish on SEALED or REDEEMED referenced upload and foreign mismatch never delete custody.

   Ticket invalidation makes upload unusable immediately, including finish and reference. Unfinished uploads expire with attempt terminality or incarnation advance, never clock alone. Referenced bytes are custody, not GC staging. MC config: lineage.upload.max_concurrent_per_attempt=2; lineage.upload.max_bytes_per_attempt=48 MiB raw; lineage.upload.max_chunk_bytes=1 MiB raw. Aggregate every upload and retry, counting accepted (upload_id,seq) once. Work consumes original D; quota or known expiry refuses before seal without invalidating late committed custody.

### gateway target semantics

31. **Durable attempt, ingress and reply gate.** Before ticket sampling, persist attempt ID, F, P, agent, lineage, incarnation, ingress ownership and positional allocation or durable reference; failure prevents dispatch and placeholder. Local CAS binds exactly one chosen sample before prepare, never rebinds. receipt_id stays none until response or resolution; MAY_HAVE_REPLIED starts false. Before any placeholder byte, persist the monotonic bit and acquire send right serialized with revocation across all aliases and retries, including cached SEALED. Bit-to-bytes crash means possible delivery, irrespective of socket failure.

   No bound ticket at crash means no remote prepare or custody even if attempt record exists; restart fence read precedes fresh attempt. Bound ticket is may-have-dispatched marker even before network dispatch: resolve original attempt, never re-prepare or infer never-sent from missing completion. Recovery uses record and current fence, not transcript F. Missing local row uses resolve(attempt_id none) before admission, never treats absence as refusal.

   Durable exclusive per-session and agent ownership sets boot and watermark. Lower or equal requests and MC mutations drain or abort before prepare; abort fails. Higher requests queue at most D or get 503 preserving appended input. In-memory cohorts are not fence. SEALED blocks P, REDEEMED routes current S, RELEASED requires fresh admission. No lost input, raw forwarding, P replay of V or transformed-body replay. Placeholder requires matching SEALED and send right; recover from receipt and archive, not discarded source.

32. **Release trigger and irreversible send revocation.** Release trigger requires caller cancellation or expired reply deadline, every receipt alias closed with no retry in flight, and all MAY_HAVE_REPLIED false. Otherwise resolve, redeem or hold. Same durable authority as send, registration, retries and bit flips atomically revokes sends and registration via RELEASE_INTENT→NEVER_SEND before remote release. NeverSendProof binds receipt, incarnation, all aliases and retries and revocation ID. No registration or bit flip after revocation; cached SEALED rechecks before send right.

   Crash after revocation leaves SEALED and retryable release with same release_attempt_id, never send permission. Release versus reply, registration or redeem has one serialized winner; socket failure cannot prove never-replied.

33. **Retained normal envelope lifecycle.** One whole normal envelope slot per session and agent, scoped to lineage and incarnation: latest managed normal committed send, including forwarded harness retry, excluding summary, rejected and unmanaged. Atomically replace segments and identities, including model, tools and system changes. Pass by value; MC retains receipt identity and estimates, not mutable envelope cache. First absence, write failure and degraded restart are unknown.

   Constraint one — order: select at committed send, not completion. Within boot only newer ingress sequence replaces; across boots durable causal order, not lexical BootId. Match session, agent, lineage and incarnation; late older completion cannot overwrite. Newer unusable sequence tombstones old slot.

   Constraint two — persistence: durably write pending{sequence}, persist whole envelope, clear marker, then send. Crash before clearing marker recovers newer_unusable{sequence}, distinct from valid old record on ordinary restart without marker. Any write or clear failure prevents send and surfaces error. Persist-before-send crash describes possibly unsent request. Explicit non-persisting degradation yields budget_unknown after restart. BootId change alone does not invalidate durable matching-incarnation evidence. Traces cannot replace missing or unusable evidence. Never mix old tools with new model or geometry.

   Constraint three — identity: compare header geometry, profile, guidance, tool and system hashes with MC intended normal successor, not summary tools, omissions or max_tokens. Model mismatch returns budget_model_mismatch; others name budget_evidence_mismatch field. Record fresh or aged by causal sequence distance, not TTL or proof. Undecidable is budget_unknown; actual outgoing gate remains.

   Constraint four — witness: independent MC store.db incarnation detects older gateway restore as absent-by-rollback; foreign or inconsistent incarnation is not match. Equal incarnation cannot detect within-incarnation rollback; joint restore rolls witness back too. There identity agreement is remaining evidence; undecidable freshness yields UNKNOWN or refusal, not universal rollback assurance.

   Constraint five — refusal: unusable evidence gives budget_unknown pre-seal, no placeholder or summary fallback. Compaction failed: AUTO resumes old-history ordinary input, manual errors. UNKNOWN resolves under current fence. Prepare proves retained-envelope fit only; post-seal overflow follows clause 24.

34. **Prepare deadline and valid late seals.** D=120 seconds fixed server policy independent of fill bounds gateway prepare wait before 503, not receipt life. Drain, projection, encoding, upload, archive and optional grace≤20 seconds consume original D. Known pre-seal expiry refuses; commit-boundary timeout is UNKNOWN. Never reset or extend D. D+epsilon seal is valid: resolve SEALED, no refresh or second prepare. Still-open request delivers placeholder, observes native successor, then redeems; after 503 or client gone use release trigger with alias proof or hold. D never removes fences, deletes SEALED, proves absence refused or bypasses admission.


## acceptance sketch

I1: Matching SEALED and F, absent receipt, REFUSED, mismatched F → placeholder only with matching receipt and durable send right; invalid redeem refuses.

I2: Served and never-served tail, applied reductions, untagged tools, duplicate equal-content positions → real coverage or V on first S; defer bytes for fresh blocks, no dropped bodies, distinct ordered MIDs.

I3: Inherited and mixed expansion before reduction, after reduction and restart → current archive projection before cache or raw fallback; ordinary chunk behavior unchanged.

I4: P publisher commits before versus after seal → fresh projection or refusal before; unchanged epoch rejects before transcript append afterward.

I5: Same-F concurrent repeats and retry markers, different-F contenders, release then new attempt → one aliased receipt, stable positional allocation; differing material refuses, new ID only at terminal boundary.

I6: Every archive, seal, reply, redeem and release crash cut → atomic archive, fence and successor; committed state recovers; missing S behind REDEEMED is lineage_corrupt, not remint.

I7: Specimen folded 1798, carry 1799–1939, anchor 1940; no real fold; real ordinal 0 → correct eligible head and positive carry tokens, no floor or offset regression.

I8: HARD, two DEFERs, partial fold, reduction, restart, DEFER → frozen bytes every unreduced pass, fold causes SOFT, reduced block uses committed unit without original resurrection.

I9: Watermark boundaries, outstanding MC mutations, appended queued ordinary input → low cohort drains or aborts, high queues≤D or503; no drop, transformed replay or raw forward.

I10: Aged SEALED, cancel, positive RELEASED clock and shared blob → sealed_unredeemed retained, only release starts24 hours, last protecting reference controls deletion.

I11: Projected union, both reserve sources, unknown fields, soft exceedance, over-hard override, byte exceedance → original geometry, units and provenance; independent soft and hard results and encoded caps; unknown refuses.

I12: Ten-message fixture real history1–6, nonempty instruction and tool result7–10, fixed placeholder instead of Durable summary alpha → content, expansion, eligibility and repeats preserve tail, not anchor 11-only coverage.

I13: Real two-connection seal and resolve paused before transaction, after CAS read and at commit → resolve-first fences future seal; seal-first returns SEALED, no contradictory tombstone or partial fence.

I14: P→S→S2, current S reduction, equal bytes from distinct origins, gateway supplies new native source only → current-A projected union preserves origin identity, remints hashes, survives ancestor deletion and obeys hop cap 5 with middle-hop collapse.

I15: Every consumer, specimen, empty real history, ordinal 0, partial1799–1905, crossing1799–1950, mixed expansion and wire anchors → real 1798 versus native 1940 proof separation, atomic absorption and non-D5 parity.

I16: REFUSED A with SEALED B, coalesced alias and restart missing local row → resolve separates A outcome from current P fence and generation; refusal alone never admits P.

I17: Read none fence then concurrent seal before ordinary admission → held ownership or current generation rejects stale permission; appended input preserved.

I18: Cached SEALED races revocation, retry registration and delivery-bit flip in both orders → one send-authority winner, no send after NEVER_SEND or release with possible delivery.

I19: Commands before, after and racing release, redeem and nested routing after prerequisite partial and covered ledger migration → current owner receives each target once in effect with provenance and ledger intact; release transfers pending and post-seal queues before clearing fence.

I20: Release races redeem, lost-response terminal retries → one CAS winner, existing S returned, no remint, duplicated queues or terminal undo.

I21: Different-F prepares before and after F1 seal, both winner orders → loser p_already_sealed cites winner diagnostically, preserves losing input and winning receipt.

I22: Negative cap full, delayed old handler, resolve bump then arrival and fresh attempt → old seal_after_resolve with no archive or fence, rowless authoritative closure; new ticket and ID succeed.

I23: Delete and recreate after refusal, delayed old prepare and resolve, fresh attempt → stale incarnation refuses independently of tombstones and counters; no issued MID reuse.

I24: P→S→S2 references, delete ancestors, unforced SEALED delete and forced destruction through real reflecting delete_session → descendant expansion survives, owner_key governs, unforced refuses and forced logs exception.

I25: Attempt and negative quotas under refuse, release, retry and MID bookkeeping retirement → refuse before unbounded growth, capped resolve still fences, terminal replay intact, issued and carried MIDs never reused.

I26: First-turn compaction, normal envelope, forwarded harness retry, overlapping sends and older late completion → first unknown refuses; latest causal committed send including retry wins atomically.

I27: Durable envelope ordinary restart, newer write failure, nonpersisting restart and old-incarnation restore → matching durable evidence survives boot; unusable, degraded or old-incarnation evidence refuses budget_unknown.

I28: Within-incarnation and joint database rollback with changed or undecidable intended surface → identity mismatch or unknown refuses; equal incarnation makes no universal rollback claim.

I29: Normal envelope differs from summary tools or max_tokens but matches intended successor; separate model, tools, system, guidance and profile mismatches → normal evidence used with age, typed field-specific mismatch.

I30: Prepare fit followed by larger actual successor tools or guidance → typed successor_overflow, appended input preserved on503, emergency fold when eligible, no send, raw-forward, discard, remint, undo or fit guarantee.

I31: AUTO persistent503 and503-503-200, manual refusal, prepare timeout around commit → calibrated D120, retained history and normal transform on refusal, UNKNOWN resolves, no provider summary or raw passthrough.

I32: Manifest omission, duplicate, forged row_version, invented unit, host-dependent unreduced bytes → reject unauthorized membership or projection, accept genuine committed reduction.

I33: Partial, crossing and repeated folds, nested descent, changed assembly generation → continuation_identity and rebase_base frozen, anchor absorbed only with atomic real publication, stale snapshots cannot append inconsistent ranges, remaining carry eligible.

I34: Full141-message specimen with reductions, untagged tools, reasoning, system, m0, m1 and wrapper → full-union estimate, soft-policy refusal distinct from physical hard, not24,073 tag tokens as total.

I35: No-D5 OpenCode and Pi, sparse retired ordinals, empty filtered-noise rows, same-key expansion → unchanged goldens, coverage, markers and no D5 bypass.

I36: Known-empty system and tools versus absent segments, unknown output reserve, missing policy fixtures, absent structured reserve flag and unresolved estimator → known emptiness accepted; each unknown refuses with provenance, not zero or successful cap.

I37: Full negative cap, A resolve, fresh B resolve, A replay and delayed prepare → identical A proof fenced_by=ticket generation+1, no replay bump or receipt row, independent current snapshot, seal_after_resolve and separate response, attempt and tombstone counts.

I38: Overflow: eligible, empty or below-minimum carry; system+tools over hard; unavailable producer; restart and post-publication overflow → eligible arm and fenced publish, named no-arm diagnoses otherwise, durable blocked counters and bounded probes; every503 retains input, no next-fit or stuck claim.

I39: Supported512 B wrapper fixtures, missing profile or fixture, drift inside or beyond 10 percent →443×1.15 rounds510, reminder separate; current estimate enters X inside tolerance, excess drift fails, unknown support refuses pre-seal.

I40: Fixed-placeholder wrapper, real continuation_summary("alpha"), flag absent with shape and noncompaction shape → D5 descended with valid anchor; old control and NotCompactionShape and ObservedFlagMissingShapePresent retain meaning; TS producer matches.

I41: Restart with no local row, stateless ticket samples and genuine bound-attempt closure → resolve(attempt_id none) writes nothing and grants no admission; sampling writes no issuance or row and bumps nothing; only genuine closure fences an absent attempt.

I42: Concurrent same-source reduced block and duplicate-content positions → same F; native_mid maps predecessor_identity for units and tags; alias tickets and allocations agree, mismatch refuses seal_material_mismatch.

I43: Crashes and failures at pending write, envelope persist, marker clear and send → no send before three commits; leftover marker tombstones old slot, ordinary no-marker restart retains matching evidence.

I44: Non-D5 continued real ordinal 0 fold and present continuation base versus D5 → old offset abort, prior boundary and migration floor unchanged outside D5; D5 Option remains eligible.

I45: Anchor across three passes, partial and crossing fold → coverage excludes anchor, m0 and m1 sequence include it, no empty m1 new anchor or false1940 coverage, stable repeated watermark.

I46: Crash before sample, before local bind, bound before dispatch, after dispatch; concurrent samples → unbound states have no prepare or custody and use restart read before fresh attempt; exactly one bound sample, original resolve without re-prepare.

I47: Forged BindIdentity, non-thalamus and legitimate reserved principal, ManagementSurface callers → daemon principal alone authorizes mc.lineage, old callers unchanged, no MCP or binary exposure.

I48: 1.5 MiB source,1 MiB raw chunk, inline threshold and typed replies → byte-identical chunk roundtrip, ref for oversized prepare, larger base64 put accepted within 64 MiB, typed results outside ErrorBody.

I49: Lost begin reply, put duplicate or conflict, repeat finish, changed begin totals, corrupt SEALED finish and foreign mismatch → stable IDs and refs; typed conflicts preserve declaration and custody.

I50: Ticket invalidation, incarnation change,2-upload and48 MiB quota, duplicate chunk and D expiry → unusable finish and reference, lifecycle-only staging expiry, counted-once aggregate refusal pre-seal; referenced bytes survive GC.

I51: D+epsilon seal with request open versus503 sent or client gone → resolve SEALED, no reset, refresh or second prepare; open delivers and observes native successor before redeem, closed uses never-replied alias proof for release.

I52: Cancelled or expired receipt with closed versus active or possible-reply aliases; release races reply and redeem; lost release reply; release after redeem → only closed never-replied set revokes, stable idempotent retry, one terminal winner, already_redeemed refusal.

Parity pins: reachable D5 receipt and lineage-boundary fixtures paired with no-D5 sparse coverage, filtered noise, decay, native boundary, real ordinal 0 fold and same-key expansion → unchanged OpencodeAiSdk and Pi differential goldens, no D5 bypass; nested source omitting injected carry → S archive actually read.

Specimen content probe: independently seeded and checked unreduced source, fixed-placeholder wrapper, recursive search of every string in full provider-visible arrays (tools, results, reasoning, system), S passes1,2, restart, expansion and outstanding S2 carry → all three probes below present; saved memory14536 in m0 and copied-note controls cannot substitute.

- predecessor ccm-1824#0 → `Your parsed disk assertions are independently verified: setup intact`.
- predecessor ccm-1864#0 → `9226\t            // A second archived project, never touched by this test's`, with \t materialized as a literal tab, not backslash plus t.
- predecessor ccm-1927#0 → `Take the real follow-up note1274: audit persistence-related test assertions in this repo`.

Fixed-placeholder specimen → `<summary>\nConversation history compacted and preserved by Magic Context. Full context continues to be served automatically.\n</summary>` inside the native wrapper, not a real tail summary; manifest IDs, tags, hashes or predecessor-only archives do not satisfy successor content presence.

Legitimately reduced probe block → committed reduced representation present, original body absent, no false verbatim-resurrection expectation.

Red-first mutation fixtures → named test below fails under the temporary control.

- Bypass the gateway SEALED and matching-F placeholder permission → `d5_placeholder_requires_matching_sealed_receipt` red.
- Remove durable attempt creation before ticket sampling → `d5_crash_after_dispatch_recovers_durable_attempt` red.
- Refresh the ticket when a delayed old prepare reaches MC → `d5_resolve_at_cap_rejects_prearrival_old_ticket` red.
- Disable the tombstone predicate while leaving ordinary seals reachable → `d5_resolve_before_seal_prevents_commit` red.
- Admit P from attempt REFUSED without consuming current fence generation → `d5_refused_attempt_does_not_admit_fenced_p` red.
- Cache a none fence across a concurrent seal without rechecking → `d5_admission_consumes_current_fence_generation` red.
- Allow a cached SEALED retry to send after NEVER_SEND → `d5_release_revokes_cached_reply_permission` red.
- Remove release transfer of parked commands → `d5_release_restores_pending_and_postseal_commands` red.
- Permit both release and redeem updates without the shared receipt CAS → `d5_release_redeem_have_one_terminal_winner` red.
- Return a newly minted successor on redeem retry → `d5_redeem_retry_returns_existing_successor` red.
- Accept a redeem retry that declares a different successor_key for a bound edge → `d5_bound_edge_rejects_new_successor_key` red.
- Replace carry with an empty provider-visible slice while keeping metadata → `d5_specimen_tail_content_on_successor_passes` red.
- Validate an unreduced block only against a newly claimed mutable digest → `d5_unreduced_carry_rejects_mutated_bytes` red.
- Accept missing or duplicated members if their individual hashes match → `d5_carry_requires_exact_ordered_membership` red.
- Compare a legitimate reduced block to its frozen original served hash → `d5_legitimate_reduction_uses_unit_projection` red.
- Let inherited ctx_expand choose a raw cache entry before current projection → `d5_inherited_expand_never_resurrects_reduced_body` red.
- Use S's gateway-native segment alone for nested prepare → `d5_nested_descent_carries_outstanding_union` red.
- Reuse ancestor V instead of applying S's current A at nested seal → `d5_nested_descent_reprojects_current_reduction` red.
- Deduplicate nested union by content hash → `d5_nested_distinct_origin_equal_bytes_survive` red.
- Read terminal anchor end as real folded frontier → `d5_all_coverage_consumers_use_real_folded_frontier` red.
- Treat absent folded frontier as zero and reject continued offsets → `d5_no_real_compartment_carry_is_eligible` red.
- Route non-D5 ordinal-zero folds through the D5 Option accessor → `d5_non_d5_ordinal_zero_fold_unchanged` red.
- Exclude the anchor row from the folded sequence watermark → `d5_anchor_never_reenters_m1_new_compartments` red.
- Rebase live native ordinals from continuation_identity.ordinal plus one → `d5_rebase_base_is_frozen_1939` red.
- Validate coverage_identity only against native input → `d5_coverage_receipt_proof_and_native_anchor_are_distinct` red.
- Remove lineage_boundary overlap exemption → `d5_crossing_fold_absorbs_anchor_atomically` red.
- Move continuation_identity to crossing fold end → `d5_crossing_fold_keeps_original_continuation_identity` red.
- Key compaction-shape recognition on provider summary text → `d5_fixed_placeholder_descends` red.
- Delete D5 protecting rows by predecessor session_id after ownership transfer → `d5_delete_ancestors_preserves_descendant_archive` red.
- Add session_id to a D5 custody table visited by reflecting delete_session → `d5_delete_session_exempts_transferred_protection` red.
- Sweep SEALED by age or delete a shared blob with a live reference → `d5_sealed_and_shared_archives_survive_sweep` red.
- Omit incarnation check after delete and recreate → `d5_stale_incarnation_prepare_never_seals` red.
- Reclaim canonical MID identities with terminal allocation bookkeeping → `d5_mid_retirement_never_reuses_issued_identity` red.
- Make ticket sampling close the attempt or persist issuance → `d5_ticket_sample_is_stateless` red.
- Fence live P from resolve(attempt_id none) → `d5_restart_resolve_read_is_non_mutating` red.
- Skip generation bump for a still-valid ticket at negative-row saturation → `d5_capped_resolve_fences_by_generation` red.
- Report generation-fenced refusals in the tombstone counter → `d5_refused_by_generation_counted_separately` red.
- Include gateway MIDs or attempt_id in F → `d5_same_source_concurrent_attempts_share_f` red.
- Resolve applied units by native_mid instead of predecessor_identity → `d5_coalesced_attempt_preserves_applied_units` red.
- Subtract output reserve twice for once_carved evidence → `d5_budget_once_carved_170k_fits_180k_soft` red.
- Neutralize real hard predicate; assert hard diagnostics directly → `d5_budget_overhard_override_reports_independent_hard_failure` red.
- Replace unknown segment or reserve with zero → `d5_budget_unknown_is_not_zero` red.
- Size carry from retained tag-token sum instead of full projection → `d5_budget_sizes_full_projected_specimen_union` red.
- Exclude successfully forwarded harness retries from envelope selection → `d5_envelope_forwarded_retry_is_latest_normal` red.
- Swap the envelope on completion instead of causal committed-send order → `d5_envelope_older_completion_cannot_overwrite` red.
- Reuse older envelope after pending-marker crash → `d5_envelope_newer_unusable_refuses_old_evidence` red.
- Forward a newer normal request before pending, record and clear commits finish → `d5_envelope_unpersistable_request_not_forwarded` red.
- Compare intended normal identity to raw summary tools → `d5_budget_uses_intended_normal_projection_identity` red.
- Skip the first successor actual outbound geometry gate → `d5_successor_first_pass_rechecks_actual_geometry` red.
- Publish the relief fold outside the revert_epoch and generation guards → `d5_relief_fold_respects_publication_guards` red.
- Drop overflow_refusals or relief state on successor restart → `d5_overflow_relief_state_is_durable` red.
- Apply D5 anchor exclusions globally to non-D5 empty rows → `d5_non_d5_filtered_noise_coverage_unchanged` red.
- Route authoritative refusal into raw lineage passthrough → `d5_refusal_never_forwards_raw_overlimit` red.
- Re-bump or use current generation on replay of an invalidated A ticket → `d5_generation_refusal_replay_is_identical_without_rebump` red.
- Count replayed A response as a new refused attempt → `d5_generation_response_and_attempt_counts_are_distinct` red.
- Rebind a chosen ticket or re-prepare bound crash recovery → `d5_bound_ticket_crash_resolves_without_reprepare` red.
- Accept a different coalesced positional allocation → `d5_coalesced_allocation_mismatch_refuses` red.
- Forward raw on successor overflow → `d5_successor_overflow_refuses_and_preserves` red.
- Return overflow without arming eligible nonempty carry → `d5_overflow_arms_fold_when_carry_nonempty` red.
- Arm despite system+tools alone exceeding hard → `d5_overflow_no_arm_when_system_tools_exceed_hard` red.
- Supply zero for missing or unsupported policy fixtures → `d5_policy_reserve_unknown_profile_or_fixture_refuses` red.
- Accept policy fixture drift beyond 10 percent → `d5_policy_reserve_fixture_drift_fails` red.
- Use pinned old reserve rather than current within-tolerance estimate → `d5_policy_reserve_current_estimate_enters_x` red.
- Skip pre-write pending marker → `d5_envelope_pending_marker_crash_tombstones_slot` red.
- Bind mc.lineage from forged BindIdentity or ordinary principal → `d5_lineage_route_requires_reserved_thalamus` red.
- Restrict existing ManagementSurface together with lineage route → `d5_lineage_authority_keeps_management_route_unchanged` red.
- Apply 1 MiB inline threshold to base64 put frame → `d5_upload_raw_chunk_and_serialized_frame_limits_are_distinct` red.
- Flatten typed receipt outcome into ErrorBody → `d5_lineage_transport_preserves_typed_outcomes` red.
- Allocate new upload on lost begin reply retry → `d5_upload_begin_retry_returns_same_id` red.
- Overwrite conflicting begin declaration or existing chunk → `d5_upload_conflicts_preserve_original` red.
- Delete SEALED upload on corrupt finish → `d5_upload_corrupt_finish_preserves_sealed_custody` red.
- Finish chunks after ticket invalidation → `d5_upload_invalidated_ticket_is_unusable` red.
- Reset quota per upload instead of aggregate attempt → `d5_upload_attempt_quota_is_aggregate` red.
- Treat D+epsilon seal as absent and issue second prepare → `d5_late_seal_resolves_to_valid_custody` red.
- Redeem before observing native transcript replacement → `d5_redeem_waits_for_native_successor` red.
- Release with active alias or possible reply → `d5_release_trigger_requires_closed_never_replied_aliases` red.
- Change release_attempt_id after lost response → `d5_release_unknown_retry_keeps_identity` red.
- Permit release against REDEEMED → `d5_release_after_redeem_returns_already_redeemed` red.

## non-goals

- Change OpenCode or Pi behavior, activate D5 on those hosts, tighten their sparse coverage rules, or replace their differential golden expectations.
- Implement the Claude Code gateway, its persistence API, send revocation, MID store or provider capture in this documentation work; only their required target semantics are specified here.
- Fall back to a provider-generated summary, forward a refused compaction body raw, replay a transformed predecessor request, or substitute the summary request's max_tokens and tools for normal-turn budget evidence.
- Time-sweep SEALED archives, infer non-delivery from age or socket failure, or reclaim UNKNOWN and MAY_HAVE_REPLIED attempts by TTL.
- Detect compaction mid-turn, change substance floors or scheduler fill policy, make the predecessor's in-flight fire the carrier, or relax the existing revert_epoch guard.
- Restore already reduced source bodies, change ordinary same-key raw chunk expansion semantics, or claim that all predecessor facts vanished despite independently saved memory and notes.
- Solve arbitrary same-incarnation or joint-database rollback, guarantee every future successor envelope fits a prior estimate, or establish a client maximum timeout from the D120 calibration.
- Fire the spec pipeline, alter product code or tests, deploy D5, or commit to master as part of this consolidation.
- Provide any post-seal capacity degradation lane or promise that arming or publishing a fold makes the next request fit.

## open_questions

None.
