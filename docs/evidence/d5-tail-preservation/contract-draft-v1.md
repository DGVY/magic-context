---
title: "D5 tail preservation: seal → reply → redeem contract (MC module + store; shared wire contract with the Claude Code gateway)"
date: 2026-09-12
status: draft
validation_only: true
includes:
  - .cortexkit/alfonso/drafts/d5-rulings-v1.md
  - .cortexkit/alfonso/drafts/d5-includes/d5-uncovered-tail-report.md
  - .cortexkit/alfonso/drafts/d5-includes/calibration-REPORT.md
  - .cortexkit/alfonso/drafts/d5-includes/policy-reserve-calc-v3.md
# rigor_proposed: r2
---

## intent
Build output: the D5 tail-preservation implementation in the MC Rust store and module — the mc.lineage InternalService route on ck-mc; ticket, prepare, resolve, redeem, release, cancel and chunked-upload operations; the seal transaction and receipt schema; successor carry with per-block digest checks; the complete consumer rebind; the session.status read-only d5 object; and executable I1–I62 identity-mapped gates, with MC-owned Rust and hermetic specimen rows shipped here. Gateway-side clauses are the shared target contract for the separate Claude Code gateway campaign; these slices do not ship its attempt store, native capture, positional allocator or send gate. Clause 27 is the sole slice partition and acceptance assigns every test to exactly one slice. Slice 0 imports immutable fixtures before product work. Product release waits for the gateway campaign's shared-contract asks, then the release owner performs one fence-bearing coordinated ck-mc bounce; slice workers never restart ck-mc.

The executed defect proof shows a successor with real history ending at ordinal 1798, an empty continuation boundary at 1940, and no automatic recovery for predecessor tail 1799–1939. Before a destructive placeholder replaces the native transcript, every predecessor tail block is covered by validated durable history or preserved as a durable reduction-respecting projection that the successor serves and expands. Magic Context owns projection, archive, transactional fencing, successor custody and recovery. The gateway owns native capture, positional MID allocation, ingress admission, budget evidence and the placeholder send gate. P's in-flight historian is not the carrier, and epoch rejection remains intact. Saved memory and copied notes survive independently; the defect is a conversation-continuity hole, not proof that every fact or token vanished.

Calibration: CURRENT-CC Claude Code 2.1.258, exact-version network-disabled fake-provider container; raw hashes and native boundaries verified. AUTO 503-503-200 recovers; persistent AUTO retains history, emits no summary or boundary and resumes the exact pending ordinary request; manual retains history and errors. Manual 120.003 s and AUTO 120.004 s tolerate 503 then retry. Success at 125 s proves tolerance, not a client maximum or 150 s extrapolation. Initial host isolation limits are not container claims. Report digest: 25145bcfaba48dfeedc0753063494a9d484af8ca9956b8d43f4ba3b7192e7693. Sanitized calibration-manifest input digest: 79cc7ac955ed220fcc9b371597fd414e7c47280c4f2038fa47dad9844746a2ad; clause 27 fixes its meaning and import gate.
## constraints

### types

1. **Vocabulary and preservation boundary.** P is predecessor, S successor and T ordered uncovered tail. Source is normalized native blocks minus recognized compaction additions. A is durable reductions, tags, drops and ledger; V=project(source,A) includes never-served blocks. Source authorizes material and A permissions. Gateway clauses are target semantics. Every unfolded block MUST have validated real coverage or durable carry. Empty boundaries cover nothing; P's publisher is not the carrier.

2. **Shared wire definitions.** schema_version 1 fixes fields, variants, order, emptiness and unknowns. Option is none or some, never zero. IDs and keys are distinct; numbers are checked nonnegative integers. Digest follows 2a. BootId is not validity. JSON bytes are base64. The request and response envelope discriminator is op; nested payload unions retain kind.

```text
MaterialFingerprint = { digest: Digest, normalization_version: integer, excluded_additions: ordered list<ProvenanceTag>
  }
AttemptKey = { predecessor_key: SessionKey, agent: AgentId, F: MaterialFingerprint, attempt_id: AttemptId, incarnation:
  Incarnation }
AdmissionTicket = { resolve_generation: ResolveGeneration, P: SessionKey, agent: AgentId, incarnation: Incarnation }
IngressEvidence = { boot_id: BootId, watermark: IngressSequence, sequence_seen: IngressSequence, ownership_id: opaque
  identity }
LineageEdge = { edge_id: EdgeId, predecessor_key: SessionKey, successor_key: SessionKey, agent: AgentId, F:
  MaterialFingerprint, lineage_id: LineageId, continuation_identity: RecognitionIdentity,
  native_continuation_identity: BlockIdentity }
RecognitionIdentity = { receipt_id: ReceiptId, recognition_token: RecognitionToken }
RecognitionObservation = { scanned_identity: BlockIdentity, scanned_bytes_sha256: Digest, scanned_role: Role,
  scanned_kind: BlockKind, native_user_index: integer, scan_source: direct_scalar | text_block | tool_result | other {
  source: text }, observed_markers: ordered list<RecognitionIdentity>, may_have_replied: boolean, ack: Option<opaque
  durable identity> }
ReceiptId = UUID36
RecognitionToken = lowercase_base32_20
NativeMessage = { position: MessagePosition, ordinal: Ordinal, mid: Mid, role: Role, blocks: ordered list<NativeBlock> }
NativeBlock = { index: BlockIndex, kind: BlockKind, bytes: bytes, provenance: ProvenanceTag, tool_links: ordered
  list<ToolArc> }
SourceSegment = { normalization_version: integer, messages: ordered list<NativeMessage>, excluded_additions: ordered
  list<ProvenanceTag> }
Role = user | assistant | system | tool | other { wire_role: text }
BlockKind = text | reasoning | redacted_reasoning | tool_use | tool_result | image | document | other {
  wire_kind: text }
ProvenanceTag = native | recognized_compaction { addition_kind: text } | inherited { receipt_id: ReceiptId,
  origin_identity: BlockIdentity }
ToolArc = { tool_use_id: text, use_identity: Option<BlockIdentity>, result_identity: Option<BlockIdentity> }
NormalizedMessage = { position: MessagePosition, ordinal: Ordinal, role: Role, blocks: ordered list<NormalizedBlock> }
NormalizedEndpoint = { position: MessagePosition, ordinal: Ordinal, index: BlockIndex }
NormalizedToolArc = { tool_use_id: text, use_source: Option<NormalizedEndpoint>, result_source:
  Option<NormalizedEndpoint> }
NormalizedBlock = { index: BlockIndex, kind: BlockKind, bytes: bytes, provenance: ProvenanceTag, tool_links: ordered
  list<NormalizedToolArc> }
AppliedStateSnapshot = { schema_version: 1, canonical_payload: bytes }
NormalProjectionIdentities = { model: ModelId, profile: ProfileId, tool_surface: Digest, guidance_surface: Digest,
  system_surface: Digest }
PFence = none | SEALED { receipt_id: ReceiptId } | REDEEMED { successor_key: SessionKey }
FenceSnapshot = { p_fence: PFence, fence_generation: FenceGeneration, resolve_generation: ResolveGeneration }
Refusal = { reason: RefusalReason, receipt_id: Option<ReceiptId>, details: RefusalDetails }
AttemptOutcome = SEALED { receipt: ReceiptV1 } | REFUSED { refusal: Refusal, negative: NegativeProof } | REDEEMED {
  receipt_id: ReceiptId, successor_key: SessionKey } | RELEASED { receipt_id: ReceiptId }
PrepareResult = { attempt_outcome: AttemptOutcome, p_fence: PFence, fence_generation: FenceGeneration }
ResolveResult = RESOLVED { attempt_outcome: Option<AttemptOutcome>, p_fence: PFence, fence_generation:
  FenceGeneration, resolve_generation: ResolveGeneration } | REFUSED { refusal: Refusal, p_fence: PFence,
  fence_generation: FenceGeneration, resolve_generation: ResolveGeneration }
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
  native_continuation_identity: BlockIdentity }
BlockIdentity = { mid: Mid, index: BlockIndex, ordinal: Ordinal }
ReceiptV1 = { schema_version: 1, receipt_id: ReceiptId, attempt_id: AttemptId, predecessor_key: SessionKey,
  successor_key: Option<SessionKey>, lineage_id: LineageId, owner_key: SessionKey, agent: AgentId, incarnation:
  Incarnation, aliases: ordered set<{attempt_id: AttemptId, ticket: AdmissionTicket}>, F: MaterialFingerprint,
  recognition_token: RecognitionToken, admission_ticket: AdmissionTicket, sealed_state: Option<{ P_state_version: StateVersion, projection_key: opaque identity, applied_state_hash: Digest, epoch_before: integer, epoch_after: integer, fence_generation: FenceGeneration,
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
  }, applied_unit: Option<UnitKey>, tool_links: ordered list<ToolArc> }> }> }
CarryProjectionV1 = { schema_version: 1, receipt_id: ReceiptId, archive_id: ArchiveId, manifest_digest: Digest,
  row_version: RowVersion, coverage_identity: Option<BlockIdentity>, native_continuation_identity: BlockIdentity, members:
  ordered list<{ identity: BlockIdentity, validation: frozen { served_sha256: Digest } | projection_digest { sha256:
  Digest, unit: UnitKey, row_version: RowVersion } }>, projection_digest: { sha256: Digest, row_version: RowVersion,
  units: ordered list<UnitKey> }, coverage_proof: receipt-backed manifest or real-compartment proof }
TransformResponse = existing response fields plus { lineage_descent_disposition: Option<text>, d5_carry:
  Option<CarryProjectionV1> }
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
PolicyReserveRecord = { tokens_estimate: TokenCount, reminder_tokens: TokenCount, recognition_suffix_tokens:
  TokenCount, recognition_suffix_bytes: ByteCount, recognition_suffix_sha256: Digest, estimator: {name: mc-tokenizer,
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
HealthD5 = { sealed_unredeemed: ordered list<{receipt_id: ReceiptId, age_seconds: integer}>, lineage_corrupt:
  ordered list<ReceiptId>, refused_by_generation_responses: integer, refused_by_generation_attempts: integer,
  refused_by_tombstone: integer, blocked: {preserved_but_blocked: boolean, overflow_refusals: integer},
  successor_relief: ordered list<{ successor_key: SessionKey, relief: SuccessorReliefState, overflow_refusals:
  integer }>, uncovered_descent: ordered list<{lineage_id: LineageId, first: Ordinal, last: Ordinal}>,
  unrecognized_successors: integer, last_unrecognized_token_prefix: Option<text>, state: ok | preserved_but_blocked |
  lineage_corrupt }
NeverSendProof = { receipt_id: ReceiptId, incarnation: Incarnation, aliases: ordered set<AttemptId>, retries: ordered
  set<RetryIdentity>, revocation_id: opaque durable identity }
CancelAssertion = cancelled_before_delivery
UploadRef = {upload_id: UploadId, digest: Digest, total_bytes: ByteCount}
PrepareSource = inline {segment: SourceSegment} | ref {upload: UploadRef}
EnvelopePending = pending {sequence: IngressSequence}
RefusalReason = invalid_arguments | ticket_invalid | stale_incarnation | budget_unknown | budget_model_mismatch |
  budget_evidence_mismatch | token_cap | byte_cap | p_already_sealed | seal_material_mismatch | resolved_absent |
  seal_after_tombstone | seal_after_resolve | attempt_quota | sealed_unredeemed | lineage_corrupt |
  successor_overflow | already_redeemed | invalid_terminal_state | upload_declaration_conflict | chunk_conflict |
  upload_digest_mismatch | upload_incomplete | upload_quota | d5_receipt_required | d5_downgrade_refused
RefusalDetails = none | field {field: text, reason: text} | cap {cap: text, actual: integer, limit: integer,
  units: text} | winner {receipt_id: ReceiptId} | resolved_absent {attempt_id: AttemptId, incarnation: Incarnation} |
  upload {upload_id: Option<UploadId>, seq: Option<ChunkSeq>, declared_digest: Option<Digest>, actual_digest:
  Option<Digest>} | successor_capacity {estimated: TokenCount, actual: TokenCount, actual_source: mc_estimate |
  provider, usable_hard: TokenCount}
TicketResult = ISSUED {ticket: AdmissionTicket} | REFUSED {refusal: Refusal where reason=invalid_arguments}
BeginResult = BEGUN {upload_id: UploadId} | REFUSED {refusal: Refusal}
PutResult = STORED {seq: ChunkSeq, chunk_digest: Digest} | REFUSED {refusal: Refusal}
FinishResult = FINISHED {upload: UploadRef} | REFUSED {refusal: Refusal}
LineageRequest = tagged union op ticket | prepare | resolve | redeem | release | cancel | begin | put | finish;
  each variant's fields are exactly its same-named clause 3 arguments, without an extra args wrapper
LineageResponse = ticket {result: TicketResult} | prepare {result: PrepareResult} | resolve {result: ResolveResult} |
  redeem {result: RedeemResult} | release {result: ReleaseResult} | cancel {result: CancelResult} | begin {result:
  BeginResult} | put {result: PutResult} | finish {result: FinishResult}
```

