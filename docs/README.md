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
| `../AGENTS.md` | Workflow authority | Evidence Factory Protocol v0.1 (ratified 2026-08-12): agent-instruction entrypoint — evidence-factory boundaries, goal envelopes, review bundles, cross-review protocol |
| `trial-log.md` | Append-only evidence | Records decisions, red/green rounds, gates, and environment facts |
| `verb-isolation-report.md` | Standalone evidence | Records the adversarial second-verb isolation trial |
| `falsification-defier-audit.md` | Standalone evidence | Separates closed counterexamples from the surviving limits of the current proof |
| `falsifier-map.md` | Historical plan | The executed overnight plan for the audit's open falsifiers (trials 007–009, all integrated); kept as evidence of the standing rules used |
| `trial-008-apply-totality-report.md` | Standalone evidence | Records the reachable mass-clamp red, aggregate-bound fix, apply totality audit, and pressure verdict |
| `trial-009-language-seam-report.md` | Standalone evidence | Records the pre-command normalization red, canonical-byte observation point, adversarial matrix, and pressure verdict |
| `transition-domain-report.md` | Standalone evidence | Records trial/007's bounded pure/Bevy parity result and the value-cell reachability pressure map |
| `active-cell-reachability-report.md` | Standalone evidence | Records trial/010's systematic 12-cell reachability and stock-boundary pressure |
| `trial-013-low-actionability-report.md` | Sealed Meaning Gate evidence | Preserves trial/013 training evidence and H-A verdict candidate while keeping the holdout unrevealed and unexecuted |
| `trial-e01-belief-actionability-taste-report.md` | Bounded presentation evidence | Records the Publication-fed wrong/matching-belief taste, exact gates, captures, and the still-open manual verdict |
| `runtime-contract-proposal.md` | Ratified law | Runtime Contract v0.1 (R1–R7 with amendments): current executable runtime law, semantic rulings, and the durability gap |
| `runtime-target-map.md` | Active work map | Orders runtime falsifiers and capability-scoped dependency sweeps by authority; carries per-target status |
| `rs01-live-render-report.md` | Standalone evidence | Records the RS01 capability red, live Publication/receipt render proof, screenshot evidence, and bounded claims |
| `rs01-visual-fact-map.md` | Implementation evidence | Classifies every meaningful RS01 default-view element by Publication, receipt, derivation, expression, or interaction source |
| `environment-evidence-cold-repro-001.md` | Standalone environment evidence | Records an independent cold-machine gate reproduction and the observed `bevy-host` Rust floor |
| `integration-consolidation-2026-08-14.md` | Integration record | Reconciles reachable RS01 identities, compute merges, held evidence, and the next safe boundary |

## Maintenance rule

- Update current-decision documents when the architecture changes.
- Append to the trial log; do not rewrite an old trial as though it happened
  under today's architecture.
- Create a standalone report when a trial needs enough evidence that it would
  obscure the log.
- Link every new report from this index and from the relevant trial-log entry.
- Keep mechanical fixtures and provisional balance values visibly labeled as
  hypotheses until evidence promotes them.
