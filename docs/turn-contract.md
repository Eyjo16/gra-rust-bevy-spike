# Turn contract — DRAFT, pending author ratification

Date: 2026-08-10. Baseline: post-007–009 master (judge `10v4`).
Status: **DRAFT.** Principles proposed for ratification; no runtime code
implements this yet. Domain contention policies remain HOLD (see bottom).

## The question this contract answers

Is a turn defined by execution order, or by a sealed set of intentions
resolved from one authoritative snapshot?

**The second.** That gives parallel planning without surrendering
deterministic truth. The trial/003 machinery (snapshot-scoped
validation, entity-revision tokens, all-or-nothing preflight, exclusive
`&mut World` commit) is the substrate this contract names and extends;
trial/008's totality bound is the precondition of its Apply phase.

## The TurnRunner phases

1. **Observe N** — expose one immutable canonical snapshot.
2. **Collect** — player and NPC intentions accumulate without mutation.
3. **Seal** — freeze commands, participants, revisions, and
   deterministic RNG scopes.
4. **Plan** — domain owners validate and produce typed tokens. May run
   in parallel.
5. **Resolve** — declared contention policies reconcile competing valid
   plans into one canonical composite plan. Scheduler order has no
   meaning.
6. **Preflight** — validate the complete composite plan before any
   mutation.
7. **Apply** — one exclusive writer performs total, deterministic owner
   applies.
8. **Publish N** — expose canonical state, receipts, and snapshot
   together.
9. **Advance** — only now does the clock become N+1.
10. **Express** — scenes, UI, saga text, and animation render the
    published facts.

Semantic invariant: **parallel planning ≠ parallel mutation.** An engine
may schedule pure planners concurrently because they read the same
snapshot and write isolated plans; an exclusive commit system owns
mutation. Serial, shuffled, or parallel planner execution must produce
byte-identical canonical sealed input, canonical composite plan,
receipts, and final state.

## Invariants the phases imply (stated so they cannot be lost)

**A. "Scheduler order has no meaning" does not mean "no order exists."**
Resolve must output a canonical apply order that is a pure function of
the sealed set (seq, actor, domain priority — ratified per domain),
never of thread interleaving. Receipts keep a deterministic chain under
that canonical order, with turn boundaries: publish-N's state hash is
turn N+1's chain anchor. Oracle 8 survives with turn-boundary awareness.

**B. No intra-turn enablement.** Plan validity is judged against
snapshot N only. `witness(K)` and `gather(K)` sealed in the same turn do
NOT chain — the witness's effect becomes visible at N+1, and the gather
is refused against the unwitnessed snapshot. Effects are turn-granular
by construction. This deliberately differs from today's sequential
submission; the bridge falsifier below is what keeps that difference
honest.

**C. Losing is an outcome, not an absence.** A plan that was valid
against the snapshot but lost contention in Resolve is neither
`Accepted` nor `Refused` (validation said yes). It receives a receipt
with a new closed outcome class — working name `Preempted(reason)` —
whose reasons are domain-owned and closed, exactly like refusal reasons.
Introducing it is a declared spec evolution of the outcome vocabulary.
No intention ever evaporates without a receipt.

**D. RNG is sealed, never streamed.** Deterministic RNG scopes are fixed
at Seal, per participant/domain, so no planner's dice depend on when it
ran. There is no RNG in the system today; when it arrives, it arrives
pre-sealed, with a same-seal-same-outcome falsifier.

## Falsifiers, in execution order

1. **Degenerate-turn bridge.** The existing 16-command trial replayed as
   16 single-command turns must reproduce today's receipts and envelope
   byte-identically. Turn size 1 is where seal semantics and sequential
   semantics provably coincide; this anchors every receipt already
   recorded to the new model. Without it the TurnRunner resets the
   evidence base to zero.
2. **Joint-feasibility composite red.** Site stock 2000 g; two actors,
   two gather plans each granted 1800 g against snapshot N. Both
   individually valid, both tokens fresh at preflight — and a naive
   per-plan preflight commits plan 1, bumps the site revision, and
   panics mid-composite on plan 2: a partial commit inside the machinery
   built to prevent partial commits. The composite preflight/Resolve
   must reconcile joint overdraw before any mutation. **This is the
   first real red of the series and it is constructible today.**
3. **Resolve determinism under scheduler shuffling.** Same sealed set,
   planners run serial / shuffled / parallel (thread counts, seeds):
   byte-identical composite plan, receipts, and final state.
4. **Three-way parity.** Pure reference turn-runner = Bevy serial
   planning = Bevy parallel planning, compared on canonical sealed
   input, composite plan, receipts (exact lines), and
   `canonical_state()` (exact, hash as checksum only — trial/006 rule).

## HOLD

Domain contention policies (economic, social, political) are not
invented by the TurnRunner and are not ratified by this draft. Each
arrives with the first real contended verb pair in its domain, as its
own closed-outcome spec evolution, behind its own red test — per the
defier audit's rule against speculative machinery. TurnRunner must never
invent a generic tie-break because two jobs happened to execute in some
order.

## Ratification checklist (author)

- [ ] Phases 1–10 as stated
- [ ] Invariants A–D as stated (A and B change/pin game semantics)
- [ ] `Preempted` outcome class approved as declared future evolution
- [ ] Falsifier order approved (bridge first, then composite red)
- [ ] HOLD boundary on contention policies confirmed
