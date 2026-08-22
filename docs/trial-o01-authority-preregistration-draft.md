# O01 — authority and intent: pre-registration DRAFT

Status: **DRAFT — UNLICENSED.** Prepared 2026-08-22 by lead for author
review, per Sol's amendment ("prepare O01 authority first"). Nothing
here is dispatched, licensed, or implemented; no vocabulary moves; no
red has been run. This draft exists so that when the author is ready to
rule on consent and requests, the question is already stated precisely
enough to be falsifiable. If the author never licenses it, this file is
a suggestion note and nothing more.

## 0. Why this precedes consent

V01 proved give as an *attributed* transfer whose consent remains
unproven, and recorded that a request "is O01, and it needs E-layer
ownership first". W01 found that "nobody can refuse": Auðr's
attestation in plan A is unconditional because the command says so
(§E4 finding 5).

The tempting fix — add an `actor_unwilling` refusal — is unsound
today, and Sol's review names why: **the truth layer cannot say whose
will a command expresses.** Every command arrives as bare input; there
is no issuer distinct from the actor, no seat, no delegation. A
refusal reason encoding "the actor did not want this" would be a
claim the layer cannot derive from any state it holds — a lie with a
vocabulary entry. Authority must exist before unwillingness can.

## 1. The question, stated falsifiably

Can the boundary distinguish, for one command, the **issuer** (whose
will submitted it) from the **actor** (whose body performs it) — such
that:

1. a command whose issuer is its actor behaves exactly as every
   existing trace (all current evidence preserved byte-identically);
2. a command whose issuer is not its actor is *representable* and its
   legitimacy is decided by explicit state (a seat, a delegation, a
   standing), never by scheduler order or host identity;
3. no projection, host, or dependency can forge issuership (R1/R7
   custody discipline extends to the new field).

## 2. Candidate closed-vocabulary shape (NOT licensed)

Recorded only so the author can see the size of the move; every item
requires its own licensed envelope with a pre-registered red:

- a command-level issuer identity (grammar/cmdfmt move — both
  fingerprints change; live documents move in the same envelope per
  CON01);
- a legitimacy gate: issuer == actor, or an explicit
  seat/delegation fact in social-owner state;
- one refusal reason for a failed legitimacy gate (a statement about
  *state*, e.g. "no standing to command this actor" — still not
  `actor_unwilling`, which asserts inner will);
- only after that exists is `actor_unwilling` even expressible as a
  distinct question, and it may belong to the E-layer, not truth.

## 3. Falsifier sketches

- **Capability red**: today no command shape can carry an issuer
  distinct from its actor — show the falsifier cannot compile
  (precedent: trials 002, 006, R01, V01).
- **Identity red**: the grammar/cmdfmt move is pre-registered with
  predicted fingerprints before implementation (precedent: RES01 §4).
- **Behavioral red**: an issuer≠actor command with no seat/delegation
  fact must refuse with zero mutation; the same command after an
  explicit delegation fact must not be distinguishable in cost or
  yield from the actor's own issuance unless the author licenses a
  difference.
- **Preservation**: the 27-command standard trial and the three W01
  traces replay byte-identically with issuer == actor throughout.

## 4. What this draft does not touch

Time, consumption, households, plans-as-things (decision packet,
rulings 1–3); theft (still unrepresentable, V01 boundary); contracts
and debts (E-layer); the player's diegetic seat (architecture question,
listed here only because a "seat" vocabulary would need to not collide
with it).
