# D5 descent: predecessor-tail preservation — design v3 (2026-09-12)

Supersedes v2. Incorporates Thalamus pm_299f6cfb. Status: for Thalamus agreement → receipt schema + D calibration evidence → Athena gate → spec/impl. No production edits before all three.

## 0. Defect and ownership
Unchanged (v2 §0, §2). One ownership model: **seal on P, redeem on the successor**. P's in-flight fire is never a carrier; the epoch guard stays.

## 1. Vocabulary (final)
- **Source blocks:** native blocks Thalamus supplies for T (transport-normalized; recognized compaction-instruction additions excluded by provenance). Authoritative *material*.
- **Applied state (A):** MC's durable reduction/strip/frozen-unit state for T at seal: `red:*` units, skeletons, strip decisions, caveman units, tag identities, pending-drop rows, ledger rows. Authoritative *permissions*.
- **Projection (V):** `project(source, A)` — the provider-visible rendering MC would serve for T given A. Defined for EVERY manifest block, including blocks never yet served (see §4). V is computed at seal from source + A; it does not depend on MC having retained last-served bytes.
- **Archive:** V-tail bytes + manifest (§8) + A snapshot, durable, content-addressed (`archive_id = sha256(manifest ∥ V ∥ A)`).
- **Prepared-archive ctx_expand semantics (explicit, new, scoped):** on the successor, `ctx_expand(ordinal ∈ T)` resolves to the archive and returns **V** (projected served body with placeholders where A applied). Existing same-key chunk-transcript expansion semantics are untouched; this is a new resolver arm keyed by inherited-range membership, not a change to "raw" anywhere else.

## 2. Lifecycle: seal → reply → redeem
```
prepare(P, agent, F, source_segment) ──→ SEALED(receipt)      [pre-reply, durable]   | REFUSED
                                          │
                        gateway replies placeholder (only after SEALED)
                                          │
redeem(edge, receipt_id)  ──────────────→ REDEEMED(successor) [atomic bind + mint]
```
- **Seal point (fixes gap 2):** `prepare` ends with ONE store transaction (no awaits inside) that commits atomically: archive rows, receipt row `SEALED`, P's **admission fence** (`P.sealed_for = receipt_id`), and P's **epoch fence** (revert_epoch bump). From this instant: no P pass may mutate provider-visible bytes (any pass on P replays V for T and refuses new reductions); any P publish assembled before the fence is rejected by the existing guard; new pending drops for T are **not** appended to P's queue — they are recorded on the receipt as `post_seal_drops` and merged at redeem (§6). All await-bearing work (archive projection, optional in-flight grace) happens BEFORE this transaction; if it exceeds D the receipt is `REFUSED` and nothing is fenced.
- **Recovery never needs the harness's discarded source** (gap 2): after SEALED, everything the successor needs is in the archive. A crash after the reply and before redeem leaves a SEALED receipt; the next descend/redeem with the same edge material finds it by (P, agent, F) and redeems it. There is no `stale-at-descend → re-prepare` path after seal; revalidation at redeem is only `receipt.state == SEALED ∧ fence intact`, both under MC's control.
- **Redeem (fixes gap 3 second half):** one store transaction: bind edge (`receipt.edge`, once), mint successor row, copy compartments/transcripts/tags/archive, copy carried pending drops, set successor frontiers (§5), mark receipt `REDEEMED`. Either all rows exist or none. `REDEEMED` with a missing successor row is **corruption**: refuse loudly (`lineage_corrupt{receipt}`), never retry-mint, surface in health.
- **Cancellation paths without a gateway report (fixes gap 2 last clause):** MC cannot know whether the placeholder was delivered. Therefore a SEALED receipt is treated as *possibly delivered* until discharged. Discharge is positive only (§7). The gateway MAY send `cancel(receipt)` as a hint (`cancelled_before_delivery` — allowed only if the gateway asserts it never replied); MC records it, keeps the archive, and the fences on P release (P resumes mutable service) only via `release(receipt)` which the gateway sends with the same assertion; absent that, P stays fenced (safe: P is the lineage that was about to be abandoned; a fenced P still serves V byte-identically).

## 3. Admission barrier (fixes O4)
- Gateway owns ingress admission: before calling `prepare`, it installs a per-(session, agent) **ingress fence** with a boot/sequence watermark: requests with sequence > watermark are queued (bounded by D) or refused 503; requests ≤ watermark that are still collecting/decoding/streaming are **drained or aborted** by the gateway before `prepare` is sent. `prepare` carries the watermark; the receipt echoes it.
- MC fences its own side at seal (§2): publisher (no new fire on P), store (P immutable for T). MC never holds a SQLite transaction across a wait; the seal transaction is the only write in the critical section and it is synchronous.
- Existing `LineageCohort` in-memory tracking is not the fence (it is post-resolution); the durable watermark is.

