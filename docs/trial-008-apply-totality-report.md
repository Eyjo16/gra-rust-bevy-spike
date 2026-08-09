# Trial/008 overview — apply totality under mass-bound pressure

Date: 2026-08-09

Branch baseline: `f5728d6`

Hypothesis: trial/003's all-token freshness preflight gives zero mutation on a
stale plan, but rollback-free commit is safe only when every subsequent owner
apply is total for the validated world. The concrete suspect was saturating
mass addition in both `apply_extract` and oracle 2's aggregate.

## Constructible defier

The defect is reachable through the economy owner's real public
validate/apply path; no private-field injection is needed:

1. Seed site S1 with `u64::MAX` grams and S2 with one gram. Each local value
   is representable, but their aggregate is not.
2. Validate and apply extraction of `u64::MAX` from S1 into C1's inventory.
3. Validate and apply the remaining one gram from S2 into the same inventory.
4. The old `saturating_add` kept the inventory at `u64::MAX`; S2 still lost
   its gram. The old `total_mass` also saturated to `u64::MAX` before and
   after, so `mass_conserved` could report a false green.

Inventories cannot be seeded directly. The two-step transfer matters because
it proves the counterexample through the actual token API rather than by
manufacturing owner internals. The normal command grammar cannot request
`u64::MAX` in one gather, but the owner API accepts arbitrary `MassGrams`, and
fixture coherence previously accepted the overfull seeded world.

## Red evidence

Against the unmodified runtime plus only the falsifier test:

```text
running 1 test
test economy::tests::falsification_overfull_inventory_must_not_silently_clamp ... FAILED

thread 'economy::tests::falsification_overfull_inventory_must_not_silently_clamp' panicked at src/economy/mod.rs:324:9:
u64::MAX + 1 inventory transfer silently clamped instead of failing

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 42 filtered out
```

This is a behavioral red, not a capability red: the second apply returned
normally after destroying one gram.

## Minimal green shape

- `FixtureFault::TotalMassOverflow` closes the fixture vocabulary over this
  invalid state. `validate_world_coherence` rejects it before any command.
- `MassGrams::checked_add` replaces saturating addition.
- `EconomyOwner::checked_total_mass` establishes the exact aggregate bound;
  `total_mass` is exact under the validated-world precondition.
- `apply_extract` computes stock subtraction and inventory addition before
  its first write. A caller inside the crate that bypasses world coherence
  gets a loud panic with zero economy mutation, never a silent clamp or a
  partial owner write.
- Oracle 2 now reads the exact total. `ORACLE_SUITE_VERSION` advances 3 → 4;
  this is a declared judge evolution. Receipt schema, receipt bytes, grammar,
  fixture, runtime values, and registry/schema contracts do not change.

Focused green:

```text
running 2 tests
test boundary::tests::falsification_overfull_mass_fixture_is_rejected ... ok
test economy::tests::falsification_overfull_inventory_must_not_silently_clamp ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 42 filtered out
```

## Totality audit after successful preflight

| Apply concern | Why it cannot become a game-time partial commit |
| --- | --- |
| Character lookup | Only a private token can name the actor; validation found it; no removal path exists; exclusive `&mut World` prevents interleaving. |
| Exact stamina subtraction | Validation proved headroom; the entity revision proves the same character has not changed. |
| Economy site lookup | Validation found the site; no removal path exists; the private token and site revision preserve that fact. |
| Economy stock subtraction | `granted = min(requested, stock)` and a fresh site revision preserves `granted <= stock`. |
| Economy inventory addition | Coherence proves total mass `<= u64::MAX`; extraction moves rather than creates mass, so `inventory + granted <= total mass`; checked arithmetic is computed before mutation. |
| Social claim lookup | Validation found the claim; no removal path exists; a fresh claim revision preserves its mutable gate. |
| Gather's `WitnessPass` | The passed fact is monotone in the current model: claims only move `false → true`; holder and site are immutable. |
| Revision increments | No API seeds or sets a revision. Overflow requires 18,446,744,073,709,551,616 successful applies in one process; even one apply per nanosecond exceeds 584 years. This is a physical execution bound, not a finite-state mathematical proof. |

Rust collection allocation can still abort the process under system-wide
out-of-memory. The current semantic totality claim, like receipt/string
construction elsewhere, is conditional on the allocator functioning; the
standard collections do not expose a recoverable allocation step that can be
bound into these proof tokens. No game input can select that failure.

## Pressure verdict

**Runtime-bound / closed-vocabulary pressure confirmed. Balance-value pressure
refuted for this trial.** The falsifier named representability and judge
arithmetic, not a yield, cost, band threshold, fixture quantity, or other
balance hypothesis. Moving a value would only hide the invalid aggregate.
The correct response was to close the world domain and make arithmetic exact.

No balance value moved. Grammar must remain `0x530003916889b952` and fixture
identity must remain `0x3805f1e20c001051`; only the declared oracle judge
version may change in the proof envelope.

## Full gate and envelope

```text
cargo fmt --check                                      PASS
cargo clippy --all-targets -- -D warnings              PASS
cargo clippy --all-targets --features bevy-host -- -D warnings
                                                        PASS
cargo test                                             44 passed
cargo test --features bevy-host                        45 passed
cargo run                                              exit 0; 10/10 oracles
cargo run --features bevy-host                         exit 0; parity true
```

```text
bevy_host_parity receipts_match=true state_match=true world_match=true receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a
envelope baseline_commit=f5728d6 grammar=0x530003916889b952 fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985 world=0x36221d3fdb8aed9a oracles=10v4
```

Compared with `f5728d6`, grammar, fixture, exact receipts, canonical final
state, receipt digest, and world checksum are unchanged. Only the declared
judge identity advances from `10v3` to `10v4`.

## Remaining proof boundary

Apply totality is established for worlds admitted by
`validate_world_coherence`, under exclusive serial `&mut World` commit and
ordinary process-resource availability. Internal crate code can still bypass
the coherence call; the checked pre-write path makes such misuse loud and
zero-mutation, but a type-level `ValidatedWorld` capability would be needed to
make bypass unrepresentable. That expansion is not justified by this trial.
