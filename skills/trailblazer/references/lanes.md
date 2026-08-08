# Lane Contracts

One worktree per lane instance, created with
`git worktree add <dir> -b <lane-branch> <base>`. A lane works only in
its own worktree, never pushes, and never touches another lane's
branch. Spawn lanes on demand when their kind of work appears; the
roster is not fixed.

## Trailblazer (the fast lane)

Mission: outline the shape of each feature — API boundaries, the core
mechanic, forward momentum. First-pass approximations are the product;
completeness is another lane's job.

- Tests: the lane attempts unit tests for what it builds — the happy
  path plus the boundary cases that define the design — but is bound
  to no coverage number; a gap is acceptable output. Property tests
  are permitted when one helps thinking, but their default home is
  the backfill lane (boundaries are the design-defining points;
  interiors belong to property tests).
- Coverage gaps are explicitly acceptable. The fast lane never runs
  coverage, mutation testing, or the full gate.
- Commits are provisional atoms: keep each one a single small
  transformation in the spirit of the `atomic-changes` contract (which
  owns commit form); verification flags non-atomic commits as findings
  for restructuring before publication.
- Inner loop. Precondition: the worktree and index start clean
  (`git status --porcelain` empty) before the atom is written —
  anything already lying around belongs to a bank, not this commit.
  With that guarantee, the worktree holds exactly the atom, so one
  failure-closed chain stages all of it (including new untracked
  files) and the gates prove the tree that gets committed:

  ```bash
  cargo clippy --all-targets --quiet --fix --allow-dirty --allow-staged \
    && cargo fmt --all \
    && git add --all \
    && cargo clippy --all-targets --quiet \
    && cargo test <scope> --quiet \
    && git diff --quiet \
    && test -z "$(git ls-files --others --exclude-standard)" \
    && staged=$(git write-tree) \
    && git commit -q -m '<subject>' \
    && test "$(git rev-parse 'HEAD^{tree}')" = "$staged" \
    && git rebase --no-autosquash --no-update-refs --no-autostash \
         --no-rebase-merges <true-branch>
  ```

  `git diff --quiet` plus the `ls-files --others` emptiness check
  prove the gates mutated nothing after staging, so the tree the
  gates saw IS the staged tree; if either fails (a lockfile update, a
  regenerated fixture), re-run the chain from the top so those bytes
  become gated input. `git write-tree` captures the staged tree, and
  comparing it to `HEAD^{tree}` after the commit catches a hook
  rewriting the commit — on mismatch, re-gate `HEAD` before rebasing.
  Never publish a commit whose exact tree the gates have not seen. The fixer pass is not a checker: `clippy --fix`
  exits 0 even when unfixable denied lints remain, so the plain lint
  pass must gate the commit. A lint needing more than a quick fix gets an expect only
  where the repository's lint policy authorizes reasoned expects; the
  expect carries its reason, and anything suppressed this way is
  reported as named debt for the backfill lanes, never silently
  neutralized.
- Rebase onto the true branch after every commit. Trivial conflicts:
  resolve inline. Judgment conflicts: abort, mark the lane diverged in
  the report, and suspend the per-commit rebase — keep committing on
  the current base until reconciliation lands, rather than retrying
  the same conflict after every commit. (Continuing is safe: the
  reconciliation landing replays commits made past its cut — see
  git-mechanics — so nothing committed meanwhile is lost.)
- Instrument the loop: per-iteration write / lint / test wall times and
  fix episodes (violation count, hand-fix duration) to a metrics log.
- Pre-load the repository's conventions (lint-wall expect patterns,
  test placement rules, commit style) so output lands mostly clean —
  fixup volume downstream measures how well this priming works.

## Verification (the mid lane)

Mission: audit each fast-lane commit under the mid-tier gates and
prove corrected states. (Standalone proof of every commit in the final
history is the folded proof pass's claim, not this lane's: until the
fold, a corrected target commit is proven only in combination with its
fixup.)

- The canonical per-commit loop (scrub prefix per git-mechanics;
  reschedule so re-runs are automatic):

  ```bash
  GIT_SEQUENCE_EDITOR=true git rebase -i \
    --reschedule-failed-exec \
    --no-autosquash --no-update-refs --no-autostash \
    --no-rebase-merges \
    --exec 'unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE; \
            <fmt-check && lint && tests && doctests>' <base>
  ```

  The `--no-*` pins matter: inherited `rebase.autoSquash` would make
  this pass fold fixups it has no authority over, and
  `rebase.updateRefs` can drag other refs along (see git-mechanics).