2a. **CE1 and digest preimages.** CE1 encoding_version 1 uses fixed unsigned big-endian U32BE and U64BE. Integer-like scalars are checked U64; bool is byte 0 or 1; Digest is 32 raw bytes; Timestamp is length-prefixed canonical UTC RFC3339; IDs, keys, text and bytes are U64BE length plus UTF-8 or raw payload. Lists are U64BE count plus members; option is byte 0 or byte 1 plus value; union is zero-based declaration-order U32BE plus fields; struct is printed field order. Ordered sets sort full encodings, reject duplicates, then encode as lists. Ratio is reduced U64BE numerator and nonzero denominator. Field names, JSON, host, clock and locale are excluded. AppliedStateSnapshot.canonical_payload deterministically exports units, tags, drops and ledger in stable key order or refuses.

   H(tag,v,x)=SHA-256(U32BE tag length || raw ASCII tag || U32BE(v) || CE1(x)). Closed assignments: F uses mc.d5.F.v1 over {normalization_version,excluded_additions,messages:NormalizedMessage}; printed normalized fields include position, ordinal, index and normalized tool endpoints but omit MID, attempt_id and retry. A normalized endpoint is source position, ordinal and block index; unresolved endpoints are none. Allocated MID changes cannot change F, while changing a linked source coordinate does. manifest_digest uses mc.d5.manifest.v1 over ManifestV1; archive_id mc.d5.archive.v1 over {schema_version:1,encoding_version,manifest,V,A} without archive_id; applied_state_hash mc.d5.applied.v1 over A. source.sha256 and served.sha256 use mc.d5.block.source.v1 and mc.d5.block.served.v1 over bytes; frozen equals served. Unit projection uses mc.d5.unit-projection.v1 over {unit,row_version,bytes}; aggregate uses mc.d5.projection.v1 over {row_version,units:ordered list<{unit,bytes}>}. KnownBytes uses mc.d5.known-bytes.v1; recognition suffix mc.d5.recognition-suffix.v1; request bytes mc.d5.envelope-request.v1; tool, guidance and system surfaces use mc.d5.surface.tool.v1, mc.d5.surface.guidance.v1 and mc.d5.surface.system.v1 over exact normal UTF-8 bytes. Digests exclude themselves. Upload and chunk digests are plain SHA-256 over ordered raw upload or chunk; fixture hashes are plain file SHA-256.

   mc-store owns canonical::encode and canonical::fingerprint; consumers share crates/mc-store/tests/fixtures/d5-canonical-v1.json with one fixed vector per assignment. Sentinel preimages are explicit: empty F has normalization_version 1 and empty excluded_additions and messages; served uses raw bytes tail plus LF; unit projection uses unit u1, row_version 7 and raw bytes red; KnownBytes system uses present raw bytes system. Their digests are respectively 8273dd5001275e071997f82a4d46744d643b5ae58748b5852d3d05e204189103, 6ccf580d213ba5843f6c4073d3f9a3885be25a84ce8c26ba1e68765f9b97bd55, 3dc9079367264990f8614660b3f0a1f5ab3b133c4ed3e841bff793f38a84f90a and 650555e729bf422eb271d9134fc00508dbd63b6bebeeffeccd0f1df045ab0722. Rust and gateway reproduce constants independently.

2b. **Session identity mapping.** SessionId and SessionKey are distinct wrappers over one exact persisted UTF-8 conversation-key string; conversions are total byte-preserving inverses with no join or normalization. Status maps session_id to owner_key; explicit delete reverses it. Tests pin both column names. Negative receipts invent no archive; absent options stay absent. Snapshot bytes and archive_id are immutable while lifecycle, owner, commands and post_redeem may change; successor_key and edge appear only at redeem. Missing proof is not empty. References carry predecessor_key, nullable successor_key, lineage_id, owner_key and incarnation; blobs may have multiple owners.

### lifecycle ops

3. **Operations and result discipline.** The MC operation surface is:

```text
attempt.ticket(P: SessionKey, agent: AgentId, incarnation: Incarnation) -> TicketResult
prepare(P: SessionKey, agent: AgentId, F: MaterialFingerprint,
  source_segment: PrepareSource, attempt_id: AttemptId, incarnation: Incarnation,
  admission_ticket: AdmissionTicket, ingress: IngressEvidence,
  budget: PrepareBudgetEvidence) -> PrepareResult
attempt.resolve(P: SessionKey, agent: AgentId, incarnation: Incarnation,
  attempt_id: Option<AttemptId>, F: Option<MaterialFingerprint>,
  admission_ticket: Option<AdmissionTicket>) -> ResolveResult
redeem(edge: LineageEdge, receipt_id: ReceiptId, incarnation: Incarnation,
  observation: RecognitionObservation) -> RedeemResult
release(receipt_id: ReceiptId, release_attempt_id: ReleaseAttemptId,
  incarnation: Incarnation, never_send_proof: NeverSendProof) -> ReleaseResult
cancel(receipt_id: ReceiptId, incarnation: Incarnation,
  assertion: CancelAssertion) -> CancelResult
lineage.begin(attempt: AttemptKey, ticket: AdmissionTicket, kind: source_segment,
  total_bytes: ByteCount, total_chunks: integer, digest: Digest) -> BeginResult
lineage.put(upload_id: UploadId, seq: ChunkSeq, bytes: base64) -> PutResult
lineage.finish(upload_id: UploadId, digest: Digest) -> FinishResult
```

   Resolve all-none is a write-free restart snapshot; all-present closes; partial arguments return typed invalid_arguments plus transactional fence and generations. Ticket rejects only invalid argument shape, writes no row and changes neither generation; every well-formed ticket call returns a sample. MC validates identities and evidence. Fresh prepare is SEALED and terminals replay; existing and already_released confer no send right. Absence and PREPARING grant nothing; timeout is UNKNOWN.

3a. **Typed operation transport.** Unary JSON is one version-1 request or matching response with op; nested payload kind remains available, so begin carries op=begin and kind=source_segment without duplicate keys. Bytes are base64, absent options omitted, PFence.none present. Lifecycle refusal with proof and fence stays inside prepare or resolve; validation and upload conflicts stay in their op result. Only framing, unidentifiable JSON, service absence and authority denial use outer ErrorBody. Fixtures pin SEALED prepare, begin, and rowless resolve beside another SEALED fence.

4. **Stateless tickets and generation fencing.** attempt.ticket samples the fence without write, issuance record, ledger, custody, orphan or ticket_id; concurrent calls and calls beside seal still write zero ticket rows, and lost samples are discardable. A post-seal call samples the advanced resolve_generation. Admission and seal compare its generation and incarnation, and delayed handlers never refresh it. resolve_generation closes potential seals; fence_generation protects ordinary admission; neither is a delivery or timeout counter.

5. **Projection and positional identity.** Before seal, applied units reduce and other blocks use defer serialization; normalized tool endpoints use source position, ordinal and block index, with none for unresolved links, never allocated MID; summary instruction creates no turn, reduction or command. Preserve order, grouping, roles, kinds and tool arcs. native_mid is positional and distinct from block index; manifest maps it to predecessor_identity used by A and tags, while serving uses native_mid. Issued IDs persist; seal allocates new positions deterministically; equal bytes retain distinct MIDs and retries reuse them. F is clause 2a. Same-F aliases share one allocation; mismatch refuses and losers bind nothing. Tags commit with A and receipt CAS. Intended identities are normal, never summary.

6. **Atomic seal and freshness.** After awaits and encoding, one no-wait transaction commits archive refs, SEALED, identities, P.sealed_for, revert_epoch and fence_generation. CAS checks incarnation, original ticket, tombstone, P and A, frontiers, drops and all dependencies. Changes recompute; stale snapshots retry whole read and CAS. Gateway timeout never aborts MC. Pre-seal assembly publishing later fails epoch before append.

7. **Prepare identity and coalescing.** CAS chooses one receipt and manifest for same-(P,agent,F) and records every observer alias. Refused attempts never revive. A different-F loser gets p_already_sealed with diagnostic winner ID but no authority; its input remains queued or refused. Both winner orders are pinned. After RELEASED, a new attempt may reuse bytes but not historical A or identities.

8. **Authoritative attempt resolution.** One transaction returns outcome, P fence and generations; later generations never erase positives. Absent resolution writes an incarnation tombstone when possible; at saturation it writes no row and bumps resolve_generation only for a valid ticket. Rowless REFUSED is resolved_absent with no receipt and generation_fence from the original ticket, fenced_by=ticket.resolve_generation+1. Its replay is identical and never re-bumps; delayed prepare returns seal_after_resolve with that proof and no archive. New ID and ticket may proceed; no refresh. NegativeProof is only tombstone_row or generation_fence. Separate response, attempt and tombstone counters count replay only as response.

9. **UNKNOWN, interleavings and admission.** Elapsed D, missing row, read absence and false delivery bit prove no freedom. Resolve bound identity and ticket; restart is all-none. p_fence still reports other attempts. Seal-first returns receipt; resolve-first blocks; lost reply retains barrier. Absence, UNKNOWN and PREPARING grant nothing. Admission holds ingress through admit or consumes current fence_generation; cached none is invalid.

10. **Redeem and successor custody.** Each new UUID36 receipt seals one random 96-bit token as 20 unpadded lowercase RFC4648 base32 characters; aliases replay it. Gateway emits `<summary>\n` + FIXED_INNER + ` mc-d5:<UUID36>:<base32_20>` + `\n</summary>`; the 64-byte marker (leading space included) sits inside the tags; receiver recognition, never client extraction or SSE, is authoritative. Gateway scans only scalar or text-block content of the first native user message and sends RecognitionObservation. MC requires role user, index 0, kind text, direct_scalar or text_block, matching bytes digest, durable may_have_replied or ack, and exactly one marker for current SEALED (P,agent,incarnation). Zero, duplicate, conflict, forbidden source or pre-SEND absence fails; wrapper, prefix, response and bare placeholder prove nothing. LineageEdge binds token and native block separately. Failure keeps SEALED, increments owner-scoped unrecognized_successors with bounded prefix, forwards nothing and mints no S; mismatch is seal_material_mismatch.

   One transaction then validates fences, binds edge, mints S, copies history and A, installs carry, transfers custody, sets frontiers and marks REDEEMED. Pre-commit crash leaves SEALED; retry returns the same S; different or missing S refuses (missing S is lineage_corrupt, never reminted); REFUSED and RELEASED cannot redeem.

11. **Terminal release and cancel.** Release and redeem share one CAS. Stable (receipt_id,release_attempt_id) survives unknown reply: SEALED becomes RELEASED or replays; REDEEMED gives already_redeemed. It restores frozen commands to P with provenance, ledger and target dedup before clearing fence; epoch never rolls back. Cancel records only a hint, never undo, discharge, retention or deletion.

   Lifecycle: absent→SEALED→REDEEMED or RELEASED; alternatively absent→REFUSED with row or generation proof. REFUSED creates no new archive or admission fence; rowless logical refusal is still fenced. Terminal attempt never returns to SEALED. Forced deletion is an explicit destructive exception, not a successful preservation state.

12. **Commands across lifecycle cuts.** Freeze T drops in A. Transactional lookup routes SEALED to receipt, REDEEMED to current descendant if carried, covered if folded or retired if reduced, and RELEASED to P. Dedup target; races join transfer or new owner. Provenance and protection persist. Folded is covered, never fake reduction; partial and covered require clause 27.

### receipt and archive

13. **Archive authority and crash atomicity.** archive_id is clause 2a's hash of manifest, V and A. Source hashes identify material, but no alternate dropped raw body is stored. Projection excludes clocks, hosts and unrelated versions; incomplete encoding refuses. External staging needs orphan cleanup and atomic receipt reachability before send permission and is never SEALED. Pre-seal crash leaves no half archive or fence; UNKNOWN resolves. Lost commit replies recover the receipt; post-placeholder crash retains SEALED for the edge. Only source-independent, state-independent and authority-independent events commute.

14. **Positive-discharge retention and deletion.** Never time-sweep SEALED; expose age_seconds and exclude it from last_activity GC. Cancel does not discharge. REDEEMED follows S. Only RELEASED starts 24-hour forensics; blobs need zero references, and REFUSED absence removes no negative proof. Redeem transfers references P→S; nested custody transfers or is self-contained. Unforced SEALED delete refuses; forced delete is logged. Clause 27 owns deletion.