## 4. Authoritative projection for every manifest block (fixes gap 4)
- `project(source, A)` is a pure function run at prepare over the supplied source segment: for each source block, if A has a unit for its identity → render per the unit (placeholder/skeleton/strip/caveman); else → render the block as the ordinary serializer would on a defer pass (no reductions, no summary-turn processing, no new minting of *decisions*).
- **Identity for never-yet-served blocks:** tag identities are minted deterministically from the source block's (mid, index) under the seal, using the same minting rule the tagger uses on a normal pass but with `decisions = none` (mint identity only; never mint reductions, strips, or frozen units at seal). These identities are written into A so redeem and later successor passes agree.
- Consequence: the archive is complete even if MC never served the newest blocks (results that arrived with the compaction request), and equal to what a defer pass would have served for already-served blocks (I2 pins both arms).

## 5. Representation on the successor (fixes gap 1)
- The successor **serves the carried V-tail on every forwarded pass** after m0/m1 and before the continuation anchor, until a **validated fold** covers it (ordinary successor historian, reading inherited ordinals from the archive) or a **legitimate reduction** retires individual blocks (ordinary successor reduction lanes operating on the inherited tag identities — the archive hash identifies the snapshot; it is not a permission to ignore later reduction state). No "admissible once" clause. Pins: first pass, second pass, post-restart pass all serve identical carried bytes absent new decisions; a fold on the successor retires the carried range and the next pass is a normal SOFT bust.
- Frontiers are three separate fields on the successor: `folded_frontier` (last real compartment end, e.g. 1798), `source_frontier` (end of the carried archive, e.g. 1939), `lineage_anchor` (continuation base, e.g. 1939/1940). The empty boundary row advances **only** `lineage_anchor`. The historian's eligible head = `folded_frontier + 1`, so inherited R is eligible immediately (I7).
- Receipt-declared carried blocks are validated by Thalamus by digest on every pass they appear (§8 manifest refs), not once.

