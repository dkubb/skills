# Git Mechanics

The pipeline's correctness rides on a small set of git behaviors, some
of them empirically sharp-edged. Everything here was learned the hard
way at least once.

## Branch topology

All lanes share one repository; each lane has its own worktree and
branch. The refs and every legal transition between them:

- `<true>` (for example `main`): the published branch. Owned by the
  coordinator. Advances only by the coordinator, only forward, only to
  a proven state: in the worktree holding `<true>`, with a clean tree,
  `git merge --ff-only <proven-oid>`; if `<true>` is checked out
  nowhere, an ancestry check followed by compare-and-swap
  `git update-ref refs/heads/<true> <proven-oid> <expected-old-oid>`.
  (`git branch -f` refuses to move a branch checked out in any
  worktree, and a bare `reset --hard` carries no clean-tree, ancestry,
  or expected-old guard.) Never advances to unfolded history.
- `trail`: the fast lane's branch, based on `<true>`. The fast lane
  rebases it onto `<true>` after every commit; content flows only
  forward from here.
- `verify`: the verification lane's branch. Each pass begins
  `git reset --hard trail` (a snapshot — trail keeps moving), then
  gates its range, recording fixup commits at each stop. The reset
  deletes whatever the previous pass left: it is legal only after that
  pass's fixups are consumed (folded into `candidate` or `<true>`, or
  cherry-picked onto `trail`) or banked to an immutable ref. A
  cherry-pick preserves the correction but does NOT make the next
  literal per-commit pass valid — the historical target commit is
  still red until the fold — so until prior fixups are folded, a new
  pass gates only the new range (from the last previously gated OID),
  not the whole span. `reset --hard` also leaves untracked and ignored
  files in place; assert the worktree clean (including untracked)
  before gating.
- `candidate`: the staging frontier. The coordinator owns the ref: it
  creates it from `verify` (or a reconciled branch), folds fixups with
  the validated fold command (the todo witness under "Fixups,
  autosquash, and folding" — the ONLY sanctioned fold form; a bare
  `git rebase --autosquash` without `-i` skips the sequence editor and
  with it the routing check), and proves it. A conflict-free
  fold is mechanical and stays coordinator work. The moment the fold
  conflicts (era-adaptation, a content judgment), the coordinator
  runs `git rebase --abort`, cuts `fold/<id>` from the untouched
  candidate OID, and dispatches a fold lane in its own worktree on
  that branch — `candidate` itself stays with the coordinator, and no
  second worktree ever checks it out (git refuses anyway). After the
  fold lane reports a proven OID: `candidate` is still checked out,
  and `git update-ref` on a checked-out branch moves the ref while
  leaving that worktree's index and files at the old tree — so in the
  candidate checkout, confirm the ref still points at the OID the
  fold was cut from, then `git reset --hard <proven-oid>` (the
  single-owner compare-and-swap). Reserve `git update-ref` CAS for
  refs checked out nowhere. Unfolded-but-proven work
  may wait here; only after folding and proof does `<true>` move to
  it. If the tip is red, bisect and advance `<true>` to the last green
  boundary instead.
- Reconciliation branches (`reconcile`): created from the diverged
  lane branch, rebased onto the advanced `<true>` with the standard
  pinned non-fold rebase flags (see "The exec gate loop"),
  conflict-resolved per the author's notes,
  re-gated, then treated as the new source for `candidate`. Landing
  is executed by the OWNING lane in its own worktree, on coordinator
  instruction — lane ownership forbids anyone else moving the branch,
  and only the owner knows whether an atom is mid-flight. The owner
  first finishes or banks the current atom so the worktree is clean,
  then checks the cut OID: if the branch still points where the
  reconciliation was cut, it resets to the reconciled result
  (single-owner compare-and-swap). If it kept committing past the
  cut — it is told to — those commits are never discarded: it
  replays `<cut>..<branch>` onto the reconciled result with the
  standard pinned flags and resets to that, banking first if the
  replay conflicts.
- Banked branches (`<lane>-backup`, `fixsrc`, etc.): snapshots of
  interrupted or countermanded work, created by the owning lane. A
  successor consumes the bank by starting from it or cherry-picking
  from it — re-derivation is the failure the bank exists to prevent.
  The coordinator deletes a bank only after its content is proven
  reachable from `<true>`.

