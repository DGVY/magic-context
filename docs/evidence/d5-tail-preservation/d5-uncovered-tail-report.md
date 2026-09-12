# D5 uncovered-tail descent: source and live specimen

## Verdict

**The uncovered conversation is not folded or carried by D5.** The successor inherits compartments ending at 1798, then an empty boundary at 1940. Its native replacement input begins at 1940, not at 1799. The first forwarded request has six messages: m0, m1, a boilerplate continuation, and three post-compaction messages. The three distinctive predecessor strings tested below are absent from all six messages and from the next two forwarded requests.

**But the literal answer to “is ANY predecessor-tail content served?” is YES.** The agent explicitly saved/merged memory **14536** during this tail; its entire **1,091-character content**, also present in predecessor `ccm-1925#0`, appears verbatim in successor m0. This is independent project-memory persistence, not a historian fold of 1799–1939. Session note **1274** was also copied as **1275**, with identical content; it remains retrievable, although it is not the missing conversation transcript. Do not report that all information from the tail was irretrievably erased.

**There is nevertheless a real continuity hole.** Original messages 1799–1939 have no compartment or chunk transcript on either key. `ctx_expand` on the successor cannot recover their original message bodies. Tag source fragments survive durably, but neither ordinary successor historian assembly nor `ctx_expand` reads them as a replacement transcript. The empty-boundary adoption deliberately advances the anchor; the unpreserved nonempty tail is an unhandled safety precondition, not an evidenced, intentional permission to lose the conversation.

## Scope, artifacts, and provenance

- Lineage: `15bf744d-5485-492e-b671-b22d5837d4ef`.
- P = `15bf744d-5485-492e-b671-b22d5837d4ef␟1789163425387`.
- S = `15bf744d-5485-492e-b671-b22d5837d4ef␟1789166336504`.
- Source citations are relative to this worktree at `d70a59b5281fff2608f6876ef6a54c7af7e073d2`. Deployed reference resolves to `37d1c9999c670fde42eab928e5f98f24a292287a`. `git diff` shows tests/test logging extraction between these references, not changes to descent, archive publication, or expansion. There is a small production logging-helper extraction, so “minus tests” is not byte-literal for lib.rs; the investigated semantics match.
- The live store was opened with SQLite `mode=ro` and **VACUUM INTO** `.cortexkit/alfonso/reviews/d5-specimen.db`. All subsequent SQL used the copy, also `mode=ro`. Neither key was swept, cleaned, transformed, or mutated. Snapshot SHA256: `f589668287f41abaeb2a6526ee6d6f9d162e7ed80b1650f1ca5ec0a45984b8c0`.
- External evidence, read-only: `/Users/ufukaltinok/Work/Projects/CortexKit/thalamus/.cortexkit/alfonso/evidence/reduction-descent-1789166531/`, containing cache/compartment/tag exports and native request/forwarded bodies.
- Reproducible checks: `python3 .cortexkit/alfonso/reviews/verify-d5-specimen.py`; results in `d5-verification.json`. Frozen m0/m1 and decoded cache metadata are retained beside this report. No source changes or commits. The pre-existing untracked `.cortexkit/alfonso/release-notes/v0.42.0.md` was left untouched.

## 1. Exactly what the successor serves

### First forwarded pass 13615

`13615-fwd-body`: **339,888 bytes**, SHA256 `6ed14f82359a2031cc7a072c669b91c663293ceefc3a48885556e34b24a52f02`.

| Array index | Source identity / content | Size or detail |
|---|---|---|
| 0 | `mc_m0#0`, user-role synthetic prefix | **65,449 Unicode characters; 65,585 UTF-8 bytes** |
| 1 | `mc_m1#0`, user-role synthetic delta | **90 characters / bytes** |
| 2 | `ccm-1940#0`, user continuation anchor | Fixed placeholder embedded in the harness continuation wrapper |
| 3 | `ccm-1941#0` thinking, `ccm-1941#1` tool call | New post-compaction `toolu_01Eq3YCUn38XgkDH7dJT9JpD` |
| 4 | `ccm-1942#0`, tool result | New result with **§643§**, persistence.rs lines starting 438 |
| 5 | `ccm-1943#0`, system-role message | **5,113-character** harness reminders: prior-read file paths, agent descriptions, environment, model, date |

Thus first/last served conceptual block IDs are **mc_m0#0 / ccm-1943#0**; raw successor first/last are **ccm-1940#0 / ccm-1943#0**. The ccm identities are correlated from the first-pass durable baseline and continuation base; native Anthropic messages themselves do not contain those ccm IDs. There are seven content blocks across six messages when the string-valued continuation counts as one block.

