# Trial RES01 — closed resource-kind vocabulary

**Bundle status: COMPLETE — review-ready.** Pre-registration (§0–§5) was
committed at `fc5e431` before any implementation; the Evidence section
below carries the red, the gate transcript, the identity table and the
claims table. Reviewed by Sol 5.6 (2026-08-18): *conditional accept*,
conditions being this status block, the `tested_commit` field, and the
tightened closure wording in claim 5 — all applied here.

`tested_commit`: **`7c30816`** — the commit that changed code. Every
commit after it on this branch touches `docs/` only, which the gate
does not compile; the tip is re-gated all the same, and the exact tip
hash plus its gate result are recorded in
`docs/sprint-2026-08-18-overview.md`, which is gated last and therefore
can name what preceded it. A tested_commit that names a docs commit
would be certifying a compile that never happened.

Branch: `trial/RES01-resource-kinds`. Author: Fable 5 (lead).
Base commit: `1f3cbc6` (master, E01 merged, full gate green).

## 0. The authorization this trial runs under

`AGENTS.md` §4 freezes the grammar fingerprint and the closed
vocabularies; §1 and §10 reserve contract and closed-vocabulary changes
to the author. This trial moves both, and it does so under an explicit
author licence given on 2026-08-18:

> Question put to the author: RES01 changes a frozen identity (grammar
> fingerprint `0x530003916889b952`) and a closed vocabulary, so it needs
> your licence and your first kind list. Which list do I build against?
>
> Author's answer: **Fodder, Food, Timber** — three kinds, no generic
> catch-all.

The licence covers exactly this: the first resource-kind list and the
single grammar move that admitting it causes. It is not a standing
permission to evolve vocabularies, and it does not license a registry,
schema, or persistence format — those remain an authority change
(§4) and would end this run with a question instead.

## 1. Authoring envelope (as run)

```text
base_commit:         1f3cbc6
objective:           Admit a closed resource-kind vocabulary
                     {fodder, food, timber} into the truth layer:
                     every site yields exactly one kind, every holding
                     is per (character, kind), every mass-moving
                     receipt names its kind, and mass is conserved
                     PER KIND. Stop condition: the full gate is green
                     on a clean tree at all feature sets and the
                     grammar fingerprint equals the value
                     pre-registered in §4 below.
authoritative_files: AGENTS.md, docs/runtime-contract-proposal.md,
                     docs/meaning-gate.md, docs/development-workflow.md
write_scope:         src/**, docs/trial-res01-resource-kinds-report.md,
                     docs/trial-log.md, docs/README.md
frozen:              Yield table, stamina cost table, witness cost,
                     band thresholds, the ten oracles' count, the
                     three-owner split, the two existing verbs and
                     their gates and refusal reasons. The grammar
                     fingerprint and fixture identity are licensed to
                     move exactly once, to the pre-registered value.
red_required:        yes (capability red — see §3)
verification:        git status --porcelain (empty)
                     cargo fmt --check
                     cargo clippy --all-targets -- -D warnings
                     cargo clippy --all-targets --features bevy-host -- -D warnings
                     cargo clippy --all-targets --features bevy-render -- -D warnings
                     cargo clippy --all-targets --features e01-taste -- -D warnings
                     cargo test / --features bevy-host / --features bevy-render
                     BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run
                     BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run --features bevy-host
                     BASELINE_COMMIT=$(git rev-parse --short HEAD) cargo run --features bevy-render
evidence:            red transcript verbatim, gate transcript tail,
                     probe + envelope lines, before/after identities,
                     numbered claims table
limits:              no new dependencies; no new owner; no time model;
                     no randomness; one grammar move
escalate_when:       the kind list cannot express the W01 winter scene;
                     a second grammar move becomes necessary; a
                     registry/schema/persistence format looks required;
                     an oracle would have to be weakened rather than
                     re-scoped
tested_commit:       7c30816 (the green implementation commit; the
                     branch tip after the docs commits is re-gated and
                     recorded in E6 below)
```

