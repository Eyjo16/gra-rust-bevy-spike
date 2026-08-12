# AGENTS.md — repository laws for automated collaborators

Status: DRAFT — pending adversarial cross-review (see § Cross-review
protocol; this file is its own first test case) and author ratification.

This file is the single instruction source for every automated
collaborator (Codex, Claude, or future workers). `CLAUDE.md` points
here; do not duplicate laws into per-agent files — one source, many
loaders, zero drift.

## 1. Authority

The author owns: meaning, priorities, acceptance boundaries, contract
and registry changes, value promotion, and final integration. Agents
are an **evidence factory, not an autonomous merger**: select one
unblocked objective, work isolated, produce a review bundle, and stop
before merging or changing authority. A merge by an agent requires the
author's explicit per-item instruction naming the branch; standing
permission does not exist.

## 2. Where the law lives

Source-of-truth order and document kinds: `docs/README.md`. Runtime
law: `docs/runtime-contract-proposal.md` (ratified v0.1). Semantic
workflow: `docs/meaning-gate.md`. Work order and status:
`docs/runtime-target-map.md`. Branch/worktree/falsification cycle:
`docs/development-workflow.md`. When this file and those disagree,
those win; report the contradiction instead of resolving it silently.

## 3. The gate (both feature sets, hard exit checks)

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --features bevy-host -- -D warnings
cargo test
cargo test --features bevy-host
BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run --features bevy-host
```

Never judge a gate through a pipe that can mask its exit code. The run
must end exit 0 with every oracle and probe line green.

## 4. Frozen unless explicitly licensed

- Grammar fingerprint `0x530003916889b952` and standard fixture
  identity `0x3805f1e20c001051` — value changes require a licensed
  value-pressure envelope with a pre-registered red hypothesis.
- Closed vocabularies (outcomes, reasons, verbs, host faults, fixture
  faults) and receipt/envelope formats — changes are declared spec
  evolution, never side effects.
- Registry/schema contracts: none exist yet; the closed Rust types plus
  tests are the executable contract. Introducing a registry, schema, or
  persistence format is an authority change — stop and escalate.
- `ORACLE_SUITE_VERSION` must be bumped on any oracle behavior change.

## 5. Goal envelope (required for every work item)

```text
baseline_commit:     <short hash — today's evidence baseline>
objective:           <one bounded outcome with a provable stop condition>
authoritative_files: <files whose current content governs the work>
write_scope:         <paths the branch may touch>
frozen:              <paths/identities that must not change>
verification:        <exact commands whose green defines done>
evidence:            <artifacts the bundle must contain>
limits:              <time/compute/dependency budget>
escalate_when:       <conditions that end the run with a question instead>
```

One envelope, one branch, one worktree, own `target-dir`. Serial
rebases onto master with full re-gates prevent semantic cross-affection.
Terminal state is a review-ready branch — never an auto-merge.

## 6. Evidence rules

- **Red first.** Capture the falsifier failing against unmodified code;
  quote it verbatim in `docs/trial-log.md`. When no behavioral red
  exists without staging a bug, label the capability red honestly
  (compile error, absent harness) — precedent: trials 002, 006, R01.
- Claims stay exactly the size of their evidence (defier-audit rule).
  Trace-scoped results say so; measurements are not meanings.
- Nothing becomes accepted merely because it was generated
  successfully. Generated assets carry provenance: prompt, seed,
  model/version, inputs, output hashes.
- Expensive tasks checkpoint progress and are resumable.

## 7. Review bundle format

A bundle is the branch plus a trial-log entry (or standalone report)
containing: the goal envelope as executed, red evidence verbatim, the
full gate transcript tail (probe lines + envelope line), before/after
measurements where relevant, and a **numbered claims table**:

```text
| # | Claim | Evidence class |
```

Evidence classes: `behavioral-red`, `capability-red`, `measurement`,
`derivation`, `assertion`. Every claim the bundle wants believed must
appear in the table — an unlisted claim is an escaped claim.

## 8. Cross-review protocol (mutual guardrails)

Every bundle authored by one agent is adversarially reviewed by the
other before the author's final review. The reviewer's duty:

1. **Re-derive, never re-read.** Run the verification commands; do not
   trust pasted output. Check envelope fields against the frozen
   identities. Diff the branch against its stated write scope.
2. **Verdict per claim**: `CONFIRMED` (re-derived), `OVERSTATED` (true
   but larger than its evidence), `UNDERSTATED` (evidence proves more),
   `WRONG`, or `UNVERIFIABLE` (say what would make it verifiable).
3. **Hunt list**, beyond the claims table: claims larger than evidence;
   vocabulary drift (one word, two meanings across docs); silent scope
   creep beyond the envelope; stale cross-references; law/evidence
   confusion (a dated record edited, or a live law left stale);
   fixture overfitting; a "fix" that moves a value without a licensed
   red.
4. **Meaning is expressed, never decided.** For gameplay/meaning work,
   agents verify *expression*: is the hypothesis falsifiable as
   stated, is the closed vocabulary coherent, does a value move carry
   its pre-registered red? The decision itself always remains the
   author's (Meaning Gate).
5. **Disagreement is a finding.** When author-agent and reviewer-agent
   disagree, both readings go to the author verbatim — never silently
   reconciled, never resolved by deference. Agents defer only to
   evidence and to the author.
6. **No self-certification.** An agent never reviews its own bundle;
   green gates prove coherence, not correctness of meaning.

## 9. Escalation

Stop and ask instead of proceeding when: an envelope field is missing
or contradictory; the objective requires touching a frozen identity;
two law documents disagree; a gate is green but a claim cannot be
re-derived; or the work reveals an authority question (sequence
ownership, persistence, contention policy, time semantics) that the
runtime contract marks as ruled-by-author.
