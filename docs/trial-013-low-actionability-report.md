# Trial/013 — Low actionability meaning trial

Date: 2026-08-09

Status: **pre-registered; training evidence unopened; holdout sealed and
unrevealed**

Branch baseline: `261f8e3`

Queue authority: explicit author priority.

Red class: **mathematical — semantic non-identifiability**. A deterministic
runtime demonstrates the consequences of the cost already installed in it; it
does not, by itself, identify whether the author intended a band label to be
descriptive or action-affording.

Held evidence admitted before this seal: trial/011's recorded threshold map.
No trial/013 fixture has been executed before this pre-registration commit.

## Competing meanings

### H-A — descriptive band

`StaminaBand` describes state; it is not an affordance promise. Gather remains
verb-specific: Exhausted is policy-refused, then every other band must pay its
own exact cost. Low gather cost remains **15**, so starts 10–14 may be Low and
still non-actionable for gather.

### H-B — action-affording band

Entering Low must unlock at least one otherwise-valid gather. Low gather cost
becomes **10**, equal to the Low floor. Thresholds and Low yields remain
unchanged.

No hybrid is in scope. In particular, this trial may not move the Low floor,
change a yield, add clamping, change refusal order/codes, or reinterpret
witness.

## Authority identities before evidence

```text
baseline_commit=261f8e3
grammar=0x530003916889b952
fixture=0x3805f1e20c001051
oracles=10v4
```

If and only if the selection rule below accepts H-B, changing the cost bytes
from `[0,15,12,10]` to `[0,10,12,10]` must naturally change grammar to the
precomputed fingerprint `0x757c13702bfc6047`. The standard fixture identity,
judge identity, coherence vocabulary, seam/encoding identity, registry,
schema, reason vocabulary, and receipt schema remain unchanged. Any literal
expectation independent of the production table, including shadow/adversarial
test literals, must move in the same trial. No identity may be bumped manually.

## Training fixture — disclosed before execution

For each start `9, 10, 14, 15`, construct a separate coherent world:

- C1 has exactly the named stamina;
- K1 is held by C1, covers S1, and is witnessed;
- S1 is Established with 10,000 g stock;
- submit exactly one `Gather(C1,K1,S1)` through the real boundary.

Record band, table cost, exact headroom, outcome/reason, spent, mass, post
stamina, and whether the world mutated. These are mechanical fixtures, not
balance authority.

### Exact H-A training prediction

| Start | Band | Table cost | Outcome / reason | Spent | Mass | Post |
| ---: | --- | ---: | --- | ---: | ---: | ---: |
| 9 | exhausted | 0 | refused / actor_exhausted | 0 | 0 | 9 |
| 10 | low | 15 | refused / insufficient_stamina | 0 | 0 | 10 |
| 14 | low | 15 | refused / insufficient_stamina | 0 | 0 | 14 |
| 15 | low | 15 | accepted / - | 15 | 600 | 0 |

Accepted Low starts: 1/3. Aggregate mass moved across the three Low worlds:
600 g. Aggregate stamina spent: 15.

### Exact H-B counterfactual prediction

| Start | Band | Table cost | Outcome / reason | Spent | Mass | Post |
| ---: | --- | ---: | --- | ---: | ---: | ---: |
| 9 | exhausted | 0 | refused / actor_exhausted | 0 | 0 | 9 |
| 10 | low | 10 | accepted / - | 10 | 600 | 0 |
| 14 | low | 10 | accepted / - | 10 | 600 | 4 |
| 15 | low | 10 | accepted / - | 10 | 600 | 5 |

Directional prediction if H-B were selected: accepted Low starts rise 1→3;
aggregate moved mass rises 600→1,800 g; aggregate spent rises 15→30 across
three independent worlds; starts 10 and 14 change from byte-identical refusal
to state-mutating acceptance. Start 9 is unchanged because Exhausted remains a
verb-policy gate.

For the standard fixture, H-B predicts command 10 changes from
`insufficient_stamina` to Accepted: C4 spends 10 (12→2), moves 250 g from S4,
S4 ends 250 g lower, and C4 inventory ends 250 g higher. Receipt grammar
changes on every command; fixture identity stays unchanged.

## Falsifiers

### H-A falsifiers

- Behavioral: under the frozen baseline grammar and the training fixture,
  start 10 or 14 is accepted, or start 15 does not spend 15 and move 600 g.
  This falsifies conformance to the currently installed H-A mechanics; it does
  not automatically select H-B.
