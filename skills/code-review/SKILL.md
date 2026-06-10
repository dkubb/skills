---
name: code-review
description: Run a clear code review across languages and change review rules. Use when the user wants a code review or a rule change.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-01-v11"
---

# Code Review

## When to Activate

- The user wants a code review.
- A change set spans multiple languages or file formats.
- The user wants to change review rules or codify review feedback.
- The user wants to change review steps or report type.
- After a successful fixup commit workflow or `git conventional-commit`.
- The user asks for a review of diffs, commits, or PR changes.

## When Not to Use

- The user only wants a summary or explanation, not a review.
- The task is not a review and does not involve code changes.

## Inputs

- The diff or files to review.
- The list of changed file types.
- Repo step list and tool rules.

## Outputs (Fixed Order)

1. A combined list of findings ordered by severity, with file references.
2. Open questions or estimates.
3. A short change summary only after findings.
4. More steps for tests or tools.
5. If there are no findings and no questions, do not emit a review report.
   Continue the task without more notes unless blocked. Only emit a review
   report when there is a special condition such as tool failure,
   rule change necessary, or when the user wants to see a report.

## Tools

- Use repo-local wrappers for formatting and checks when they exist.
- Use one combined command to decrease shell calls.

## Process

Use one combined command to decrease shell calls.

### Scope

1. Confirm the step list used:
   - A skeleton commit + fixup commit chain, or
   - `git conventional-commit`.
2. Trigger this review only after a successful commit from that step list.
3. If the project uses a skeleton commit + fixup chain, review the diff from
   the skeleton commit through `HEAD`. Do not limit to the last fixup commit
   unless the user or LLM states a different preference.
4. Also review the most recent commit in isolation to spot local regressions.
5. If there is no fixup chain, for example after autosquash, set the
   review range to `HEAD`.
6. Identify languages and file types in the change set.

### Review Flow

1. Load `references/core-principles.md`.
2. Call out primitive obsession as a review blocker. Reference
   `references/core-principles.md` and its "Primitive obsession" section for
   common fixes.
3. When the change introduces or modifies types, boundaries, or domain models,
   load the `state-space-minimization` skill's reference for type design,
   domain shrinking, and invariant encoding guidance.
   Prefer smart constructors when domain invariants exist, so invalid states
   are rejected at creation time. Request both boundary tests and property
   tests for each smart constructor.
4. Load `references/commit-review.md` and review commits as artifacts:
   commit message quality, granularity, ordering, and reviewability.
5. When review feedback will be left on a GitHub pull request, load
   `references/github-pr-comments.md` and follow its label, structure, and
   inline-range rules.
6. Load `references/cli.md` when CLI behavior or command output is in scope.
7. When the change set touches an I/O boundary (external API, database,
   file parsing, user input), load the `external-integration` skill's
   `references/external-integration.md` for trust model, mirror layer,
   and translation testing guidance. All I/O boundaries are untrusted.
8. Scan the repo for rules (README, AGENTS, docs, or other files) and record
   rules not in `references/core-principles.md`.
9. Load the applicable language references from `references/languages/` for
   the file types and internal parts in the change set.
10. Load `references/testing.md` when tests are in scope. Load
   `references/property-based-testing.md` when property tests, generators,
   or smart constructors are in scope, and enforce its strategy
   (valid/invalid generators, boundary-biased sampling, and
   round-trip/inverse properties when serializers/parsers are involved).
11. Load `references/coverage.md` when coverage is in scope.
12. For user-visible behavior changes, require integration test backfill for
    the changed contract (inputs, outputs, side effects).
13. Load `references/determinism.md` when behavior must be the same each run.
14. Use the language list below to select language references.
15. If Markdown is present, run the markdown review steps in the Markdown
    language reference.
16. Use sub-agents for review to avoid context damage. Give only relevant
    context and let them decide.
17. Put together findings, remove same items, and keep order by severity.
18. If you find problems, report them in point form. If the user wants, give
    before and after diffs and the cause for the change.
19. If there are no findings and no questions, suppress the review output and
    continue the task unless a special condition applies.
20. After the review, use follow-up feedback as material for the global
    review files and apply those changes when the user gives them.
21. When feedback refers to a specific case, check if it applies
    to a group of issues. Write the rule and use the same method.

### Language List

- Bash files: `references/languages/bash.md`.
- Rust files: `references/languages/rust.md`.
- SQL files: `references/languages/sql.md`.
- Ruby files: `references/languages/ruby.md`.
- Markdown files: `references/languages/markdown.md`.
- Use `regex` in each file: `references/languages/regex.md`.
- Internal SQL in other languages: use the 2 language references and
  `references/languages/sql.md`.
- Internal `regex` in other languages: use the 2 language references and
  `references/languages/regex.md`.

### Rule Steps

1. Record contradictions in rules and ask the user which condition applies
   before you write the rule into the global rules. Do not discard rules.
2. Record automation methods across languages. If a tool can do a rule,
   move it out of manual review.
3. Capture new guidance immediately when the user provides it. Prefer to
   update project guidelines during an actual review and keep them in a
   separate atomic commit. If the repo is not under the `dkubb` GitHub org,
   ask before you change project rules.
4. Use the project's standard task runner wrappers when they exist. Note
   missing wrappers as review feedback.
5. Stop here unless the user asks for further follow-up.

## Check List

- The review happened after a successful fixup commit workflow or
  `git conventional-commit`.
- Load `references/core-principles.md` and the relevant language references.
- Deduplicate findings and order them by severity.
- Record automation opportunities.
- Any rule change is in its own early, atomic commit when applicable.
- Stop here unless the user asks for further follow-up.
