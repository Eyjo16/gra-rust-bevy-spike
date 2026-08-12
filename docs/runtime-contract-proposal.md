# Runtime contract v0.1 — proposal for author and lead review

Date: 2026-08-12. Baseline: `44662a8` (`master`).

Status: **PROPOSED.** This document records executable behavior, identifies
runtime gaps, and offers a minimal contract for ratification. It changes no
runtime code, registry/schema, command, receipt, reason, value, or authority
identity.

Review status: the lead programmer independently checked the proposal against
the repository and recommends ratifying R1–R7 with the custody and R5
evidence-status amendments now incorporated here. Author ruling remains
pending.

## Outcome sought

Make the boundary between deterministic truth and execution hosts hard enough
that dependencies can be selected by required capability rather than by what
an engine happens to provide.

The immediate decision is not “which Bevy crates do we want?” It is:

> What is the smallest runtime promise every host must preserve?

Only after that answer is ratified can a dependency sweep distinguish a
required capability from accidental framework policy.

## 1. Executable law today

The following statements describe the current code and tests. They are not
new design claims.

| Surface | Executable behavior now | Evidence |
| --- | --- | --- |
| Canonical state | One `World` containing three private single-writer owners | Private owner storage and proof-token apply APIs |
| Transaction unit | One immediate `submit(&mut World, seq, Command)` call | `src/boundary.rs` |
| Ordering | Submit order is meaningful; command N+1 validates against command N's result | The 16-command fixture, including witness then later gather |
| Validation | All participating owners validate read-only before apply | `GatherPlan` / `WitnessPlan` |
| Commit | The boundary checks every token fresh before the first owner write | trial/003 stale-later-token falsifier |
| Apply arithmetic | Fallible arithmetic is computed before mutation and exact under coherent-world bounds | trial/008, judge `10v4` |
| Refusal | Produces a canonical receipt and byte-identical truth state | oracle 8 and hash-chain tests |
| Internal invariant failure | Panics loudly; it is not converted into a game outcome | stale-token and incoherent-apply tests |
| Pure/host parity | Pure Rust and Bevy ECS reproduce exact receipt lines and canonical final state on bounded traces | trials 002, 006, 007 |
| Host shape | Bevy stores the whole truth `World` as one ECS resource and invokes one `submit` per `Schedule::run` | `src/host_bevy.rs` |
| Time | Frames, ticks, wall time, and fixed-step time have no canonical meaning | No time value crosses the boundary |
| Persistence | None is defined | Canonical state is serializable for equality, but no restore/checkpoint contract exists |

Two consequences must be stated plainly:

1. The current runtime is **immediate and sequential**, not sealed-turn based.
2. `World::canonical_state()` is an exact observation format, not yet a
   versioned persistence schema.

## 2. Minimal current-semantics runtime contract

This is the narrow contract recommended for ratification before adding turn,
time, persistence, or contention semantics.

### R1 — one canonical writer

Exactly one runtime locus owns mutable canonical `World` access. Hosts may
concurrently collect input, plan read-only work, or derive presentation, but
canonical mutation occurs through exclusive `submit(&mut World, ...)` only.

**Custody amendment:** a host may custody canonical `World` inside its native
container—for Bevy, a resource—but custody does not grant semantic authority.
The canonical resource type and fields remain private to a dedicated custody
module. Exactly one registered host system may request mutable access to that
resource: the commit system that calls `submit`. Projection systems may read a
published canonical observation; projection components must not contain
`World`, owner storage, proof tokens, or another mutable handle back to truth.

The one-mutable-system condition is a code-topology and review invariant. R01
adds the behavioral falsifier it supports: corrupting a projection cannot
alter truth, and republishing replaces the corruption. That runtime test does
not pretend it can enumerate every future system signature; review and module
visibility enforce the custody topology.

### R2 — immediate transaction visibility

Each submitted command validates against the latest committed canonical
state. Its receipt and resulting state become authoritative before the next
command is submitted. Therefore canonical submit order is meaningful today.

Changing this to snapshot/turn visibility is semantic evolution, not a
scheduler refactor.

### R3 — one attempt, one canonical disposition

Given a coherence-validated world and no internal invariant breach, once a
command reaches `submit` it produces exactly one canonical receipt: Accepted,
Partial, or Refused. A host must not silently retry it, reorder it, or
manufacture a game disposition around it.

Exactly-once delivery across crashes is **not** yet claimed because command
identity, durable admission, and recovery are undefined.

### R4 — publication is downstream

Receipts and canonical state are the published facts. ECS view components,
UI state, scenes, animation, logs, and analytics are replaceable projections.
They may lag or fail, but they may not write back or make a transition valid.

A projection should carry the canonical revision/state identity from which it
was derived and be replaceable in full on the next publish.

### R5 — failure classes do not collapse

