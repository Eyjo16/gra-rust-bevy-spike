# gra-rust-bevy-spike — truth-layer slice

A pure-Rust scaffold for the game's truth layer: three single-writer owners,
a validating boundary, canonical receipts, deterministic world hashes, and
exactly nine bounded oracles.

> All numbers in the yield/cost tables and fixtures are **mechanical
> examples** — they are not balance and not historical truth.

## Invariants this slice enforces

1. **Three single-writer owners** — `Character`, `Economy`, `Social`. Each
   owner's state is private to its module; nothing else can mutate it.
2. **Typed IDs** (`CharacterId`, `SiteId`, `ClaimId`), bounded `Stamina`
   (`0..=100`), and `MassGrams` backed by `u64` — negative mass is
   unrepresentable by construction.
3. **Witnessed claim as a boolean gate** — an unwitnessed claim can never
   move mass.
4. **One active 4×4 cell** — Stamina band × GatheringInfrastructure tier is
   the only mechanic interaction in this slice (`YIELD_TABLE_GRAMS`).
   Two closed verbs share the grammar: `gather` (band-based cost,
   exhausted gate, moves mass) and `witness` (flat cost, no exhausted
   gate, flips a claim's boolean gate, moves no mass). All verb policy
   lives in the boundary — see `docs/verb-isolation-report.md`.
5. **Accepted / Partial / Refused outcomes** with closed reason enums —
   every reason code round-trips through `from_code`. Overdraw is refused
   (`insufficient_stamina`), never silently clamped.
6. **Validate everything, then apply** — owners return private proof
   tokens (`WitnessPass`, `StaminaSpend`, `Extraction`) from read-only
   validation. Applies consume their token **by value** (one token, one
   apply — reuse is a compile error) and each token is bound to the owner
   **revision** it was minted against; a stale token panics loudly instead
   of silently minting mass. Applies never produce a wrong game outcome.
7. **Canonical receipts** — one deterministic line per command carrying
   the world hash **before and after**, plus a **grammar fingerprint** of
   the tables/bands/reasons that produced it, so every trial record says
   which grammar version made it. World hashing is FNV-1a over BTreeMap
   order — no floats, no platform dependence.
8. **Exactly nine bounded oracles** — the count is enforced by a
   fixed-size array type in `src/oracles.rs`. They are deliberately not
   all receipt-trusting: oracle 8 checks the hash chain and that refusals
   are byte-identical no-ops, and oracle 9 is an independent shadow
   evaluator that recomputes every expected outcome from the immutable
   fixture without reading any receipt field — an internally consistent
   receipt lie still fails.
9. **Fixture coherence gates** — seeding rejects duplicate IDs (no silent
   last-write-wins) and `validate_world_coherence` rejects claims that
   reference an unknown holder or site, both via the closed
   `FixtureFault` set.

## File map

| File | Role |
| --- | --- |
| `src/boundary.rs` | Shared types, 4×4 cell, closed outcome vocabulary, receipts, hashing, orchestrator |
| `src/character/mod.rs` | Character owner (stamina) |
| `src/economy/mod.rs` | Economy owner (site stock, inventories, mass conservation) |
| `src/social/mod.rs` | Social owner (claims, witness gate) |
| `src/oracles.rs` | The nine bounded oracles, incl. the independent shadow evaluator |
| `src/main.rs` | Pure-Rust host: fixture trial + oracle gate (non-zero exit on failure) |
| `docs/trial-log.md` | Decision and verification log |

## The compiler gate

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

`cargo run` prints the grammar fingerprint, the canonical receipts, the
world hash, an end-of-run state summary (including owner revisions), and
the nine oracle verdicts; it exits non-zero if any oracle fails.

## Bevy host: ON HOLD

The Bevy host stays on hold until the pure Rust boundary passes the gate.
The `bevy`/`bevy_ecs` dependencies remain pinned behind the off-by-default
`bevy-host` feature, so the default build is the pure boundary. Nothing in
the truth layer references Bevy.