## 6. Pending drops and reductions across the transition
- At seal: pending drops targeting T are frozen into A (`pending` with command provenance). Drops arriving after seal are recorded on the receipt (`post_seal_drops`) — never lost, never applied on fenced P.
- At redeem: pending + post_seal drops targeting carried blocks are copied to the successor queue (still queued-until-aged there, protection honored); drops whose targets were folded on P before the fence get ledger disposition `covered`. Ledger dispositions `partial`/`covered` require the banked store migration (#2732) — a hard dependency, sequenced first.
- Dropped bodies never resurrect: V already carries placeholders; the archive stores no dropped source bodies.

## 7. Retention and discharge (fixes gap 3)
- **No time-based reclamation of a SEALED archive, ever.** States and discharge:
  - `REFUSED`: no archive committed (refusal precedes the seal transaction) — nothing to retain.
  - `SEALED`: retained until positive discharge: `REDEEMED` (lives with the successor lineage; swept only by explicit lineage deletion) or `RELEASED` (gateway asserted never-delivered; archive kept 24 h *after release* for forensics, then swept — this is the only clock, and it starts at a positive event).
  - `cancel` hint without `release`: archive retained; P fenced; surfaced in `session.status` as `sealed_unredeemed{age}` so an operator can see a stuck transition; no auto-sweep.
- Explicit lineage deletion (`session.delete`) removes receipts/archives for that lineage atomically with its other rows.

## 8. Receipt schema (O3) — wire, versioned
```
receipt/v1 {
  receipt_id, attempt_id, schema_version: 1,
  predecessor_key P, session_id, agent,
  F: { digest, normalization_version, excluded_additions: [provenance tags] },
  sealed_state: { P_state_version, projection_key, applied_state_hash, epoch_before, epoch_after },
  ingress: { watermark, sequence_seen },
  frontiers: { folded: 1798, source: 1939, anchor: 1939 },
  representation: "carry" | "refused", refusal_reason?,
  manifest: ordered MESSAGE groups → BLOCKS: { ordinal, mid, index, role, kind,
             source: { len, sha256 }, served: { len, sha256 }, applied_unit?: key },
  refs: { archive_id, manifest_digest, carry_digest },      // per-pass validation by ref
  pending_drops: [{ command_id, target, state }], post_seal_drops: [],
  state: SEALED | REDEEMED | RELEASED | REFUSED, edge?: id, successor_key?: key
}
```
Per-pass validation uses `refs.carry_digest` + block digests; the full manifest travels once. Codec and anchor checks to be validated against current SubcTransform anchor logic (which assumes submitted-native anchors today) — a wire change on both sides, versioned.

## 9. Budget (fixes the 25% cap)
- Two separate caps, both stated in the refusal reason:
  - **Token headroom:** carry fits iff `estimate(system + tools + m0 + m1 + carry + anchor_reserve) ≤ usable_soft − output_reserve`, computed by the real estimator on the projected bytes at seal (same estimator the scheduler uses); no fixed percentage.
  - **Resource cap:** archive/carry encoded bytes ≤ a configured byte ceiling (default sized from the current 64 MiB frame body / page limits, stated in config, not policy) — protects the store and the wire, not the model.
- A carry that only fits by ignoring overhead is refused; refusal is before the seal transaction. No floor-policy changes.

## 10. Deadline D
Provisional 120 s total (ingress drain + projection + archive + optional ≤20 s in-flight grace + seal). Frozen only after Thalamus's hermetic CURRENT-CC compaction 503/timeout probe (isolated fake provider). Never scaled by fill.

## 11. Invariants (executable; replaces v2 §9)
- I1 No placeholder without SEALED (gateway pin); redeem without SEALED refuses (MC pin).
- I2 For every manifest block: successor first pass serves either a compartment covering it (folded before seal) or the carried block with bytes == archive V; blocks never served on P render as a defer pass would; applied units render as placeholders; no source body of a dropped block appears.
- I3 `ctx_expand(ordinal ∈ T)` on the successor returns archive V; existing chunk-transcript semantics unchanged (pin both).
- I4 Epoch guard unchanged; a P publish assembled pre-seal landing post-seal is rejected.
- I5 Idempotency: same (P, agent, F) → same receipt; retry-marker deltas do not mint; F change before seal → new attempt; after seal → the SEALED receipt wins (redeem finds it).
- I6 Seal atomicity: crash at any point before the seal txn leaves no fence and no receipt; after it leaves SEALED+archive+fences; redeem atomicity likewise; REDEEMED-without-successor is loud corruption.
- I7 Frontier split: successor historian eligible head = folded_frontier+1; specimen shape yields eligible_tokens > 0 on the first successor trigger.
- I8 Carry persistence: passes 1, 2 and post-restart serve identical carried bytes absent new decisions; a successor fold retires the range; a successor reduction retires a block.
- I9 Barrier: a request above the watermark during prepare is queued ≤ D or refused 503; never replayed, never dropped.
- I10 Retention: SEALED archives are never time-swept; RELEASED starts the only clock; `sealed_unredeemed` is visible in status.
- I11 Budget: refusal reason names which cap failed with the numbers.
- I12 The gap-accepting test is rewritten to a nonempty unfolded tail with the fixed placeholder asserting I2/I3/I7/I8.

## 12. Parity / dependencies
OpenCode/Pi: no D5. Depends on #2732 (ledger dispositions migration) and a versioned wire change on both seats. Athena gate after Thalamus agrees v3 and the receipt schema, with D's calibration evidence attached.

## 13. Normative amendments (v3.1, from Thalamus pm_aa06fec7 — folded, not editorial)

A) **SEALED P admission (replaces the "replay V" wording in §2).** After seal, P serves NO normal requests: managed turns arriving on P are queued behind the admission barrier (bounded by D) or refused 503 until REDEEMED or RELEASED. Appended source is never ignored and never replayed. The gateway's existing ban on transformed-body replay stands. I5 corrected: a request whose F differs from the SEALED receipt does NOT silently lose to the sealed one — redeem requires F match; mismatched material is refused (`seal_material_mismatch`) without deleting the archive and without discarding the new input (it stays queued/refused at admission, never dropped).

B) **Gateway durable attempt record.** Before the first reply byte, the gateway durably records `{attempt_id, P, agent, F, receipt_id, MAY_HAVE_REPLIED: false}` and flips `MAY_HAVE_REPLIED := true` (monotonic, durable) before writing the reply. Recovery keys on this record, never on finding F in a successor. RELEASE requires the gateway's proof of never-replied across ALL retries of that attempt (the record's bit false in every retry row for the attempt), not for one socket. After RELEASED, a new prepare with the SAME F is allowed and mints a NEW attempt/receipt (idempotency is scoped to non-RELEASED receipts); the archive is reused by content address.

C) **Two digest classes.** `manifest_digest` and `archive_id` are immutable and identify the sealed snapshot membership + source identities. `projection_digest{row_version}` is per-response and changes with every legitimate later reduction/tag/encoding change on the successor; per-pass validation compares the projection digest MC declares for THAT pass (with its row_version) — never the frozen carry bytes. §8 corrected: `refs.carry_digest` → `refs.projection_digest` (mutable, versioned); `archive_id` (immutable). After a newly applied drop, re-render and ctx_expand serve the reduced form; original bytes are never resurrected.

