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
grammar=0x7dd8c6706e0b949f  cmdfmt=0xfa37eefa3594cfe3  rcptfmt=0x7e62152622bb9132
fixture=0x93afba3f312bd89d  receipts=0xc0b4da51744bcf19
world=0xb500dee0e5d883d8    oracles=10v7
```

Canonical language is identified three ways since 2026-08-18 (author
licence): **grammar** = gameplay semantics and policies; **cmdfmt** =
canonical command bytes; **rcptfmt** = canonical receipt fields and
order. A presentation change moves one number and not the others.

Those are the identities of the **V01 line** (RES01 + Give), which is
review-ready and not yet merged. Merged master still carries the
pre-RES01 grammar `0x530003916889b952` / fixture `0x3805f1e20c001051` /
oracles `10v4`. Runs are cross-comparable only within one line.

Gate (clean tree first; never judge through a pipe): `cargo fmt
--check`; clippy `-D warnings` for default, `bevy-host`, `bevy-render`
and `e01-taste`; `cargo test` for each; `BASELINE_COMMIT=$(git rev-parse
--short HEAD) cargo run --features bevy-host` exit 0 — all oracles +
bevy_host_parity/projection/publication/faults probes green. On the W01
line, `cargo run winter` too (thirty oracle verdicts, three plans).

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

## Standing state (refresh me!) — as of 2026-08-18

- Master `1f3cbc6`: trials 001–010, 013, 014, R01–R03, D01, TS01 and
  **E01** merged. E01 closed the belief→action legibility question.
- **Three stacked branches, reviewed once and repaired, gates green,
  nothing pushed**: `trial/RES01-resource-kinds` (`d4f7ebe`) — closed
  kind vocabulary fodder/food/timber, author-licensed, per-kind
  conservation; `trial/V01-give` (`f83796d`) — the third verb,
  **attributed** transfers (consent is NOT proven), attester recorded by
  identity; `trial/W01-winter-crisis` (`8656e73`) — the first playable
  scene, three stockpiling plans, thirty oracle verdicts, per-plan host
  parity. Synthesis: `docs/sprint-2026-08-18-overview.md`.
- Four identity values moved or were created this sprint, every one
  predicted before implementation and now pinned by tests.
- **Vocabulary discipline**: say *attributed*, never *voluntary* or
  *consented*, about a transfer. Consent needs a seat, an issuer,
  delegation or actor intent, none of which exists.
- **The load-bearing open finding: nothing is consumed, so nothing is at
  stake.** W01 can state a shortfall; it cannot make one hurt. Time,
  household, refusal of labour, and turf/fuel names are the next gaps.
- Still open and author-owned: chronology/seat cluster (workbook 08 + 11
  + 13), MAP01 push, RS01-human execution, slavery scope, and the merge
  words for the three branches above.

## How to be useful in one sentence

Verify before trusting, measure before claiming, escalate instead of
assuming authority — and spend tokens on the game's meaning, not on
polishing the machine that waits for it.
