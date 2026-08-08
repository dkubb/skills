---
name: code-review
description: >-
  Review diffs, commit ranges, fixup chains, or PRs across languages and
  report findings ordered by severity. Enforces primitive-obsession and
  smart-constructor blockers, property-test and boundary-test requirements,
  complexity thresholds and per-language rules (Rust, TypeScript, Lean, SQL,
  bash, Ruby, Markdown, regex). Delegates commit-history structure and repair
  to git-review. Also the home for changing review rules or codifying review
  feedback.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-08-v13"
triggers:
  - "code review"
  - "review this diff"
  - "review the diff"
  - "review my changes"
  - "review the changes"
  - "review the PR"
  - "review this PR"
  - "review commits"
  - "review the branch"
  - "review findings"
  - "review rules"
  - "codify review feedback"
  - "pedantic review"
  - "review blockers"
---

# Code Review

## When to Activate

- The user wants a code review.
- A change set spans multiple languages or file formats.
- The user wants to change review rules or codify review feedback.
- The user wants to change review steps or report type.
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
- A request for code review authorizes ordinary local, non-mutating
  verification commands within the review scope. Use check-only modes for
  formatters and linters.
- Do not run commands that rewrite files, update dependencies, commit, push,
  publish review feedback, change remote state, or otherwise apply fixes
  unless the user requested that mutation.
- Treat a nominally diagnostic command as mutating when the command or its
  project wrapper has meaningful side effects. Provide the command instead of
  executing it when its effects cannot be established safely.

### Verification Gates

- Use the repository's standard review or CI command when it exists. Otherwise,
  select individual gates from the change set and its transitive effects:
  applicable formatter checks and linters; tests and coverage when executable
  behavior can change; and documentation checks when documentation, public
  APIs, or generated documentation can change.
- A gate adopted by repository rules, configuration, CI, or an adopted profile
  **MUST** remain executable. If it cannot be verified, block approval and
  report the problem against the tooling surface rather than an arbitrary
  changed source line.
- A missing wrapper does not block approval when equivalent native commands
  can produce reliable evidence for the required gate. Use those commands and
  record wrapper consolidation as Advice. The wrapper itself is a finding only
  when it is an explicit acceptance criterion; the gate remains the blocking
  concern. Reviewers **MUST NOT** invent missing infrastructure during a
  read-only review.
- **Advice:** When an invocation repeatedly needs project-specific flags,
  environment, ordering, or multiple tools, capture it in one deterministic
  repository wrapper. Keep the wrapper efficient by selecting only applicable
  work and reusing exact-state evidence according to this section.
- **Advice:** Periodically mine the skill's repeated instructions and tool
  sequences for wrapper candidates. Prefer wrappers that reduce tool calls,
  token usage, and agent-managed steps while preserving the full check,
  actionable failure output, and direct native-tool access for diagnosis.
- Before running an applicable gate, determine whether reliable evidence
  already shows that it passed for the exact state under review. Reuse evidence
  from the current session or CI only when it covers the same revision or
  working-tree content, relevant configuration, dependencies, and toolchain.
- Run a gate only when its result is unknown. Rerun it when the reviewed inputs
  changed, the earlier result was incomplete or failed, or its applicability
  cannot be established confidently.
- Report each applicable gate as executed or satisfied by prior evidence, with
  the command or evidence source. Record each inapplicable or unexecutable gate
  and the reason it was skipped.

## Process

### Scope

1. Identify the review target: a working-tree diff, a commit range, a fixup
   commit chain, or a pull request. Any of these is a valid entry point; a
   successful commit is not a precondition.
2. For a working-tree target, review staged changes, unstaged changes, and
   relevant untracked files against `HEAD`. Read surrounding baseline code as
   needed, but do not report an unrelated `HEAD` commit or baseline defect as a
   finding about the working-tree change.
3. When a baseline defect interacts with the target or makes the resulting code
   incorrect, report that interaction explicitly. Do not discard a
   change-relevant problem merely because part of its cause predates the target.
4. For a commit, commit range, branch, fixup chain, or pull request, review the
   aggregate target and each commit in that target as an artifact. Review the
   most recent commit in isolation only when it belongs to the target or the
   user explicitly requests it.
5. When the project uses a skeleton-commit and fixup workflow, review from the
   skeleton commit through `HEAD`; do not limit the target to the last fixup.
   After autosquash, preserve the user-requested branch, range, or pull-request
   boundaries instead of collapsing the target to `HEAD`.
6. Identify languages and file types in the change set.

### Review Flow

