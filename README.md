# gra-rust-bevy-spike — truth-layer slice

A pure-Rust scaffold for the game's truth layer: three single-writer owners,
a validating boundary, canonical receipts, deterministic world hashes, and
exactly seven bounded oracles.

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
5. **Accepted / Partial / Refused outcomes** with closed reason enums —
   every reason code round-trips through `from_code`.
6. **Validate everything, then apply infallibly** — owners return private
   proof tokens (`WitnessPass`, `StaminaSpend`, `Extraction`) from read-only
   validation; only then do the infallible applies run.
7. **Canonical receipts** — one deterministic line per command, and a
   deterministic FNV-1a world hash after every apply (BTreeMap ordering,
   no floats, no platform dependence).
8. **Exactly seven bounded oracles** — the count is enforced by a
   fixed-size array type in `src/oracles.rs`.

## File map

| File | Role |
| --- | --- |
| `src/boundary.rs` | Shared types, 4×4 cell, closed outcome vocabulary, receipts, hashing, orchestrator |
| `src/character/mod.rs` | Character owner (stamina) |
| `src/economy/mod.rs` | Economy owner (site stock, inventories, mass conservation) |
| `src/social/mod.rs` | Social owner (claims, witness gate) |
| `src/oracles.rs` | The seven bounded oracles |
| `src/main.rs` | Pure-Rust host: fixture trial + oracle gate (non-zero exit on failure) |
| `docs/trial-log.md` | Decision and verification log |

## The compiler gate

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

`cargo run` prints the canonical receipts, the world hash, an end-of-run
state summary, and the seven oracle verdicts; it exits non-zero if any
oracle fails.

## Bevy host: ON HOLD

The Bevy host stays on hold until the pure Rust boundary passes the gate.
The `bevy`/`bevy_ecs` dependencies remain pinned behind the off-by-default
`bevy-host` feature, so the default build is the pure boundary. Nothing in
the truth layer references Bevy.