Corrections flow: fixups ride in-range on lane branches → fold into
their targets on `candidate` → reach `<true>` folded. A fixup that has
not reached `<true>` before the fast lane rewrites the same code must
be cherry-picked onto `trail` immediately, or it will be dropped; the
cherry-picked copy keeps its `fixup!` identity and recorded target, so
the historical target commit (still red until fold) is repaired when
the fold obligation is paid.

## The exec gate loop

The universal proving and fixing pattern:

```bash
GIT_SEQUENCE_EDITOR=true git rebase -i \
  --reschedule-failed-exec \
  --no-autosquash --no-update-refs --no-autostash --no-rebase-merges \
  --exec '<gate command>' <base>
```

Pin configuration-sensitive behavior by flags, never inherit it: a
repo or global `rebase.autoSquash=true` would let a verification pass
fold fixups it has no authority over, `rebase.updateRefs=true` can
silently move bank or candidate refs that happen to point into the
range, and `rebase.rebaseMerges=true` emits merge-mode todo lines
(`merge -C <oid>`) that the fold witness rightly rejects — the
pipeline's replays are linear by contract. Every rebase in the
pipeline therefore carries
`--no-update-refs --no-autostash --no-rebase-merges`; non-fold
rebases add `--no-autosquash`, and only the fold operation passes
`--autosquash`, explicitly.

- A failing exec stops the rebase with the line rescheduled; fix at
  the stop, then `git rebase --continue` re-runs the same gate — now
  over the fix — before proceeding. How the fix is recorded depends on
  the lane's authority: reconciliation and fold lanes amend the
  stopped commit (they own their replays); verification instead
  records `git add --all && git commit --fixup=<target-oid>` right at
  the stop, so the fixup rides adjacent to its target through the rest
  of the pass. Either way, every regression in unpublished history
  gets fixed at its introducing commit.
- Without `--reschedule-failed-exec` (not a git default), a failed
  exec is consumed and will NOT re-run — after fixing, run the gate by
  hand before continuing.
- A killed exec (timeout, crash) is consumed into the done list before
  it ran to completion; a bare continue silently skips that gate.
  Always re-run the gate manually for the stopped commit after any
  interruption.
- Time out the exec inside the command (`timeout <secs> bash -c ...`)
  and scale the bound with the commit's era — a bound sized for early
  commits clips mature ones whose test suites grew tenfold.
- Era-guard gates when rebasing across the project's birth:
  `if [ -f Cargo.toml ]; then <gates>; fi`.

## Environment hazards (repository hijack)

`git rebase --exec` exports `GIT_DIR` (and in linked worktrees this is
routinely observable) into child processes, and `-C <dir>` does NOT
override an inherited `GIT_DIR`. Consequences observed repeatedly:
child `git init`/`config`/`commit` operations aimed at scratch
directories instead mutate the host repository — including flipping
`core.bare = true`, which breaks the checkout.

Defenses, all of them:

- Library code spawning git scrubs the family:
  `env_remove` for `GIT_DIR`, `GIT_WORK_TREE`, `GIT_INDEX_FILE` (plus
  config isolation: `GIT_CONFIG_GLOBAL=/dev/null`,
  `GIT_CONFIG_NOSYSTEM=1` where determinism matters).
- Test helpers that spawn git scrub identically.
- Deliberately env-inheriting spawns (user-facing operations that must
  respect config, hooks, signing) instead pin the target with explicit
  `--git-dir <resolved-git-dir> --work-tree <worktree-root>` — two
  DIFFERENT paths: resolve the administrative directory first with
  `git rev-parse --absolute-git-dir` under a scrubbed environment,
  because a linked worktree's git dir is not `<worktree>/.git`.
  Command-line flags outrank inherited environment, unlike `-C`.
- Gate/exec command strings in linked worktrees begin with
  `unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE;` followed by a space.
- Prove immunity with a hostile-environment test: export `GIT_DIR` at
  a byte-snapshotted victim repository, run the operation, assert the
  victim untouched.

## Fixups, autosquash, and folding

- `fixup!` subjects survive rebases (autosquash matches by subject),
  which is what lets lanes rebase freely while fixups stay routable.
- `git commit --fixup=<oid>` records intent at authoring time, but
  autosquash ROUTES by the generated `fixup! <subject>` line — with
  duplicate subjects among eligible targets, a fixup can fold into the
  wrong commit, and the byte-identical final tree will NOT catch it
  (the end tree is the same; only intermediate commits are wrong).
  So: subjects among eligible target commits (non-fixup commits) in
  the unpublished range stay unique — verification flags duplicates
  as findings. Several `fixup!` commits sharing one subject are
  normal; they share one target, and each report entry records that
  target OID.