| Failure class | Canonical receipt? | Truth mutation? | Meaning | Evidence status |
| --- | --- | --- | --- | --- |
| Domain refusal | Yes | No | Expected game result | Proven by oracle 8 and refusal tests |
| Accepted / Partial | Yes | Yes | Expected game result | Proven by boundary/oracle fixtures |
| Startup coherence fault | No | Runtime must not start | Invalid canonical seed/checkpoint | Pure runner proven; host startup gate still needs an explicit path |
| Stale proof or impossible apply | No new game outcome | Must be zero-mutation at the guarded boundary | Runtime/truth bug; fail loudly | Proven by trials 003 and 008 |
| Host failure before `submit` | No | No | Transport/scheduling failure | **Proposed; no injectable host path exists until R02** |
| Presentation/projection failure | No | Canonical commit remains valid | Downstream failure | Proposed; R01/R02 make it executable |
| Crash after commit but before durable publish | Undefined today | Potentially committed | Recovery contract required before persistence | Explicitly unproven / HOLD |

The last row is the current durability gap. It must not be disguised as a
Refused receipt. The evidence-status column is normative restraint: R00 may
ratify the classification before every host path exists, but only R01/R02 can
promote the proposed host rows to tested claims.

### R6 — time is metadata until modeled

Wall time, render frames, Bevy schedule counts, and thread interleavings are
non-canonical. If a tick, deadline, cooldown, or turn becomes game truth, it
must cross the boundary as typed/versioned input or canonical state and gain
its own fixtures and authority identity.

### R7 — dependencies cannot acquire authority

A dependency may implement scheduling, transport, storage, input, projection,
or rendering. Its native entity IDs, clocks, event ordering, serialization,
randomness, or error types do not become canonical merely because the host
uses them.

## 3. What the current contract deliberately does not decide

These are real runtime targets, but each changes or extends meaning enough to
need an explicit ruling and, where applicable, a numbered trial.

### A. Immediate sequence or sealed turns

The draft on `agent/turn-contract` proposes Observe → Collect → Seal → Plan →
Resolve → Preflight → Apply → Publish → Advance → Express. That is a coherent
candidate architecture, but its “no intra-turn enablement” rule differs from
today's sequential visibility.

Author ruling required:

- **A1:** keep immediate sequential transactions as the canonical model; or
- **A2:** introduce sealed turns as a new semantic layer with a degenerate
  one-command-turn bridge to all existing evidence.

### B. Contention disposition

The turn draft proposes a new `Preempted(reason)` outcome for a valid plan
that loses resolution. That changes the closed outcome vocabulary and receipt
meaning. It cannot be ratified as generic runtime plumbing. A real contended
verb pair and domain-owned policy must arrive first.

### C. Sequence ownership and command identity

The caller currently supplies `seq`; the boundary does not enforce monotonic
sequence or deduplicate a command identity. Durable retries and exactly-once
recovery cannot be claimed until authority over sequence/identity is chosen.

### D. Checkpoints and recovery

There is no versioned deserialization format, atomic receipt/checkpoint write,
or recovery rule. Introducing one is a registry/schema task with a migration
account, not an incidental serialization dependency.

### E. Deterministic RNG

There is no RNG today. Sealed scopes are a promising rule if turns are chosen,
but adding an RNG API now would be speculative machinery.

## 4. Review of the existing turn-contract draft

The draft contains useful pressure, but should remain unmerged until the
author rules on its semantic surfaces.

| Draft element | Disposition |
| --- | --- |
| Parallel planning ≠ parallel mutation | Compatible with current ownership doctrine |
| Composite preflight before any mutation | Strong target if batch/turn commits are authorized |
| Degenerate one-command-turn bridge | Required first falsifier if sealed turns are chosen |
| Scheduler-independent canonical resolve | Required, but impossible to specify without contention policy |
| No intra-turn enablement | Semantic change; explicit author ruling required |
| `Preempted(reason)` | Closed-vocabulary/receipt evolution; real domain trial required |
| Sealed deterministic RNG scopes | HOLD until RNG exists |
| Three-way pure/Bevy serial/parallel parity | Correct final host test after the preceding authorities exist |

## 5. Ratification questions for lead and author

1. Do we ratify R1–R7, including the custody and evidence-status amendments,
   as the minimal current-semantics host contract?
2. Is canonical execution immediate (A1), or do we deliberately open sealed
   turns (A2) as semantic evolution?
3. Who owns sequence and durable command identity: boundary, runtime journal,
   or an external registry contract?
4. Is the next concrete host slice projection isolation, or must durable
   recovery precede presentation work?
5. May `bevy-full` remain only a future convenience feature while all next
   runtime slices select explicit capabilities?

Until these are ruled, dependency work may audit what is already present but
must not choose clocks, event buses, persistence formats, or contention
semantics on the architecture's behalf.