1. Load `references/core-principles.md` before interpreting any other review
   rule. Apply its BCP 14 requirements language to distinguish blockers,
   justified exceptions, optional choices, and non-normative advice. Determine
   that a rule applies before assigning severity.
2. Call out primitive obsession as a review blocker. Reference
   `references/core-principles.md` and its "Primitive obsession" section for
   common fixes.
3. When the change introduces or modifies types, boundaries, or domain models,
   load the `state-space-minimization` skill's
   `references/constructive-vs-predicative.md` and
   `references/ingress-and-boundaries.md` for type design, domain shrinking,
   and invariant encoding.
   Prefer smart constructors when domain invariants exist, so invalid states
   are rejected at creation time. Require both boundary-focused unit tests and
   property tests for every smart constructor, serializer, emitter, output
   generator, deserializer, and parser. Require round-trip tests for each
   paired producer and consumer.
4. When the target contains commits, load `git-review` and use it for commit
   boundaries, messages, ordering, exact-state gates, tree identity, and
   history repair. Keep code correctness and language-specific review here.
5. When review feedback will be left on a GitHub pull request, load
   `references/github-pr-comments.md` and follow its label, structure, and
   inline-range rules.
6. Load `references/cli.md` when CLI behavior or command output is in scope.
7. When the change set touches an I/O boundary (external API, database,
   file parsing, user input), load an anti-corruption-layer skill if one
   is available (such as `external-integration`) for trust model, mirror
   layer, and translation testing guidance. All I/O boundaries are
   untrusted.
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

Apply the core principles first. Language references own portable idioms,
tool-specific mechanisms, language- and framework-specific test organization,
and conditional profiles. This entire skill is the user's personal review
policy: including a rule makes its enforcement a personal choice regardless of
whether the rule is based on language semantics, community practice, project
experience, or a deliberate departure. Labels explain that basis and a rule's
scope; they do not divide the rubric into personal and objective rules or
suppress a check. When a rubric rule conflicts with an established project
convention, report the conflict and ask the user to choose its scope. Do not
silently apply the rule or migrate surrounding code. A rule that names
repository-specific commands, files, domains, or architecture applies only
when the repository provides that profile or explicitly adopts it. A language
reference can require adoption of community-standard tooling, but it cannot
assume that a named repository wrapper already exists.

- Bash files: `references/languages/bash.md`.
- Lean files: `references/languages/lean.md`.
- Rust files: `references/languages/rust.md`.
- TypeScript and TSX files: `references/languages/typescript.md`.
- SQL files: detect the dialect first. For PostgreSQL, use
  `references/languages/sql.md`; for other dialects, apply the core principles
  and repository rules without importing PostgreSQL-specific guidance.
- Ruby files: `references/languages/ruby.md`.
- Markdown files: `references/languages/markdown.md`.
- Use `regex` in each file: `references/languages/regex.md`.
- Internal PostgreSQL in other languages: use the two language references and
  `references/languages/sql.md`. For another SQL dialect, do not load the
  PostgreSQL reference.
- Internal `regex` in other languages: use the 2 language references and
  `references/languages/regex.md`.

### Rule Steps

1. Record contradictions in rules and ask the user which condition applies
   before you write the rule into the global rules. Do not discard rules.
   When a rubric rule conflicts with project convention, present the explicit
   choices: retain the project convention as a documented exception, apply the
   rubric rule in a defined local scope, or migrate every affected site.
   Reviewers **MUST NOT** perform any migration without the user's choice.
2. Record automation methods across languages. If a tool can do a rule,
   move it out of manual review.
3. Capture new guidance immediately when the user provides it. Prefer to
   update project guidelines during an actual review and keep them in a
   separate atomic commit. If the repo is not under the `dkubb` GitHub org,
   ask before you change project rules.
4. Use the project's standard task-runner wrappers when they exist. If a
   wrapper is missing, run equivalent native commands when they can establish
   the required result. Do not block on wrapper availability alone; record
   consolidation as Advice when it would improve determinism, token usage, or
   the number of steps the reviewer must manage.
5. Stop here unless the user asks for further follow-up.

## Check List

- The review target (working-tree diff, commit range, fixup chain, or PR)
  was identified and the full target reviewed without silently adding an
  unrelated `HEAD` commit or baseline audit.
- Baseline code was read for context, and only defects that interact with the
  target were included as findings.
- `references/core-principles.md` was loaded first, and its BCP 14 requirements
  language was applied to every relevant language and supporting reference.
- Deduplicate findings and order them by severity.
- Record automation opportunities.
- Any rule change is in its own early, atomic commit when applicable.
- Stop here unless the user asks for further follow-up.