15. **Negative quotas and identity retirement.** Recreation advances incarnation; stale work refuses. Keep current negatives while sealing is possible and retire stale incarnations oldest-first. attempt_quota and negative bounds reject growth; aliases cannot evade them and saturation fences rowlessly. Retire optimization rows only with replay intact, never possible delivery. Gateway allocation retires after terminality, non-replyable aliases, no custody and incarnation or retention boundary. TTL never removes UNKNOWN, MAY_HAVE_REPLIED or nonterminal work. Issued IDs, MID high-water and descendant identities never reuse; retain release proof.

16. **Budget evidence and unknowns.** Gateway supplies normal raw system and tool bytes plus geometry; MC estimates all input and resolves reserve. Summary max_tokens is irrelevant. Known empty is valid; absent is unknown; reserve flag conflict refuses. Unknown geometry, reserve, segment, estimator, fixture, byte cap or identity is budget_unknown, never zero. Model mismatch is typed; other mismatch names field and caps retain values. REFUSED concerns one attempt and never enters lineage_protocol_passthrough.

16a. **Output reserve resolution.** D5 v1 gets R only from lineage.output_reserve.tokens_by_model[model]. Gateway values, summary max_tokens and prose cannot override it. Calibrated Claude 200,000 defaults R=20,000 with source=config; missing entry, units or provenance gives budget_unknown. window-geometry remains non-D5 only. The none_declared pin soft=167,000, hard=200,000, R=20,000, X=140,000 records fit_soft=147,000, hard_required=160,000 and both true.

17. **Two-variable budget test.** X estimates system, tools, m0, m1, full carry and policy_reserve; R is output_reserve_mc.tokens. Both carry model and estimator provenance; reserve_accounting selects:

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

   R>soft_bounded returns REFUSED token_cap and cap {cap:output_reserve,actual:R,limit:soft_bounded,units:tokens}, never wrap. BudgetRecord preserves geometry and R; fit_soft and soft_ok are unknown{reason:output_reserve_exceeds_soft_bounded}; clamp_applied=false; known X still yields hard_required=X+R and hard_ok. Ordinary soft failure also records hard_ok. Claude none_declared uses soft 167,000, hard 200,000 and R 20,000. once_carved soft 180,000 accepts X 170,000; above-hard remains independent. Retain operands, results, caps, provenance and unknowns.

18. **Policy reserve and drift pin.** Gateway owner supplies wrapper fixtures only for slice-0 import; runtime supplies none. MC fixes receipt and nonce, appends the exact 64 B marker, and computes ceil(max tokenizer[306 B reminder + placeholder + dynamic-max wrapper]×(1+margin)); reminder and suffix count once. Missing profile or fixture is budget_unknown; drift above 10 percent fails, otherwise the estimate enters X. At 512 B path, the three R14 samples are 485–487 before margin and 558, 559, 561 after; policy_reserve is their max, 561 estimate. Pre-R14 510 is only the drift baseline. Recompute with identical provenance before mint. Post-seal surprise is preserved_but_blocked.

19. **Independent byte caps and full specimen sizing.** Gate archive, manifest, A, carry frame and envelope bytes independently. Gateway ingress is 64 MiB; the smaller gateway or MC frame bound wins, and archive has explicit config. Size the full specimen union including untagged tools, reasoning, system, m0, m1 and policy reserve; 83 tags and 24,073 stored tokens are only fragment mass. Soft refusal below hard is policy. Every render rechecks actual tokens and bytes.

### successor semantics

20. **Every-pass carry.** TransformResponse.d5_carry serves carry after m0 and m1, before continuation, on first HARD, every DEFER, growth and restart for a redeemed D5 successor; serde defaults and omits none; absence means no receipt-backed carry this pass. Validated folds or reductions alone retire intervals; partial fold causes SOFT. Carry precedes retention and pruning; validation is clause 21a; tags, versions, hosts and fresh digests authorize nothing.

21. **Distinct frontiers and proofs.** folded_frontier ends real compartments, first_inherited_ordinal starts carry, and source_frontier MAY advance. lineage_anchor and native_continuation_identity are live-native; frozen rebase_base alone rebases. Specimen: folded 1798, carry 1799–1939, base 1939, continuation 1940. Folds advance coverage_identity, never native continuation; no fold is none. Eligible head is folded_frontier+1 or first inherited. Historian uses archive plus suffix without floor change. Receipt or real-compartment proof drives coverage; native continuation and base drive rebase, marker, reconcile and trim. Native absence never truncates carry.

21a. **Gateway carry-proof path.** While a receipt-backed carry is locally outstanding (REDEEMED, not discharged by verified fold, reduction or custody transfer) the gateway evaluates d5_carry first every pass, native anchor or not. Required: schema_version=1; the locally REDEEMED receipt_id for (P,agent,incarnation); the sealed manifest_digest; receipt-backed coverage_proof or real-compartment proof covering the required members at or beyond folded_frontier; row_version equal or above last accepted. Custody is proved by locator: before gateway markers the array is m0, m1, carry span, native suffix; a member (mid, index, ordinal) is found via the sealed manifest entry with that native_mid and block index; the carry span is the run of returned messages whose mids the manifest names, in manifest order, each once; a folded or reduced member is absent or its committed unit, never source; location is by mid and index, never offset; frozen served_sha256 under mc.d5.block.served.v1; reduced digest under mc.d5.unit-projection.v1; aggregate projection_digest under mc.d5.projection.v1; 2a forms only. Metadata, high ordinal, flag or empty boundary is insufficient; missing: 503 d5_carry_proof_missing; mismatch: 503 d5_carry_proof_mismatch; never raw or upstream. Absence declares no served carry, discharging nothing. Native checks then run unchanged: envelope, provider shape, native-owned lineage_anchor, continuation_identity, rebase_base; after a valid proof the inherited anchor (coverage_identity, folded_frontier, first_inherited_ordinal) is never re-required natively, the original gap. Omitted carry fails beside a valid native anchor. Without outstanding carry only the native path runs with the pre-D5 anchor check; absence is normal; I54–I62 pin these.

22. **Anchor absorption and sequence accounting.** Only lineage_boundary is coverage and overlap-exempt. Coverage excludes it; max_sequence and m0 and m1 compartment sequences include it; partition excludes by type; rendered m1 advances only on real compartments. Partial 1799–1905 leaves anchor 1940 and remainder. Crossing 1799–1950 atomically removes it while preserving native_continuation_identity and anchor MID; coverage advances while continuation and base stay. Snapshot, generation and append agree.

23. **Expansion and nested descent.** Inherited ctx_expand precedes cache and raw fallback, returning V with current permitted reductions, never dropped originals. Range mode resolves carry before cutoff and merges ordered archive and ordinary ranges with existing bounds and identity dedup; restart uses custody. Nested prepare reads S archive because native source excludes carry, unions and budgets new tail, and applies hop cap 5 with middle-hop collapse. Dedup receipt plus block identity, re-project current S A and re-mint hashes; custody survives ancestor deletion.

24. **Post-seal overflow preserves custody without a fit promise.** Recheck every S body. SuccessorOverflow yields gateway 503 and preserves input; actual_source is mc_estimate unless provider count exists. Never send, raw-forward, discard carry, undo REDEEMED, remint or degrade. Atomically persist count, diagnosis, last_overflow, blocked and bounded overflow_probe. The pass arms eligible carry; empty carry, system and tools over hard, too-small chunk or unavailable producer records no-arm. Durable armed→assembled→published uses ordinary historian guards and survives restart. Missing producer refuses; publication promises no fit.

### consumer rebind

25. **D5 isolation and accessor fence.** D5 changes require a receipt or lineage_boundary; OpenCode, Pi, COALESCE(MAX(end_message),0) and unlisted callers stay unchanged. The Option sibling's five (file,symbol) callers are mc-module/src/lib.rs::prepare_historian_fire and ::handle_ctx_expand_facade, mc-module/src/transform.rs::coverage_ordinal_from_compartments and ::apply_once, and mc-module/src/m0_compose.rs::compose_m0_from_store_timed. The mc-store accessor is only the definition. d5_option_callers_are_a_closed_set_of_five uses literal 5 against declared and discovered pairs, ignores line drift and fails set drift.

25a. **Receipt-absent descent.** lineage.d5.require_receipt_for_descent ships true for Claude Code D5 after the bounce; false is explicit non-D5 only. Nonempty uncovered tail without matching SEALED returns d5_receipt_required without mutation. False retains legacy descent and writes mc_d5_uncovered_descent{owner_key,lineage_id,first,last,observed_at}; the existing specimen keeps (1,3),(4,6),(11,11), coverage 11 and reports 7–10. Clear only after full redeemed or real-fold proof, or owner deletion. I12 has a receipt. Redeem uses its dedicated lineage_corrupt variant; other ops use the refusal reason.

26. **Single consumer inventory.** mc-store and mc-module paths are under crates/. Re-cite rows at dispatch; each has one slice and owner. The three TS rows across two files are not-this-campaign.

