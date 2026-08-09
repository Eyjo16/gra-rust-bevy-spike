# Trial/011 overview — stamina threshold edge chains

Date: 2026-08-09

Branch baseline: `f5728d6`

Scope: map pressure at starts `9, 10, 14, 15, 39, 40, 79, 80` through
real `submit` transitions for gather and witness. This trial may name balance
hypotheses, but it has no license to change values, grammar, the standard
fixture, contracts, registry, or schema.

## Falsifier and capability red

The baseline had isolated band and exact-spend assertions, but no harness that
composed band selection, verb policy, resource headroom, receipt cost/yield,
post-state, and repeated-chain length at the named edges. The test was written
first against an absent collector:

```text
error[E0425]: cannot find function `collect_threshold_edge_observations` in this scope
    --> src/boundary.rs:1032:28
     |
1032 |         let observations = collect_threshold_edge_observations();
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 1 previous error
```

This is an honestly labeled capability red. It does not claim the current
values were already contradicted; it proves the previous tests could not
express the cross-threshold chain comparison requested by the hypothesis.

## Test-only pressure fixtures

- Gather: one actor at the named stamina, one witnessed claim, one Established
  site with ample stock. Submit the same valid gather until the first refusal.
- Witness: the named witness plus another claim holder, one valid site, and 17
  distinct unwitnessed claims. Submit one claim after another until the first
  refusal. Seventeen gives the 80-point start all 16 affordable witnesses plus
  one stopping attempt.
- Both fixtures pass `validate_world_coherence`. They are test-only and do not
  alter `main.rs` or the standard fixture identity.

`exact_headroom` means the starting stamina can pay the selected verb cost by
exact subtraction. It is kept separate from policy eligibility: exhausted
gather has arithmetic headroom for its table's zero cost but is deliberately
refused before the table is used.

## Gather observations

Established-site cell. `first` describes the first receipt; `chain` counts
accepted transitions before the stopping refusal.

| Start | Band | Policy cost | Headroom | First / reason | Spent | Yield | First post | Chain | Total yield | Final | Stop |
| ---: | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| 9 | exhausted | 0 | yes | refused / actor_exhausted | 0 | 0 | 9 | 0 | 0 | 9 | actor_exhausted |
| 10 | low | 15 | no | refused / insufficient_stamina | 0 | 0 | 10 | 0 | 0 | 10 | insufficient_stamina |
| 14 | low | 15 | no | refused / insufficient_stamina | 0 | 0 | 14 | 0 | 0 | 14 | insufficient_stamina |
| 15 | low | 15 | yes | accepted / - | 15 | 600 | 0 | 1 | 600 | 0 | actor_exhausted |
| 39 | low | 15 | yes | accepted / - | 15 | 600 | 24 | 2 | 1,200 | 9 | actor_exhausted |
| 40 | steady | 12 | yes | accepted / - | 12 | 1,200 | 28 | 2 | 1,800 | 13 | insufficient_stamina |
| 79 | steady | 12 | yes | accepted / - | 12 | 1,200 | 67 | 6 | 6,000 | 1 | actor_exhausted |
| 80 | fresh | 10 | yes | accepted / - | 10 | 1,800 | 70 | 6 | 6,600 | 4 | actor_exhausted |

Exact accepted paths, encoded `band:cost/yield->post`:

```text
9  -
10 -
14 -
15 low:15/600g->0
39 low:15/600g->24,low:15/600g->9
40 steady:12/1200g->28,low:15/600g->13
79 steady:12/1200g->67,steady:12/1200g->55,steady:12/1200g->43,steady:12/1200g->31,low:15/600g->16,low:15/600g->1
80 fresh:10/1800g->70,steady:12/1200g->58,steady:12/1200g->46,steady:12/1200g->34,low:15/600g->19,low:15/600g->4
```

## Witness observations

Witness uses its declared flat cost 5, has no exhausted gate, and moves no
mass. Every named start can pay its first witness.

