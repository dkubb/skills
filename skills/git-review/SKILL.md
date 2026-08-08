---
name: git-review
description: >-
  Review commit ranges, branches, and fixup chains against the canonical
  atomic-changes contract. Check commit boundaries, semantic verbs, ordering,
  messages, per-commit gates, dates, and tree identity before merge or history
  normalization.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-08-v1"
triggers:
  - "git review"
  - "review commits"
  - "review commit history"
  - "review the branch history"
  - "review this commit range"
  - "review the fixup chain"
  - "audit commits"
  - "audit git history"
  - "are these commits atomic"
  - "normalize commit history"
---

# Git Review

Review commits as individually valid transformations and as one ordered
history. Load `atomic-changes` for every review. Its
`references/commits.md` is the sole authority for atomic boundaries,
semantic verbs, ordering, messages, gates, and fixup ownership.

Git supplies deterministic evidence. It cannot decide whether a diff is one
transformation, whether its verb matches its semantics, or whether its gate
set is sufficient. The reviewer owns those judgments.

## When to Activate

- The user asks to review a commit, range, branch, or fixup chain.
- A branch may need splitting, reordering, rewording, or autosquashing.
- The user wants every retained commit independently verified.
- A history rewrite needs before-and-after tree evidence.

## When Not to Use

- The task reviews code without reviewing commit history.
- The user only wants to create atomic changes; use `atomic-changes`.
- The user only wants a pull-request title or body.

## Inputs

- Repository path.
- Base and head refs, or `--root` for all reachable commits.
- The affected surfaces and their relevant gate commands.
- Exact-state gate evidence already available for individual commits.
- Any authorized repair or history-rewrite scope.

## Outputs

Emit results in this order:

1. Findings ordered by severity, with commit hashes and file references.
2. Open questions or missing evidence.
3. A per-commit table of atom, verb, dependencies, gates, and result.
4. A range-level summary of ordering, fixup ownership, dates, and tree state.
5. A repair plan only when repair is requested or findings require one.

## Utilities

- Use `git log`, `git show`, `git diff`, and `git rev-list` to collect
  deterministic evidence.
- Prefer repository wrappers when they produce the same evidence with fewer
  project-specific arguments.
- A missing wrapper does not block review when native Git can prove the fact.
- `references/review.md` defines the semantic review and repair procedure.

## Process

1. Load `atomic-changes` and its `references/commits.md` completely.
2. Identify the full target and pin its base, head, commits, and tree hashes.
3. Reuse exact-state gate evidence; run only missing read-only checks.
4. Review every commit against the semantic procedure in
   `references/review.md`.
5. Review the range as an ordered composition, not only as an aggregate diff.
6. Report findings before summaries. Do not rewrite history during review.
7. If repair is requested, create auditable fixups against the owning commits
   and delegate splitting, rebasing, or per-commit gate execution to the
   available specialist tools.
8. After an authorized rewrite, compare tree hashes and rerun every affected
   message, date, and project gate.

## Validation Checklist

- `atomic-changes` was the sole normative commit contract.
- The exact base, head, and complete target range were recorded.
- The aggregate range and every individual commit were reviewed.
- Each retained commit contains one indivisible transformation.
- Each subject verb matches its diff and every message has canonical form.
- Dependencies precede dependents and remaining ties follow canonical order.
- Every commit has sufficient exact-state evidence for its relevant gates.
- Fixups are assigned to the commits that own their corrections.
- Review remained read-only unless the user explicitly authorized repair.
- Any rewrite preserved the intended trees and left the worktree clean.
