# Code Review Core Principles

These rules apply across languages. Keep them here to avoid drift.

## Review workflow

- Run the review after a successful `fixup-commit` or `git conventional-commit`.
- Default to review blockers for guideline violations unless the user says
  otherwise.
- Scan the repo for guidance (README, AGENTS, docs, and other files). Record
  rules that are not in the core principles.
- For setup docs, prefer CLI/API steps and capture the commands. Use the web UI
  only when no CLI/API exists.
- Keep setup docs, commit sequences, and guides linear so each step builds on
  the last. Reduce context switching unless it is necessary.
- Prefer polling as the primary mechanism for state changes. Use event
  notifications only as a latency-reduction layer, not as the single source of
  truth.
- Treat every interaction as part of an ongoing review. Capture new guidance
  and preferences promptly so the system improves with every exchange.
- When the user states code review preferences, record each one in the
  owning reference module of this skill (the language file, `testing.md`,
  `github-pr-comments.md`, `cli.md`, or this file) — never in a catch-all
  log.
- Prefer to provide command steps for the user to run. Execute them only when
  the user asks, and keep monitoring for completion.
- Prefer a sub-agent for review if the harness supports it. Share key context
  only.
- Record automation candidates. If a rule can be automated, remove it from
  human review and add it to tooling.
- If guidelines conflict, ask the user which condition applies before adding
  to global guidance. Do not drop rules.
- Prefer stronger constraints first. Relax or remove them later if real usage
  proves they are too tight. Adding constraints after data drifts is harder.
- Capture new guidance immediately when the user provides it. When a change
  touches project guidelines, prefer to do it during a review and keep it in a
  separate atomic commit. If the repo is not under the `dkubb` GitHub org, ask
  before updating project guidelines.
- Prefer the project’s `just` workflow when it exists. Note missing wrappers as
  review feedback.
- Stop here unless the user asks for further follow-up.

## Automation first

- Always run the repo’s lint, format, test, docs, and coverage commands.
- If a rule can be checked by a tool, remove it from human review and require
  the tool instead.
- When a new issue is found, ask: “Can this be automated?” If yes, log it as a
  tooling task.
- Prefer fixing existing violations before adding new lints.
- Prefer pushing conventions down into executable scripts (with shebangs)
  over embedding them in task-runner wrappers or docs.

## Primitive obsession (review blockers)

- Go directly to the bounded form: `BoundedString` / `BoundedVec` with both
  bounds over `String` / `Vec<T>` when constraints exist. `NonEmpty*`
  proves only the lower bound and is a placeholder, not a destination —
  flag it unless the missing upper bound is recorded with a reason.
- Use `NonZero*` over numeric types when zero is invalid or impossible.
- Common fixes:
  - Replace primitives with domain types and smart constructors.
  - Use enums for restricted strings.
  - Use `Option` over empty values for absence.
- Semantics: `state-space-minimization` `references/primitive-obsession.md`
  and `references/principles.md` § "Bound ranges and cardinality".

## Workflow and tooling

- Use `rumdl` for Markdown checks when the repo documents it.
- Use `just new-commit` and `just fixup-commit` when the repo has a justfile.
- Otherwise, use `git conventional-commit` instead.
- Apply review feedback, then commit again with the same workflow.
- Run formatters before linting. If lint fails, run the formatter again and
  re-run the linter.
- Re-open files after formatting to avoid stale edits.
- Goal: always trust the autoformatter. If it breaks output, fix the formatter
  so the next run is safe.

## Coverage ratchet

- Default to 100% coverage or the highest achievable number.
- Record the exact coverage number used.
- Do not allow coverage to drop unless the user approves.
- If coverage increases, lock in the higher number.
- Floor semantics: `state-space-minimization` `references/ratchet.md`
  (the floor only tightens; weakening only on explicit user request).

## Review quality

- Keep review findings ordered by severity.
- Keep review feedback high signal. Do not spend review budget on praise unless
  the user explicitly asks for that style.
- Avoid synthetic variables or synthetic code in review suggestions; prefer
  direct, minimal expressions when no added clarity exists.
- Fix small issues promptly ("no broken windows"); do not leave known
  problems behind.
- Keep core logic pure; isolate side effects.
- Prefer explicit error types and clear failure messages.
- Do not add placeholder `let` bindings. Use underscore parameters or handle errors.
- Review all regex use (Rust strings, SQL, templates) with the same rigor.
- Construct structured languages (YAML, JSON, SQL, etc.) via serializers or
  builders. Avoid string concatenation or manual assembly except for query
  builder implementations that are explicitly designed for it.
