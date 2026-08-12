# Runtime and dependency target map — pre-sweep proposal

Date: 2026-08-12. Baseline: `44662a8`.

Status: **ACTIVE** — gate zero (R00) passed 2026-08-12: R1–R7 ratified with
amendments, A1 immediate/sequential ruled law, R01 authorized as the next
trial, `bevy-full` held as future convenience. Targets are ordered by
authority dependency. Branch IDs are provisional until author/lead ruling.

## Current position

Integrated master contains Meaning Gate v0.2 and trials 007–010. The full gate
passes with exact Bevy parity and envelope:

```text
grammar=0x530003916889b952
fixture=0x3805f1e20c001051
receipts=0x6c5b0e011471d985
world=0x36221d3fdb8aed9a
oracles=10v4
```

Dependency surfaces currently visible:

| Build | Purpose | Local `cargo tree` size | Authority reading |
| --- | --- | ---: | --- |
| default | Pure truth and oracle gate | 1 line; zero external dependencies | Canonical fast gate |
| `bevy-host` | ECS scheduling/parity | 126 lines | Host adapter only |
| `bevy-full` | Default Bevy engine surface | 1,502 lines | Unallocated capability; do not sweep as one unit |

Line counts describe the expanded local Cargo tree, not a security score.
Their purpose is to show the size discontinuity before a broad engine feature
is allowed to choose runtime shape.

## Gate zero — author/lead ruling

### Target R00 — ratify a minimal runtime contract

- Input: `docs/runtime-contract-proposal.md`.
- Falsifier: name one current execution path or required host task that R1–R7
  misclassifies or blocks.
- Output: accepted clauses, amendments, and an explicit A1 immediate vs A2
  sealed-turn ruling.
- Dependencies: none.
- Contract/schema change: none.

No runtime implementation branch should jump this gate.

## Safe host-hardening path under either A1 or A2

### Target R01 — projection non-authority

Status: **PASSED** 2026-08-12 (trial/R01, red→green; custody topology
test + behavioral corruption falsifier; `bevy_projection` line in the
`bevy-host` gate). Evidence in `docs/trial-log.md`.

Hypothesis: Bevy can project canonical state into query/render components
without creating a second truth owner.

Custody topology:

- canonical `Truth` and its fields live behind a private custody-module
  boundary;
- exactly one registered system—the commit system—requests `ResMut<Truth>`;
- projection systems receive read-only published observations and mutate only
  downstream projection data;
- projection components contain no `World`, owner, proof token, or mutable
  handle back to canonical truth.

The first two bullets are visibility/review gates. The behavioral test below
proves their required consequence rather than claiming Rust reflection can
count every future system signature.

Falsifier:

1. publish a projection derived from canonical state;
2. mutate or corrupt the ECS projection out of band;
3. prove canonical `World`, receipts, and hash remain unchanged;
4. republish and prove the projection is replaced from canonical truth.

This is already required by `docs/architecture.md` and adds no gameplay
meaning. It uses existing `bevy_ecs`; no full engine dependency is needed.

### Target R02 — host failure boundary

Status: **PASSED** 2026-08-12 (trial/R02, red→green; host-local closed
`HostFault` vocabulary, injected admission fault with zero canonical
trace and unbroken canonical sequence on retry, projection-consumer
failure isolated from the commit, no-unwind-catching topology pin;
`bevy_host_faults` line in the `bevy-host` gate). Evidence in
`docs/trial-log.md`. The R5 host rows are now tested claims.

Hypothesis: failures outside `submit` cannot become game outcomes or silently
change truth.

This target supplies the first executable evidence for the R5 row “host
failure before `submit`”; that row is proposal-level classification until the
injectable path exists here.

Falsifiers:

- injected admission/scheduling failure before `submit`: zero mutation, no
  canonical receipt;
- projection failure after a successful submit: canonical receipt/state stay
  valid and the downstream failure is reported separately;
- internal stale-plan failure stays loud and zero-mutation rather than being
  translated to Refused.

This likely needs a host-local result type, not a new receipt reason. No new
dependency is justified yet.

### Target R03 — publication identity

