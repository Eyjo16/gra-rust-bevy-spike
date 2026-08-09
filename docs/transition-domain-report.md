# Trial/007 report — bounded transition-domain parity

Date: 2026-08-09. Branch: `trial/007-transition-domain`. Baseline:
`f5728d6`. Audit target: defier 2 — agreement on one recorded trace is
not universal transition-function equivalence.

## Question and falsifier

Can the Bevy host reproduce the pure host beyond the recorded 16-command
history without acquiring semantics of its own?

The falsifier is a deterministic, feature-gated test harness over the
standard seeded world:

- actors `1..=4` plus unknown `9`;
- claims `1..=9` plus unknown `99`;
- sites `1..=4` plus unknown `9`;
- all 250 possible `Gather { actor, claim, site }` forms and all 50
  possible `Witness { witness, claim }` forms;
- LCG seed `0x007007006d617065`;
- 1,000 traces of depth 32, resetting the standard world per trace;
- exact canonical receipt-line equality, exact `canonical_state()`
  equality, and hash equality after every complete trace.

Illegal commands remain valid inputs to the boundary; refusal is a
result, not a generator filter. The run asserts that all 300 enumerated
command forms occur at least once. If a trace diverges, the harness
greedily removes commands until it has a one-minimal counterexample and
prints the seed, trace index, minimal command list, receipts, states, and
hashes.

## Red — capability absent

Against unmodified `f5728d6`, the test named the missing bounded
comparison and failed before implementation:

```text
error[E0425]: cannot find function `bounded_transition_domain_parity` in this scope
  --> src/host_bevy.rs:93:9
   |
93 |         bounded_transition_domain_parity();
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 1 previous error
```

This is honestly a capability red, not a behavioral divergence: the
old code could express parity only for the recorded history. No host bug
was staged to manufacture a behavioral red.

## Green evidence

```text
transition_domain_parity seed=0x007007006d617065 traces=1000 depth=32 commands=32000 command_space=300 unique_commands=300 receipts_match=true state_match=true world_match=true
transition_domain_verbs {"gather": 26677, "witness": 5323}
transition_domain_outcomes {"accepted:-": 1163, "partial:site_nearly_depleted": 105, "refused:actor_exhausted": 104, "refused:cannot_witness_own_claim": 932, "refused:claim_already_witnessed": 2782, "refused:claim_not_held_by_actor": 19231, "refused:claim_not_witnessed": 251, "refused:claim_site_mismatch": 3878, "refused:insufficient_stamina": 150, "refused:site_empty": 7, "refused:unknown_actor": 230, "refused:unknown_claim": 3167}
transition_domain_bands {"exhausted": 6997, "fresh": 6176, "low": 12341, "steady": 86}
transition_domain_tiers {"advanced": 5253, "crude": 5375, "established": 5404, "none": 5313}
transition_domain_cost_cells gather={"fresh": 216, "low": 359, "steady": 1} witness_flat_uses=1079
transition_domain_yield_cells {"fresh/advanced": 115, "fresh/established": 101, "low/crude": 103, "low/established": 17, "low/none": 116, "steady/established": 1}
```

All 32,000 pure/hosted transitions agreed. No counterexample existed in
the enumerated trace set, so the shrinker did not activate.

## Reachability and balance-pressure map

Receipt observation and value consultation are deliberately separated.
A receipt can report an actor's band or a site's tier even when an
earlier claim gate prevented the cost or yield table from being read.

| Dimension | Reached | Pressure meaning |
| --- | --- | --- |
| Verbs | `gather`, `witness` | Both host paths received substantial traffic. |
| Outcomes | accepted, partial, refused | All three outcome classes occurred. |
| Refusal reasons | 10 of 11 | `unknown_site` did not occur. In a coherent fixture, the social claim/site equality gate runs before economy lookup, so an unknown command site becomes `claim_site_mismatch`; this reason is not boundary-reachable through `gather` here. |
| Observed stamina bands | exhausted, low, steady, fresh | All bands appeared in receipt context, but that does not mean all table rows were consulted. |
| Observed infrastructure tiers | none, crude, established, advanced | All tiers appeared in receipt context, but only six band/tier pairs reached the yield lookup. |
| Gather cost rows consulted | low 359, steady 1, fresh 216 | The exhausted row is intentionally gated out. Steady received only one consultation, so it carries almost no comparative pressure. |
| Witness flat cost consulted | 1,079 | The flat-cost path was exercised without changing its value. |
| Yield cells consulted | 6 of 16 | `low/{none,crude,established}`, `steady/established`, and `fresh/{established,advanced}`. |

The ten yield cells not reached were all four exhausted-row cells
(structurally gated), `low/advanced`, `steady/{none,crude,advanced}`,
and `fresh/{none,crude}`. Future value-pressure work must use a
purpose-built fixture or a named scenario before it can say anything
about those cells. Random receipt volume is not a substitute for cell
consultation.

## Pressure verdict

**Assumption tested:** execution-host neutrality across a wider set of
commands and histories on the standard seeded world.

**Result weight:** materially stronger than trial/002's single
16-command history: 1,000 deterministic histories, 32,000 transitions,
and all 300 enumerated command forms. It is still finite,
fixture-specific evidence. The possible history space is
`300^32`; this run sampled 1,000 of those histories. Both hosts also
share `submit`, so this falsifies host scheduling drift, not a bug shared
by the truth implementation itself.

**Claim permitted:** parity holds for the enumerated trace set
(seed `0x007007006d617065`, 1,000 traces × depth 32) by exact canonical
receipts and exact final state.

**Value permission:** **none.** No parity divergence or incoherent
outcome named a yield, cost, band threshold, fixture number, or other
balance value as faulty. The coverage map says where later pressure can
land; it does not promote the currently reached values from mechanical
examples to balance.

## Change and proof boundaries

Only feature-gated test code and evidence documentation changed. The
registry, schema, contracts, command grammar, standard fixture, balance
values, receipt schema, canonical state, runtime semantics, dependencies,
oracles, and oracle version did not change.

The main proof envelope therefore remains pinned to grammar
`0x530003916889b952`, fixture `0x3805f1e20c001051`, and oracles `10v3`.
Generated test traces have their own command histories and are identified
by seed/index; they do not replace or mutate the standard fixture
sequence.

## Gate

- `cargo fmt --check`: clean.
- strict `clippy`, default and `bevy-host`: clean.
- default tests: 42 passed.
- `bevy-host` tests: 44 passed, including the bounded harness.
- pure and `bevy-host` runs: exit 0; all ten oracles pass.

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope baseline_commit=f5728d6 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v3
```

### Cross-elimination replay before integration

After trials 008 and 009 landed, this branch was rebased and the complete
gate was rerun against oracle judge `10v4`. The bounded result remained
byte-exact across all 32,000 transitions and all 300 command forms; the
default suite passed 47/47 and the `bevy-host` suite passed 49/49. Grammar,
fixture, receipt, and world fingerprints remained unchanged. This replay is
the integration evidence; the `10v3` envelope above remains the historical
envelope of the original trial run.