- Semantic: independent meaning-bearing evidence, not derived from the cost
  table or its receipts, establishes that entering every non-exhausted band is
  an affordance guarantee for gather.

### H-B falsifiers

- Mechanical: with Low cost 10 and all other bytes frozen, any otherwise-valid
  Low start 10–15 fails to gather exactly once, spends other than 10, or moves
  other than its unchanged Low × Established 600 g cell.
- Scope: achieving actionability requires changing a threshold, yield,
  refusal code/order, witness policy, or contract in addition to cost 15→10.
- Holdout: after an authorized H-B move, the one-time sealed fixture below
  fails its exact directional prediction.

## Selection rule — fixed before evidence

1. A training match to H-A proves only that runtime conforms to its installed
   cost 15. It does **not** select H-A as authorial meaning.
2. A simulated or implemented match to H-B proves only that cost 10 produces
   actionability. It does **not** select H-B as authorial meaning.
3. H-B is selected only if the trial observes an independent, already
   authoritative meaning signal that makes “Low” an affordance promise. A
   receipt generated from either candidate table is not such a signal.
4. H-A is selected only if independent authority explicitly makes bands
   descriptive and verb actionability separate. Existing mechanics alone are
   not such a selection signal.
5. If no independent signal exists, verdict is **inconclusive**, both
   hypotheses remain evidence rather than law, and no value moves.

This rule intentionally permits an inconclusive result. Creating a new
cross-verb invariant, treating the English word “Low” as a hidden contract, or
counting current conformance as authorial preference would manufacture a
discriminator and violate the Meaning Gate.

## Sealed holdout commitment — do not execute during training

Status at seal: **SEALED / UNREVEALED**.

Seal mechanism: this exact fixture and both predictions are committed before
any trial/013 runtime command. The pre-registration commit is the durable
commitment. Reveal is permitted once only, and only after the selection rule
has independently accepted H-B and before that value may ship.

Holdout fixture:

- C7 stamina 29 (Low);
- witnessed K7 held by C7 over S7;
- S7 is Advanced with exactly 1,000 g stock;
- repeatedly submit `Gather(C7,K7,S7)` until first refusal;
- record exact receipts, chain length, total mass, final stamina and stock.

Sealed H-A prediction (cost 15): first gather Accepted, spent 15, moved 900 g,
post stamina 14 and stock 100; second command Refused
`insufficient_stamina`; accepted/partial chain length 1, total mass 900 g,
final stamina 14, final stock 100.

Sealed H-B prediction (cost 10): first gather Accepted, spent 10, moved 900 g,
post stamina 19 and stock 100; second gather Partial
`site_nearly_depleted`, spent 10, moved 100 g, post stamina 9 and stock 0;
third command Refused `actor_exhausted`; accepted/partial chain length 2,
total mass 1,000 g, final stamina 9, final stock 0.

Holdout failure after H-B selection rejects the proposed value. It does not
license retuning after reveal.

## Execution ledger

Pre-registration complete. Training evidence, verdict, gates, proof envelope,
and final holdout status will be appended without rewriting this sealed
section.

## Execution evidence — appended after seal

The disclosed training fixture ran once against unchanged baseline mechanics:

```text
trial013 training start=9 band=exhausted table_cost=0 exact_headroom=true outcome=refused reason=actor_exhausted spent=0 mass=0 post=9 mutated=false
trial013 training start=10 band=low table_cost=15 exact_headroom=false outcome=refused reason=insufficient_stamina spent=0 mass=0 post=10 mutated=false
trial013 training start=14 band=low table_cost=15 exact_headroom=false outcome=refused reason=insufficient_stamina spent=0 mass=0 post=14 mutated=false
trial013 training start=15 band=low table_cost=15 exact_headroom=true outcome=accepted reason=- spent=15 mass=600 post=0 mutated=true
trial013 training_summary accepted_low=1/3 low_spent=15 low_mass=600 meaning_signal=none verdict=inconclusive holdout=sealed_unrevealed
```

Focused green:

```text
test boundary::tests::trial_013_low_actionability_training_does_not_select_meaning ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out
```

The behavioral rows match H-A's mechanical prediction byte for byte. That
falsifies neither installed H-A mechanics nor the implementation of their
current table. It does not satisfy H-A's semantic selection condition: no
independent authority in the observation says that bands are descriptive.

