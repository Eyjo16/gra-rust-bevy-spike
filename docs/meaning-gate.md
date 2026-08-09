# Invariant: The Meaning Gate — v0.2

**Status:** RATIFIED — 2026-08-09

**Provenance:** v0.1 proposal → Driftmaster scrutiny (F3 and the
rules-6/7 self-reference conflict confirmed) → v0.2 incorporating all seven
amendments plus containment expiry (rule 9b) → ratified by author ruling.

**Scope:** Semantic authority over the truth layer and its projections.

**Authority:** The Meaning Gate is workflow/proposal authority. It became
operative through author ratification under existing proposal authority. It
does not bootstrap through a meaning trial.

## 1. The invariant

> A semantic decision becomes authority only through a numbered falsifiable
> trial. Questions may be discussed to construct competing hypotheses,
> fixtures, and falsifiers; discussion alone may not select or ship meaning.
> Unprepared questions enter the append-only pressure ledger with an explicit
> state and wait until they are trial-ready.

## 2. Rules

1. A meaning question without a fixture and falsifier enters the pressure
   ledger. Discussion may make it trial-ready; it may not resolve it.
2. A trial-ready question runs in ledger order unless its dispatch records one
   queue-jump class: safety/exploit, dependency unblock, judge-first,
   evidence-decay risk, or explicit author priority.
3. Semantic evolution ships with a trial ID, named red class (behavioral,
   capability, or mathematical), canonical evidence envelope and verdict, and
   the bump of the **relevant authority identity**. Identities left unchanged
   are named explicitly. The identities are distinct authorities: grammar,
   fixture, judge (oracle suite), coherence vocabulary, seam/encoding,
   expression policy, registry/schema.
4. A balance value moves only after a pre-registered directional prediction
   and a sealed holdout verdict revealed once. Retuning after reveal is F6.
5. A hypothesis enters the codex as normative meaning only when its verdict
   accepts it. Rejected and inconclusive hypotheses remain evidence, not law.
6. Reopening accepted meaning requires a new trial. Repairing implementation
   back to already-ratified meaning is conformance work, not semantic
   evolution: it needs a regression falsifier and evidence, not a new meaning
   decision, and must not manufacture identity bumps for untouched semantics.
7. Competing meanings are written as H-A, H-B, and so on before evidence is
   revealed.
8. Architecture and workflow proposals remain governed by proposal authority.
9. Reversible emergency containment (disable, rollback, restore existing
   contract) may precede a trial, shipping with an incident ID and a regression
   falsifier. Permanent semantic replacement may not.
   **9b (expiry):** containment decays—within one sprint it must either convert
   to a dispatched trial, be reverted, or record a renewed incident
   justification. Standing containment without renewal is F5 evidence:
   semantics installed by inertia.
10. The pressure ledger is append-only as history and uses explicit states:
    `candidate → promoted | rejected | superseded | blocked |
    withdrawn-by-author`. Nothing disappears; disposition becomes evidence.
    `blocked` requires a named blocking dependency, renewed each sprint it
    persists.
11. New **normative** meaning cites its trial ID. **Descriptive** text cites
    the executable contract, trial, or observation it reports. Rejected
    hypotheses are labeled rejected and never phrased as current law.

## 3. Mechanical enforcement

### Structured trial records, not commit prose alone

Commit trailers are checked by CI but the append-only trial record is the
durable source:

```text
Trial: 013
Red-Class: behavioral
Authority-Change: grammar 0x…→0x… (Low gather cost)
Judge: unchanged 10v4
Fixture: unchanged 0x…
Holdout: sealed 2026-08-XX, revealed once, verdict …
```

### Diff-to-authority mapping

| Changed surface | Required identity/evidence |
| --- | --- |
| Yield/cost tables, band mapping, verb policy, receipt reason codes | grammar fingerprint changes naturally; trial states direction |
| Standard seed or command trace | fixture identity |
| Oracle behavior or set | oracle suite version |
| Fixture admission faults / coherence | coherence evolution record; grammar only if receipt meaning changes |
| Canonical command encoding | seam/schema identity or extraction proof |
| Expression derivation rules | expression-policy identity |
| Registry/schema | explicit contract version + migration account |

A derived fingerprint must change because its governed semantic bytes changed,
never by a manual bump to satisfy CI and never left unchanged by routing a
semantic edit through an ungoverned surface.

## 4. Falsifiers

- **F1 — Compliance:** after ratification, one normative semantic change
  lacking a trial and relevant authority-identity evolution.
- **F2 — Discussion bypass:** one accepted codex meaning selected by argument
  rather than a recorded verdict.
- **F3 — Overreach:** necessary work blocked despite being hypothesis
  construction, conformance repair, documentation repair, or reversible
  containment.
- **F4 — Dead actionable ledger:** a qualified, trial-ready entry receives no
  promotion, disposition, or renewed block reason across two sprints.
- **F5 — Identity laundering:** a trial bumps an unrelated identity, fails to
  bump the identity governing the changed bytes, or sustains containment past
  expiry without renewal (9b).
- **F6 — Value overfit:** a moved value lacks a sealed pre-registered holdout,
  or is retuned after that holdout is revealed.

## 5. Purpose

This invariant exists to kill cyclic redefinition: shapes re-argued from
scratch each session, meaning living in conversation instead of receipts, and
future-possibility weight assessed by loudness instead of by the ordered
ledger. Driftmaster observing that pattern recur under ratification is F2/F4
evidence and must be logged.
