# Trial/010 report — active-cell reachability

Date: 2026-08-09. Branch:
`trial/010-active-cell-reachability`. Baseline: `f5728d6`.

## Question

Can every non-exhausted stamina-band × infrastructure-tier cell in the
active 4×4 yield table actually be reached through a coherent gather,
including the exact-full, one-gram-short partial, and empty-stock
boundaries?

This is reachability pressure, not balance evaluation. It asks whether a
future value hypothesis can reach each number through the real boundary.
It does not ask whether any number is good.

## Red — capability absent

The unmodified baseline had examples for only a subset of the table and
no executable all-active-cell reachability claim. The new falsifier
failed honestly before its purpose-built fixture helper existed:

```text
error[E0425]: cannot find function `assert_all_active_cells_reachable` in this scope
   --> src/boundary.rs:950:9
    |
950 |         assert_all_active_cells_reachable();
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `gra-rust-bevy-spike` (bin "gra-rust-bevy-spike" test) due to 1 previous error
```

This is a capability red. No table value or runtime defect was changed or
staged to manufacture a behavioral failure.

## Falsifier shape

For each of the 12 non-exhausted cells, the test constructs a minimal
coherent world:

- one known actor at a mechanical probe inside the selected band;
- one known site at the selected tier;
- one witnessed claim held by that actor for that site;
- one real `Gather` submitted through `submit`.

The standard fixture is never edited. Probe stamina is test-only:
low 39, steady 79, fresh 100. Stock is derived from the existing cell,
not independently tuned:

1. exactly the requested yield → `Accepted`, full mass moved;
2. requested yield minus one gram → `Partial(site_nearly_depleted)`;
3. zero stock → `Refused(site_empty)` with exact zero mutation.

The full and partial cases expose the selected cell's numeric yield in
the receipt. The empty case traverses the coherent social, stamina-cost,
tier, and yield-selection route before economy refuses; because zero
mass moves, its receipt proves the boundary behavior but cannot
independently reveal the selected numeric yield.

## Green evidence

```text
active_cell band=low tier=none yield_g=250 gather_cost=15 full=accepted partial_g=249 empty=site_empty
active_cell band=low tier=crude yield_g=400 gather_cost=15 full=accepted partial_g=399 empty=site_empty
active_cell band=low tier=established yield_g=600 gather_cost=15 full=accepted partial_g=599 empty=site_empty
active_cell band=low tier=advanced yield_g=900 gather_cost=15 full=accepted partial_g=899 empty=site_empty
active_cell band=steady tier=none yield_g=500 gather_cost=12 full=accepted partial_g=499 empty=site_empty
active_cell band=steady tier=crude yield_g=800 gather_cost=12 full=accepted partial_g=799 empty=site_empty
active_cell band=steady tier=established yield_g=1200 gather_cost=12 full=accepted partial_g=1199 empty=site_empty
active_cell band=steady tier=advanced yield_g=1800 gather_cost=12 full=accepted partial_g=1799 empty=site_empty
active_cell band=fresh tier=none yield_g=750 gather_cost=10 full=accepted partial_g=749 empty=site_empty
active_cell band=fresh tier=crude yield_g=1200 gather_cost=10 full=accepted partial_g=1199 empty=site_empty
active_cell band=fresh tier=established yield_g=1800 gather_cost=10 full=accepted partial_g=1799 empty=site_empty
active_cell band=fresh tier=advanced yield_g=2700 gather_cost=10 full=accepted partial_g=2699 empty=site_empty
active_cell_reachability cells=12/12 cases=36 full=12 partial=12 empty=12
```

| Band (cost) | None | Crude | Established | Advanced |
| --- | ---: | ---: | ---: | ---: |
| Low (15) | 250 | 400 | 600 | 900 |
| Steady (12) | 500 | 800 | 1,200 | 1,800 |
| Fresh (10) | 750 | 1,200 | 1,800 | 2,700 |

Every displayed value was reached through both an exact-full and a
one-gram-short partial gather. Every corresponding empty-stock path
refused without mutation.

## Structural reachability

- **Reachable:** all 12 active cells.
- **Structurally unreachable:** the four exhausted-row cells. This is
  intentional grammar: `plan_gather` returns `actor_exhausted` before
  consulting `STAMINA_COST_BY_BAND` or `YIELD_TABLE_GRAMS`.
- **No accidental holes:** claim ownership, witnessed state, site
  identity, stamina headroom, and stock can all be made coherent for
  each active pair without bypassing the boundary.

The exhausted row's four zeros are therefore sentinels in the table,
not runtime balance choices exercised by gather. Moving them could not
change gather behavior while the gate order remains unchanged.

## Pressure verdict

**Result weight:** strong reachability evidence for this grammar. The
test systematically covers all active indices and their three stock
boundaries, rather than sampling histories. It is not independent
balance evidence: expected yields are deliberately read from the same
table under test, and the synthetic worlds contain no pacing, scarcity,
competition, time, or player-choice model.

**Value permission:** **none.** No value failed a preregistered
expectation. Trial/010 establishes that all 12 active yield values and
all three active gather costs are mechanically pressure-testable. A
later value branch may now name any active cell and bring an independent
directional or metamorphic expectation; it may not cite this reachability
pass as a reason to tune.

## Change and proof boundaries

Only test code and evidence documentation changed. No registry, schema,
contract, command grammar, standard fixture, yield, cost, threshold,
receipt, runtime path, dependency, oracle, or oracle version changed.
The standard envelope remains:

```text
grammar=0x530003916889b952 fixture=0x3805f1e20c001051 oracles=10v3
```

## Gate

- `cargo fmt --check`: clean.
- strict `clippy`, default and `bevy-host`: clean.
- default tests: 43 passed.
- `bevy-host` tests: 44 passed.
- pure and `bevy-host` runs: exit 0; all ten oracles pass.

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope baseline_commit=f5728d6 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v3
```