H-B's proposed direction remains mechanically plausible, but no training
field says that `Low` promises an affordance. Accepting H-B because cost 10
would make starts 10 and 14 act would merely restate the proposal as its own
evidence—the manufactured discriminator prohibited by the seal.

## Verdict

**INCONCLUSIVE. No hypothesis is promoted to new normative meaning. No value
moves.**

- H-A remains the currently installed executable behavior, not a newly
  selected semantic law.
- H-B remains an unselected candidate; this trial does not mathematically
  disprove it.
- `STAMINA_COST_BY_BAND` stays `[0,15,12,10]`.
- Grammar stays `0x530003916889b952`; the counterfactual
  `0x757c13702bfc6047` is not installed.
- Fixture, judge, coherence, seam/encoding, registry/schema, reasons, and
  receipt contracts remain untouched.

Final holdout status: **SEALED / UNREVEALED / NOT EXECUTED**. The selection
rule did not accept H-B, so revealing it would be both unnecessary and a
violation of the pre-registration order. No holdout result was used in this
verdict.

## Full gate and evidence envelope

```text
cargo fmt --check                                      PASS
cargo clippy --all-targets -- -D warnings              PASS
cargo clippy --all-targets --features bevy-host -- -D warnings
                                                        PASS
cargo test                                             48 passed
cargo test --features bevy-host                        50 passed
cargo run                                              exit 0; 10/10 oracles
cargo run --features bevy-host                         exit 0; parity true
```

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope baseline_commit=261f8e3 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4
```

Every authority identity and standard-fixture evidence field is unchanged.
The only executable addition is the test-only disclosed training fixture; no
holdout code exists in the test suite, preventing accidental reveal by the
full gate.

## Current-master verdict preparation — 2026-08-14

Status: **VERDICT CANDIDATE — AWAITING EXPLICIT AUTHOR RATIFICATION.** The
sealed report and its disclosed training-only probe were ported from
`d7fdd1b` onto truth master `2dd4db5`. No old trial-log content was imported.
The holdout remains sealed, unrevealed, unexecuted, and absent from code.

~~~text
base_commit:         2dd4db5db6f52b287ebf4f6b8a3d259bf30ba028
objective:           reproduce 013 training evidence on current master and
                     bind the author's later answers to a reviewable verdict
authoritative_files: AGENTS.md; docs/meaning-gate.md;
                     docs/runtime-contract-proposal.md; src/boundary.rs
write_scope:         src/boundary.rs; this report; docs/README.md
frozen:              values; grammar/fixture/judge identities; contracts;
                     registry/schema; closed vocabularies; sealed holdout
red_required:        no — current-base port of a pre-registered meaning trial
verification:        focused training test; full three-feature gates;
                     default and feature-enabled runtime envelopes
limits:              no verdict inference, value move, or holdout reveal
escalate_when:       author ratifies H-A or H-B, or any frozen surface moves
tested_commit:       final pushed branch tip named in the review handoff
~~~

The later decision-frame answers provide an independent semantic signal that
was unavailable to the original training fixture:

- stamina means current bodily capacity and combat readiness;
- a band describes the actor's present condition;
- the cost of a direct action belongs to the verb;
- the player should understand concrete causes and consequences, without the
  simulation exposing every exact internal number.

Those answers strongly support H-A: descriptive band, action cost per verb,
and the currently installed Low gather cost remaining a fixture rather than
a promise that every Low actor can gather. They do not, by themselves, give
an unambiguous branch-named Meaning Gate instruction, so this port records
H-A as the candidate and does not rewrite the original INCONCLUSIVE verdict.

The exact ratification sentence requested from the author is:

> Ratify H-A for trial/013: stamina bands are descriptive; gather retains its
> per-verb fixture cost 15 for now; keep the holdout sealed and do not run it.

If ratified, the next change is documentation-only: append the author verdict
and leave code, values, and the holdout untouched. H-B would require a new
explicit instruction and a separately reviewed value branch before its
one-time holdout protocol could advance.

Current-port claims (trial/013-current#N):

| # | Atomic claim | Scope | Evidence mode | Evidence reference |
|---|---|---|---|---|

Current-port gate on the staged tree: the focused disclosed-training test
passed and printed `holdout=sealed_unrevealed`; format and all three strict
Clippy suites passed; tests passed 57 default / 66 bevy-host / 74
bevy-render. Both feature-enabled runtime probes exited 0 with receipts,
state, and world parity true. Source search again found no holdout fixture in
executable code; the frozen `10v4` envelope remained unchanged.
