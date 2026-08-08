# Trial log — truth-layer slice 001

## 2026-08-08 — scaffold rebuilt and verified in remote session

- The earlier local scaffold branch (`codex/truth-layer-slice-001`) never
  reached the remote: the GitHub App push was refused (403) and the git
  bundle only exists on the author's machine, unreachable from this
  environment. The slice was therefore rebuilt from its written spec on
  `claude/truth-layer-scaffold-verify-2fhvhp`.
- Spec honored: three single-writer owners (Character, Economy, Social);
  typed IDs, bounded stamina, `MassGrams` with negative mass
  unrepresentable; witnessed claim as boolean gate; one active 4×4 cell
  (Stamina × GatheringInfrastructure); Accepted/Partial/Refused with closed
  reasons; validate-everything-before-infallible-apply; canonical receipts
  and deterministic world hashes; exactly seven bounded oracles.
- All fixture and table numbers are mechanical examples — not balance, not
  historical truth.
- Bevy host decision: still ON HOLD. `bevy`/`bevy_ecs` stay pinned but move
  behind the off-by-default `bevy-host` feature so the default build is the
  pure boundary and the compiler gate stays fast and deterministic.

### Compiler gate results (this environment, rustc 1.94.1)

| Check | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo test` | 21 passed, 0 failed |
| `cargo run` | all 7 oracles PASS, exit 0 |

### The seven oracles

1. `stamina_in_bounds` — every stamina stays in `0..=100`
2. `mass_conserved` — total mass equals the fixture baseline
3. `witnessed_gate` — no unwitnessed claim ever moves mass
4. `exhausted_gate` — no exhausted actor ever yields
5. `closed_reasons` — every reason code round-trips the closed enums
6. `cell_bounds` — every yield stays inside the active 4×4 cell
7. `replay_determinism` — replaying the fixture reproduces receipts and hash
