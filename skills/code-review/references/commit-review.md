# Commit Review Guidelines

Review commits as artifacts, not only the aggregate diff.

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
- Prefer series of commits that can be reviewed independently, in order.
- If a single commit is too large to review, recommend splitting it. Use
  `git-factor` when the user asks to split a commit.

## Diff size thresholds

Use these as defaults when there is no repo-specific rule:

- 10-20 changed lines: ideal for review.
- 30-50 changed lines: maximum for good human understanding.
- Above 50 lines: expect review quality to drop; require justification.
- Above 300 lines: near zero review quality in one pass. Treat as a blocker.
- Above 1000 lines: not reviewable. Split before review.

These numbers include both code and tests. If both change in one commit, the
commit should still be small and linear.

## Commit message quality

- Prefer conventional commits. Subject line should explain why, not how.
- Subject line must not include "and" or "or". Split into separate commits.
- Messages must make the series reviewable:
  - stable ordering in dependency order
  - later commits rely on earlier commits in obvious ways
  - reverts and fixups are clearly targeted

## Review output

- If commit structure is the problem, report it as a finding with the commit
  hash and a concrete split recommendation. Do not bury it under code nits.
