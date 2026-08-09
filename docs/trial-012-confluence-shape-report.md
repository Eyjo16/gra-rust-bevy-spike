# Trial/012 — confluence-shape pressure report

Date: 2026-08-09. Branch: `trial/012-confluence-shape`. Shared baseline:
`f5728d6`.

## Trial hypothesis

> The active `StaminaBand × InfraTier` yield cell expresses genuine
> non-separable interaction.

This is an explicit falsifiable hypothesis about the provisional table, not
a contract, historical claim, or permission to tune values. “Non-separable”
has a precise multiplicative meaning here: the effect of infrastructure
cannot be represented by one infrastructure vector scaled independently by
one stamina factor.

The exhausted row is excluded because the boundary refuses exhausted actors
before consulting the table. The reachable yield matrix is therefore:

```text
              none  crude  established  advanced
low            250    400          600       900
steady         500    800         1200      1800
fresh          750   1200         1800      2700
```

## Exact falsifier

For rows `r < s` and columns `i < j`, define the exact minor:

```text
Δ(r,s;i,j) = Y[r,i] × Y[s,j] − Y[r,j] × Y[s,i]
```

A non-zero matrix is rank one exactly when every `2×2` minor is zero. The
probe evaluated all `C(3,2) × C(4,2) = 18` active minors using `u128`
cross-products: no floating point, ratios, tolerance, or rounding.

The red was:

```text
running 1 test
test boundary::tests::falsification_active_cell_must_contain_nonseparable_interaction ... FAILED

thread 'boundary::tests::falsification_active_cell_must_contain_nonseparable_interaction' panicked:
all 18 active 2x2 minors are zero; exact factorization is [1, 2, 3] outer
[250, 400, 600, 900] — the table is rank one

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 42 filtered out
```

Two representative cancellations are:

```text
250 × 800  − 400 × 500  = 200000  − 200000  = 0
600 × 2700 − 900 × 1800 = 1620000 − 1620000 = 0
```

The complete exact factorization is:

```text
Y = [1, 2, 3]ᵀ × [250, 400, 600, 900]
```

The hypothesis is therefore **falsified**, not repaired.

## What separability means

Each axis has an independent multiplier:

- steady yield is exactly `2×` low yield at every infrastructure tier;
- fresh yield is exactly `3×` low yield at every infrastructure tier;
- crude/none is always `1.6×`;
- established/none is always `2.4×`;
- advanced/none is always `3.6×`.

Absolute infrastructure gains still grow with stamina. Moving none → advanced
adds `650g`, `1300g`, then `1950g`. That is ordinary multiplicative
compounding, not an additional cell-specific interaction. Infrastructure
never changes the proportional stamina curve, and stamina never changes the
proportional infrastructure curve.

This conclusion is scale-specific and narrow. On an additive-effects model,
a product can have a non-zero mixed finite difference. Across command chains,
band thresholds, declining stamina, site depletion, and partial grants can
also create dynamic interactions. Trial/012 falsifies only the claim that the
single-step active yield table itself contains a non-separable multiplicative
term.

## Degrees of freedom

An unrestricted active `3×4` table has 12 cell values. A non-zero rank-one
`3×4` table has `3 + 4 − 1 = 6` continuous degrees of freedom: three stamina
factors plus four infrastructure factors, minus one arbitrary shared scale.
Equivalently, normalizing low stamina to `1` leaves two relative stamina
factors and four infrastructure values.

Rank one therefore removes six independent interaction degrees of freedom:

```text
(3 − 1) × (4 − 1) = 6
```

That reduction may be excellent discipline if the axes are intended to be
independent. It is a false expressive claim if the design intends synergy,
antagonism, or diminishing proportional returns in particular cells. Rank
alone cannot choose between those meanings.

## Efficiency compounding

Yield is not the whole pressure surface because stamina cost also changes by
band:

```text
low=15, steady=12, fresh=10 stamina points per gather
```

Mass per stamina point is still separable:

```text
E[band,tier] = (stamina_multiplier[band] / cost[band])
               × infrastructure_base[tier]
```

Relative to low stamina, steady has `2 × 15/12 = 2.5×` the mass efficiency,
and fresh has `3 × 15/10 = 4.5×`, at every infrastructure tier. Thus the cost
curve amplifies the yield curve: fresh produces `3×` the mass but `4.5×` the
mass per stamina point of low. Infrastructure still contributes exactly the
same `1.0/1.6/2.4/3.6` multipliers at all bands.

This is meaningful pressure, but not yet a defect. Whether better stamina
should compound output and efficiency this strongly is a balance-semantic
choice requiring an external expectation, not something rank algebra can
decide.

## Pressure verdict

**Shape hypothesis falsified; values held.**

The result pushes on the meaning of confluence, not on any particular number:

- If stamina and infrastructure are intended as independent efficiency axes,
  rank one is coherent and no interaction value should be invented.
- If “confluence” means that one axis must change the proportional effect of
  the other, the current table cannot express that intent. At least one of its
  six zero interaction degrees must become non-zero—but only after the
  direction has semantic authority.

Changing a cell merely to make a determinant non-zero would be tuning to the
test. The deliberately failing probe was removed, and no green assertion was
left to freeze either rank-one or rank-greater-than-one as grammar.

## Required semantic choice and holdout

Before a value branch may move the table, pre-register one of these meanings:

1. **Independent multipliers:** proportional infrastructure uplift must remain
   equal across stamina bands (`Δ = 0`).
2. **Positive synergy:** infrastructure matters proportionally more at higher
   stamina. For the outer corners, pre-register:

   ```text
   Y[fresh,advanced] × Y[low,none]
     > Y[fresh,none] × Y[low,advanced]
   ```

3. **Diminishing proportional return:** choose the same cross-product with
   `<` instead of `>`.

Then seal a behavioral holdout before fitting any cell. A suitable holdout is
an untouched command chain at a tier not used for calibration, with a
directional prediction for total mass, stamina spent, or time-to-depletion
across a band transition. Reveal it once after the proposed values satisfy
the training contrast. If it fails, record the falsification; do not retune
against the revealed trace.

The efficiency claim needs its own pre-registered holdout as well: state
whether fresh-vs-low mass per stamina should remain `4.5×`, be lower, or be
higher on an untouched tier/trace. Without that semantic choice, moving either
the yield table or stamina costs would be arbitrary even though the present
compounding is mathematically clear.

## Final branch state

The exact-minor probe was intentionally failing evidence and was removed after
capture. The final branch contains documentation only. It changes no runtime
code, production test, dependency, contract, registry/schema, grammar,
fixture, receipt, oracle, or balance value.

Final gate:

- `cargo fmt --check`: clean.
- strict Clippy, default and `bevy-host`: clean.
- default tests: 42/42.
- `bevy-host` tests: 43/43.
- hosted run: exit 0; `receipts_match=true state_match=true
  world_match=true`.

Frozen proof envelope:

```text
envelope baseline_commit=f5728d6 grammar=0x530003916889b952
  fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985
  world=0x36221d3fdb8aed9a oracles=10v3
```
