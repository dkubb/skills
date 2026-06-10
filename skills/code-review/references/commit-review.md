# Commit Review Guidelines

Review commits as artifacts, not only the aggregate diff.

The canonical commit form — type and verb sets, subject / body /
action-line rules, size bounds, transformation priority, and
anti-patterns — is `atomic-changes` `references/commits.md`. This file
is the review lens on that form: what to check and how to report it.

## Commit size and granularity

- Treat reviewability as a hard constraint. If a commit is too large, it is a
  review failure even if the code is correct.
- Prefer small, atomic commits: one behavior change, one refactor, or one
  mechanical transformation per commit.
- Avoid mixing unrelated concerns in one commit:
  - formatting with behavior changes
  - rename refactors with semantic changes
  - dependency bumps with feature work
  - broad edits across unrelated modules without a single purpose
  - code and test edits in one refactor commit — without a fixed half
    there is no frame of reference for behavior preservation; expect a
    Change classification or back-to-back refactor commits instead
- Prefer series of commits that can be reviewed independently, in order.
- If a single commit is too large to review, recommend splitting it. Use
  a commit-splitting tool (such as `git-factor`, when available) when the
  user asks to split a commit.

## Diff size thresholds

Apply the canonical size bounds from `atomic-changes`
`references/commits.md` § "Atomic" as the review defaults when there is
no repo-specific rule. A commit in the needs-justification band without
a stated reason is a finding; a commit above the split threshold is a
blocker.

## Commit message quality

- Review subjects and bodies against the canonical rules in
  `atomic-changes` `references/commits.md`: conventional-commit form,
  imperative subject naming the transformation, no "and" / "or", the
  subject verb matching the diff's actual effect.
- Messages must make the series reviewable:
  - stable ordering in dependency order
  - later commits rely on earlier commits in obvious ways
  - reverts and fixups are clearly targeted

## Review output

- If commit structure is the problem, report it as a finding with the commit
  hash and a concrete split recommendation. Do not bury it under code nits.
