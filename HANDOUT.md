# HANDOUT — Grágás in one page

Derived, non-authoritative context loader for any fresh agent session.
When this disagrees with the law documents, the law wins. Refresh at
integration points; a stale handout is worse than none.

## What is being built, by whom

**Grágás**: a colony-sim of economic, social and dynasty-political
struggle, through a deep meaningful RPG layer and turn-based high-stakes
combat — "Clanfolk meets Crusader Kings", hints of 4X, survival
management, deep production. Above all: **clarity**. One causal chain,
six genre-projections: *people change the material world; the material
world changes relations; relations create rights, duties and conflict;
consequences become story.*

**Eyjó** is the author: sole authority over meaning, priorities, values,
contracts, merges. Works from intuition; the machine exists to make his
judgment executable and falsifiable, never to replace it.
**Fable 5** (Claude) is lead programmer; **Sol 5.6** (Codex) is
driftmaster/reviewer. Agents are an evidence factory, not autonomous
mergers: author-dispatched envelopes, isolated worktrees, review-ready
branches, cross-review (re-derive, never merely re-read), one
disagreement circle then verbatim to the author. Never suggest when the
author should work or rest.

## The two repos

1. `gra-rust-bevy-spike` — executable truth, laws, proofs, trials.
   Rust; default build has zero dependencies. Read `AGENTS.md` first
   (Evidence Factory Protocol v0.1, ratified), then `docs/README.md`
   for the source-of-truth order. Runtime Contract v0.1 is law: A1
   immediate/sequential execution; time is not modeled yet; A2 sealed
   turns is an unratified draft (`agent/turn-contract`).
2. `gragas-local-compute` — queues, runs, artifacts, model lane
   (Nemotron expert-ensemble). Nothing there is truth; provenance
   mandatory; six meaning statuses travel with artifacts.

## Frozen identities and the gate

```
grammar=0x530003916889b952  fixture=0x3805f1e20c001051
receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a  oracles=10v4
```

Gate (clean tree first; never judge through a pipe): `cargo fmt
--check`; clippy `-D warnings` both feature sets; `cargo test` both;
`BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run --features
bevy-host` exit 0 — all oracles + bevy_host_parity/projection/
publication/faults probes green.

## Core doctrine, compressed

- Claims exactly the size of their evidence; red-first (capability reds
  honestly labeled); spec evolution declared, never a side effect.
- Meaning statuses (closed): ratified · proven · measured · fixture ·
  hypothesis · counterfactual. Values are fixtures until a licensed,
  pre-registered red moves them. Holdouts open once.
- The machine measures consequences; only the author assigns meaning.
  Playtests measure engagement; no logic proof can prove fun.
- Truth layer: three single-writer owners (character/economy/social),
  proof tokens bound to entity revisions, two-phase all-or-nothing
  commit, canonical receipts + hash chain, ten oracles incl. an
  independent shadow evaluator. Hosts (Bevy today) custody truth but
  hold no authority: byte-parity, disposable projections, publication
  identity, host faults beside — never inside — receipts.
- Player: not in truth as a hand; the diegetic seat may be canonical.
  Receipts are the transition ledger, not personal memory.

## Standing state (refresh me!) — as of 2026-08-14

- Truth master `8c1baca`+: trials 001–010, R01–R03, D01, TS01 merged.
  Held unmerged: 011/012/013 (gameplay evidence awaiting the author's
  Meaning Gate verdicts — **the critical path**), 014 (Sol's), MB01
  (meaning brief, review-ready), turn-contract (A2 candidate).
- `run/NM00` (local-compute): checkpoint bakeoff Nemotron-9B-v2 vs
  Llama-8B-v1; eval seal `628b8d90…` + measurement seal `0ddd8106…`
  both frozen pre-inference; baselines await Sol gate. Expert adapters
  gated: gameplay/UI experts wait on Meaning Gate verdicts; historical
  expert waits on Icelandic baseline + licensed corpus.
- The author's decision-frame workbook (13 questions, printed) is the
  single most valuable pending input; A1/C1 answers unlock 013, stamina
  semantics shape the first value hypothesis, autonomy rung defines the
  third verb (`contest`-shaped, story-led, opens contention/T03).

## How to be useful in one sentence

Verify before trusting, measure before claiming, escalate instead of
assuming authority — and spend tokens on the game's meaning, not on
polishing the machine that waits for it.