## 2. The shape under test

**S-RES01.** Mass is never undifferentiated. A resource kind is a
closed enum with exactly three members — `fodder`, `food`, `timber` —
each with a stable code. A site yields exactly one kind, fixed at seed
time. A holding is a `(character, kind)` pair. Extraction moves mass
from a site into the holding of the *site's* kind, never another.
Receipts that move mass name the kind that moved.

What the shape deliberately does NOT claim: no kind has behaviour of
its own yet (no spoilage, no feeding, no construction). Kinds are
identity and conservation only. Consumption chains are W01's business.

Why no catch-all kind: a generic "material" member would absorb exactly
the pressure that is supposed to force a named kind and a permissioned
move, and cross-kind leakage becomes unfalsifiable when any leak can be
re-labelled as the generic. If the winter scene cannot be expressed in
three kinds, that failure is evidence about the list, not a reason for
an escape hatch.

## 3. Falsifiers (each must fail before the change and pass after)

| ID | Falsifier | Failure meaning |
|----|-----------|-----------------|
| F1 | Cross-kind leakage: gathering at a timber site must not increase any fodder or food holding | kinds are cosmetic labels over one pool |
| F2 | Per-kind conservation: the total of every kind, across sites and holdings, equals its fixture baseline after any command sequence | mass moves between kinds |
| F3 | Kind binding: the kind on a receipt equals the kind of the site it drained; a refusal names no kind it did not touch | receipts can misreport what moved |
| F4 | No kind outside the registry: `ResourceKind::ALL` is exhaustive, every code round-trips, and the grammar fingerprint covers every code | a kind could be admitted silently |
| F5 | Zero-holding canonicality: a holding that reaches zero is indistinguishable — in state, in hash, and in canonical text — from one that never existed | the world hash stops being a function of visible truth |
| F6 | The identity move is exactly one and exactly as predicted (§4) | an unlicensed or accidental second grammar move |

Red evidence for F1–F5 on `1f3cbc6` is a **capability red**: the type
`ResourceKind` does not exist, so the falsifiers cannot be written, let
alone pass. Precedent for capability reds: trials 002, 006, R01
(`AGENTS.md` §6). The transcript is quoted verbatim in the Evidence
section and in `docs/trial-log.md`.

## 4. Pre-registered identity move

The grammar fingerprint is a pure function of declared inputs. Before
touching the crate, those inputs were re-declared in a standalone
program that reproduces `boundary::Fnv1a` and the update order, and its
control stage reproduces today's frozen fingerprint exactly — which is
what makes its predictions for the next stages checkable rather than
decorative.

| Stage | Declared inputs | Predicted grammar fingerprint |
|-------|-----------------|-------------------------------|
| control (master `1f3cbc6`) | yields, costs, witness cost, band map, 11 refusal + 1 partial codes | `0x530003916889b952` — matches the frozen value |
| RES01 tip | control + the three kind codes, hashed after the witness cost and before the band map | `0xc5d782ec145af0a5` |
| V01 tip (next trial, recorded here so the second move is pre-registered too) | RES01 + `GIVE_COST` byte after the witness cost + 6 give refusal codes appended | `0x7dd8c6706e0b949f` |

The fixture, receipt-chain and world identities also move, because
site kinds and per-kind holdings are part of the hashed state. Those
are recorded as *measured* at green, not predicted: they are functions
of the fixture, not of the declared grammar.

Reconstruction recipe for the prediction (`rustc -O predict_grammar.rs`;
kept here rather than in the crate, because nothing in the crate may
read a projection back):

