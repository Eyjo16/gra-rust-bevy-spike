# Trial/009 — language-seam pressure report

Date: 2026-08-09. Branch: `trial/009-language-seam`. Shared baseline:
`f5728d6`.

## Question under pressure

Can a foreign text source reach the typed command boundary without losing or
silently rewriting meaning before receipts, replay, and host parity begin
observing it?

The falsifier is deliberately outside game semantics. It introduces a small,
test-only text ingestion path for:

```text
gather actor=1 claim=1 site=1
witness witness=3 claim=8
```

No production text format is claimed. The harness exists to locate the proof
boundary and to make pre-boundary loss executable.

## Named observation point

`Command::canonical_bytes()` is now the seam artifact. It is not a new
encoding: trial/009 extracted the exact verb-and-big-endian-ID bytes already
hashed by `fixture_identity`, then made that hash consume the named method.
The frozen fixture identity proves the extraction was byte-preserving.

The resulting claim is narrow:

> Receipt, replay, and host parity can prove meaning only from canonical
> command bytes onward. Every source adapter owns the proof that its input
> reached those bytes without loss.

## Red evidence

The first parser used Rust's ordinary `u64` parsing. The language accepts a
leading `+`, so `actor=+1` silently became the same typed identifier as
`actor=1`. The red demanded rejection before canonical command construction:

```text
running 1 test
test boundary::tests::falsification_text_seam_rejects_leading_plus ... FAILED

thread 'boundary::tests::falsification_text_seam_rejects_leading_plus' panicked:
leading plus silently normalized before canonical command bytes

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 42 filtered out
```

This is a genuine behavioral red, not a staged transition bug: the standard
library parser accepted the source and erased a representation distinction
before the existing evidence chain could see it.

## Minimal green

The test parser now applies one canonical decimal rule before `u64` parsing:
ASCII digits only, no sign, and no leading zero unless the whole value is
`0`. Overflow remains a separate closed fault. Structure is equally strict:
one ASCII space, fixed field names/order, no duplicate or unknown fields.
Every failure is one variant of the closed `TextCommandFault` enum; no input
panics and no input is coerced.

| Pressure input | Verdict |
| --- | --- |
| canonical gather / witness | accept; canonical bytes equal the hand-built command |
| `u64::MAX` | accept; canonical bytes equal the hand-built command |
| leading `+` | reject: non-canonical integer |
| leading `-` | reject: non-canonical integer |
| leading zero | reject: non-canonical integer |
| Unicode digit | reject: non-ASCII |
| `u64::MAX + 1` | reject: integer out of range |
| leading/double/tab/newline whitespace | reject: non-canonical whitespace |
| reordered or duplicate fields | reject: unexpected field |
| unknown field | reject: unexpected field |
| missing or extra field | reject: wrong field count |
| unknown verb | reject: unknown verb |
| empty value | reject: empty value |
| empty line | reject: empty line |
| UTF-8 BOM | reject: non-ASCII |

Targeted green evidence:

```text
running 3 tests
test boundary::tests::falsification_text_seam_rejects_leading_plus ... ok
test boundary::tests::text_seam_accepts_only_canonical_command_meaning ... ok
test boundary::tests::text_seam_rejects_noncanonical_or_ambiguous_sources ... ok

test result: ok. 3 passed; 0 failed
```

Full gate after green:

- `cargo fmt --check`: clean.
- strict Clippy, default and `bevy-host`: clean.
- default tests: 45/45.
- `bevy-host` tests: 46/46.
- hosted run: exit 0; `receipts_match=true state_match=true
  world_match=true`.

## Frozen evidence envelope

```text
envelope baseline_commit=f5728d6 grammar=0x530003916889b952
  fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985
  world=0x36221d3fdb8aed9a oracles=10v3
```

The grammar, fixture, receipt chain, final world, and judge are byte-identical
to the shared baseline. There is no new dependency. No registry/schema,
receipt schema, receipt-reason vocabulary, fixture, grammar, oracle, or
balance value moved. The new parse-fault enum exists only inside the test
harness.

## Pressure verdict

**The pressure lands on representation and normalization, not balance.**

The failure occurs before a command is submitted and before yield, stamina
cost, bands, stock, or any other game value participates. Changing a balance
value cannot falsify or repair it; doing so would merely move downstream
behavior while leaving the source ambiguity intact. The correct response is
to close the accepted source language and name the exact byte boundary.

This is useful negative evidence for value work: when a result diverges before
canonical command bytes, balance is exonerated for that trial. Only a red that
survives byte-identical command construction may exert pressure on transition
rules or values.

## Limits and next attacks

- The parser is a test harness, not a promised production text protocol.
  A real adapter may choose another syntax, but it inherits the same
  reject-or-byte-identical obligation.
- Canonical command bytes currently have no independent schema/version; they
  inherit the closed Rust `Command` shape and fixture identity. Introducing a
  registry/schema later remains a separate, explicitly approved contract task.
- Units are absent from this tiny language. A future quantity-bearing command
  should be attacked with unit aliases and rounding cases before its adapter
  is trusted.
- Unicode rejection is intentionally strict evidence, not a statement that
  authoring tools must be ASCII. Such tools may normalize for users only if
  they retain enough source diagnostics to prove the intended canonical
  command.
