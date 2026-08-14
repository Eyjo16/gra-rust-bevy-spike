# Integration consolidation — 2026-08-14

Status: **AUTHOR-DISPATCHED INTEGRATION RECORD**. Derived and non-authoritative;
reachable refs and executable truth win if this snapshot becomes stale.

## Envelope

```text
base_commit:         cab61be
objective:           make every completed RS01/compute idea reachable and
                     integrated, resolve conflicting identity reports from
                     accessible objects, and give the pre-inference NM00
                     shape executable values before further work
authoritative_files: AGENTS.md; docs/README.md; docs/architecture.md;
                     docs/runtime-contract-proposal.md;
                     docs/development-workflow.md; executable truth and tests
write_scope:         documentation on this branch; compute run branches under
                     their existing scopes
frozen:              registry/schema absent; grammar, fixture, receipt/world
                     identities, closed vocabularies, canonical owner and
                     boundary semantics, held Meaning Gate outcomes
red_required:        no — integration/documentation and tooling hardening;
                     the real reds were missing objects, TP01 false-green
                     paths, and NM00 stub/fail-open paths
verification:        full default, bevy-host, and bevy-render truth gates;
                     NM00 scorer+dual-seal selftest; local truth proofer;
                     clean trees and exact ref/object checks
evidence:            this record; cold-environment record; merge graph;
                     proofer transcript in the integration handoff
limits:              no model inference, value move, holdout reveal,
                     registry/schema creation, or semantic promotion
tested_commit:       branch tip named by the integration handoff after the
                     clean-tree gate
```

## Reachable identity verdict

The object store, not the pasted handoff, resolves the RS01 conflict:

| Item | Verified reading | Disposition |
| --- | --- | --- |
| `e666cb6` | reachable commit; contained by truth `master`; parent of merge `cab61be` | canonical live RS01 implementation tip |
| `d43927c` | absent after fetch; absent from the accessible bundle | unavailable label; no integration input |
| `gra-spike.bundle` | valid complete bundle, SHA-256 `71a4b3634c1310846c01dcc6c86441c5124726b4ddd26d80fa9a2ff4afee82e6`; old head `f5728d6` | archive evidence only; contains neither RS01 candidate |
| `origin/claude/truth-layer-scaffold-verify-2fhvhp` | mixed archive: cold evidence plus superseded renderer and later false identity corrections | never merge as code; only cold evidence commit `b2436cb` preserved |

The earlier claim that `e666cb6` was a dead label is therefore wrong in this
object store. No coordination file may replace a reachable identity with an
unreachable one merely because the latter appeared in a report.

## Integrated compute estate

Compute master `a508276` serially merges six bounded run branches:

| Run | Integrated role | Authority limit |
| --- | --- | --- |
| MB01 | author-held meaning brief | measured questions only; no verdict |
| DOS01 | controlled claim-audit dossier | not cleared for public release |
| NM00 | frozen 21-item eval plus measurement seal v2 | no model output observed yet |
| RS00 | truth-side render contract evidence | evidence copy, not truth owner |
| RS01 | aligned HTML visual reference | fixture/reference; not the live proof |
| TP01 | fail-closed local estate proofer | tool judges refs; never becomes truth |

NM00 v2 leaves the eval bytes at
`628b8d90d59b476b93c5fdd343ee181b866c7ad343bdc340db17c7e66892c716`
and binds the harness to
`2d5373e965c69022ceb4cecafd85b0b603df23b8273ed95847a314809af90f32`.
Its executable pilot values are: A reasoning on/off 2,048/512 tokens; B
4,096/512; temperature 0.6 and top-p 0.95 for reasoning on; greedy off;
per-item seed `20260814 + item_index`; 300-second generation ceiling; perf
512 input → 128 output tokens with context probes 1,024–16,384; PEFT rank 8,
alpha 16, 128 tokens, learning rate 0.0002, targets `q_proj` + `v_proj`.

## Held rather than silently promoted

`trial/014-anticipation-drive` is reachable remotely at `bd2f8ca`. Its shape
is explicit and test-only: cap 1; BAD/AVERAGE choose the legal cheap intent;
GOOD/SUPERB choose the legal costly intent; committed pairs `(10,750)` and
`(12,1800)` remain unchanged. The bundle itself requires independent
cross-review and says it does not promote drive vocabulary or anticipation
meaning into production. It is therefore pushed but not merged here.

Trials 011–013 remain held by the Meaning Gate, including the sealed 013
holdout. No integration action in this record supplies their missing meaning
verdicts.

## Safe next boundary

TS02 may now consume integrated TS01 projections and be judged by integrated
TP01, but it remains a derived YAML/HTML review layer. Making YAML, HTML, a
registry, or a schema authoritative still requires a separate explicit
contract decision and migration account. NM00 inference may begin only from
the v2 dual seal; any harness or eval byte change creates a declared new
version before another result is compared.