- Avoid explicitly specifying default values in code or config. Only set
  non-defaults so each explicit value signals intentional behavior. If a
  non-obvious override is required, add a brief comment to explain why.
- Only specify code when it can change behavior.
- Keep intentional changes obvious: avoid extra directives that do not change
  behavior so reviewers can focus on the meaningful differences.
- When embedding scripts in structured config, prefer literal block scalars or
  file references over splitting scripts into command arrays. Keep script
  formatting intact and avoid inline blobs when a file can be referenced.
- When a template engine (for example CloudFormation `Fn::Sub`) processes
  scripts, ensure every `${...}` token is a valid template variable or escaped.
  Prefer `$VAR` plus explicit checks in shell to avoid accidental substitution.
- Prefer `tempfile` for temporary files in tests. Avoid custom temp helpers
  unless they add behavior beyond tempfile.
- Avoid hard-coded `/tmp` paths. Use platform temp dirs (e.g., `tempfile`,
  `mktemp -d`, or `std::env::temp_dir`) and allow overrides when needed.
- Validate at construction time. Do not allow types to be created without a
  constructor or smart constructor that enforces invariants.
- Deserialization must not bypass constructors. Keep parsing at the boundary
  and return validated domain types.

## Complexity thresholds (strict defaults)

Use tool output when available. Otherwise, review against the intent.

- Application code (slightly looser):
  - Cyclomatic complexity per function: ≤ 4.
  - Cognitive complexity per function: ≤ 5.
  - NPath complexity per function: ≤ 10.
  - Halstead metrics per function:
  - Volume ≤ 300.
  - Difficulty ≤ 12.
  - Effort ≤ 3600.
  - Maintainability Index per file: ≥ 85.
  - Essential complexity per function: 1.
  - Decision density per function (cyclomatic / LOC): ≤ 0.20.
  - ABC metric per function:
    - A, B, C counts ≤ 4 each.
    - ABC score (sqrt(A^2+B^2+C^2)) ≤ 8.
  - Fan-out per function/module: ≤ 7.
  - Fan-in per function/module: flag if 0; review if extremely high.

## State space and types (review lens)

The semantics are owned by `state-space-minimization`:
`references/principles.md` (six operations, bounds and provenance,
allowlists, weaken-before-strengthen),
`references/constructive-vs-predicative.md` (smart constructors and
trusted boundaries), and `references/ingress-and-boundaries.md`
(boundary parsing and revalidation). This section is what to flag
during review.

- Defense in depth: treat all external and persisted data as adversarial.
  Revalidate at every trust boundary (ingress, storage, and reuse) before
  creating domain types or mutating state.
- Push constraints down to the lowest layer (primitive types, domains, DB
  constraints) and avoid redundant checks at higher layers when a more
  primitive guarantee exists.
- The database is the safety net, not the first line of defense. Application
  and domain layers should reject bad data before it reaches the database.
  If a non-uniqueness database constraint fails in normal operation, treat it
  as a bug in upstream validation.
- Aim for equivalent or stronger validation in the application layer than in
  the database layer — uniqueness is the main exception because concurrent
  enforcement is naturally owned by the database — and keep the two aligned
  so integrity errors indicate bugs.
- Treat the database schema as a snapshot of the current mental model of the
  data. Use real data and constraint violations as feedback to refine that
  model over time.
- When a max bound is needed and no spec gives the limit, choose a plausible
  maximum rounded up to the nearest power of two, and record the bound's
  provenance (spec-derived vs estimated) and unit. Prefer a bounded error
  over silently accepting runaway values.
- Assume string types should be tightened toward the bounded form (min, max,
  and grammar). Treat `NonEmpty*` without a recorded upper-bound reason as
  an unfinished narrowing. Use `Option` when empty and missing are
  distinct, meaningful states.
- When a string has no specific format, allow printable characters only;
  default the minimum length to 0 unless a non-empty value is required to
  be meaningful.
- Prefer a parsed, specialized type over a raw string when one exists
  (URL, UUID, path, timestamp, semantic version).
- For plain data containers where invariants are enforced at construction,
  prefer `pub` fields over trivial accessors unless encapsulation or behavior
  is required.
- If constraints tighten later, migrate or repair old data; do not keep
  invalid states around.

## Sub-agent review

- Prefer a sub-agent for review to avoid context poisoning.
- Share only the necessary context and let the reviewer decide.
