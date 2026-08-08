# Trial log — truth-layer slice 001

## 2026-08-08 — falsification round 1: red first, then green

External review (Codex + author) identified false-green risks in the
scaffold. Discipline followed: falsification tests were written and run
**red against the unmodified code first**, then the fixes were applied,
then the tests went green for the right reasons.

### Red evidence (captured before any fix)

```
economy::tests::falsification_token_replay_must_not_create_mass
  assertion `left == right` failed: token replay created mass
    left: MassGrams(3900)
   right: MassGrams(2300)      <- one &Extraction applied twice minted 1600 g

character::tests::falsification_low_headroom_must_be_refused
  panicked: 12-point actor approved for a 15-point spend
```

### Fixes, in the review's priority order

1. **Token replay / stale token.** Proof tokens (`StaminaSpend`,
   `Extraction`) are now consumed **by value** — reusing one token is a
   compile error. Each token carries the owner revision it was minted
   against; owners bump their revision on every apply, and applying a
   stale token (two tokens minted against the same revision, or a token
   applied after unrelated state change) panics with
   `stale proof token — boundary bug`. Loud, never silent.
   `#[should_panic]` tests pin both owners.
2. **Exact stamina headroom.** `saturating_spend` (the silent clamp) is
   deleted. Validation requires exact headroom and refuses with the new
   closed reason `insufficient_stamina`; apply performs an exact
   subtraction that cannot clamp. A 12-point actor asked for a 15-point
   spend is now receipt line `outcome=refused reason=insufficient_stamina`
   (fixture seq=10 demonstrates it).
3. **Refusal = byte-identical state.** Receipts now record
   `world_before` and `world` (after). New oracle 8
   `refusal_zero_mutation` walks the hash chain: each receipt's before
   must equal the previous after (starting from the fixture hash),
   refusals must not change the hash, yields must change it.
4. **Paired state+receipt lie.** New oracle 9 `shadow_expectation`: an
   independent evaluator that rebuilds initial state from the fixture and
   recomputes every expected outcome with its own state tracking and its
   own band-threshold literals — it never reads a receipt field. A test
   doctors a receipt into an internally consistent lie (band=low,
   mass=600, spent=15) and shows oracles 3–6 all accept it while the
   shadow refuses it.
5. **Fixture coherence.** Owner seeding now rejects duplicate IDs
   (`FixtureFault::Duplicate*` — no silent last-write-wins) and
   `validate_world_coherence` rejects claims referencing an unknown
   holder or site.
6. **Grammar fingerprint.** Every receipt carries `grammar=0x…`, an
   FNV-1a hash of the yield table, cost table, realized band mapping over
   0–100, and all closed reason codes — a trial record now names the
   grammar version that produced it.
7. **Second-verb isolation (preparation).** The gather cost table moved
   out of `CharacterOwner` into boundary verb policy; the owner API is
   now `validate_spend(id, cost)` — verb-agnostic resource semantics
   only. The actual second verb remains future work, by design.

### Spec evolutions this round (conscious changes)

- Oracles: exactly **seven → nine** (count still type-enforced).
- Refusal reasons: **eight → nine** (`insufficient_stamina` added).
- "Infallible apply" restated honestly: an apply never produces a wrong
  game outcome; it panics on a stale token because that is a boundary
  bug, not a game outcome.
- Owner revisions are part of the hashed state, so the world hash also
  advances with every apply.

### Gate after the round

| Check | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo test` | 32 passed, 0 failed (incl. 4 falsification tests) |
| `cargo run` | all 9 oracles PASS, exit 0 |

Bevy host remains ON HOLD. The first four falsification items are done
red→green; the standing recommendation is to keep HOLD until the second
verb lands and the isolation claim survives it.


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
