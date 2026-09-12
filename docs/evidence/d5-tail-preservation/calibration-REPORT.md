# Native compaction calibration — reviewed evidence

## Recommendation

Keep the D5 **120s provisional preservation budget** unchanged. The earlier 125.010154s success shows that Claude Code 2.1.258 tolerated that response delay; it does not identify a client timeout and does not justify a 150s server budget. Do not spend a 140/150/160s grid without a new design reason.

## Measured behavior

| trigger / fault | observed summary sequence | higher-level outcome |
|---|---|---|
| manual normal | `200` | unique fake summary in native transcript, one `compact_boundary`, following ordinary turn completed |
| manual first two 503 | `503, 503, 200`; gaps `0.624150s, 1.214972s` | recovered; summary replacement and following turn present in native transcript |
| manual persistent 503 | 11 × `503`; first-to-last `183.046892s` | model-visible API 503; no fake summary/boundary; warmup history retained; following ordinary turn completed |
| manual delayed success | `200` after `20.009973s`; `200` after `125.010154s` | both replaced history and completed following turns; no provider-observed close/cancel |
| automatic first two 503 | `503, 503, 200`; gaps `0.620728s, 1.259356s` | recovered; following request body carried unique summary, omitted all warmups, and native transcript recorded one boundary |
| automatic persistent 503 | 11 × `503`; first-to-last `181.295152s` | silent fallback: exact pending ordinary request started `0.028376s` after final refusal and completed 200; body retained every warmup, no summary, no boundary |
| manual 120s then 503 | first response `503` after `120.003473s`, then `200`; retry gap `120.523991s` | no close/cancel; unique summary replaced warmups; one boundary; following request completed |
| automatic 120s then 503 | first response `503` after `120.004016s`, then `200`; retry gap `120.611659s` | no close/cancel; unique summary replaced warmups; one boundary; following request completed |

A provider send alone is not treated as acceptance. New successful cases require the actual following provider request body to contain the per-case summary marker and omit every warmup marker, plus a native `compact_boundary`. Automatic persistent refusal is the complementary control: its following body retained all warmups with no summary, and its native transcript had no boundary.

## Isolation correction

The initial host runs used sanitized environment variables and loopback-only provider URLs, but were **not OS- or network-sandboxed**. Debug logs show that Claude probed the absent main-checkout path `/Users/ufukaltinok/Work/Projects/CortexKit/thalamus/.claude/settings.local.json`. They also show per-HOME `claude-cli://` handler registration. A read-only LaunchServices inspection found nine registered bundles for `com.anthropic.claude-code-url-handler`, including test-specific paths; the current resolved application remained `/Users/ufukaltinok/Applications/Claude Code URL Handler.app`. No default handler was changed during review, and no unverified environment knob is claimed to prevent registration.

Follow-up runs used image `ck-claude-calibration:2.1.258` (`sha256:92c02fcf781a7e9304f9b62d9f6fc70d6490f974ad787c96323d1a81f8ac6b00`) with Docker network mode `none`, no published host ports, read-only source at `/repo`, disposable workspace under `/evidence`, fake API key, strict empty MCP, and exact Claude Code `2.1.258`.

## Artifacts and provenance

- Initial raw evidence: `/Users/ufukaltinok/.local/share/cortexkit/alfonso/worktrees/cf59575c7ffe72b4/pool-2/.cortexkit/alfonso/evidence/native-compaction-calibration-claude-2.1.258-20260911` — source `198433fdfac09c8a35f6304bf95ee37355ff3606`, modified=`true`, tree SHA-256 `dad132d6baa50b8ba9b94d82a8f0bc77046f65f33ff5fe2b7c5eb7b6cd626a1c` (78 files).
- Follow-up raw evidence: `/Users/ufukaltinok/.local/share/cortexkit/alfonso/worktrees/cf59575c7ffe72b4/pool-2/.cortexkit/alfonso/evidence/native-compaction-followup-claude-2.1.258-container-20260912` — source `dffd8df5f3f9426c37f4fe3709d2c1fa3a10ab3a`, modified=`true`, tree SHA-256 `8dd4fad4e501f6b2a7086d41f8133c5d777993aa0bf59c99cfe887e29fdb3e1e` (122 files).
- Sanitized machine-readable conclusions: `/Users/ufukaltinok/.local/share/cortexkit/alfonso/worktrees/cf59575c7ffe72b4/pool-2/.cortexkit/alfonso/evidence/native-compaction-calibration-review-20260912/sanitized-manifest.json`.

Raw files remain local and ignored rather than tracked. The follow-up outer Docker command reported exit 137 only after all four result files and manifest `completed_at` were present; no container remained. The automatic persistent case's original detector timed out waiting for a manual-style visible error, so its corrected conclusion is a bounded reassessment of request 21 and the saved native transcript, not a rerun or an inferred timeout.

## Limits

These results cover one Claude Code version, a deterministic fake provider, delayed response headers, and local/container loopback. They do not measure delayed body streaming, a client timeout, production network variance, or a server-side deadline other than the explicit 120s policy arm.

## Retained copies

The original paths above record execution provenance. Raw artifacts also have
local, ignored copies outside the worker worktree:

- `/Users/ufukaltinok/Work/Projects/CortexKit/thalamus/.cortexkit/alfonso/evidence/native-compaction-calibration-claude-2.1.258-20260911`
- `/Users/ufukaltinok/Work/Projects/CortexKit/thalamus/.cortexkit/alfonso/evidence/native-compaction-followup-claude-2.1.258-container-20260912`