Status: **PASSED** 2026-08-12 (trial/R03, red→green; `Publication`
carries the monotone canonical apply count plus the canonical state
hash — both existing canonical observations, no new registry ID;
`ViewConsumer` rejects delayed deliveries by identity alone, downstream
only; `bevy_publication` line in the `bevy-host` gate). Evidence in
`docs/trial-log.md`.

Hypothesis: every projection/render snapshot names the exact canonical state
from which it was derived, and stale views can be detected without becoming
truth.

Falsifier: delay/reorder two publications and prove consumers can reject or
replace the stale projection while canonical execution remains unchanged.

The identity should reuse an existing canonical revision/state observation if
adequate; inventing a registry/schema ID requires explicit approval.

## Targets blocked on author/runtime rulings

### Target R10 — command sequence ownership

Blocked by: choice of runtime journal/boundary/external-registry authority.

Required before exactly-once or retry claims. Falsifiers must cover duplicate,
out-of-order, and replayed admissions without laundering host faults into
domain refusals.

### Target R11 — checkpoint and recovery

Blocked by: explicit registry/schema permission and recovery atomicity model.

Required falsifier: crash at every boundary around command admission, commit,
receipt durability, and checkpoint publication; recovery must produce either
the whole committed transition once or the whole prior state, never an
ambiguous half-publication.

`World::canonical_state()` alone is not permission to create a restore format.

## Sealed-turn path, only if A2 is ratified

### Target T01 — degenerate-turn bridge

Run the existing 16 commands as sixteen one-command turns. Exact receipts,
canonical states, and envelope must remain byte-identical. This is the first
falsifier because it preserves all existing evidence.

### Target T02 — composite joint feasibility

Two plans may validate independently against one snapshot yet overdraw a
shared site jointly. The whole composite must resolve/preflight before any
mutation. A panic or partial first apply is the red.

No contention winner policy is introduced here; the fixture may use a case
whose valid composite result is unambiguous, or remain blocked until a real
domain policy exists.

### Target T03 — real contention policy

Blocked until a real contended verb pair supplies domain meaning. Precedence,
sharing, lottery, priority, or `Preempted` cannot be inferred from scheduler
order. Any new outcome/reason is explicit contract evolution.

### Target T04 — resolve determinism and three-way parity

After T03: identical sealed input under serial, shuffled, and parallel
planning must yield byte-identical composite plan, receipts, and final state
across pure, Bevy-serial, and Bevy-parallel hosts.

## Dependency sweep order

Dependency sweeps begin only after R00 and should be capability-scoped:

1. **D01 — existing ECS slice.** Audit `bevy_ecs` and its feature tree against
   R01–R03. No renderer/window/assets.
2. **D02 — app/time slice.** Consider `bevy_app`/`bevy_time` only after the
   author rules whether ticks are host metadata or canonical input. A clock
   dependency must not decide this.
3. **D03 — persistence/assets slice.** Consider storage, asset, or scene
   dependencies only after R10–R11 define identity, schema, and recovery.
4. **D04 — input/window/render slice.** Add presentation capabilities only
   after projection non-authority and expression-policy seams are pinned.
5. **D05 — broad engine comparison.** Compare the explicit slices with
   `bevy-full`; retain the broad feature only if its extra surface has named
   consumers and gates.

Standing dependency invariant:

> The default pure truth gate remains zero-dependency. A host dependency may
> fail, upgrade, or be replaced without changing canonical commands, truth,
> receipts, or replay meaning.

## Held evidence outside the runtime path

These branches should not be mistaken for dependency prerequisites:

| Branch | Evidence | Current disposition |
| --- | --- | --- |
| trial/011 | Low dead interval and 39→40 / 79→80 dominance cliffs | Measurement evidence; needs rebase/review before any integration |
| trial/012 | Active yield table is exactly rank one | Mathematical hypothesis falsified; direction still requires author meaning |
| trial/013 | H-A descriptive band vs H-B action-affording band | Inconclusive; cost remains 15; holdout sealed and unexecuted |

None of them authorizes a dependency to decide gameplay meaning. Their next
movement belongs to the Meaning Gate after runtime authority is clearer.