| Consumer and source site | Owner | Observable |
|---|---|---|
| `mc-store/src/lib.rs:10916,10997–11007,11080–11095,11230–11244` | s1 mc | Native continuation, real coverage, archive. |
| `mc-module/src/compartment_coverage.rs:180–202` | s3 mc | Coverage and anchor sequence. |
| `mc-module/src/m1_compose.rs:313,328–344` | s3 mc | Real-compartment coverage only. |
| `mc-module/src/m1_compose.rs`, partition_by_folded_seq and m1_compartment_seq | s3 mc | Anchor excluded; sequence included. |
| `mc-module/src/m0_compose.rs:434–449` | s3 mc | Option caller; ordinal and sequence. |
| `mc-module/src/transform.rs:7296–7308` | s3 mc | coverage_ordinal_from_compartments. |
| `mc-module/src/transform.rs:4150–4151` | s3 mc | apply_once coverage and trim. |
| `mc-module/src/transform.rs:4744–4751,4753–4789,4779–4790` | s3 mc | History plus carry. |
| `mc-module/src/transform.rs:4812–4854,4912–4917` | s3 mc | Receipt and native proofs. |
| `mc-module/src/transform.rs:5261–5309` | s3 mc | Partial carry and native trim. |
| `mc-module/src/lib.rs:5198–5260` | s4 mc | Historian Option caller. |
| `mc-module/src/historian_chunk.rs:626–674` | s4 mc | Carry plus suffix snapshot. |
| `mc-store/src/lib.rs:12451–12495,16663–16713` | s1 mc | Absorption transaction; s3 semantics. |
| `mc-store/src/lib.rs:10445–10473` | s1 mc | Accessor definition; non-D5 isolation. |
| `mc-module/src/lib.rs:11907–11935` | s4 mc | Projection before fallback. |
| `mc-module/src/lib.rs:11952–11999` | s4 mc | ctx_expand Option caller. |
| `mc-module/src/transform.rs:7843–7864` | s3 mc | Membership proofs. |
| `mc-module/src/transform.rs:7866–7904` | s3 mc | Coordinate-specific proofs. |
| `mc-module/src/transform.rs:7894–7995,8002–8009` | s3 mc | Coverage, trim, endpoint. |
| `mc-module/src/transform.rs:6134–6140` | s3 mc | Coverage versus native marker. |
| `mc-module/src/transform.rs`, continuation_summary_anchor and the NotCompactionShape and ObservedFlagMissingShapePresent dispositions | s4 mc | Marker dispositions. |
| `packages/plugin/src/features/magic-context/compaction-marker.ts` | outside | Shape parity only. |
| `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:682–712` | outside | Native mapping. |
| `packages/plugin/src/hooks/magic-context/rust-mode-transform.ts:3377–3379` | outside | Native presence. |
| `mc-module/src/decay_render.rs:45–53,235–237,328–342,395–398` | s4 mc | Empty parity; D5 exclusion. |
| `mc-module/src/transform.rs:37308–37329,37371–37441` | s4 specimen | I12 tail. |
| `mc-module/src/transform.rs:4922–4939` | s3 mc | Carry permissions. |
| `mc-store/src/lib.rs:11070–11076` | s1 mc | Reduction-permission read. |
| `mc-store/src/lib.rs:7444–7490` | s1 mc | Owner deletion. |
| `mc-store/src/lib.rs:7697–7829` | s1 mc | Health snapshot. |
| mc-module/src/lib.rs::handle_session_status_value | s2 mc | ManagementSurface attachment. |
| `mc-module/src/transform.rs:2540–2589` | s3 mc | Frozen rebase_base. |

26a. **Status surface.** session.status always adds sorted d5:HealthD5 for mapped owner_key, incarnation and lineage. Receipt, corruption, refusal and recognition data use current owner_key; descent uses owner_key plus lineage_id. After REDEEMED, all post_redeem data belongs to S; P has no S relief and blocked {preserved_but_blocked:false,overflow_refusals:0}. One snapshot read composes it. No rows means empty lists, zero counters, no prefix, that blocked object and state ok. Other surfaces only relay.