`13615-req-body` has four messages; D5 adds the two synthetic prefixes. Its predecessor raw array is not appended. Both frozen m0 and frozen m1 from SQLite compare **exactly equal** to the corresponding text on all three forwarded bodies examined. The first-pass m0 is therefore directly retained, not reconstructed from an unverifiable hash.

m1 is exactly:

`<session-history-since>(no new content since last materialization)</session-history-since>`

S has `rendered_m0_coverage={max_sequence:14,boundary_ordinal:1940}`, `rendered_m1_coverage={max_sequence:null,boundary_ordinal:null}`, `folded_compartment_seq=14`, `coverage_start_ordinal=1025`, `coverage_ordinal=1940`, `publication_floor_ordinal=1799`, `ordinal_continuation_base=1939`, and `lineage_descent_materialized=true`. The publication floor did NOT advance to 1941, but the terminal compartment/coverage did; the old floor does not make old messages available.

Its counters are `compaction_seen=7`, `compaction_answered=7`, `fork_arm=6`, `descended=6`, `observed_flag_missing_shape_present=1`, and zero for unknown_ancestor, already_bootstrapped, not_compaction_shape, cycle_detected, pending_build_skew, pending_no_responses.

`mc_pass_trace` records the first scheduler pass at **1789166336511**, Force85/execute, drain latch true; there are 23 receives in the retained S row, no rejection or divergence. The latest stored fingerprints are **not the first pass**: 66 block fingerprints through `served_message:49#0` at 22:40:51.935Z, representing 50 messages. Their first two serialized block lengths are 66,051 and 124, which include JSON/block overhead and are not m0/m1 text lengths. Source `transform.rs:2352–2380` explains fallback `served_message:N` identities. Hashes are not reversible native-message storage.

Growth checks: 13616 has **8** messages / **341,491 B**; 13617 has **10** / **342,902 B**. Both retain identical m0/m1 and grow with new post-compaction work, not the old raw array.

### Content proof, not an ID search

Recursive searches covered every string value of every served message, including tool inputs/results, thinking/signatures, the continuation wrapper and system reminders. Each probe was first positively located in P's `mc_tags.source_bytes` and the actual 13610 raw request. Message indices below are zero-based.

| Probe | Predecessor location and exact string | P raw 13610 | S m0 / m1 | All messages of 13615 / 13616 / 13617 |
|---|---|---|---|---|
| A | tag576, `ccm-1824#0`: `Your parsed disk assertions are independently verified: setup intact` | Present, index89 | Absent / absent | Absent / absent / absent |
| B | tag597, `ccm-1864#0`: `9226\t            // A second archived project, never touched by this test's` (literal tab after 9226) | Present, index129 | Absent / absent | Absent / absent / absent |
| C | tag636, `ccm-1927#0`: `Take the real follow-up note1274: audit persistence-related test assertions in this repo` | Present, index192 | Absent / absent | Absent / absent / absent |

For each: **not carried verbatim on wire; not archived/folded as its original conversation; recoverable as a tag fragment by an operator; unavailable through successor ctx_expand**. Exact-string absence alone does not prove absence of every paraphrased fact. The no-fold conclusion additionally rests on the identical compartment rows and the complete m0 session-history section ending at 1798. Positive counterexample/control: all 1,091 characters of memory14536 match both P tag634 (`ccm-1925#0`) and S m0. That proves the search can find inherited content and falsifies a blanket “no content from the tail survives” claim.

### Placeholder

Thalamus `crates/thalamus-core/src/compaction_response.rs:48–49` defines:

`<summary>\nConversation history compacted and preserved by Magic Context. Full context continues to be served automatically.\n</summary>`

`synthesize` at lines100–130 emits that constant, not a summary of the request. The exact prose occurs under `Summary:` in message2 of 13615, surrounded by the standard continuation preamble, a path to the Claude `.jsonl` transcript, and instructions to resume. No predecessor conversation is embedded there. The transcript path offers possible **external-file** recovery via a file-reading tool, not automatic context retention or `ctx_expand` recovery.

## 2. Source decision and contract

