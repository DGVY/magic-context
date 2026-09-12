# policy_reserve calculation v3 (ESTIMATES) — supersedes v2; inputs are artifacts with hashes

Corrections from Thalamus pm_cc6f2319: v2 used the OBSERVED 1002 B calibration anchor (1309 B input) where the contract's input is the DERIVED fixed-placeholder anchor (998 B → 1305 B with reminder + newline); and v2's "1024 B" row rendered only 773 B. v3 asserts `len(rendered_path_bytes) == limit` and ships every estimator input as a file with its sha256 (directory `policy-reserve-inputs-v3/`).

Estimator: `mc_tokenizer::estimate_tokens` (mc-tokenizer 0.1.0, tiktoken byte-BPE) at magic-context 711b097eefe1. Base anchor: `auto-first-120s-503-then-200-derived-fixed-placeholder.json` anchor text, 998 B (derived_text_sha256 7a8767b6…), observed transcript path 287 B; reminder: the 306 B currentDate block (sha 0a534c0d…). Policy paths extend the REAL observed path with real-shaped segments and are cut to exactly the limit.

| input file | bytes | sha256[:16] | est. tokens |
|---|---|---|---|
| v3-derived-observedpath.txt (reminder + LF + derived anchor, path 287 B) | 1305 | 9f8d7ce2590f18a3 | 334 |
| v3-derived-path512.txt (path rendered = 512 B) | 1530 | 632fa5b75a1e691b | 443 |
| v3-derived-path1024.txt (path rendered = 1024 B) | 2042 | 245505c485b536a0 | 716 |
| v3-path512.txt (path alone) | 512 | becd2643ccf5aaea | 226 |
| v3-path1024.txt (path alone) | 1024 | 4dc9777b7dc16be1 | 499 |

**policy_reserve(v3)** = whole-message estimate at the path limit × (1 + 0.15 margin for unrepresented harness-version/instruction drift): limit 512 B → 443 × 1.15 = **510**; limit 1024 B → 716 × 1.15 = **824**. The limit is set from Thalamus's path census (scoped maxima over retained prod/ckdev captures), not chosen here; observed maximum so far is 291 B. supported_profiles: CC 2.1.258 --bare and full, manual and automatic, scalar and text-block-array. Outside the limits → uncertainty recorded, preserved-but-blocked post-seal; pre-seal unknown refuses. Not claimed: universal bound, provider counts, other CC versions. Re-encode pin: re-run this exact table (same input files) at a later estimator version; >10 percent drift on any row fails the pin.

## Path limit settled from the deployment census (Thalamus pm_98735105)
Census artifact: thalamus/.cortexkit/alfonso/evidence/anchor-path-census-1789186194/census.json — 13,638 production + 869 ckdev request bodies read, 9,910 exact-native-format first-user anchor observations, 13 unique transcript paths, maximum 116 UTF-8 bytes; the current CC 2.1.258 subset (727 observations) has a single unique path of 116 B. Together with the supplied calibration maximum (291 B) the `transcript_path` limit is set at **512 B**, which covers every retained deployment observation and the calibration set with headroom; it is not a bound on unseen paths or other wrapper formats (scope stated in the census). **policy_reserve(v3, limit 512 B) = 510 tokens (estimate)** from input v3-derived-path512.txt (sha 632fa5b7…) → 443 × 1.15. Recorded as unaccepted until Thalamus reads v3's exact inputs; no validation restart needed.
