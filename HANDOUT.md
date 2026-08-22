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
grammar=0x7dd8c6706e0b949f  cmdfmt=0xfa37eefa3594cfe3
rcptfmt=0x7e62152622bb9132  fixture=0x93afba3f312bd89d
receipts=0xc0b4da51744bcf19 world=0xb500dee0e5d883d8  oracles=10v7
```

Gate (clean tree first; never judge through a pipe): `cargo fmt
--check`; clippy `-D warnings` and `cargo test` for all four feature
sets — default, `bevy-host`, `bevy-render`, `e01-taste` (90 / 100 /
108 / 114 tests, CON01 included); `BASELINE_COMMIT=$(git rev-parse --short HEAD)
cargo run --features bevy-host` exit 0, and the same with `winter` —
all oracles, the host probes, and the three winter parity lines green.
The CON01 conformance tests hold this block to the code's values.

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

## Standing state (refresh me!) — as of 2026-08-22

- Truth master `5884f27`: trials 001–010, 013 (holdout still sealed),
  014, R01–R03, D01, TS01, RS01 (bounded D04 slice), E01 (human
  verdict PASS), RES01 (three resource kinds), V01 (give — bounded
  mechanics, consent unproven) and W01 (winter scene — pressure
  evidence only) merged. Held unmerged: 011 (`5b52e81`) and 012
  (`ab45f40`), one unique commit each, awaiting the author's
  disposition; turn-contract (A2 candidate draft).
- W01's inexpressibility list is the semantic frontier: nothing is
  consumed, no time, no household, a plan is not a thing, nobody can
  refuse labour. None of it is licensed for implementation.
- Pending author rulings (decision packet, 2026-08-25): canonical time
  authority, execution visibility (A1 vs A2), and stakes/consumption —
  three separate questions, not one. Consent (`actor_unwilling`) waits
  behind the O01 issuer/seat/delegation model.
- `run/NM00` (local-compute, as of 2026-08-14): checkpoint bakeoff
  Nemotron-9B-v2 vs Llama-8B-v1; eval seal `628b8d90…` + measurement
  seal `0ddd8106…` frozen pre-inference; baselines await Sol gate.
  Expert adapters gated on Meaning Gate verdicts and licensed corpus.

## How to be useful in one sentence

Verify before trusting, measure before claiming, escalate instead of
assuming authority — and spend tokens on the game's meaning, not on
polishing the machine that waits for it.
