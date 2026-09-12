# policy_reserve calculation v2 (ESTIMATES, not provider measurements) — supersedes v1

v1 was wrong on two counts (Thalamus pm_245191e8): the transcript paths in the supplied fixtures are 116 B (live scalar) and 285/287/291 B (array specimens) — every array specimen already exceeded v1's declared 256 B limit, and the "≤ 6 B variance" was a within-array subset; and v1 added approximate byte→token terms instead of running the estimator over whole policy-sized texts.

Estimator: `mc_tokenizer::estimate_tokens` (mc-tokenizer 0.1.0, tiktoken byte-BPE Claude estimator, model-agnostic) at magic-context 711b097eefe1. Manifests: anchor-reserve-observations sha c86a62153ba38185…; anchor-wrapper-shapes sha 2fea2a21ec447a53…. Method: the estimator runs over the FULL continuation message text (currentDate reminder 306 B + newline + array-form anchor text with the fixed placeholder), with the observed transcript path replaced by a policy-sized path built by extending the REAL observed path with real-shaped segments (project-dir slugs and hex ids). Synthetic repeated-character paths were rejected: a 287 B synthetic path costs 305 tokens for the whole message vs 346 with the real 287 B path — synthetic paths under-count by ~12 percent.

| text | bytes | est. tokens |
|---|---|---|
| live scalar anchor (no reminder; path 116 B) | 827 | 197 |
| observed array message: reminder + anchor (path 287 B) | 1309 | 346 |
| policy message, real-shaped path 512 B | 1534 | 455 |
| policy message, real-shaped path 773 B (extension capped at 1024 B input) | 1795 | 598 |
| path alone 116 / 287 / 512 / 773 B | — | 57 / 117 / 226 / 369 |

Path token density on real-shaped paths ≈ 0.45–0.48 tokens per byte. The reminder is 76 tokens and is included in every array-form row above (not an additive term).

**policy_reserve(v2)** with `dynamic_field_limits.transcript_path = 512 B` (observed max 291 B; 512 B covers a deep worktree path with hex ids — Thalamus to confirm their deployments' longest observed path): estimator over the full 512 B-path message = 455 tokens; margin 0.15 for harness-version/instruction drift not represented in fixtures (no observed variance, so a smaller margin than v1's 0.25 which was covering the path uncertainty now measured) → **524 tokens** (estimate). supported_profiles: CC 2.1.258 --bare and full, manual and automatic triggers, anthropic scalar and text-block-array shapes. dynamic_field_limits: {transcript_path: 512 B (≈ 226 tokens at real density), harness_version: 16 B}. If the path limit is raised to 1024 B the same method gives 598 × 1.15 = 688. Outside the limits → uncertainty recorded, preserved-but-blocked post-seal; pre-seal unknown refuses.

Not claimed: a universal upper bound; provider token counts; other CC versions. The re-encode pin re-runs this exact table at a later estimator version; drift beyond 10 percent on any row fails the pin and the number is recomputed. This is unaccepted reserve evidence until Thalamus confirms the path limit.
