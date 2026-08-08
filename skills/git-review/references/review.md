# Git history review

Review a history at two scales: each commit as a deployable transformation,
then the range as an ordered composition. A green final tree cannot excuse a
broken, non-atomic, or misleading intermediate commit.

## Target

- Resolve the base and head before reviewing.
- Record the exact head SHA. Do not silently follow a moving branch.
- Review every commit in `BASE..HEAD`, including fixups and merge commits.
- For a skeleton-plus-fixup workflow, review from the skeleton through the
  latest fixup. Do not inspect only the newest commit.
- Read the aggregate diff for intent, then read each parent-to-commit diff for
  ownership and atomicity.
- Include staged, unstaged, and untracked files only when the user asks to
  review a candidate commit or the working tree affects the claimed state.

## Deterministic evidence

Collect mechanical facts before semantic judgment:

1. Resolve the base and target commits.
2. Record each commit's subject, parent, changed files, diff size, and tree.
3. Check canonical message form.
4. Check author and committer date order.
5. Identify `fixup!`, `squash!`, merge, revert, and WIP commits.
6. Record exact-state gate evidence for each commit.

Mechanical checks can reject invalid syntax or metadata. They cannot prove
that a commit is one transformation, that its verb describes the diff, or
that its tests and gates are sufficient.

## Per-commit review

For each commit, answer these questions in order.

### Atom

- What externally visible symbol, behavior, schema object, configuration
  surface, documentation claim, or test contract does the commit change?
- Is there exactly one semantic action verb?
- Can any dependency-closed subset stand alone and pass its relevant gates?
  If yes, the commit is not maximally atomic.
- Would splitting below the proposed boundary break compilation, behavior,
  deployability, or a relevant gate? If no, justify why the smaller unit was
  not retained.
- For an `Add`, does each independent public symbol have its own commit and
  direct tests?
- For a `Refactor`, did code or tests remain untouched as the oracle? If both
  changed, classify it as `Change` or split it.

### Message

- Does the subject use one allowed semantic verb?
- Does that verb match the actual transformation?
- Is the summary imperative, simple, and free of `and` or `or`?
- Does the body explain why the commit exists without repeating the diff?
- Are action lines canonical when the body makes multiple observations?

### Integrity

- Does the commit build and remain deployable on its own?
- Does every new or changed behavior have direct, mutation-sensitive tests?
- Are dependencies, generated outputs, policy changes, and required license
  updates present in the same atom when no green intermediate cut exists?
- Is every changed surface and transitive consumer covered by a relevant gate?
- Is the gate evidence for this exact tree, configuration, dependency set,
  and toolchain?

### Deletion pressure

- For every introduced syntax node or dependency-closed group, ask whether it
  can be deleted or deferred while preserving the commit's promised behavior
  and all gates.
- If deletion preserves the promise, report the node as premature.
- Treat tests and documented contracts as promises, not disposable syntax.
- Use mutation testing as one oracle, not as a substitute for the deletion
  review.

## Range review

- Dependencies must precede dependents.
- Among ready commits, apply the ordering rules from `atomic-changes` in their
  stated priority. Presentation order must not invent dependencies.
- Related commits stay adjacent only after dependency and transformation
  ordering are satisfied.
- A correction belongs to the commit that introduced the defect or churn.
- A pre-existing defect remains a standalone `Fix` at the earliest valid
  position.
- No `fixup!`, `squash!`, WIP, or knowingly broken commit remains in the
  merge-ready history.
- The final range must contain the intended aggregate behavior without
  duplicate changes, dropped edits, or unexplained tree changes.

## Findings

Order findings by severity:

- **Blocker**: a retained commit is broken, non-deployable, non-atomic, lacks
  required behavioral evidence, or cannot run an adopted gate.
- **Major**: the verb, ownership, dependency order, or fixup target is wrong
  enough to mislead review, reversion, or bisection.
- **Minor**: message or metadata form is invalid without obscuring the
  transformation itself.
- **Advice**: an optional improvement that does not violate the contract.

Every finding names the commit, the violated contract, and the smallest
repair. Do not hide history findings under aggregate code-review notes.

## Repair

Review is read-only by default. When the user authorizes repair:

1. Record the original refs and tree hashes.
2. Create visible `fixup!` commits against the commits that own corrections.
3. Use a commit-splitting tool for non-atomic commits.
4. Use a rebase-verification tool to execute relevant gates at every retained
   commit.
5. Autosquash only when requested.
6. Verify messages, dates, commit order, per-commit gates, tree identity, and
   worktree cleanliness after the rewrite.
7. Push rewritten shared refs only with explicit authorization and lease
   protection.