```rust
struct Fnv1a(u64);
impl Fnv1a {
    fn new() -> Self { Self(0xcbf2_9ce4_8422_2325) }
    fn update(&mut self, bytes: &[u8]) {
        for &b in bytes { self.0 ^= u64::from(b); self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3); }
    }
    fn finish(&self) -> u64 { self.0 }
}
const YIELD_TABLE_GRAMS: [[u64; 4]; 4] = [
    [0, 0, 0, 0],
    [250, 400, 600, 900],
    [500, 800, 1200, 1800],
    [750, 1200, 1800, 2700],
];
const STAMINA_COST_BY_BAND: [u8; 4] = [0, 15, 12, 10];
const WITNESS_COST: u8 = 5;
const GIVE_COST: u8 = 3;
const KINDS: [&str; 3] = ["fodder", "food", "timber"];
const REASONS_BASE: [&str; 11] = [
    "unknown_actor", "unknown_site", "unknown_claim", "claim_not_held_by_actor",
    "claim_site_mismatch", "claim_not_witnessed", "actor_exhausted",
    "insufficient_stamina", "site_empty", "claim_already_witnessed",
    "cannot_witness_own_claim",
];
const REASONS_GIVE: [&str; 6] = [
    "unknown_recipient", "cannot_give_to_self", "insufficient_holding",
    "empty_transfer", "unknown_witness", "witness_is_party",
];
const PARTIALS: [&str; 1] = ["site_nearly_depleted"];
fn band_index(points: u8) -> u8 { match points { 0..=9 => 0, 10..=39 => 1, 40..=79 => 2, _ => 3 } }
/// stage: 0 = master today (control), 1 = RES01 tip, 2 = V01 tip.
fn fingerprint(stage: u8) -> u64 {
    let mut h = Fnv1a::new();
    for row in YIELD_TABLE_GRAMS { for cell in row { h.update(&cell.to_be_bytes()); } }
    h.update(&STAMINA_COST_BY_BAND);
    h.update(&[WITNESS_COST]);
    if stage >= 2 { h.update(&[GIVE_COST]); }
    if stage >= 1 { for kind in KINDS { h.update(kind.as_bytes()); } }
    for points in 0..=100u8 { h.update(&[band_index(points)]); }
    for reason in REASONS_BASE { h.update(reason.as_bytes()); }
    if stage >= 2 { for reason in REASONS_GIVE { h.update(reason.as_bytes()); } }
    for reason in PARTIALS { h.update(reason.as_bytes()); }
    h.finish()
}
fn main() {
    println!("control = 0x{:016x}", fingerprint(0));
    println!("RES01   = 0x{:016x}", fingerprint(1));
    println!("V01     = 0x{:016x}", fingerprint(2));
}
```

## 5. Oracle-suite consequence, declared in advance

Oracle 2 (`mass_conserved`) becomes per-kind: it checks every kind's
total against its own fixture baseline, and the sum of the per-kind
totals against the old aggregate baseline. That is strictly stronger —
the old check passes on any world where a gram of fodder became a gram
of timber, and the new one does not. `ORACLE_SUITE_VERSION` is bumped
accordingly (v4 → v5); the oracle *count* stays ten and stays
type-enforced.

No oracle is weakened by this trial. Where V01 later needs oracles 3
and 4 to distinguish "extraction from a site" from "transfer between
characters", that re-scoping is declared in V01's own pre-registration
and is not smuggled in here.

---

# Evidence

Author: Fable 5 (lead). Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`,
`cargo 1.97.1 (c980f4866 2026-06-30)`. Base commit `1f3cbc6`;
`tested_commit` `7c30816` (clean tree).

## E1. Red, verbatim

The falsifiers were written first, against `1f3cbc6`, and the tree did
not compile — a capability red (`AGENTS.md` §6):

```text
      1 error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 17 previous errors
     11 error[E0433]: cannot find type `ResourceKind` in this scope
      2 error[E0599]: no method named `holding` found for struct `economy::EconomyOwner` in the current scope
      1 error[E0599]: no method named `holdings_iter` found for struct `economy::EconomyOwner` in the current scope
      1 error[E0599]: no method named `kind` found for struct `Extraction` in the current scope
      2 error[E0599]: no method named `total_mass_of` found for struct `economy::EconomyOwner` in the current scope
