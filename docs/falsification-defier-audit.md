# Falsification defier audit

Date: 2026-08-09

Baseline: `08db100`

Scope: proof claims and next falsifiers only; no runtime, grammar, value,
registry, schema, or receipt-contract change.

This note records the result of an external, hostile review of the current
single-writer deterministic architecture. The review was driven through
counterexamples: a challenge counted only when it preserved the stated model
and exposed a claim that was stronger than the runtime evidence.

## Mathematical shape under review

Let canonical world state be

```text
S = CharacterState x EconomyState x SocialState
```

and let a command transition be

```text
T(c, S) = (S', receipt)
```

Single-writer ownership is sufficient only while every canonical fact lives in
exactly one private factor of `S`, cross-owner behavior enters through the
boundary, and every host/ECS view is a disposable projection of `S`. A second
authoritative projection would invalidate the product-state claim even if both
copies happened to agree on the current fixture.

For a finite command trace `tau`, the current trial observes

```text
E_tau(T) = (canonical receipts for tau, final world)
```

Equality of `E_tau(T)` between two hosts proves observational equivalence on
that trace. It does not prove `T_host = T_pure` for every legal command and
state: two deterministic implementations can agree on every visited pair and
differ on the first unvisited one.

## Defiers already caught and closed

| Attack | Current answer | Scope of the answer |
| --- | --- | --- |
| Owner-wide revisions false-conflict on disjoint entities | Trial 003 binds proof tokens to the entities they touch | Closed for the current owner/entity operations |
| A later stale token leaves an earlier owner mutation committed | Trial 003 preflights every token before applying any owner mutation | Closed under the current exclusive, serial `&mut World` commit |
| Run and replay share a bug and agree on the wrong final state | Oracle 10 independently steps the fixture and compares all four truth domains | Closed for the modeled domains and exercised command trace |
| Receipt-only shadow checks miss an unlogged final mutation | Oracle 10 compares the shadow result with the actual world, not just receipts | Closed for stamina, inventories, site stocks, and claim gates |

These are real closures, not merely documentation answers. Each has a named
red test or oracle and a green runtime result in `trial-log.md`.

## Surviving defiers

### 1. Hash equality is checksum evidence, not mathematical equality

FNV-1a is deliberately deterministic, but it is not injective. Therefore

```text
hash(x) = hash(y)  does not imply  x = y
```

The proof envelope is an excellent comparison index and tamper/difference
detector. It is not, by itself, an exact state proof. The Bevy trial compares
canonical receipt lines exactly but currently compares final worlds by hash.
The stronger evidence package is the exact canonical final-state serialization
plus its hash; the serialization carries equality evidence and the hash makes
it convenient to address and compare.

### 2. Fixture parity is not universal host equivalence

Trial 002 proves that Bevy adds no observable semantics for the recorded
fixture and 16-command trace. It does not prove that no legal unvisited input
can reach a host-specific difference. The claim must remain trace-scoped until
systematic generation, exhaustive bounded enumeration, or a stronger formal
argument expands the observed domain.

### 3. Entity revisions do not automatically protect shared predicates

Per-entity versions remove false conflicts, but a future invariant such as

```text
value(e1) + value(e2) <= limit
```

can still suffer write skew when two plans read both entities and write one
each. A plan token must bind every mutable read that made validation pass, not
only its write targets. Immutable facts and monotone facts that cannot
invalidate the decision do not need artificial conflicts. Cross-entity queries
or aggregates need their own versioned predicate/domain revision when
enumerating every validity dependency is impractical.

No such mutable aggregate invariant exists in the current two verbs. This is a
frontier rule for the first feature that introduces one, not a reason to add
speculative machinery now.

### 4. Preflight safety also depends on apply totality

The weakest sufficient rule for rollback-free multi-owner commit is:

1. Capture the complete validity-relevant entity and predicate revision vector
   during planning.
2. Under exclusive commit access, compare the complete vector before mutation.
3. On mismatch, perform zero mutation.
4. On match, consume all tokens without interleaving.
5. After the successful preflight, every apply is total for the validated
   inputs: it cannot fail, panic, overflow, allocate fallibly, or discover a
   new guard.

The current `&mut World` boundary supplies exclusive serial commit access and
trial 003 proves stale-token zero mutation. The remaining audit question is
post-preflight totality. A future truly concurrent commit implementation would
also need an explicit atomic barrier or lock; the current code does not claim
to be that implementation.

### 5. A language seam can lose meaning before parity begins

If another language converts units, rounds a number, reorders a collection, or
normalizes an identifier before constructing the canonical command, both hosts
can execute the same boundary command perfectly while the source intent has
already been lost. Host parity must therefore name its observation point.

The seam proof should compare canonical command bytes immediately on both sides
of normalization, with adversarial fixtures for units, bounds, ordering,
encoding, and numeric representation. Runtime receipt parity begins after that
claim; it cannot substitute for it.

### 6. Red-first value pressure can still overfit the visible fixture

A red test prevents unexplained tuning, but repeated tuning against one visible
fixture can still fit noise. Before moving a value, pre-register:

- the reason the old value is false;
- the exact proposed change;
- the corresponding independent shadow change;
- a directional or metamorphic prediction on an untouched holdout fixture.

Reveal the holdout once. A failure creates a new hypothesis; it does not license
retuning the same proposal after seeing the answer. Trial 005's null result is
consistent with this discipline: no incoherence named a value, so no value
moved.

### 7. Four trials are not four independent proofs

The four branches isolate implementation questions and force re-evaluation
after each merge. Their evidence still shares code, fixtures, tables, and parts
of the judging machinery. Cross-elimination is a strong workflow property, not
statistical or logical independence. Claims should name shared assumptions
rather than counting green branches as independent confirmations.

## Evidence envelope: strong and weak readings

The current envelope is:

```text
(baseline_commit, grammar, fixture, receipts, world, oracles)
```

Its safe reading is: “this code identity, grammar identity, fixture identity,
receipt checksum, final-state checksum, and verifier version belong to the same
recorded run.” It supports fast cross-trial comparison.

The strongest reproducible evidence bundle additionally retains:

```text
exact canonical command trace
exact canonical receipt chain
exact canonical final-state serialization
executable code identity
verifier code identity
```

The envelope hashes may address those artifacts, but should not replace them
when the claim requires exact equality. `baseline_commit` must be populated for
recorded evidence; `-` is acceptable for an informal local run, not for a trial
claim.

## Recommended next falsifiers

In priority order:

1. **Exact host final state:** add a canonical world serialization and require
   pure/Bevy equality before comparing the checksum.
2. **Transition-domain pressure:** generate or enumerate legal bounded command
   traces and shrink any host divergence to a minimal fixture.
3. **Apply-totality panic injection:** make every post-preflight apply step face
   its plausible failure mode and prove no reachable failure remains.
4. **First aggregate invariant:** when one is introduced, write the disjoint
   write-skew red test before choosing read-set or predicate-version machinery.
5. **First foreign-language normalizer:** compare pre-boundary source values
   and canonical command bytes before running receipt parity.
6. **Value holdout:** on the first value-pressure change, pre-register and seal
   the holdout prediction before the value branch sees its result.

The central correction is not to weaken the architecture. It is to keep every
claim exactly the size of the evidence that can currently falsify it.