- On a failure the rebase stops at the offending commit: make the
  smallest fix, record it right at the stop with
  `git add --all && git commit --fixup=<target-oid>` (it rides
  adjacent to its target through the rest of the pass), then
  `git rebase --continue` — the rescheduled exec re-runs the gate over
  the fix and proves it. Target the commit the correction
  semantically belongs to (see git-mechanics). `--fixup` takes an
  OID, but autosquash later routes by subject — a duplicate subject
  among eligible targets is itself a finding; repeated `fixup!`
  subjects for one shared target are normal. Verification never
  amends, reorders, or redesigns; amend authority belongs only to
  reconciliation and fold lanes, over their own replays.
- Before any `git reset --hard trail` for a fresh pass: previous-pass
  fixups must be consumed or banked to an immutable ref (see
  git-mechanics topology) — the reset deletes them otherwise. And a
  fresh literal per-commit pass is valid only over history whose
  prior fixups are folded; until then, gate only the new range from
  the last previously gated OID. After the reset, assert the worktree
  clean including untracked files.
- Judgment-level problems become findings in the report, not fixes.
- Safety preconditions before running any commit's tests: verify the
  commit's process-spawning seams carry the environment scrubs (see
  git-mechanics); skip the tests and report if not.
- If another writer appears in the worktree, stop mutating and report
  — a collision means the coordinator broke the one-worktree rule.

## Fixup / backfill lanes

Mission: pay slow-gate debt without touching the fast lane.

- Coverage: two independent debts, each iterated until the raw
  uncovered count is within its approved debt — once under unit tests
  alone, once under property tests alone — so neither class borrows
  coverage from the other (counts per the contract in git-mechanics;
  zero debt is the destination for both). Prefer real behavioral tests via the
  repository's fault-injection idioms (command shims with call
  counters, fifo write failures, permission children, fabricated state
  directories); restructure genuinely dead branches away rather than
  faking coverage; remaining exceptions go in an explicit named
  burndown, never silence.
- Author each fixup against current-tip context and target the commit
  the correction semantically belongs to (see git-mechanics): a
  regression's introducing commit, era-adapted when needed; the last
  rewriter of the context only for purely contextual corrections.
- Mutation testing: kill survivors with targeted tests; report the
  mutant classes the tool cannot generate so nobody mistakes 0-missed
  for total proof.

## Reconciliation lanes

Mission: replay the fast lane onto an advanced true branch.

- Input: the fast-lane author's own reconciliation notes (which side
  wins per file, adaptations needed). Resolution rule: produce at each
  commit the content that commit should have had, consulting the final
  tree as the oracle.
- May amend its replayed commits (it owns them); re-gates the replayed
  range per commit before reporting.

## Fold lane (endgame)

Mission: fold every fixup into its target, then prove the folded
history. (Dispatched when a fold conflicts; a conflict-free fold is
mechanical and the coordinator runs it directly. The lane receives
its own `fold/<id>` branch cut from the candidate OID — never a
worktree on `candidate` itself — and reports a proven OID for the
coordinator to advance to; see git-mechanics.)

- Before executing, validate the generated autosquash todo against
  the recorded fixup-to-target mapping with the skill's
  `scripts/validate-todo` sequence editor (fail-closed; `-i` form
  mandatory — see git-mechanics) — after the fold, the original
  target OIDs no longer exist to check.

- Era-adaptation: a fixup that presupposes later structure (a field, a
  module layout) is adapted to the commit's own era; the later commit
  that introduced the structure absorbs the rest. The final folded
  tree must be byte-identical to the pre-fold tree — that catches
  content-resolution drift; routing and commit-boundary mistakes need
  the todo witness and the per-commit proof pass.
- Proof pass over the folded history: mid-tier gates plus the
  per-commit coverage check — the numeric ceiling at the era-graded
  edge AND the uncovered-item set diffed against the era's enumerated
  debt (the set check is what makes it inductive; the count alone
  admits swaps — see git-mechanics) — with `--reschedule-failed-exec`;
  fix stops by amending the folded commit. Era-grade the debt
  (shrink-only growth converging at the tip's burndown) and era-guard
  the exec (skip gates before the build system exists).

## Integration (the coordinator)

Mission: everything the lanes escalate, plus the true branch.

- Fold audited fixups when the fold is conflict-free mechanics;
  dispatch a fold lane the moment a fold conflicts. Prove candidates,
  advance promptly (a stalled true branch sheds corrections),
  partial-advance to the last green boundary when the tip is red.
- Full gate runs first-and-last on folded candidates only — unfolded
  fixups make early commits definitionally incomplete, so the gate
  policy and the fold policy are coupled.
- Route each lane report per coordination.md: convergence raises
  priority for adjudication, divergent findings are arbitrated,
  operator-decision items are surfaced and never guessed.