```

Red commit: `ffdc44f` (falsifiers only, no implementation). Reproduce
with `git checkout ffdc44f && cargo test`.

The red is honest about its own limit: it proves the questions could not
be *asked* before, not that a leak existed. Undifferentiated mass has no
cross-kind behaviour to catch — that is exactly why the falsifiers
needed a vocabulary before they could exist.

## E2. Green, verbatim gate tail

```text
oracle PASS stamina_in_bounds (all stamina within 0..=100)
oracle PASS mass_conserved (fodder=2000g/2000g food=5000g/5000g timber=1300g/1300g total=8300g/8300g)
oracle PASS witnessed_gate (0 unwitnessed receipts moved mass)
oracle PASS exhausted_gate (0 exhausted or band-less receipts moved mass)
oracle PASS closed_reasons (0 receipts with unclosed reason codes)
oracle PASS cell_bounds (0 receipts outside the 4x4 cell)
oracle PASS replay_determinism (states_match=true hashes_match=true receipts_match=true)
oracle PASS refusal_zero_mutation (0 hash-chain or mutation violations)
oracle PASS shadow_expectation (0 receipts diverge from the shadow evaluator)
oracle PASS shadow_final_state (0 truth domains diverge from the shadow final state)
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x392e759fb4238743 world=0x77100bd059984f29
bevy_projection views_match=true derived_from=0x77100bd059984f29
bevy_publication revisions=14 derived_from=0x77100bd059984f29 stale_rejected=true
bevy_host_faults admission_zero_mutation=true projection_isolated=true faults=admission_failed,projection_consumer_failed
envelope baseline_commit=7c30816 grammar=0xc5d782ec145af0a5 fixture=0x13524a85dd14d068 receipts=0x392e759fb4238743 world=0x77100bd059984f29 oracles=10v5
```

Test counts, all four feature sets green:
`default 67` · `bevy-host 76` · `bevy-render 84` · `e01-taste 90`
(before: 58 / 67 / 75 / 89).

## E3. Identity movement

| Identity | Before (`1f3cbc6`) | After (`7c30816`) | Status |
|----------|--------------------|-------------------|--------|
| grammar | `0x530003916889b952` | `0xc5d782ec145af0a5` | **matches the pre-registered prediction exactly** |
| standard fixture | `0x3805f1e20c001051` | `0x13524a85dd14d068` | measured; moved because site kinds are hashed state |
| receipts | `0x6c5b0e011471d985` | `0x392e759fb4238743` | measured; receipt lines carry `kind=` |
| world | `0x36221d3fdb8aed9a` | `0x77100bd059984f29` | measured; per-kind holdings are hashed state |
| oracle suite | `10v4` | `10v5` | count type-enforced at ten; version bumped for the per-kind check |

Runs before and after this commit are **not** cross-comparable: both the
grammar and the fixture identity moved (`docs/development-workflow.md`
§ Cross-trial comparison protocol). Any earlier measurement quoted
against the new envelope is a category error.

## E4. Claims table

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|--------------|-------|---------------|--------------------|
| 1 | A gather's grant lands only in the holding of the drained site's kind | every path through `apply_extract`; the kind is read from the site, never from a caller argument | derivation + behavioral | `src/economy/mod.rs` `Extraction.kind` is private and set from `state.kind` in `validate_extract`; test `falsification_cross_kind_leakage_must_be_impossible` |
| 2 | Every kind's total is conserved separately, not merely in aggregate | the standard fixture run and the owner-level extraction sequence | oracle + behavioral | oracle 2 line above (`fodder=2000g/2000g food=5000g/5000g timber=1300g/1300g`); tests `falsification_each_kind_total_is_conserved_separately`, `falsification_kind_swap_must_fail_conservation_at_equal_total` |
| 3 | The v5 conservation oracle is strictly stronger than v4, not merely different | the staged kind-swap where the aggregate total is unchanged | behavioral-red | `falsification_kind_swap_must_fail_conservation_at_equal_total` asserts the aggregate is equal *and* the oracle fails |
| 4 | A receipt's kind is the drained site's kind, and a mislabelled kind is caught by a judge that trusts no receipt field | every receipt in the standard fixture run, refusals included | behavioral-red | `falsification_receipt_kind_is_bound_to_the_drained_site`; `falsification_shadow_oracle_catches_a_mislabelled_kind` (cell_bounds and mass_conserved both still pass on the doctored log) |
| 5 | Within this crate as written, a kind can only come from `ResourceKind::ALL`: three members, distinct codes, contiguous indexes, round-tripping codes, and no runtime path that admits a kind from data or text | the compiled crate at `7c30816` — **not** a claim about future edits, which is governance, not a machine property | derivation | `resource_kinds_are_a_closed_vocabulary`; `ResourceKind::ALL` is the only enumeration, `from_code` is `#[cfg(test)]`, and no owner, boundary or host path constructs a kind from input |
| 6 | The grammar moved exactly once and exactly to the pre-registered value | this branch's history | measurement | prediction in §4 recorded at `fc5e431`, before implementation; envelope line at `7c30816` reads `grammar=0xc5d782ec145af0a5`; `grammar_fingerprint_matches_the_licensed_value` pins it |
| 7 | The host projection carries kinds without gaining authority over them | `bevy-host` and `bevy-render` feature sets on the standard fixture | parity | `bevy_host_parity state_match=true`, `bevy_projection views_match=true` above; `PublishedCharacter::holding_g` is a copied array read, no handle into truth |
| 8 | A holding that reaches zero is stored, hashed and printed identically to one that never existed | the economy owner's storage rule; the *reachable* case (giving to zero) arrives in V01 | derivation | `set_holding` removes on zero; test `zero_holdings_are_never_stored`; shadow mirror `ShadowState::add_holding` |
| 9 | The RS01 and E01 render scenes refuse to draw when their site kind drifts | those two scenes only | derivation | kind guards in `src/render_bevy.rs` and `src/e01_taste.rs` fact extraction, alongside the existing tier guard |

