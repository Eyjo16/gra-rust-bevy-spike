# Environment evidence — cold-machine reproduction 001

Date: 2026-08-14. Recorded under author dispatch ("Skrá cold
reproduction og rustc ≥1.95 floor sem environment-evidence"). This is
environment evidence, not a trial: no grammar, value, fixture, judge,
vocabulary, or Meaning Gate change. No verdict is implied for held
trials 011/012/013.

## Authoring envelope

```text
base_commit:         fca5237 (master)
objective:           record one cold-environment gate reproduction and
                     the observed host-gate toolchain floor; stop
authoritative_files: AGENTS.md (gate definition), docs/runtime-target-map.md
write_scope:         docs/environment-evidence-cold-repro-001.md
frozen:              all src/, all identities, all other docs
```

## Environment

A remote ephemeral container never used by the author: Linux 6.18.5
(x86_64), fresh clone. Stock toolchain rustc 1.94.1; upgraded in place
via `rustup update stable` to rustc 1.97.1 (8bab26f4f 2026-07-14),
cargo 1.97.1. Distinct OS image, filesystem, and rustc from every
environment that produced the frozen identities.

## Result: the full ratified gate reproduces byte-identically

On `fca5237`, clean tree, under rustc 1.97.1:

| Gate step | Result |
| --- | --- |
| `cargo fmt --check` | clean |
| `cargo clippy --all-targets -- -D warnings` | clean |
| `cargo clippy --all-targets --features bevy-host -- -D warnings` | clean |
| `cargo test` | 56 passed, 0 failed |
| `cargo test --features bevy-host` | 65 passed, 0 failed |
| `BASELINE_COMMIT=fca5237 cargo run --features bevy-host` | exit 0; all ten oracles PASS; `bevy_host_parity`, `bevy_projection`, `bevy_publication`, `bevy_host_faults` all green |

Emitted envelope, byte-identical to the frozen identities:

```text
envelope baseline_commit=fca5237 grammar=0x530003916889b952
fixture=0x3805f1e20c001051 receipts=0x6c5b0e011471d985
world=0x36221d3fdb8aed9a oracles=10v4
```

Scope of the claim: this proves the gate and envelope reproduce on one
additional x86_64-linux environment and one additional rustc (1.97.1)
for the standard fixture and command trace. It does not extend to other
architectures, targets, or traces, and it is not evidence about any
held meaning question.

## Observed toolchain floor for the host gate

Under the container's stock rustc 1.94.1, both `bevy-host` gate steps
fail before compiling any project code:

```text
error: rustc 1.94.1 is not supported by the following package:
  bevy_ecs@0.19.0 requires rustc 1.95.0
```

The pure (default) gate is unaffected: fmt, clippy, and all 56 pure
tests pass under 1.94.1. So the effective floors observed are:

- pure truth gate: rustc 1.94.1 suffices (older not probed);
- `bevy-host` gate: rustc **>= 1.95.0**, imposed by `bevy_ecs@0.19.0`,
  surfaced as a clear resolver error — an environment red, not a code
  red.

Whether to pin a toolchain (e.g. `rust-toolchain.toml`) or amend the
gate text is a governance decision for the author; this note only
records the observed behavior.

## Incidental process evidence

The first gate attempt in this environment judged the run through a
piped chain that masked a failing exit code, producing an ambiguous
result until rerun unmasked. This is a live instance of the failure the
gate law already names ("never judge a gate through a pipe that can
mask its exit code") and is recorded as evidence that the rule is
load-bearing, not ceremonial.
