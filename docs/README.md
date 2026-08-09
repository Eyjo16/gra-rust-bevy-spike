# Documentation map

The documents are split by purpose so a current architectural decision is not
confused with historical trial evidence.

## Source-of-truth order

When two artifacts disagree, resolve them in this order unless a specific
decision explicitly says otherwise:

1. Registry and schema contracts, once present and versioned.
2. Truth-layer types, invariants, transition code, and executable tests.
3. Canonical receipts, world hashes, and recorded trial evidence.
4. Architecture and dependency documentation.
5. Host adapters, tools, visualizations, and presentation code.

Registry or schema contracts are never changed as a side effect of an
implementation or documentation task. A contract change needs its own explicit
decision, migration account, and approval.

## Current documents

| Document | Kind | Purpose |
| --- | --- | --- |
| `architecture.md` | Current decision | Defines truth, execution hosts, seams, and acceptance criteria |
| `dependency-map.md` | Current map | Shows code dependencies, ownership, command flow, oracles, and tunables |
| `development-workflow.md` | Current process | Defines branches, worktrees, falsification, review, and integration |
| `meaning-gate.md` | Ratified workflow invariant | Governs how semantic questions become trial-backed authority |
| `trial-log.md` | Append-only evidence | Records decisions, red/green rounds, gates, and environment facts |
| `verb-isolation-report.md` | Standalone evidence | Records the adversarial second-verb isolation trial |
| `falsification-defier-audit.md` | Standalone evidence | Separates closed counterexamples from the surviving limits of the current proof |
| `falsifier-map.md` | Current plan | Executable overnight plan for the audit's open falsifiers (trials 007–009), with standing rules and handoff checklist |
| `trial-008-apply-totality-report.md` | Standalone evidence | Records the reachable mass-clamp red, aggregate-bound fix, apply totality audit, and pressure verdict |
| `trial-009-language-seam-report.md` | Standalone evidence | Records the pre-command normalization red, canonical-byte observation point, adversarial matrix, and pressure verdict |
| `transition-domain-report.md` | Standalone evidence | Records trial/007's bounded pure/Bevy parity result and the value-cell reachability pressure map |

## Maintenance rule

- Update current-decision documents when the architecture changes.
- Append to the trial log; do not rewrite an old trial as though it happened
  under today's architecture.
- Create a standalone report when a trial needs enough evidence that it would
  obscure the log.
- Link every new report from this index and from the relevant trial-log entry.
- Keep mechanical fixtures and provisional balance values visibly labeled as
  hypotheses until evidence promotes them.
