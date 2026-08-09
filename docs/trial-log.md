# Trial log — truth-layer slice 001

## 2026-08-09 — trial/003-parallel-plan: red→green

Hypothesis: owner-wide revision binding false-conflicts independent
plans, and the boundary's commit sequence can leave a partial commit
when only a later owner's token has gone stale.

Preparation (behavior-neutral, before any red): the commit phase was
extracted into `GatherPlan::apply` / `WitnessPlan::apply` — the one
place a multi-owner plan mutates the world — so the falsifiers could
exercise the real commit path with planning separated from application.
Receipts and hashes unchanged by the refactor.

Red evidence (captured against baseline `466f272`, three failures):

```
character::falsification_independent_spends_must_not_false_conflict
  panicked: stale proof token (character) — boundary bug
    left: 0 / right: 1        <- C2's spend killed by C1's apply

boundary::falsification_independent_plans_against_one_snapshot_must_both_commit
  panicked: stale proof token (character) — boundary bug
                              <- two fully disjoint plans false-conflict

boundary::falsification_stale_later_token_must_not_leave_partial_commit
  first panic:  stale proof token (economy) — boundary bug
  second panic: stale plan committed partially: the character spend
                landed without the extraction
    left:  5778436795119231595   <- world hash CHANGED across a refused
    right: 4302712438088730884      commit: partial commit
```

Green, two mechanisms (both required — neither alone passes all three):

1. **Entity-revision binding.** Tokens now bind to the entities they
   touch: per character (`StaminaSpend`), per site + per inventory
   (`Extraction`), per claim (`WitnessGrant`). Disjoint plans validated
   against the same snapshot are independent and both commit; same-entity
   replay still panics (all prior `should_panic` tests unchanged).
2. **Two-phase commit.** `GatherPlan::apply` / `WitnessPlan::apply`
   check every token fresh BEFORE any owner mutates; a stale plan panics
   with zero mutation — all-or-nothing, verified by hash equality across
   the refused commit.

Entity revisions are derived bookkeeping, excluded from the world hash;
owner-wide apply counters remain hashed, so **grammar, fixture,
receipts, and world hashes are all byte-identical** to the baseline —
the envelope proves this trial changed conflict semantics without
touching game semantics. No spec evolution: no new oracles, reasons, or
receipt fields.

## 2026-08-09 — trial/004-shadow-final-state: red→green

Hypothesis: final-world truth rested on oracle 7 alone, which replays
through the same implementation it audits — a bug shared by run and
replay presents a divergent final world no independent oracle sees.

Red evidence (captured against baseline `a91023d` before the fix): the
falsifier applies one extra gather to the world after the logged trial —
mass conserved, log untouched, so oracles 1–6 and 8–9 stay green — and
demands a failure from any oracle other than `replay_determinism`:

```
falsification_divergent_final_world_must_fail_an_independent_oracle FAILED
  panicked: divergent final world was visible only to the
  self-trusting replay oracle
```

Green: new oracle 10 `shadow_final_state` — the shadow evaluator (already
independent for receipts) now steps every command and compares its final
state against the actual world across all four truth domains: stamina,
inventories, site stocks, claim gates. It reads the world only through
read-only owner iterators; zero trust in the implementation, receipts, or
replay. The falsifier and the full gate are green.

Spec evolution (conscious): oracles **nine → ten** (count still
type-enforced), `ORACLE_SUITE_VERSION` **1 → 2**. Envelope before/after
the merge proves the protocol's promise: `grammar`, `fixture`,
`receipts`, and `world` byte-identical, only the judge changed
(`oracles=9v1` → `oracles=10v2`).

## 2026-08-09 — sprint armed: four concurrent trials, cross-elimination protocol

Baseline for the whole sprint: `3ce9efc` (master). Protocol recorded in
`docs/development-workflow.md` § Cross-trial comparison protocol; every
run now emits a proof envelope
(`baseline_commit / grammar / fixture / receipts / world / oracles`).

The four trials, each one falsifiable hypothesis, with its named falsifier:

1. `trial/004-shadow-final-state` — **hypothesis:** final-world truth
   currently rests on oracle 7 alone, which replays through the same
   implementation it audits; an implementation whose run and replay share
   a bug can present a divergent final world that no independent oracle
   sees. **Falsifier:** a divergent final world paired with an
   internally consistent log must fail at least one oracle *other than*
   `replay_determinism`. Red today; green when the shadow evaluator's
   final state is compared against the actual world.
