# Branch and worktree development loop

The repository uses short-lived branches in separate worktrees so testing,
reviewing, and refinement can happen without repeatedly disturbing the known
green checkout.

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

## Review-again rule

A green merge is a new baseline, not final proof. The next cycle should attack
the strongest remaining assumption with a different fixture, verb, owner,
host, or language seam. Repeated falsification is how flexible code remains
inside hard truth constraints without allowing the grammar to become a false
guardrail.