| Start | Starting band | Cost | First / reason | First post | Chain | Final | Stop |
| ---: | --- | ---: | --- | ---: | ---: | ---: | --- |
| 9 | exhausted | 5 | accepted / - | 4 | 1 | 4 | insufficient_stamina |
| 10 | low | 5 | accepted / - | 5 | 2 | 0 | insufficient_stamina |
| 14 | low | 5 | accepted / - | 9 | 2 | 4 | insufficient_stamina |
| 15 | low | 5 | accepted / - | 10 | 3 | 0 | insufficient_stamina |
| 39 | low | 5 | accepted / - | 34 | 7 | 4 | insufficient_stamina |
| 40 | steady | 5 | accepted / - | 35 | 8 | 0 | insufficient_stamina |
| 79 | steady | 5 | accepted / - | 74 | 15 | 4 | insufficient_stamina |
| 80 | fresh | 5 | accepted / - | 75 | 16 | 0 | insufficient_stamina |

The full executable path is printed by the test. Every accepted step records
its current band and `5/0g->post`; chain length is exactly
`floor(start / 5)`. The path continues through Exhausted when at least five
points remain, confirming that band is observational for witness and not a
hidden global action gate.

## Policy verdict versus pressure

No runtime contradiction was found. Every receipt follows the currently
declared verb policy:

- gather refuses Exhausted before consulting its zero-cost/zero-yield row;
- non-exhausted gather still requires exact headroom for its band's cost;
- band is recomputed from post-state before the next command;
- witness stays flat-cost and band-independent, including at start 9.

The composition nevertheless exposes three **candidate balance hypotheses**:

1. **Low-band dead interval:** starts 10–14 are classified non-exhausted but
   cannot gather because Low costs 15. Additional stamina from 10 through 14
   changes neither availability nor outcome. Candidate hypothesis: either the
   band name is not meant to imply gather capability, or the Low floor/cost
   relationship should be pressured in a later value-licensed trial.
2. **39→40 dominance cliff:** one extra point simultaneously lowers first
   cost 15→12 and doubles Established yield 600→1,200. Chain length remains
   two, while total chain yield rises 1,200→1,800 and spent stamina falls
   30→27. Candidate hypothesis: a threshold may improve efficiency, but the
   double leverage needs explicit progression intent.
3. **79→80 dominance cliff:** one extra point lowers first cost 12→10 and
   raises first yield 1,200→1,800. Both chains accept six gathers, while total
   yield rises 6,000→6,600 and spent stamina falls 78→76. Same candidate, at a
   smaller chain-total jump but a larger first-step efficiency jump.

The 9→10 gather edge changes refusal taxonomy (`actor_exhausted` →
`insufficient_stamina`) without changing availability. That is consistent
with the two-layer policy/resource model, but presentation must not imply that
entering Low alone unlocks gather.

## Pressure verdict

**Balance pressure mapped; value change withheld.** The dead interval and two
dominance cliffs are real consequences of the current numbers, not runtime
bugs. They are candidates for falsifiable value hypotheses, not evidence for
which threshold, cost, or yield should move. A later value branch must choose
one candidate, pre-register the intended directional effect and untouched
holdout, then move exactly one hypothesis surface.

The test is a measurement harness and snapshot ratchet, not a declaration that
the current values are correct: it asserts the declared policy relationships,
pins the observed dead interval/dominance signatures, and prints the exact
chains. An intentional value branch is expected to turn this snapshot red,
state its predicted replacement shape, and update the evidence consciously;
an unrelated change is not allowed to move the map silently.

## Focused green

```text
test boundary::tests::falsification_threshold_edges_need_transition_chain_evidence ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out
```

No production code, value, grammar, standard fixture, receipt, oracle,
contract, registry, or schema changed. The proof envelope must remain exactly
`10v3` with the baseline identities.

## Full gate and envelope

```text
cargo fmt --check                                      PASS
cargo clippy --all-targets -- -D warnings              PASS
cargo clippy --all-targets --features bevy-host -- -D warnings
                                                        PASS
cargo test                                             43 passed
cargo test --features bevy-host                        44 passed
cargo run                                              exit 0; 10/10 oracles
cargo run --features bevy-host                         exit 0; parity true
```

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope baseline_commit=f5728d6 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v3
```

All envelope fields are byte-identical to baseline. The additional test is
evidence about a separate pressure fixture, not a new standard fixture or
judge.