D) **Late-drop routing (any state).** A drop/command for a T target arriving at ANY time after seal is routed atomically by lineage lookup: P SEALED → recorded on the receipt; P REDEEMED → dedup by (command_id, target) and enqueue on the CURRENT successor if the target is carried there, else stamp ledger `covered` (folded) or `retired` (already reduced); P RELEASED → returned to P's own queue. No arrival is lost between copy and delivery; the route is a single store transaction keyed by the lineage table, retry-safe.

E) **Identity allocation for never-seen blocks.** MID allocation is the gateway's (its MID store); tag identity is MC's. `prepare` requires every supplied block to carry a gateway-allocated MID; MC never invents MIDs. Never-seen blocks: the gateway allocates MIDs idempotently for the attempt (same block bytes + attempt → same MID) BEFORE calling prepare, durably in its MID store keyed by attempt; MC then mints tag identities deterministically from (mid, index) under the seal — identity only, no decisions, no deferred commands bound, the summary instruction never processed as a turn. Crash recovery: gateway MID rows for an attempt persist; a repeated prepare with the same F reuses them; MC's seal is atomic so tag identities exist iff the receipt does.

F) **Geometry and freshness.** Token headroom uses the SAME estimator and reserves the scheduler uses (`usable_soft` is the scheduling denominator, not capacity): `estimate(system + tools + m0 + m1 + carry + anchor_reserve + output_reserve) ≤ usable_soft`, with an unknown estimate → refuse. Byte caps are two: the gateway ingress cap (64 MiB) and MC's subc frame/page capacity as configured for transform pages — the smaller governs; the final outbound gate still applies to every carry render. Freshness: the projection and any optional grace are computed BEFORE seal, so the seal transaction CAS-checks every input it depended on (A hash, coverage frontiers, P.state_version, pending-drop count); if an optional publisher committed meanwhile, the seal recomputes (bounded by D) or refuses — never seals a stale projection.

D calibration: Thalamus worker wi_1544da69 (current-CC native 503/delay, isolated fake provider) — results attach to the gate; D stays provisional until then. Receipt schema v1 updated per C (projection_digest{row_version}) and B (gateway attempt record referenced by attempt_id).

E-pin) **MID idempotency is positional.** The gateway's MID allocation key is `(attempt_id, predecessor P, ordered native MESSAGE position)` — a message-level identity; block index is separate and never part of the MID. Two byte-identical messages (or blocks) at different positions get distinct MIDs; a repeated prepare with the same F reuses the ordered allocation exactly. I2/I5 gain a duplicate-content regression: two identical messages in T must appear as two manifest entries with distinct MIDs on every prepare repetition and on the successor's served array.

## 14. D frozen and two post-refusal cases (v3.2, from Thalamus calibration pm_14a6176f)

**D = 120 s, server policy (frozen).** Evidence (CC 2.1.258, isolated fake provider, validated from raw request hashes and native boundaries, not worker labels): AUTO 503,503,200 recovers and the summary replaces all warm-up markers; AUTO persistent 11×503 retains all warm-ups, writes no summary/compact_boundary, and silently resumes the exact pending ordinary request with a 200; manual persistent 503 shows a compaction error and retains history; both manual and auto first responses tolerate a 120.003/120.004 s wait before a 503 and the next retry succeeds; 125 s success is tolerance evidence, not a client max. No 150 s extrapolation.

**Case A — ordinary P request immediately after a failed AUTO compaction (no user-visible error).** The gateway must classify by attempt state before forwarding: (i) prepare definitively REFUSED (receipt state REFUSED, or no attempt row exists) → transform the request normally on P (P was never fenced); (ii) attempt SEALED, or outcome UNKNOWN (e.g. the prepare call timed out) → do NOT forward raw and do NOT replay: query the attempt by (P, agent, F) (stable lookup handles a late commit), enforce the admission barrier (queue ≤ D or 503), preserving the appended input. Never lose or replay the appended request.

**Case B — client timeout at or near the seal commit is OUTCOME-UNKNOWN.** A gateway-side timeout cannot promise "nothing fenced"; only MC's REFUSED (durable) proves it. The gateway records the attempt as `outcome_unknown` and resolves it by the stable attempt lookup on its next touch of P; MC's seal transaction is atomic so the lookup returns exactly one of SEALED / REFUSED / absent (absent = never reached the seal, safe to treat as REFUSED after D has elapsed since the attempt's own start, recorded gateway-side).

Both cases and every §13 correction are Athena inputs. Evidence provenance: exact-version container, network=none; initial host runs were not OS-sandboxed and registered non-default URL handlers (documented by Thalamus); only four sanitized code/evidence files integrate; raw artifacts stay local to Thalamus.
