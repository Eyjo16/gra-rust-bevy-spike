# Trial RES01 — closed resource-kind vocabulary

Status while this section stands alone: **pre-registration**. Nothing
below is evidence until the "Evidence" section carries a gate
transcript and a claims table.

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
tested_commit:       <filled at completion>
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