1. **Descent precedes normal transform work.** `crates/mc-module/src/transform.rs:3308–3367` validates the edge, identifies the continuation anchor, calls `store.descend_lineage`, and forces materialization/rebases ordinals. There is no preceding historian drain/fold in this arm.
2. **Store chooses newest-live, not last summarized.** `crates/mc-store/src/lib.rs:10913` reads `source_meta.newest_live_ordinal`; lines10994–11005 require ordered nonoverlapping compartment ranges ending no later than that value, but do not require the last range to reach it. Lines11006–11036 choose the placeholder at `prior_last+1` and next sequence.
3. **Fence then jump.** Store lines11053–11060 bump P's `revert_epoch`. Lines11067–11092 clone core/meta, set coverage to the placeholder, reset S historian to default, clear pending rewrite/output-fingerprint state, and set continuation base. Lines11173–11202 copy compartments, chunk transcripts and all tag source bytes. Lines11126–11171 copy session notes.
4. **Boundary-only mint.** Store lines11227–11242 insert one row with start=end=1940, empty title/content/p1/p2/p3/p4 and `episode_type='lineage_boundary'`. No chunk transcript is created for 1799–1939 or the boundary. D5 does not fetch, serialize, prepend or fold the predecessor raw tail.
5. **No renderer rescue.** `transform.rs:2540–2585` rebases only messages in the replacement request. Lines4741–4784 compose m0 from durable compartments/memories/profile/docs and validate against the **current** live input. `first_uncovered_live_block` at 7832–7846 cannot see absent predecessor messages. `compartment_coverage.rs:175–201` permits sparse coordinates and returns the last compartment's end. `decay_render.rs:45–53,331–335` excludes empty payload rows from summary/decay input. Hence the coverage metadata can say 1940 while m0 contains no history for the gap.
6. **No later historian rescue.** `lib.rs:5198–5260` uses the max compartment end as the historian boundary. `historian_chunk.rs:626–644` chooses the first present request message strictly after that end, then builds from those request messages at 668–674. On S that means after1940, not after1798. Its first historian decision records eligible_tokens=0/below_proactive_floor; later attempts start1941. It neither consults P's missing archive nor reconstructs it from tags.

### Intent versus unhandled case

The present adoption behavior is explicitly tested: `transform.rs:36652–36672` seeds ten prior messages but only compartments1–3 and4–6; `fake_compaction_descends_materializes_and_write_free_replay_acks` at 36716–36773 expects replacement coverage11 and ranges `(1,3),(4,6),(11,11)`. It verifies inheritance/replay/anchor shape and preserves a synthetic “Durable summary alpha” supplied in the new request. It does **not** assert preservation of predecessor messages7–10 or enforce that a real summary covers them. Thus a gap is accepted by a test, but preservation of its contents is not tested.

Readable contract evidence (Thalamus paths, not this worktree):
- `docs/fake-compaction-acceptance.md:3–9` promises history inheritance; item2, lines27–29, calls for gap-free coverage; item9, lines54–57, says an older checkpoint costs a re-copy and “never a loss.” It pins the unavailable frozen contract by SHA prefix86e3ae26c2ea5a1b (lines14–20).
- `docs/fake-compaction-drive-card.md:10–21` only requires seed compartments, not a fully folded tail. Lines49–55 require gap-free coverage. Lines198–208 document the fixed local placeholder replacing a real summary and becoming the harness's entire prior conversation.
- `compaction_response.rs:3–11` explicitly assumes durable history will be assembled elsewhere after the harness truncates its own history.

**No inspected clause authorizes discarding an uncovered nontrivial tail; no inspected clause requires waiting for/re-running an in-flight historian.** The original v10.2 frozen document was not available in the supplied artifacts, so I do not invent a clause for it. The readable acceptance rules and placeholder premise are inconsistent with interpreting this as intentionally safe tail disposal. The hole exists whenever a nontrivial uncovered tail is replaced by the fixed placeholder—backoff increases exposure but is not necessary.

## 3. In-flight historian race, with corrected timestamps

All times UTC on 2026-09-11:

| Evidence | Milliseconds | Time |
|---|---:|---|
| P last historian `fired` decision | 1789166311297 | **22:38:31.297** |
| P last transform receive | 1789166332632 | **22:38:52.632** |
| Compaction request13610 receive, req-meta.json:96 | 1789166335597 | **22:38:55.597** |
| S key epoch | 1789166336504 | **22:38:56.504** |
| Boundary14 created_at / S first scheduler pass | 1789166336511 | **22:38:56.511** |
| Final P row last_activity_at | 1789166351821 | **22:39:11.821** |
| P failure_backoff_at_ms | 1789166371297 | **22:39:31.297 — cooldown deadline, NOT rejection time** |