2. `trial/002-bevy-host-parity` — **hypothesis:** Bevy can host the
   existing truth and reproduce identical receipts and hashes without
   adding gameplay semantics. **Falsifier:** envelope comparison — the
   Bevy-hosted run must reproduce `receipts` and `world` exactly, on the
   same `grammar` and `fixture`, or the host has acquired semantics.
3. `trial/003-parallel-plan` — **hypothesis:** owner-wide revisions
   false-conflict independent plans and can leave a partial commit.
   **Falsifiers (both red today):** two plans for independent characters
   validated against the same snapshot must both apply without a stale
   panic; a stale second-owner token must never leave the first owner's
   apply committed. Green requires finer conflict granularity and an
   all-tokens-fresh check before any apply.
4. `trial/005-value-pressure` — **hypothesis:** an incoherent result
   under the trials above is pressure to *move a value* under a logged,
   red-first hypothesis rather than to lock it. This branch is the only
   one allowed to change the grammar fingerprint, and it merges last.
   Its concrete target is chosen from whatever incoherence the other
   three surface — deliberately not predetermined.

Merge order: 004 (strengthen the judge) → 002 and 003 (either order,
first-green-first) → 005 (re-baselines everyone). After each merge the
remaining branches rebase and re-run red→green — the cross-elimination
moment. Envelope fields make any silent divergence visible immediately.

Spec evolution this round (conscious): `cargo run` output gained the
envelope line; `ORACLE_SUITE_VERSION` (v1) added to name the judge.
Receipts, grammar, and world hashes are unchanged.

## 2026-08-09 — architecture decision: Bevy host and worktree loop

- The Bevy HOLD is lifted. The round-2 condition was satisfied: the
  adversarial second verb landed without leaking policy into established
  owners, and the hidden single-verb assumptions surfaced in the oracles.
- Bevy is assigned a narrower role than the truth layer: execution,
  scheduling, ECS projection, interaction, rendering, and additional test
  surfaces. It must submit commands through the same boundary and reproduce
  the pure host's canonical receipts and final hash.
- The pure-Rust truth layer remains the semantic authority below every host.
  Registry/schema contracts remain higher authority when present; this
  decision changes no registry or schema.
- Multi-language work is allowed only across explicit, versioned seams. A
  language or tool may express a form where it is clearest, but it may not
  acquire a second mutation path into canonical truth.
- `docs/architecture.md` records the refined layer model and host acceptance
  criteria. `docs/development-workflow.md` records the repeatable branch and
  worktree cycle for hypothesis, red-first falsification, implementation,
  review, integration, and cleanup.
- No runtime semantics changed in this decision round.

## 2026-08-08 — falsification round 2: second verb (`witness`) vs isolation

Full standalone record: `docs/verb-isolation-report.md`. Summary:

- Second verb `witness` added, deliberately inverting every gather
  pattern: flat cost 5, no exhausted gate, Social+Character only,
  Economy untouched, zero mass, and the social owner's first mutation
  path (revision-bound `WitnessGrant`, by-value, stale panics).
- **Isolation claim held:** `git diff` on `src/character/mod.rs` and
  `src/economy/mod.rs` is empty — zero lines changed for the new verb.
  All verb policy lives in the boundary (`Command` dispatch,
  `plan_witness`, `WITNESS_COST`).
- **Falsification found and fixed:** the first full run turned oracles 3
  and 4 red on a legal trial — both had encoded "Accepted ⇒ mass moved",
  a single-verb assumption. Corrected to key on actual mass movement,
  which is verb-agnostic and stronger.
- Closed vocabulary grew consciously: RefusalReason 9 → 11
  (`claim_already_witnessed`, `cannot_witness_own_claim`), new closed
  `Verb` enum in every receipt, `WITNESS_COST` folded into the grammar
  fingerprint.
- Gate: fmt clean, clippy -D warnings clean, 35/35 tests, run exit 0,
  all nine oracles PASS across the 16-command two-verb trial.
- Round-1 HOLD condition (second verb + isolation survives) is met;
  lifting HOLD remains the author's call.

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