- The routing check happens BEFORE execution, on the todo, with an
  executable mechanic. Maintain a mapping file of
  `<fixup-oid> <target-oid>` lines built from the lane reports;
  whenever a rebase rewrites the range, refresh it to current OIDs by
  matching `git log --format='%H %s'` output back to the report
  entries. Fold through the validating sequence editor shipped with
  this skill (`scripts/validate-todo`):

  ```bash
  GIT_SEQUENCE_EDITOR="<skill-dir>/scripts/validate-todo <mapping-file>" \
    git rebase -i --autosquash \
      --no-update-refs --no-autostash --no-rebase-merges <true>
  ```

  The script walks the generated todo top to bottom, tracking the
  most recent `pick` line's OID; for every `fixup`/`squash` line it
  expands the abbreviated OID with `git rev-parse`, looks up that
  fixup's recorded target in the mapping file, and requires the
  tracked preceding pick to be exactly that target — any mismatch,
  unknown fixup, or unparseable line exits nonzero, which aborts the
  rebase before a single commit is rewritten. The `-i` form is
  mandatory: without `-i`, `--autosquash` folds while ignoring
  `GIT_SEQUENCE_EDITOR` entirely, so no unguarded fold form is
  sanctioned anywhere in the pipeline. After the fold the original
  OIDs no longer exist, so this pre-execution witness is the only
  routing check there is.
- Target the commit the correction semantically belongs to: for a
  regression, its introducing commit — so no commit in the range
  stays broken — era-adapting content authored against a later
  layout; for a purely contextual correction (formatting, naming,
  layout), the last commit that rewrote that context. Retargeting a
  semantic fix to a later commit merely because it last touched the
  file leaves the commits in between red.
- A fixup not yet folded into the true branch is at risk: if the fast
  lane rewrites the same code from its own base, the fixup's content
  is silently dropped. Fold promptly, or cherry-pick such fixups onto
  the fast lane immediately.
- Fold conflicts resolve by era-adaptation (see lanes.md). The
  byte-identical final tree catches content-resolution drift — and
  only that: misrouting and commit-boundary mistakes leave the end
  tree identical, so they need the todo witness above and the
  per-commit proof pass.

## Gate hygiene

- `set -o pipefail`; never pipe a gate through `head`/`tail` in a
  success-chain — pipes launder exit codes.
- `grep -c` exits nonzero on zero matches; do not use it as a chain
  link.
- Network-touching steps (dependency audits) get hard timeouts; they
  are the classic silent-wedge.
- Coverage contract: measure uncovered-item COUNTS, never
  percentages. Three numbers exist and must not be conflated: the raw
  uncovered count (what the tool reports), the approved debt (an
  enumerated burndown: each excepted region listed, with its reason,
  in the tracked gate configuration), and the enforced gate condition
  (raw count is at most the approved debt — expressed by setting the
  fail-uncovered thresholds to exactly the debt). Debt is shrink-only
  and zero is the target. Gate each test class separately — unit-only
  and property-only runs, each with its own debt — so neither class
  can mask the other's oracle holes. The per-commit INDUCTIVE
  guarantee is set-membership: every uncovered item appears in the
  era's enumerated debt, checked per metric and per test class. The
  numeric threshold — raw count at most the era-graded debt, held at
  the exact current edge — is the fast automatic proxy, but alone it
  admits swaps (an approved item gains coverage while an unapproved
  item loses it; the count never moves), so diff the uncovered-item
  set against the enumeration whenever the coverage report changes.
  A touched-functions-only check is weaker still — a test-only
  commit touches no functions, passes vacuously, and can drop an
  untouched function's coverage — use it only as a mid-pass
  heuristic, never as the guarantee.
- Coverage attributes each function to its best single binary copy —
  completing tests must land in whichever test binary's copy is
  already ahead.

## Bug protocol

1. Reproduction script first — the executable witness.
2. `git bisect run <repro>` when the culprit is not obvious; cheap
   because every commit is gate-green, so the repro is the only
   discriminator.
3. When the introducing commit is still unpublished, fix as a
   `fixup!` there, promoting the repro into that commit's tests: the
   history that publishes never contained the bug. When the
   introducer is already reachable from `<true>`, published history
   is never rewritten — land a forward corrective commit carrying the
   repro as its regression test.