What this trial does **not** claim: that three kinds are the right list
(W01 is the test of that); that any kind has behaviour; that historical
labels are admitted — `fodder`, `food` and `timber` are functional
truth-layer names, and any historical claim about them still needs an
H02 dossier.

## E5. Findings for the author (not resolved here)

1. **Turf vs timber.** The RS01 scene's visual vocabulary says "turf
   blocks"; the licensed kind list's nearest member is `timber`. RS01's
   visual-fact map is frozen evidence, so this trial bound the scene to
   `timber` and left the label alone. Either the kind list eventually
   grows a turf/peat member, or the scene's label changes — both are
   author calls, and W01 will put pressure on this.
2. **F5's reachable half is V01's.** Extraction only adds mass, so no
   RES01 command can drive a holding to zero. The storage rule is
   implemented and unit-tested here so that V01's give-to-zero does not
   have to move state semantics while proving a verb.
3. **Kinds have no behaviour yet.** Nothing prevents a fixture from
   feeding cattle timber, because no consumption exists at all. That is
   W01's question, and stating it here keeps the claim honest.

## E6. Review response (Sol 5.6, 2026-08-18)

Verdict received: **conditional accept**, with independent reproduction
of the 17-error red and of the extended gate at 67 / 76 / 84 / 90 tests,
and independent reproduction of the fingerprint chain
`0x530003916889b952 → 0xc5d782ec145af0a5`.

| Condition | Response |
|-----------|----------|
| Finalize the report status | Status block added at the head of this file |
| Finalize `tested_commit` | Recorded as `7c30816` with the docs-commit rule stated explicitly |
| Tighten the closure wording | Claim 5 rewritten: closure is a property of the compiled crate, scoped away from any claim about future edits, which are governance and not a machine property |

No claim was withdrawn and no evidence changed; the three edits narrow
wording to what the evidence carries. The branch is unchanged in code
from the certified `7c30816`.
