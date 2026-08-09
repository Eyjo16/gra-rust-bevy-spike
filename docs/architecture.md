# Truth, execution, and expressive seams

Decision date: 2026-08-09.

## Decision

The game is organized around a deterministic truth kernel with replaceable
execution hosts. Bevy is the first serious host because it supplies ECS
storage and projection, scheduling, interaction, rendering, and a broad test
surface. It does not become the authority for game truth.

The practical distinction is:

> The truth layer decides what state means and which transitions are legal.
> A host decides when to request a transition and how to expose its result.

This preserves flexibility inside hard constraints. Hosts, renderers, tools,
and authoring languages may change freely while canonical identities, state
shapes, transition rules, receipts, and invariants remain stable and testable.

## Authority layers

| Layer | Owns | Must not own |
| --- | --- | --- |
| Registry and schema contracts | Canonical identities, shapes, closed vocabularies, versions | Runtime scheduling or presentation policy |
| Truth kernel | Single-writer owners, validation, proof tokens, apply, invariant-preserving state | Input devices, frames, rendering, editor state |
| Boundary | Commands, outcomes, reason codes, receipts, hashes, canonical serialization | Hidden host-specific mutation paths |
| Host adapters | Scheduling, command submission, ECS projections, replay drivers, persistence transport | Canonical truth or alternate transition semantics |
| Presentation and tools | Views, controls, diagnostics, balance exploration | Authority to declare a transition valid |

The current repository has no independent registry/schema artifact yet. Until
one is introduced deliberately, the closed Rust types plus their tests are the
executable contract. Adding a registry later must be an explicit contract task,
not an incidental refactor.

## Canonical transition shape

Every meaningful change follows one observable form:

```text
current state + typed command
    -> read-only validation
    -> revision-bound proof plan
    -> consume proofs exactly once
    -> next state + canonical receipt
```

A refusal has no apply phase and must leave the world hash byte-identical. A
host cannot skip validation, manufacture proof tokens, or write owner storage.
This makes the boundary a proof surface rather than a grammar-shaped API that
only appears safe.

Grammar is useful after the shapes are hard. It names commands and outcomes,
but executable ownership, transition, and oracle constraints decide whether a
well-formed expression is true.

## Bevy's role

Bevy may:

- receive input and produce typed boundary commands;
- schedule command processing at an explicit deterministic point;
- project canonical state into ECS components used for queries and rendering;
- run fixtures, replays, and host-level integration tests;
- visualize receipts, oracle failures, and state-transition chains;
- replace projection data whenever canonical truth advances.

Bevy may not:

- mutate `CharacterOwner`, `EconomyOwner`, or `SocialOwner` state directly;
- treat an ECS component as a second canonical copy of owner state;
- define a competing reason code, identifier, balance table, or transition;
- make frame timing, system order, floating-point behavior, or query order part
  of a canonical result unless that behavior is explicitly modeled;
- turn a host failure into a game outcome.

The `bevy-host` feature stays off by default. This is not a hold: it preserves a
fast engine-independent truth gate while host-specific checks opt into the
larger dependency surface.

## Multi-language seams

No single notation expresses every game-making problem best. Rust can express
ownership and invariant-preserving transitions; tables can express tunable
data; graphs can express dependency chains; shader languages can express
spatial transformation; other tools can express authorial intent or analysis.

Polyglot work is accepted when every seam has all of the following:

1. A canonical identity and version.
2. A schema or closed type for data crossing the seam.
3. Deterministic normalization before the truth boundary sees the value.
4. An explicit owner for validation and mutation.
5. A canonical receipt or diagnostic for the result.
6. Replay evidence that the alternate host or language preserves meaning.

Generated adapters are preferred when a registry/schema exists. Handwritten
adapters must be tested against the same fixtures. Shared mutable state across
languages is forbidden; one side submits a value or command, and the receiving
owner decides whether it is valid.

The governing principle is not language uniformity but preserved meaning.

## Testing ladder

Tests are layered so agreement inside one implementation cannot certify itself:

1. Type and owner unit tests reject locally invalid states and stale proofs.
2. Boundary tests cover accepted, partial, and refused transition shapes.
3. Oracles audit state, receipts, hash chains, closed vocabularies, and replay.
4. The independent shadow evaluator recomputes expected semantics without
   trusting receipt fields.
5. Each host must replay the canonical fixture and match receipts and final
   world hash from the pure host.
6. Falsification trials deliberately introduce a new verb, owner, host, or
   seam that should break any hidden assumption.

Balance values remain hypotheses inside this ladder. Passing invariants proves
coherence, not historical accuracy, fairness, or fun.

## First Bevy-host acceptance slice

The first host slice should stay deliberately narrow:

1. Initialize the same immutable fixture as the pure host.
2. Project owner state into read-only ECS view components.
3. Submit the existing command sequence through the existing boundary.
4. Apply projection updates only after a canonical receipt is produced.
5. Assert receipt-for-receipt and final-hash equality with the pure host.
6. Add one adversarial host test that attempts or simulates an out-of-band ECS
   mutation and demonstrates that it cannot alter canonical truth.

No new gameplay verb, balance change, registry, or schema belongs in that
slice. Its single hypothesis is that Bevy can host the existing truth without
becoming another owner of it.
