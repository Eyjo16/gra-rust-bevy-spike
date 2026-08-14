# Branch and worktree development loop

The repository uses short-lived branches in separate worktrees so testing,
reviewing, and refinement can happen without repeatedly disturbing the known
green checkout.

Semantic authority inside this loop is governed by the ratified
`docs/meaning-gate.md`: discussion may construct hypotheses, but only a
numbered falsifiable trial may select or ship normative meaning.

## Branch roles

- `master` is integrated truth: it must pass the full default compiler gate.
- `trial/<id>-<hypothesis>` carries one falsifiable architecture, behavior, or
  balance hypothesis. Example: `trial/002-bevy-host-parity`.
- `agent/<description>` carries a bounded implementation, documentation, or
  review task performed by an automated collaborator.
- Long-running personal or tool-generated branches may exist remotely, but
  they are integrated only through the same evidence and gate requirements.

A branch should answer one reviewable question. If its description needs
“and” more than once, split it unless the changes form one inseparable proof.

## Worktree location

Keep linked worktrees outside the primary checkout:

```sh
mkdir -p ~/worktrees/gra-rust-bevy-spike
git worktree add \
  ~/worktrees/gra-rust-bevy-spike/trial-002-bevy-host-parity \
  -b trial/002-bevy-host-parity master
```

Do not put a linked worktree inside the repository it belongs to. Use one
directory per branch, and never attach the same branch to two worktrees.

Rust build output can dominate disk use. Sequential worktrees may share a
cache by exporting a common target directory:

```sh
export CARGO_TARGET_DIR="$HOME/.cache/gra-rust-bevy-spike/target"
```

Use separate target directories for genuinely concurrent builds; Cargo's lock
will otherwise serialize them.

## One complete cycle

### 1. Start from integrated truth

```sh
git fetch --prune origin
git switch master
git pull --ff-only origin master
```

Name the hypothesis and its falsifier before implementation. Record it at the
top of `docs/trial-log.md`; use a standalone report when the evidence becomes
large.

### Cross-machine claim and identity check

A dispatch has one executor at a time. When more than one machine can see the
work, the executor claims the branch by pushing its starting ref before making
implementation commits. A second machine may review that ref, but does not
start a parallel implementation under the same branch identity.

Commit identities in handoffs and coordination records are observations, not
authority. Before a commit is recorded as reachable, verify it against an
accessible ref and object store:

```sh
git fetch --prune origin
git cat-file -e <commit>^{commit}
git branch -a --contains <commit>
```

For a bundle, record its SHA-256, run `git bundle verify`, and inspect
`git bundle list-heads` before naming any contained commit as available. A
hash copied from an agent report is never promoted into a coordination record
by memory alone. A superseded evidence branch is filtered commit-by-commit;
its useful standalone evidence may be cherry-picked, but its code is not
merged merely to preserve the history.

### 2. Create an isolated worktree

```sh
git worktree add \
  ~/worktrees/gra-rust-bevy-spike/<branch-slug> \
  -b <branch-name> master
cd ~/worktrees/gra-rust-bevy-spike/<branch-slug>
```

### 3. Make the dangerous assumption fail first

Add the smallest test, oracle case, fixture, or replay comparison capable of
falsifying the hypothesis. Capture the red result before changing the
implementation. A test that begins green has not demonstrated that it can see
the proposed failure.

### 4. Implement within ownership boundaries

- Preserve one writer per truth domain.
- Route cross-owner behavior through the boundary.
- Keep host and presentation state downstream of canonical truth.
- Do not change a registry or schema contract without a separately approved
  contract decision and migration account.
- Label all new balance values and fixture numbers as hypotheses.