P's last fired range was **1799–1905**, not the entire 1799–1939 tail; eligible_tokens=2118, emergency trigger, model `google/antigravity-claude-opus-4-6-thinking`. Earlier ring entries include validation-backoff/rate_limit decisions. Final state is idle, firing_seq10, last_failure=`publish rejected: revert epoch mismatch (session was re-cut mid-firing)`, consecutive_publish_failures1. Producer IDs and selected chunk are cleared on abandonment, and this fire has no completed_at/apply_row_version. Do not describe the whole descent as occurring only while an idle historian was in backoff: an in-flight publication was fenced out.

Parent supplied this exact live stderr evidence (untimestamped; not independently read from a log artifact):

`mc-module: historian firing failed for 15bf744d-5485-492e-b671-b22d5837d4ef␟1789163425387: state: publish: publish CAS conflict: expected Some(203), found 203: revert epoch mismatch (session was re-cut mid-firing)`

**This is not an Option-versus-integer comparison defect.** Store `lib.rs:12381–12390` explicitly matches `Some(v)` and compares `current == v as i64`. That row-version test passed. The later, separate check at **12435–12441** compares `meta.revert_epoch != request.expected_revert_epoch` and returns the same `CasConflict` variant with a reason even when row versions are equal. D5 bumped P from epoch0 to1; the historian retained expected epoch0 from assembly (`historian_chunk.rs:626–629`). “expected Some(203), found203” is harmless formatting of the row-version fields; the actual failed comparison is the epoch.

The epoch rejection is **after descent** and before the snapshot/observed stderr. The exact wall-clock rejection timestamp is unavailable. `lib.rs:5494` sets failure_backoff_at_ms to firing-time now+60,000; `historian.rs:769–784` reuses that deadline on rejection. P's last_activity_at provides a nearby persisted state-write observation, not a proven rejection timestamp or strict upper bound: abandonment SQL (`mc-store/src/lib.rs:12286–12290`) does not update last_activity_at, and the snapshot has no cache-state timestamp trigger. It would be misleading to promote either22:39:11.821 or22:39:31.297 to an exact rejection event.

**Rejected content is not archived.** Epoch validation precedes append at12465–12479 and transcript insertion at12480–12489. `historian.rs:769–784` abandons the matching run; store12265–12280 retains failure text/ring/counter but clears producer/run/chunk state. No rejected-publish transcript ledger appears in the store schema. The rejected output may exist in external producer logs, but no durable MC recovery path to it is evidenced here. Even accepting that one run would only have covered its selected range through1905; a preservation fix must still handle1906–1939.

## 4. Loss, surviving bytes, and recovery

**Exact native span:** 1799–1939 is **141 messages** in the captured predecessor raw array:70 assistant,70 user,1 system. 13610 contains205 messages with base1735; tag-source matches independently confirm index89→1824,129→1864,192→1927,204→1939. The final message also has the compaction instruction appended; it is not all original conversational content. P's identity map has140 of those mids, omitting the system message1938. These are not141 guaranteed unique, previously served semantic messages: strips/reductions already existed, system reminders can repeat, and explicit memory survives.

**Exact retained tag mass for that span:**83 tags, sum `token_count` **24,073**, source_bytes **81,453 B** on **each** key;66 tool-result tags and17 message tags. Token_count is the store's accumulated estimate, not exact Anthropic billed tokens (`mc-store/src/lib.rs:8325–8333` preserves the maximum observation). It omits untagged tool inputs/reasoning/system content. Therefore **24,073 must not be reported as an exact total number of tokens lost to the model**. An exact unique semantic-token loss count is not obtainable from these data, and is ill-defined where memory/duplicate content survives. The exact conclusion is loss of the old raw-message continuation, not loss of every token/fact.

**Archive census:** Both keys have exactly8 `mc_chunk_transcripts` rows:1026–1037,1438–1518,1519–1528,1529–1547,1548–1595,1596–1629,1736–1765,1766–1798. All16 raw deflate blobs were successfully decoded:268 raw messages per key; **zero** in1799–1939. SQL overlap queries also return zero compartments and zero chunk rows for the target range. There is no boundary14 raw archive.

**ctx_expand:** `crates/mc-module/src/lib.rs:11884–11919` resolves the channel's conversation key, checks a same-key in-memory request snapshot for message mode, then that key's chunk transcript; absence returns “Message … is no longer recoverable from persisted chunk transcripts.” Range mode11936–11985 reads same-key compartments/transcripts. The ephemeral snapshot is explicitly same-session and bounded (4408–4416). There is no tag-source lookup or ancestor walk in this handler. Thus S cannot expand these old messages. A still-resident P in-memory request might support P-bound message-mode expansion until eviction, but its survival was not probed and is not durable recovery. P's durable transcripts cannot do it either.

