# Review Preferences

- Review for correctness, safety, API, tests, docs, and style with Rust +
  Markdown focus.
- When requested, focus on correctness, CLI behavior, regressions, missing
  updates to docs/tests, and API/UX issues.
- When requested, focus on logic, edge cases, and docs for Rust + Markdown
  diffs.
- When requested, focus the review on correctness, safety, tests, and style for
  Rust/Axum changes.
- Pay close attention to serde deny_unknown_fields, smart constructors,
  validation, and config parsing.
- Prefer reviews focused on Rust/Cargo; apply core/testing principles (NonZero
  types, AAA tests, avoid placeholder bindings).
- When requested, focus on correctness, safety, tests, Rust guidelines,
  primitive obsession, validation, error handling, headers/content-type,
  streaming, and status codes.
- When requested, focus on Rust diffs and enforce deny_unknown_fields, NonEmpty
  types, avoidance of primitive obsession, and smart constructors.

- When requested, focus on correctness, API design, primitive obsession, tests,
  and docs.
- When requested, focus on correctness, determinism, primitive obsession, and
  doc clarity.
- When requested, review diffs for potential bugs only.
- When requested, focus on correctness and tests, and return findings/questions
  only if issues.
- When requested, focus on correctness, tests, and API, and report issues only.
- When requested, return findings/questions only if issues.
- When requested, focus on correctness, tooling, and missing steps for justfile
  bash + Markdown diffs.
- When requested, include severity and file refs with line numbers in findings.
- When requested, focus review on specified functions or line ranges and use the
  provided constants/context when assessing correctness.
- When requested, limit review scope to the specified src files and avoid
  assumptions about unreviewed modules unless needed.
- When requested, provide findings with severity and file refs.
- When requested, provide findings with severity and file+line refs.
- When requested, focus on doc clarity and consistency.
- When requested, review justfile (bash) and Markdown diffs against core
  principles and language guidance; order findings by severity with file refs.
- When requested, focus on correctness, tooling, and missing steps for nightly
  coverage workflows (prereqs, CI, docs), ensure ensure-llvm-cov only installs
  cargo-llvm-cov, and require strict bash.
- When requested, focus on doc clarity, missing steps, nightly prereqs, CI
  guidance, and command usage; return findings ordered by severity with file
  refs.
- When requested, provide concrete suggestions in concise bullets and include
  line numbers when possible.
- When requested, provide concrete suggestions with severity and line
  references.
- When requested, review doc steps for clarity, missing prereqs
  (nightly/llvm-cov), CI guidance, command usage, and piping; return concrete
  suggestions with severity and line refs, plus open questions and a short
  summary.
- When requested, focus on doc clarity, missing steps, nightly prereqs, CI
  guidance, and command usage; return findings ordered by severity with file
  refs.
- When requested, provide concrete suggestions, list findings with severity and
  file refs (line numbers when possible), and use concise bullets.
- When requested, focus on off-by-one, NonZeroU64 changes, and exit behavior for
  iteration/duration/exit edge cases.
- When requested, focus on correctness and edge cases for loop
  termination/iteration caps, and include severity, rationale, and file+line
  refs.
- When requested, focus on correctness, edge cases, and doc clarity for Rust +
  Markdown diffs.
- When requested, return findings with severity and file/line refs.
- When requested, focus on clarity, accuracy, and edge cases for Markdown diffs;
  use markdown guidance and include severity.
- When requested, use code-review core principles and Markdown guidance.
- When requested, review Markdown diffs for clarity, accuracy, and edge cases;
  provide findings with severity.
- Prefer long-form CLI options in docs/examples; avoid single-letter flags
  (reserve them for repeated interactive use).
- When requested, use minimal context and read files directly instead of relying
  on extra references.
- When requested, treat tests as strict contracts; avoid fallback accessors (for
  example, `unwrap_or_default`, `unwrap_or`, `unwrap_or_else`, `map_or`,
  `map_or_else`) that can hide missing values, and prefer explicit
  `expect`/assertions.
- When requested, for one-line CHECK constraints, do not add spaces after `(` or
  before `)`.
- When requested, name CTE legs using concise nouns (e.g., `new_resource`,
  `new_provider`, `resource`, `provider`).
- When requested, review changes vs main one file at a time.
- When requested, focus reviews on correctness, strictness, and tests.
- When requested, output findings with file refs, open questions, a brief
  summary, and test suggestions.
- When requested, focus review on SQL guidelines and whether constraints match
  the user's preferred wording/structure rather than formatting.
- When requested, focus on Rust style, lint attributes (`#[allow]` vs
  `#[expect]`), AAA test structure, NonEmptyString usage, and repo consistency.
- When requested, review changes as an independent reviewer who did not author
  the changes.
- When requesting Copilot reviews, hide the previous Copilot "Pull request
  overview" since new reviews supersede it.
- When triggering a new Copilot review, hide all previous Copilot reviews; use
  API dismissal with reason/message "Outdated" when available.
- When Copilot reports "generated no new comments" after a review, stop
  requesting further Copilot reviews for that PR.
- Prefer URL types for URL fields; default to non-empty strings when empty is
  ambiguous, and allow empty only for free-form user input where absence is
  meaningful.
- Prefer strict spec-aligned validation: accept all valid inputs, reject all
  invalid inputs; allow empty only when the spec allows it; when unsure, be
  stricter and relax later; make impossible states unrepresentable with domain
  types and smart constructors.
