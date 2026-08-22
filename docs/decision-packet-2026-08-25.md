# Decision packet — 2026-08-25 (time, visibility, stakes)

Status: **PENDING AUTHOR RULING.** Prepared 2026-08-22 by lead, per the
author's dispatch and Sol's architecture review. This document decides
nothing: it separates three questions W01 exposed so each can be ruled
on its own evidence, and records where lead and Sol already agree. It
licenses no vocabulary, no value, no implementation.

W01's finding was one sentence: *nothing is consumed, so nothing is at
stake.* Lead's first reading collapsed that into "consumption needs
sealed turns (A2)". Sol's review corrected it, and lead accepts the
correction: canonical time, execution visibility, and the bearer of
consequences are **three separate decisions**. Runtime Contract R6
already permits canonical time if it crosses the boundary as typed
input or state; nothing about consumption forces the A1/A2 question,
and nothing about A1/A2 decides what a winter eats.

## Ruling 1 — time authority

**Question.** What advances canonical time, and what is the smallest
canonical unit?

What evidence says today: there is no time. "The ninth week" is prose
(W01 §E4 finding 2); every plan happens in an instant under A1
immediate/sequential. The D02 warning stands: a clock dependency must
not decide whether ticks are host metadata or canonical input.

Options:

- **1a. No canonical time yet** (status quo). Costs nothing, blocks
  consumption/seasons; W01's largest gap stays open.
- **1b. An explicit canonical period-advance command** — an
  `AdvancePeriod`-shaped boundary event, issued like any command,
  receipted like any command. A1-compatible (Sol's observation).
  Smallest unit is then a *named period*, not a wall-clock tick; the
  host clock never gains authority (R6, R7 respected).
- **1c. Host-driven tick.** Listed for completeness; it hands a
  dependency authority over canonical sequence and falls to R6/R7.
  Neither lead nor Sol proposes it.

If 1b: the exact verb name, its cost (if any), who may issue it, and
what it touches are a separate licensed vocabulary move with its own
pre-registered red — this packet does not draft it.

## Ruling 2 — execution visibility

**Question.** Retain A1 immediate/sequential, or open A2 sealed turns
(`agent/turn-contract` draft)?

What evidence says today: A1 is ratified law (R00 gate). Every
existing trace — the 27-command standard trial and the three W01
plans — is A1 evidence. W01 does **not** prove A2 is required; it
proves consequences are absent, which Ruling 1+3 can address under A1.

If A2 is chosen: T01 must bridge the current **27-command** standard
trial (not the stale 16 the target map used to name) as one-command
turns, byte-identical receipts, states and envelope — and Sol
recommends bridging the three W01 traces the same way. T02/T03/T04
follow only after T01 holds.

If A1 stands: T01–T04 stay parked; composite/contention semantics wait
until a real domain policy needs them (T03's own condition).

## Ruling 3 — stakes

**Question.** What consumes, and what canonical consequence follows an
unmet need?

What evidence says today: the winter need is arithmetic in a
projection; no oracle and no rule reads it (W01 §E4 finding 1, and W3:
the herd-loss shape is inexpressible — no herd, no consumption, no
game-over). The author's stated shape for herd loss is *"threatens the
preservation chain and forces painful alternatives, never immediate
game-over."*

This ruling depends on Ruling 1 (consumption needs a "when") but not
on Ruling 2. It needs from the author, in order:

1. **What eats/burns/rots**: characters, cattle-as-entity, sites,
   structures — which exist, which stay prose for now.
2. **The consequence vocabulary shape**: what happens at an unmet
   need — a state change, a new receipt outcome, a forced-alternative
   mechanism. Any new outcome/reason is explicit contract evolution
   (frozen-vocabulary rule).
3. **Household**: whether "the household" becomes a canonical thing or
   stays a projection sum (W01 §E4 finding 3).

## Sequenced consequence of the three rulings

1b + A1 + a minimal Ruling-3 answer is the smallest path to "something
is at stake" without touching sealed turns. 1a parks the frontier.
A2 reroutes work through T01 first. Any combination is coherent; the
packet only insists the three are ruled separately.

## Annex — held-branch disposition (small, administrative)

For each of trial/011 (`5b52e81`) and trial/012 (`ab45f40`): choose
exactly one — (a) schedule rebase + cross-review toward integration,
or (b) archive by exact commit as historical evidence. Physical
branch/worktree deletion is separate housekeeping and happens only
under explicit instruction.

## Related but not in this packet

- **Consent** (`actor_unwilling`): waits behind the O01
  issuer/seat/delegation model — see
  `trial-o01-authority-preregistration-draft.md`. Not a Tuesday ruling
  unless the author wants to license O01's question.
- **TOOL01** (pinned toolchain + gate script): serial after CON01
  ratification; needs no meaning ruling.
- The 28-day roadmap: shaped Tuesday/Wednesday per the author, on top
  of whatever this packet decides.