27. **Fixtures, store fence and disjoint slices.** The MC specimen is on master at crates/mc-module/tests/fixtures/d5-specimen: source-segment-v1, scaffold manifest and archive with digests pending, README, fixture-index-v1 (per-member provenance, per-file SHA-256, readiness, gateway per-artifact hashes), a sha-gated generator and test. Slice 0 imports the remaining owner data (d5-canonical-v1.json, I34's 141-message union, both anchor JSON files, policy-reserve-v3 inputs with the three R14 samples) into fixture-index-v1; OpenCode and Pi goldens are re-cited; no directory census or identity-table JSON. Slice 0 fills the specimen digests from independent CE1 preimage vectors, never the codec under test, then freezes them; input digests live in R4a and the index; missing input fails. I1–I62 cover product and gateway work.

   Migration mc_store_d5_lineage_v1 adds all named D5 tables and mc_reduce_command_ledger partial and covered; #2732 is not banked. D5 state never uses legacy JSON. Every D5 table has owner_key and no session_id, including shared blobs; explicit deletion uses 2b.

   Rollback backport is out of scope; the substitute deployment guard blocks an older ck-mc lacking the D5 gate before it opens, transforms or admits a D5-bearing store, preserving bytes; no slice builds that binary. Ungated downgrade is unsupported; the route returns d5_downgrade_refused and non-D5 stays unchanged.

   Slice 1 owns mc-store schema, CE1, upload, lifecycle, reads, CAS and deletion, then one fence-bearing coordinated ck-mc bounce by the release owner. Slice 2 owns lineage route, wire, budget, status attachment; dispatch re-cites the ManagementSurface file and handle_session_status_value region in mc-module/src/lib.rs. Slice 3 owns carry, d5_carry, digests, identities, absorption, overlap, rebind, nesting; dispatch re-cites apply_once as the second coverage symbol beside coverage_ordinal_from_compartments. Slice 4 owns historian, ctx_expand, markers, parity, specimens. Hop cap 5 stays; regions are disjoint; transform.rs is sequential; release waits for gateway shared-contract asks; slice workers never restart.

28. **Defect and verification.** Preserve 1799–1939 after the failed revert_epoch; memory14536 is not proof. Numeric counts are Rust literals in named seams compared independently with declared input and implementation output; no JSON supplies expectations. Silent guards follow green, staged empty diff, NON-VACUITY BREAK, red named test, restore and empty diff. Record failures; directly assert hard_ok, hard_required and admission. Undefended controls need a same-target red control. Documentation changes no product or spec state.

### transport

29. **Dedicated route and trusted authority.** ck-mc InternalService{module_id:"magic-context",service_id:"mc.lineage"} admits only daemon-stamped Principal::Reserved{module_id:"thalamus"}; BindIdentity is untrusted. ManagementSurface stays open. No MCP exposure, generic wire change or pin bump. Unary Vec<u8> JSON bodies never set FLAG_BINARY, allow at most 64 MiB and do not stream requests. Typed outcomes remain whole; only framing, authority and unidentifiable JSON use ErrorBody.

30. **Chunked prepare and upload custody.** Gateway MUST use inline prepare at serialized body≤1 MiB and ref above; raw_chunk_bytes≤1 MiB and serialized_body_bytes≤64 MiB are distinct, so the larger base64 put frame is valid. source_segment uploads use ordered raw SHA-256. begin keyed by (AttemptKey,ticket,kind,digest) replays upload_id; changed totals refuse. Same seq and bytes replay; changed bytes conflict; finish verifies and replays UploadRef. Digest failure deletes only unreferenced staging. Invalid tickets cannot finish or reference; staging expires only with terminal attempt or new incarnation. max_concurrent_per_attempt=2, max_bytes_per_attempt=48 MiB raw and max_chunk_bytes=1 MiB raw are per-attempt aggregate limits; accepted seq counts once. D expires neither staging nor seal.

### gateway target semantics

31. **Durable attempt, ingress and reply gate.** Persist attempt identity, F, lineage, incarnation, ingress and positional allocation before sampling; CAS binds one ticket before prepare. receipt_id begins none and MAY_HAVE_REPLIED false. Before sending, persist MAY and acquire authority. Unbound-ticket crash means no prepare; bound means resolve without re-prepare. Recovery uses records. Per session-agent ingress drains lower work; higher work uses max_pending_per_session_agent=1 and max_wait_seconds=120, with excess 503 preserving input. D is not cardinality. SEALED blocks P, REDEEMED routes S and RELEASED needs fresh admission; never replay raw or transformed input.

32. **Release trigger and irreversible send revocation.** Release needs cancellation or deadline, all aliases closed, no retry in flight and all MAY_HAVE_REPLIED false; otherwise resolve, redeem or hold. Send authority atomically performs RELEASE_INTENT→NEVER_SEND before release and forbids later sends, registration or bit flips. NeverSendProof binds receipt, incarnation, aliases, retries and revocation ID. Crash retries the same release_attempt_id. Reply, registration, redeem and release have one winner; sockets do not prove non-delivery.

33. **Retained normal envelope lifecycle.** Retain one atomic whole envelope per session-agent-lineage-incarnation: latest managed normal committed send, including forwarded retry and excluding summary, rejected and unmanaged sends. MC stores receipt evidence, not this mutable cache; unusable evidence is unknown.

   Constraint one, order: committed send, not completion, wins; ingress and durable causality order it; identity matches; late completion cannot overwrite; newer unusable tombstones old. Constraint two, persistence: write pending{sequence}, envelope, clear, then send; a leftover marker is newer_unusable; failed persistence prevents send; restart without persistence is budget_unknown; BootId alone does not invalidate; records never mix. Constraint three, identity: compare normal-successor geometry, profile, guidance, tool and system hashes; model mismatch is budget_model_mismatch, any other field budget_evidence_mismatch; age is causal distance, not TTL; uncertainty is budget_unknown. Constraint four, witness: MC incarnation detects older restore; foreign or inconsistent values fail; equal-incarnation and joint rollback stay undetectable, so uncertainty refuses. Constraint five, refusal: unusable evidence is pre-seal budget_unknown without fallback; AUTO resumes ordinary history, manual errors, UNKNOWN resolves under the current fence; clause 24 handles post-seal overflow.

34. **Prepare wait and valid late seals.** D=120 seconds is only the gateway prepare wait before compaction 503, not queue depth, MC deadline, receipt life or staging expiry. MC MUST NOT expire elapsed-D work and the gateway never resets D. A D+epsilon seal resolves SEALED without refresh or re-prepare. An open request sends the verified placeholder, observes the successor, then redeems; otherwise hold or release with proof. D never removes custody or bypasses admission.


## acceptance sketch

Syntax is `I-id: test | target | slice | owner — observable`; list marker is immaterial. Slices are s1–s4 or gw; owners are mc, e2e-specimen or gateway-not-shipped. Names unique; each test has one slice, and gateway rows use gw and gateway campaign. MC and specimen rows close here; gateway rows there; simulators do not count. d5_acceptance_identity_cardinalities checks rows and gates against Rust literals 62 and I1–I62, without JSON.

I1: d5_placeholder_requires_matching_sealed_receipt | gateway campaign | gw | gateway-not-shipped — only matching SEALED, F, token and send right emit; invalid redeem refuses.
I2: d5_carry_requires_exact_ordered_membership | crates/mc-module/src/transform.rs | s3 | mc — served and never-served blocks, reductions, tools and equal bytes retain coverage or ordered V and distinct MIDs.
I3: d5_inherited_expand_never_resurrects_reduced_body | crates/mc-module/src/lib.rs | s4 | mc — mixed expansion before and after reduction and restart uses current archive projection before cache.
I4: d5_atomic_seal_publisher_race | crates/mc-store/src/lib.rs | s1 | mc — publisher-before-seal recomputes or refuses; publisher-after-seal fails epoch before append.
I5: d5_coalesced_allocation_mismatch_refuses | crates/mc-store/src/lib.rs | s1 | mc — same F aliases one receipt and allocation; different F loses; post-release uses a new attempt.
I6: d5_crash_atomic_receipt_and_successor | crates/mc-store/src/lib.rs | s1 | mc — every archive, seal, redeem and release cut is atomic; missing redeemed S is lineage_corrupt.
I7: d5_no_real_compartment_carry_is_eligible | crates/mc-module/src/lib.rs | s4 | mc — folded 1798, carry 1799–1939, anchor 1940, no fold and real ordinal 0 choose the right eligible head.
I8: d5_unreduced_carry_rejects_mutated_bytes | crates/mc-module/src/transform.rs | s3 | mc — HARD, repeated DEFER, partial fold, reduction and restart validate frozen or committed-unit bytes.
I9: d5_ingress_bounds_preserve_input | gateway campaign | gw | gateway-not-shipped — watermark drains low work and separately bounded pending count and wait return 503 without loss.
I10: d5_sealed_and_shared_archives_survive_sweep | crates/mc-store/src/lib.rs | s1 | mc — aged SEALED and cancel retain; RELEASED starts 24 hours; last reference controls blobs.
I11: d5_output_reserve_none_declared_uses_config_once | crates/mc-module/src/lineage.rs | s2 | mc — full union records R=20,000, independent soft and hard, bytes and unknowns without zero-fill.
I12: d5_specimen_tail_content_on_successor_passes | crates/mc-module/tests/d5_specimen.rs | s4 | e2e-specimen — receipt-present ten-message fixture preserves nonempty 7–10 content, expansion, eligibility and repeats.
I13: d5_resolve_before_seal_prevents_commit | crates/mc-store/src/lib.rs | s1 | mc — real two-connection interleavings produce one seal or one durable negative fence.
I14: d5_nested_descent_carries_outstanding_union | crates/mc-module/src/transform.rs | s3 | mc — P→S→S2 applies current A, preserves distinct origins, survives deletion and obeys hop cap 5.
I15: d5_all_coverage_consumers_use_real_folded_frontier | crates/mc-module/src/transform.rs | s3 | mc — all s3 inventory sites separate real 1798 coverage, native 1940, receipt proof, crossing absorption and parity.
I16: d5_refused_attempt_does_not_admit_fenced_p | crates/mc-store/src/lib.rs | s1 | mc — A refusal and B seal resolve attempt outcome separately from current P fence and generations.
I17: d5_admission_consumes_current_fence_generation | crates/mc-store/src/lib.rs | s1 | mc — a none snapshot cannot admit after a concurrent seal.
I18: d5_release_revokes_cached_reply_permission | gateway campaign | gw | gateway-not-shipped — cached SEALED, registration and delivery-bit races have one send-authority winner.
I19: d5_release_restores_pending_and_postseal_commands | crates/mc-store/src/lib.rs | s1 | mc — commands around release, redeem and nesting reach current owner once after ledger migration.
I20: d5_release_redeem_have_one_terminal_winner | crates/mc-store/src/lib.rs | s1 | mc — release and redeem race to one terminal CAS and lost replies replay without duplication.
I21: d5_different_f_winner_orders_are_pinned | crates/mc-store/src/lib.rs | s1 | mc — both winner orders return p_already_sealed diagnostically and preserve losing input.
I22: d5_resolve_at_cap_rejects_prearrival_old_ticket | crates/mc-store/src/lib.rs | s1 | mc — saturated negatives still fence delayed old prepare and permit a new ID and ticket.
I23: d5_stale_incarnation_prepare_never_seals | crates/mc-store/src/lib.rs | s1 | mc — delete and recreate rejects stale prepare and resolve without MID reuse.
I24: d5_delete_ancestors_preserves_descendant_archive | crates/mc-store/src/lib.rs | s1 | mc — SessionId mapping, owner deletion, forced exception and P→S→S2 references preserve descendants.
I25: d5_quotas_and_identity_retirement_are_bounded | crates/mc-store/src/lib.rs | s1 | mc — attempt and negative bounds fence without unbounded growth; terminal replay and canonical IDs persist.
I26: d5_envelope_forwarded_retry_is_latest_normal | gateway campaign | gw | gateway-not-shipped — first absence, retry and overlapping sends select latest causal committed normal envelope.
I27: d5_envelope_newer_unusable_refuses_old_evidence | gateway campaign | gw | gateway-not-shipped — durable evidence survives boot; failed, degraded and old-incarnation evidence refuses.
I28: d5_envelope_rollback_limits_are_honest | gateway campaign | gw | gateway-not-shipped — identity mismatch or uncertainty refuses without universal same-incarnation rollback claim.
I29: d5_budget_uses_intended_normal_projection_identity | gateway campaign | gw | gateway-not-shipped — normal model, tool, system, guidance and profile mismatches are typed and field-specific.
I30: d5_successor_first_pass_rechecks_actual_geometry | crates/mc-module/src/transform.rs | s3 | mc — larger actual successor surface returns overflow and preserves input without send, raw fallback or remint.
I31: d5_unknown_resolves_without_summary_fallback | crates/mc-module/tests/d5_specimen.rs | s4 | e2e-specimen — UNKNOWN resolves under the current fence to durable outcome; MC creates no summary, boundary or history rewrite, and prior history stays byte-identical. Client recovery remains cited evidence and a gateway gate.
I32: d5_carry_membership_forgery_is_rejected | crates/mc-module/src/transform.rs | s3 | mc — omissions, duplicates, forged versions, invented units and host serialization reject.
I33: d5_crossing_fold_keeps_original_continuation_identity | crates/mc-module/src/transform.rs | s3 | mc — partial, crossing, repeated and nested folds freeze native_continuation_identity and rebase_base under generation guards.
I34: d5_budget_sizes_full_projected_specimen_union | crates/mc-module/tests/d5_specimen.rs | s4 | e2e-specimen — 141-message fixture sizes full carry, not 24,073 tagged tokens.
I35: d5_non_d5_filtered_noise_coverage_unchanged | crates/mc-module/src/transform.rs | s4 | mc — OpenCode and Pi sparse, empty, same-key and ordinal-zero goldens remain byte-identical.
I36: d5_budget_unknown_is_not_zero | crates/mc-module/src/lineage.rs | s2 | mc — known empty succeeds while missing segments, R, fixtures, flags or estimator refuse with provenance.
I37: d5_generation_refusal_replay_is_identical_without_rebump | crates/mc-store/src/lib.rs | s1 | mc — rowless A proof stays fixed as B advances; response, attempt and tombstone counters remain distinct.
I38: d5_overflow_relief_state_is_durable | crates/mc-module/src/transform.rs | s3 | mc — eligible, empty, below-minimum and over-hard variants persist diagnoses, bounded probes and guarded relief.
I39: d5_policy_reserve_current_estimate_enters_x | crates/mc-module/src/lineage.rs | s2 | mc — each of the three imported realistic markers is 64 bytes and matches its pinned suffix hash and token count; their 561 maximum enters X, while missing fixture or drift refuses.
I40: d5_fixed_placeholder_descends | crates/mc-module/src/transform.rs | s4 | mc — marker shape remains reachable while old real-summary and noncompaction dispositions keep meaning.
I41: d5_restart_resolve_read_is_non_mutating | crates/mc-store/src/lib.rs | s1 | mc — all-none restart writes nothing; ticket sample is stateless; only bound closure fences.
I42: d5_coalesced_attempt_preserves_applied_units | crates/mc-module/src/transform.rs | s3 | mc — linked tools retain units and positions; allocated MID changes leave F unchanged, while a linked source-position change alters F.
I43: d5_envelope_pending_marker_crash_tombstones_slot | gateway campaign | gw | gateway-not-shipped — pending, record, clear and send cuts never reuse an older slot or send before persistence.
I44: d5_non_d5_ordinal_zero_fold_unchanged | crates/mc-module/src/transform.rs | s3 | mc — non-D5 ordinal zero keeps old offset, boundary and floor while D5 Option stays eligible.
I45: d5_anchor_never_reenters_m1_new_compartments | crates/mc-module/src/m1_compose.rs | s3 | mc — three passes, partial and crossing folds exclude anchor coverage but include sequence watermarks.
I46: d5_bound_ticket_crash_resolves_without_reprepare | gateway campaign | gw | gateway-not-shipped — unbound crash starts fresh after fence read; bound crash resolves original sample exactly once.
I47: d5_lineage_route_requires_reserved_thalamus | crates/mc-module/src/lineage.rs | s2 | mc — forged identity and ordinary principal fail while reserved principal succeeds and ManagementSurface stays open.
I48: d5_lineage_transport_preserves_typed_outcomes | crates/mc-module/src/lineage.rs | s2 | mc — inline and ref, 1 MiB raw chunks, larger base64 frame and lifecycle results retain exact JSON algebra.
I49: d5_upload_conflicts_preserve_original | crates/mc-store/src/lib.rs | s1 | mc — lost begin, duplicate or conflicting put, finish replay and foreign mismatch preserve declaration and custody.
I50: d5_upload_attempt_quota_is_aggregate | crates/mc-store/src/lib.rs | s1 | mc — invalid ticket, incarnation, two-upload and 48 MiB bounds count chunks once; referenced bytes survive.
I51: d5_late_seal_resolves_to_valid_custody | crates/mc-module/src/lineage.rs | s2 | mc — D+epsilon resolves SEALED; MC expires nothing, resolve returns SEALED, a second prepare is p_already_sealed and same-receipt redeem mints S once. Open-request and release choices are gateway-owned.
I52: d5_release_trigger_requires_closed_never_replied_aliases | gateway campaign | gw | gateway-not-shipped — cancellation or deadline releases only closed never-replied aliases with stable identity and one winner.
I53: d5_receipt_token_recognizes_vendored_successor_anchor | crates/mc-module/tests/d5_specimen.rs | s4 | e2e-specimen — hermetic anchors accept one current first-user direct-text marker with reply evidence; all forbidden marker, source, state and pre-SEND cases keep SEALED, mint no S and update HealthD5. Scanning is gw-owned.
- I54: d5_carry_field_serves_redeemed_successor | crates/mc-module/src/transform.rs | s3 | mc — first DEFER populates d5_carry whose ordered members match returned pre-marker messages; cites gateway acceptance.
- I55: d5_carry_missing_refuses | gateway campaign | gw | gateway-not-shipped — outstanding carry plus absent field gives proof_missing; cites I54.
- I56: d5_carry_wrong_receipt_refuses | gateway campaign | gw | gateway-not-shipped — another receipt gives proof_mismatch; cites I54.
- I57: d5_carry_tampered_manifest_refuses | gateway campaign | gw | gateway-not-shipped — changed manifest gives proof_mismatch; cites I54.
- I58: d5_carry_regressed_row_version_refuses | gateway campaign | gw | gateway-not-shipped — lower row_version gives proof_mismatch; cites I54.
- I59: d5_carry_missing_member_refuses | gateway campaign | gw | gateway-not-shipped — correct metadata with an omitted member gives proof_mismatch; cites I54.
- I60: d5_carry_changed_member_bytes_refuses | gateway campaign | gw | gateway-not-shipped — correct metadata with changed bytes gives proof_mismatch; cites I54.
- I61: d5_carry_duplicate_or_reorder_refuses | gateway campaign | gw | gateway-not-shipped — duplicate or reordered members give proof_mismatch; cites MC I54.
- I62: d5_carry_native_anchor_is_not_escape_hatch | gateway campaign | gw | gateway-not-shipped — native anchor plus outstanding carry and absent field gives proof_missing; cites I54.

Parity pins D5 fixtures against non-D5 sparse coverage, noise, decay, native boundary, ordinal-zero fold and same-key expansion; OpenCode and Pi goldens stay unchanged. Nested source omits injected carry and reads S archive.

The specimen independently seeds and verifies these strings in predecessor source, then recursively searches every provider-visible successor string across tools, results, reasoning and system on S pass 1, pass 2, restart, expansion and outstanding S2 carry:
- Your parsed disk assertions are independently verified: setup intact
- 9226\t            // A second archived project, never touched by this test's — materialize \t as one literal tab.
- Take the real follow-up note1274: audit persistence-related test assertions in this repo

The ORIGINAL unmarked native user anchors are vendored from ex13508 and ex13615 (variable Claude wrapper plus the inner summary text, no summary tags); the marked anchor shapes are DERIVED fixtures, and the real marked observations are the gateway owner's container captures on Claude Code 2.1.258. Derived marked inner text: `Conversation history compacted and preserved by Magic Context. Full context continues to be served automatically. mc-d5:<36-char UUID receipt_id>:<20-char lowercase base32 nonce>`. Captured positives: marker INSIDE the summary tags, manual and automatic compaction (native inner text 177 bytes both). Captured control: marker AFTER the closing tag, MANUAL compaction only, also carried into the first native user anchor in that measured shape (automatic outside was not tested). Therefore the inside-the-tags placement is the normative EMITTER format, not a claim about client extraction; the receiver requires exactly one well-formed marker matching the scoped current SEALED receipt and never accepts arbitrary trailing text. Wrapper equality, manifest IDs, hashes, tags and predecessor-only archives do not satisfy content or recognition. Saved memory14536 and copied notes are independent controls. After legitimate reduction, require the committed reduced representation and original-body absence.

Required red-first controls inherit the owner and slice heading below. Each starts from its named passing test and follows clause 28; controls selected in the identity table retain the same assignment. Gateway controls close only in the gateway campaign.

- s0 mc: d5_fixture_import_pins_upstream_hashes (change source hash); d5_f_canonical_vector_matches_committed_fixture (change CE1 byte); d5_policy_fixture_is_independent_of_evaluator (derive expected from evaluator).
- s1 mc: d5_resolve_at_cap_rejects_prearrival_old_ticket (refresh old ticket); d5_resolve_before_seal_prevents_commit (disable tombstone); d5_refused_attempt_does_not_admit_fenced_p (admit from refusal); d5_admission_consumes_current_fence_generation (cache none); d5_release_restores_pending_and_postseal_commands (drop transfer); d5_release_redeem_have_one_terminal_winner (split CAS); d5_redeem_retry_returns_existing_successor (remint); d5_bound_edge_rejects_new_successor_key (accept new S); d5_delete_ancestors_preserves_descendant_archive (delete by predecessor); d5_delete_session_exempts_transferred_protection (reflect D5 tables); d5_sealed_and_shared_archives_survive_sweep (age sweep); d5_stale_incarnation_prepare_never_seals (omit incarnation); d5_mid_retirement_never_reuses_issued_identity (reclaim MID); d5_ticket_sample_is_stateless (persist issuance); d5_restart_resolve_read_is_non_mutating (write on restart); d5_capped_resolve_fences_by_generation (skip bump); d5_refused_by_generation_counted_separately (merge counters); d5_generation_refusal_replay_is_identical_without_rebump (use current generation); d5_generation_response_and_attempt_counts_are_distinct (count replay as attempt); d5_upload_begin_retry_returns_same_id (new retry ID); d5_upload_conflicts_preserve_original (overwrite declaration or chunk); d5_upload_corrupt_finish_preserves_sealed_custody (delete referenced bytes); d5_upload_invalidated_ticket_is_unusable (finish after invalidation); d5_upload_attempt_quota_is_aggregate (reset quota); d5_old_cache_rmw_preserves_d5_fence_rows (touch D5 table); d5_session_id_owner_key_mapping_is_total (normalize key); d5_receipt_absent_strict_refuses_without_mutation (descend in strict); d5_receipt_absent_compat_reports_range (omit incident); d5_coalesced_allocation_mismatch_refuses (accept different allocation).
- s2 mc: d5_same_source_concurrent_attempts_share_f (hash MID); d5_budget_once_carved_170k_fits_180k_soft (subtract R twice); d5_budget_overhard_override_reports_independent_hard_failure (remove hard predicate); d5_budget_unknown_is_not_zero (zero unknown); d5_output_reserve_none_declared_uses_config_once (wrong R source); d5_policy_reserve_unknown_profile_or_fixture_refuses (zero missing fixture); d5_policy_reserve_fixture_drift_fails (accept drift); d5_policy_reserve_current_estimate_enters_x (reuse old estimate); d5_lineage_route_requires_reserved_thalamus (trust BindIdentity); d5_lineage_authority_keeps_management_route_unchanged (restrict ManagementSurface); d5_upload_raw_chunk_and_serialized_frame_limits_are_distinct (cap encoded put at 1 MiB); d5_lineage_transport_preserves_typed_outcomes (flatten to ErrorBody); d5_rowless_resolve_json_preserves_proof_and_fence (outer refusal); d5_late_seal_resolves_to_valid_custody (expire at D); d5_late_upload_does_not_get_fresh_prepare_deadline (apply MC D timer); d5_status_d5_no_rows_is_explicit (omit d5); d5_status_health_is_owner_scoped (global counters); d5_recognition_suffix_recomputed_before_seal (reuse 510).
- s3 mc: d5_unreduced_carry_rejects_mutated_bytes (mutable digest); d5_carry_requires_exact_ordered_membership (omit or duplicate member); d5_legitimate_reduction_uses_unit_projection (compare frozen original); d5_nested_descent_carries_outstanding_union (native-only nesting); d5_nested_descent_reprojects_current_reduction (reuse ancestor V); d5_nested_distinct_origin_equal_bytes_survive (content dedup); d5_all_coverage_consumers_use_real_folded_frontier (use anchor or miss either coverage site); d5_non_d5_ordinal_zero_fold_unchanged (global Option); d5_anchor_never_reenters_m1_new_compartments (exclude watermark); d5_rebase_base_is_frozen_1939 (rebase from continuation); d5_coverage_receipt_proof_and_native_anchor_are_distinct (native-only coverage proof); d5_crossing_fold_absorbs_anchor_atomically (remove overlap exemption); d5_crossing_fold_keeps_original_continuation_identity (move continuation); d5_successor_first_pass_rechecks_actual_geometry (skip first gate); d5_successor_overflow_refuses_and_preserves (raw-forward); d5_overflow_arms_fold_when_carry_nonempty (omit arm); d5_overflow_no_arm_when_system_tools_exceed_hard (arm impossible case); d5_overflow_relief_state_is_durable (drop restart state); d5_carry_field_serves_redeemed_successor (omit d5_carry on first DEFER); d5_coalesced_attempt_preserves_applied_units (resolve by native MID); d5_refusal_never_forwards_raw_overlimit (passthrough); d5_recognition_token_mismatch_refuses_redeem (accept wrong token); d5_unrecognized_successor_stays_sealed (mint without recognition).
- s4 mc: d5_inherited_expand_never_resurrects_reduced_body (raw cache first); d5_no_real_compartment_carry_is_eligible (none as zero); d5_fixed_placeholder_descends (recognize provider summary text); d5_non_d5_filtered_noise_coverage_unchanged (global exclusion); d5_option_callers_are_a_closed_set_of_five (add a sixth pair); d5_acceptance_identity_cardinalities (remove I62).
- s4 e2e-specimen: d5_specimen_tail_content_on_successor_passes (empty served slice); d5_receipt_token_recognizes_vendored_successor_anchor (require wrapper equality); d5_budget_sizes_full_projected_specimen_union (size tag sum); d5_unknown_resolves_without_summary_fallback (invent summary).
- gw gateway-not-shipped: d5_placeholder_requires_matching_sealed_receipt (bypass permission); d5_crash_after_dispatch_recovers_durable_attempt (dispatch before record); d5_release_revokes_cached_reply_permission (send after NEVER_SEND); d5_envelope_forwarded_retry_is_latest_normal (exclude retry); d5_envelope_older_completion_cannot_overwrite (swap on completion); d5_envelope_newer_unusable_refuses_old_evidence (reuse old); d5_envelope_rollback_limits_are_honest (claim universal rollback detection); d5_envelope_unpersistable_request_not_forwarded (send before persistence); d5_envelope_pending_marker_crash_tombstones_slot (skip marker); d5_budget_uses_intended_normal_projection_identity (use summary tools); d5_bound_ticket_crash_resolves_without_reprepare (rebind or reprepare); d5_redeem_waits_for_native_successor (redeem before recognition); d5_release_trigger_requires_closed_never_replied_aliases (release active alias); d5_release_unknown_retry_keeps_identity (change retry ID); d5_release_after_redeem_returns_already_redeemed (release REDEEMED); d5_ingress_queue_bound_is_not_d (treat D as cardinality); d5_carry_predicate_accepts_matching_proof (reject matching proof); d5_carry_missing_refuses (accept absence); d5_carry_wrong_receipt_refuses (accept another receipt); d5_carry_tampered_manifest_refuses (accept changed digest); d5_carry_regressed_row_version_refuses (accept regression); d5_carry_missing_member_refuses (omit member); d5_carry_changed_member_bytes_refuses (change bytes); d5_carry_duplicate_or_reorder_refuses (duplicate member); d5_carry_native_anchor_is_not_escape_hatch (accept native escape).

## non-goals

- Change OpenCode or Pi behavior, activate D5 on those hosts, tighten their sparse coverage rules, or replace their differential golden expectations.
- Implement the Claude Code gateway side (its attempt record, send revocation, MID allocation, provider capture); those target semantics are specified here as the shared contract and built by the gateway campaign.
- Fall back to a provider-generated summary, forward a refused compaction body raw, replay a transformed predecessor request, or substitute the summary request's max_tokens and tools for normal-turn budget evidence.
- Time-sweep SEALED archives, infer non-delivery from age or socket failure, or reclaim UNKNOWN and MAY_HAVE_REPLIED attempts by TTL.
- Detect compaction mid-turn, change substance floors or scheduler fill policy, make the predecessor's in-flight fire the carrier, or relax the existing revert_epoch guard.
- Restore already reduced source bodies, change ordinary same-key raw chunk expansion semantics, or claim that all predecessor facts vanished despite independently saved memory and notes.
- Solve arbitrary same-incarnation or joint-database rollback, guarantee every future successor envelope fits a prior estimate, or establish a client maximum timeout from the D120 calibration.
- Independently deploy D5, build an older-binary rollback backport, or restart ck-mc. Release waits for the gateway shared-contract asks, and only the release owner performs the coordinated bounce.
- Provide any post-seal capacity degradation lane or promise that arming or publishing a fold makes the next request fit.

## open_questions

None.