- Keep constraints strong at every layer (production, tests, utilities, internal
  transforms) and validate on each transition.
- Start strict, then loosen only when real data proves it necessary.
- Audit types for primitive obsession: avoid raw `String` fields without smart
  constructors; every string must have an explicit maximum length (and
  min/pattern when applicable), and URLs should use URL types instead of
  strings.
- Avoid constructing `NonEmpty` values directly; require smart constructors so
  validation happens at the boundary.
- When string constraints are unknown, allow printable characters only and
  enforce explicit min/max lengths (default min length to 0 unless non-empty is
  required).
- When validating identifiers or structured strings, prefer official specs or
  vendor-maintained libraries and be stricter when uncertain.
- Construct structured languages (YAML/JSON/SQL/etc.) via serializers or
  builders; avoid string concatenation except for explicit query builders.
- Avoid explicitly specifying default values in code or config; set only
  non-defaults and comment on non-obvious overrides.
- Keep intentional changes obvious; avoid extra directives that do not change
  behavior.
- Prefer linear, step-by-step docs and commit sequences that minimize context
  switching.
- Prefer polling as the primary mechanism for reacting to state changes; use
  event notifications only to reduce latency.
- Keep work close to the last known good state: change, iterate until it works,
  capture success in a commit, then move on.
- Test each meaningful change (ideally each atomic commit) and keep CI running
  in the background; fix or roll back after failures.
- Follow the "No Broken Windows" principle: fix small issues promptly and do not
  leave known problems behind.
- Let "The Pragmatic Programmer" influence code and process decisions.
- Prefer single quotes for bash strings that do not use variable interpolation.
- For SQL formatting reviews, assume input is adversarial and normalize case:
  uppercase primary keywords, keep `true`/`false` lowercase, lowercase function
  names and unquoted identifiers, and preserve quoted identifiers.
- For SQL formatting reviews, do not touch psql meta-commands or `:` variables;
  format only the SQL portion of `\\copy` and keep COPY options as `WITH (FORMAT
  CSV, HEADER)`.
- For SQL formatting reviews, keep semicolons on the same line for single-line
  statements and only move `;` to its own line for multi-line statements.
- For SQL formatting reviews, treat keyword-looking identifiers contextually
  (e.g., `type`, `schema`, `ordinality` as columns) while keeping
  context-required keywords uppercased (e.g., `BEGIN ATOMIC`, `ON CONFLICT`,
  `NULLS FIRST/LAST`, `WITH ORDINALITY`, `WITHIN GROUP`, `AT TIME ZONE`,
  `INTERVAL`, `EPOCH` inside `extract`).
- When requested, focus SQL formatting review on keyword/identifier casing and
  leave psql commands untouched except for the SQL portions (e.g., `\\copy`).
- When requested, perform SQL-formatting diff audit only.
- When requested, focus SQL formatting review on leading-comma alignment
  consistency and UNION ALL block spacing.
- For SQL formatting reviews, format multi-line CTEs as: `WITH` line, CTE name
  on its own line, `AS` aligned with `WITH`, opening `(` aligned with the CTE
  name indentation, and fully formatted inner query.
- For SQL formatting reviews, align multi-line `CREATE TABLE` columns with
  separate columns for name, type, `NOT NULL`, `DEFAULT`, and other column specs
  (`UNIQUE`/`PRIMARY KEY`/`CHECK`/etc.), in that order (NOT NULL before DEFAULT
  before UNIQUE/PRIMARY KEY).
- For SQL formatting reviews, when contiguous lines represent sibling AST nodes
  at the same level (e.g., list items), align their indentation consistently
  rather than drifting per line.
- Prefer `use module::*` when importing multiple items from a module; prefer
  explicit imports when only one item is used.
- Avoid synthetic variables or synthetic code in reviews; prefer direct, minimal
  expressions when no added clarity exists.
- When resolving review comments, add 👍 to helpful feedback and 👎 to feedback
  you will ignore, to reinforce reviewer preferences.
- Always run `git up` before `git push-each`.
- Avoid `#[cfg(test)]` outside test modules; when sharing helpers across
  modules, place them in a `test_support` module instead.
- When requested, perform full pedantic code review for branch vs main and use
  RepoPrompt MCP to gather diff context.
- Prefer refactoring in small, safe steps; keep code passing after each step.
- Prefer refactoring code and tests in separate steps (not both in one step).
- Consider refactoring via low-level “strangler” helpers or higher-level
  wrappers with tests that fully exercise the underlying code.
- Tighten invariants early (smart constructors, assertions, explicit error
  paths) so invalid state is detected sooner.
- Prefer fixing existing violations before adding new lints.
- Prefer `git conventional-commit --action` to explain the why instead of
  duplicating the subject.

- Prioritize improving skills as the highest leverage activity (“sharpening the
  saw”), and then apply those improvements to the task at hand.
- Prefer pushing down conventions into executable scripts (with shebangs) to
  avoid `rust-script` wrappers in justfiles/docs when possible.
- When requested, after completing the review, run the full gate set (fmt-check,
  clippy, coverage, docker-build, dylint, dylint-test) on the squashed commit
  and on each extracted commit before proceeding.
- When requested, review shell/script/products changes (scripts/*.sh, products/*.sh,
  Dockerfile) for correctness issues like missing strict mode, wrong paths, or
  logic bugs, and include file+line refs.
- When requested, review the current on-disk snapshot only and ignore commit
  history or commit-by-commit review when the branch will be squashed or split
  later.
