from pathlib import Path
import re
p=Path('.cortexkit/alfonso/drafts/2026-09-12-d5-tail-preservation.md')
s=p.read_text();b=s.encode()
expected='''---
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
'''
assert s.startswith(expected)
sections={m[1]:m[0] for m in re.finditer(r'(?ms)^## ([^\n]+)\n(.*?)(?=^## |\Z)',s)}
assert list(sections)==['intent','constraints','acceptance sketch','non-goals','open_questions']
c=sections['constraints'];a=sections['acceptance sketch']
assert sections['open_questions'].split('\n',1)[1].strip()=='None.'
assert len(b)<=85000
# Constraints ceiling: 50,000 through v1.3.9. Raised to 51,000 at v1.3.10 for
# R17 (CoverageProofV1 became a real tagged type, +~450 B of shared contract);
# the pipeline's own intake ceiling is 100,000 total, so this stays a fold-growth
# guard, not a pipeline limit. Do not raise again for prose; only for types.
assert len(c.encode())<=51000
assert len(a.encode())<=22000
subs={m[1]:m[0] for m in re.finditer(r'(?ms)^### ([^\n]+)\n(.*?)(?=^### |\Z)',c)}
assert list(subs)==['types','lifecycle ops','receipt and archive','successor semantics','consumer rebind','transport','gateway target semantics']
assert all(len(v.encode())<=24000 for v in subs.values())
nums=[int(x) for x in re.findall(r'(?m)^(\d+)\. \*\*',c)]
assert nums==list(range(1,35))
assert [int(x) for x in re.findall(r'(?m)^I(\d+):',a)]==list(range(1,54))
assert not re.search(r'\b(?:MUST|MAY)\b',s.replace(c,''))
assert chr(167) not in s
assert not re.search(r'\b\d+/\d+\b',s)
body=s.split('---\n',2)[2]
assert '.cortexkit/' not in body
assert not re.search(r'\b[0-9a-f]{7,40}\b',s)
for bad in ['anchor_reserve','AnchorReserveRecord','current_resolve_generation','admission.acquire','fence.read','serving_encoding','wrapper_sha256','measured_at','ticket_id:', 'MintPending','mint_only','Settled','amended by']:
 assert bad not in s,bad
for k in ['AdmissionTicket','NegativeProof','ReceiptV1','ManifestV1','SuccessorFrontiers','BudgetRecord','PolicyReserveRecord','PostRedeemRecord','CapacityDiagnosis','EnvelopeRecordV1','EnvelopePending','UploadRef','PrepareSource']:
 assert len(re.findall(r'(?m)^'+k+r' = ',c))==1,k
for needle in ['fenced_by=ticket.resolve_generation+1','refused_by_generation_responses','refused_by_generation_attempts','post_redeem','last_diagnosis','overflow_probe','preserved_but_blocked','policy_reserve','reminder_tokens','510','512 B','rebase_base','native_mid','predecessor_identity','pending{sequence}','mc_reduce_command_ledger','owner_key','one fence-bearing coordinated ck-mc bounce','InternalService','Principal::Reserved','FLAG_BINARY','max_concurrent_per_attempt=2','max_bytes_per_attempt=48 MiB','max_chunk_bytes=1 MiB','release_attempt_id','already_redeemed','RELEASE_INTENT→NEVER_SEND','Constraint one','Constraint two','Constraint three','Constraint four','Constraint five','hard_required = X + R','token_fit = soft_ok AND hard_ok']:
 assert needle in c,needle
fold=Path('.cortexkit/alfonso/task-outputs/consult-ct_00000000-0000-4002-98d5-37f1a05302b8.fold.md').read_text()
old=fold.split('48. **Consumer-rebind table',1)[1].split('49. **',1)[0]
# The inventory rows are written in a shorthand the contract defines once
# (crates/<crate>/src/<file> as <crate>/<file>, hyphen ranges, no backticks).
# Normalize the pinned fold's sites to that shorthand before checking that
# every cited site is still present.
def _site_norm(x):
    x=x.replace('–','-').replace('`','')
    return re.sub(r'^(mc-(?:store|module))/src/',r'\1/',x)
old_sites={_site_norm(x) for x in re.findall(r'`([^`]+(?:\.rs|\.ts):[0-9,–]+)`',old)}
new_sites=set()
for x in re.findall(r'((?:mc-(?:store|module)|packages)/[^\s|`]+(?:\.rs|\.ts):[0-9,–-]+)',c):
    x=_site_norm(x); path,ranges=x.rsplit(':',1)
    new_sites.add(x)
    for r in ranges.split(','): new_sites.add(f'{path}:{r}')
assert old_sites<=new_sites,old_sites-new_sites
assert 'Your parsed disk assertions are independently verified: setup intact' in a
assert "9226\\t            // A second archived project, never touched by this test's" in a
assert 'Take the real follow-up note1274: audit persistence-related test assertions in this repo' in a
print('PASS: front matter, five sections, byte ceilings, contiguous clauses and fixtures, constraint-only MUST/MAY, single types, required ruling fields, complete cited file:line inventory and three exact probes.')
print('UTF-8 byte counts (headings and inter-section whitespace included):')
for k,v in sections.items():print(f'{k}: {len(v.encode()):,}')
for k,v in subs.items():print(f'constraints / {k}: {len(v.encode()):,}')
print(f'front matter and separator: {len(b)-sum(len(v.encode()) for v in sections.values()):,}')
print(f'whole file: {len(b):,}')
