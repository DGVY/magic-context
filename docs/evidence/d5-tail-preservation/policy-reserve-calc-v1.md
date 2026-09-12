# policy_reserve calculation v1 (ESTIMATES, not provider measurements)

Estimator: `mc_tokenizer::estimate_tokens` (mc-tokenizer 0.1.0, tiktoken byte-BPE Claude estimator) at magic-context 711b097eefe1; model/profile: claude-code-anthropic serializer profile (the estimator is model-agnostic BPE; no per-model table).
Fixture manifests: anchor-reserve-observations-1789183743/manifest.json sha256 c86a62153ba38185…; anchor-wrapper-shapes-1789184112/manifest.json sha256 2fea2a21ec447a53… (Thalamus, CC 2.1.258, isolated; observed vs derived distinguished by the manifests).

| fixture | form | bytes | est. tokens |
|---|---|---|---|
| ex13508 anchor (observed, live) | scalar | 827 | 197 |
| ex13615 anchor (observed, live) | scalar | 827 | 197 |
| auto-120s observed anchor text | array[1] | 1002 | 269 |
| auto-120s derived (fixed placeholder) | array[1] | 998 | 257 |
| auto-two-503 observed | array[1] | 999 | 266 |
| auto-two-503 derived | array[1] | 996 | 255 |
| manual-120s derived | array[1] | 1002 | 254 |
| currentDate reminder (all array-form) | array[0] | 306 | 76 (identical sha across 6 samples) |

Terms: reminder_tokens = 76 (separate named term; present in array-form continuation messages only — the scalar live anchors carry none, so the term is conditional on serving shape and is counted once when present). anchor_text max_observed = 269 (observed) / 257 (derived-with-fixed-placeholder). Dynamic fields: transcript path (variable; observed variance in these fixtures ≤ 6 bytes ≈ 2 tokens, but the path is user-controlled and unbounded — limit assumed: 256 bytes ≈ 80 tokens), harness version string (≤ 16 bytes ≈ 5 tokens), instructions (no variance observed across --bare/full; not modelled beyond max_observed).

policy_reserve(v1) = (reminder 76 + anchor max_observed 269 + dynamic headroom [path 80 + version 5]) × (1 + margin 0.25) = 430 × 1.25 = **538 tokens** (estimate). supported_profiles: CC 2.1.258 --bare and full, manual and automatic triggers, anthropic scalar and text-block-array serving shapes. dynamic_field_limits: {transcript_path: 256 B, harness_version: 16 B}. Outside these → uncertainty recorded; preserved-but-blocked governs post-seal.

Not claimed: a universal upper bound; provider token counts; coverage of other CC versions. The 10 percent re-encode pin applies to fixture drift only (re-estimating these fixtures at a later estimator version must stay within 10 percent or the pin fails and the number is recomputed).