**MC historian:** no automatic recovery from these archives because they do not contain the range; no tag-to-transcript fallback; S assembly is after1940. Operator recovery is a different claim: the captured13610 request, tag source fragments, the Claude transcript path, and preserved memory/note remain possible recovery sources. “Discarded by descent” here means discarded from automatic conversation continuity, not physically deleted from all storage.

## 5. Pending drops and ledger

The requested577–608 range has32 tags. **Tag608 is ccm-1882#0**, not1881 (tag607). P has29 pending tool-result targets; message tags595/605/607 were already absent from the pending queue and have predecessor frozen reduction units. The29 remaining targets total **12,636 stored tokens /39,637 source bytes**.

P ledger command `toolu_01J19s6gEDeNTGomb1QGvZXy`: queued_at1789164662220; first_applied_at1789164662239; disposition NULL. A first-applied timestamp proves some application, not complete eviction. Store9999–10013 stamps command first application separately from deleting individual consumed queue rows.

**True terminal outcome for those29 old pending targets on the successor is discarded by descent, not covered by a compartment and not still live as raw tail.** Durable P rows remain pending/stale because D5 does not resolve them. S has zero pending rows and zero ledger rows for this command. The copied mc_tags rows include number/id/kind/token count/time/source bytes only (`mc-store/src/lib.rs:11195–11202`); there is no pending/drop column on a tag. Core is initially cloned during descent, so it is too strong to say no reduction metadata is ever copied; the materialized S snapshot has no `red:*` units. The29 pending targets were not consumed by ordinary reduction. A ledger cleanup must not falsely stamp them as summarized or applied.

## Fix shape and parity

1. **Before acknowledging replacement, establish a preservation invariant for every nontrivial predecessor tail block.** Require a validated fold plus recoverable raw archive, or durable raw-tail carry. An empty lineage boundary may anchor a replacement but must never substitute for preservation of real content.
2. **Coordinate the in-flight publisher.** Drain/accept it under a lineage transition barrier before bumping its epoch and copying; then fold/carry the remainder through1939. Alternatively retain an immutable predecessor-tail snapshot and re-fire on the successor against an explicitly inherited range. Merely changing the epoch predicate or accepting a stale publish under P is insufficient: a late P compartment would still not automatically appear in already-descended S.
3. **Persist before discarding.** Existing chunk archives are only written on accepted publication. A retry after key replacement requires a durable full-tail snapshot, not hashes or the current successor request. Carry must preserve tool arcs, ordering, source identities and reduction state.
4. **If neither fold nor carry is possible, refuse/defer the destructive transition before the harness's local summary reply.** Refusing only after the fixed placeholder has replaced the harness array is too late without a retained predecessor snapshot. Backoff and an active producer must be explicit cases, not treated as empty input.
5. **Use real gap/preservation tests:** nonempty user instruction and tool result beyond the last compartment; fixed placeholder, not “Durable summary alpha”; an in-flight producer deliberately completing on either side of descent; failure/backoff; crash/restart; archive/ctx_expand recovery; first/second native wire content assertions; and terminal drop ledger disposition. No red-pin test was added during this report-only task.
6. **Parity:** OpenCode/Pi have **no D5** predecessor→successor descent path. Their TS/Pi same-session compaction/clone behavior is not evidence that this CC-specific lineage transition preserves tails. Shared Rust rendering tolerates sparse ordinals, but tightening its global coordinate checks is not an adequate CC fix and could regress legitimate retired ordinals. Enforce preservation at the actual CC/D5 transition; audit analogous TS/Pi raw-archive/coverage invariants separately.

## Verification and limits

- Snapshot extraction via read-only VACUUM INTO: passed.
- `python3 .cortexkit/alfonso/reviews/verify-d5-specimen.py`: passed. Verifies compartment equality/boundary emptiness, both archive censuses, all three content negatives in full wire messages across three passes, predecessor positive matches, exact frozen/wire m0/m1 equality, positive memory control and note-copy equality.
- Source/deployed-reference diff inspected; no implementation edits or runtime calls made.
- Cargo tests/clippy/fmt and mutation tests skipped: no Rust edits/test added, report-only. No subagents used.
- Limits explicitly retained: original frozen v10.2 doc unavailable; exact rejection wall-clock unavailable; no exact provider-token or unique semantic-token loss count; no claim that ephemeral P cache or external producer logs were inspected. These limits do not weaken the directly measured absence of the tested conversation content on S or the empty archival range.