### 5. Run the gate

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
```

Host branches add their own parity/replay gate without weakening the default
pure-Rust gate.

### 6. Review the proof, not only the diff

```sh
git status -sb
git diff --check master...HEAD
git diff --stat master...HEAD
git log --oneline master..HEAD
```

The review asks:

- Is the hypothesis narrow and actually falsifiable?
- Was red evidence captured before the fix where applicable?
- Can any new path mutate canonical truth without an owner proof?
- Do refusal and replay properties still hold?
- Is an oracle trusting the same fields as the code it claims to check?
- Is the claim explicitly scoped to the fixtures and traces that could
  falsify it, or is finite evidence being stated as universal equivalence?
- Does a planned write bind every mutable entity and predicate whose value
  made validation pass, rather than only the entities it will mutate?
- Can any apply step still fail after the all-tokens-fresh preflight?
- At a language seam, was meaning preserved before the canonical command was
  constructed, or does parity begin after a lossy normalization?
- Did a closed vocabulary, contract, or registry change? If so, was that
  change explicitly authorized and versioned?
- Do current documents and historical evidence say which claims are proven,
  provisional, or merely mechanical examples?

### 7. Integrate deliberately

After review and a fresh green gate:

```sh
git switch master
git pull --ff-only origin master
git merge --no-ff <branch-name>
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run
git push origin master
```

The merge commit preserves the trial or task as a visible unit. Do not force
push `master`.

### 8. Remove the completed worktree

Run these from the primary checkout after the merge is pushed:

```sh
git worktree remove ~/worktrees/gra-rust-bevy-spike/<branch-slug>
git branch -d <branch-name>
git worktree prune
```

Remote trial branches may be retained while their evidence is useful. Delete
one only after confirming `master` contains it and no review still references
it.

## Cross-trial comparison protocol

When several trial branches run concurrently, their evidence must stay
mutually comparable so that one branch can falsify another's assumptions.
The grammar fingerprint is the shared language, but not the whole judge —
it covers tables, bands, costs, and the closed vocabulary, not the oracle
implementation or the fixture. Every recorded run therefore carries a
**proof envelope**:

| Field | Source |
| --- | --- |
| `baseline_commit` | git, recorded by the runner (`BASELINE_COMMIT` env or log entry) |
| `grammar` | `grammar_fingerprint()` — tables, bands, closed vocabulary |
| `fixture` | `fixture_identity()` — seeded world hash + canonical command sequence |
| `receipts` | `receipt_chain_digest()` — digest of every canonical receipt line |
| `world` | final world hash |
| `oracles` | `ORACLE_COUNT` + `ORACLE_SUITE_VERSION` — which judge evaluated the run |

`cargo run` prints the envelope as its final line. Two runs are
cross-comparable evidence only when `grammar` and `fixture` match. These
64-bit hashes are deterministic checksums and comparison indices, not
collision-free equality proofs. A strong host-parity claim retains and
compares the exact canonical receipt chain and exact canonical final-state
serialization; matching `receipts` and `world` envelope fields alone is
checksum-level evidence. Until exact final-state serialization exists, state
parity claims must say that they are hash-bounded.

Concurrency rules for a multi-branch sprint:

1. All branches spring from the same recorded `baseline_commit`.
2. Only a designated value-pressure branch may change the grammar
   fingerprint; all other branches must hold it constant.
3. Values may move only under a logged, red-first hypothesis: state why
   the old value was wrong, capture the red, then move it. Contracts
   (reason codes, receipt schema, hash chain, registry/schema) change
   only as declared spec evolution.
   For a value claim rather than a mechanical fixture adjustment, also
   pre-register a directional or metamorphic prediction on an untouched
   holdout and reveal it once; do not retune after seeing that result.
4. Merges are serial. Strengthen the judge first (oracle branches merge
   before host or semantics branches); value-pressure merges last because
   it re-baselines everyone. After each merge, every remaining branch
   rebases onto the new master and re-runs red→green before its own
   merge — that re-run is the cross-elimination moment.
5. Every concurrent worktree sets its own build directory (untracked
   `.cargo/config.toml` with a distinct `build.target-dir`), otherwise
   Cargo's lock silently serializes "parallel" branches.

## Review-again rule

A green merge is a new baseline, not final proof. The next cycle should attack
the strongest remaining assumption with a different fixture, verb, owner,
host, or language seam. Repeated falsification is how flexible code remains
inside hard truth constraints without allowing the grammar to become a false
guardrail.
