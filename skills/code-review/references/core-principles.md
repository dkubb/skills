# Code Review Core Principles

These rules apply across languages. Keep them here to avoid drift.

## Cross-language review rules

The review workflow itself (scope, flow, rule capture, sub-agents) lives in
`SKILL.md`; these are the rules applied across languages.

## Requirements language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**,
**SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and
**OPTIONAL** in this rubric are to be interpreted as described in
[BCP 14](https://www.rfc-editor.org/info/bcp14),
[RFC 2119](https://www.rfc-editor.org/info/rfc2119), and
[RFC 8174](https://www.rfc-editor.org/info/rfc8174) when, and only when, they
appear in all capitals.

- An unqualified imperative rule in this rubric is a **MUST**. Prefer explicit
  BCP 14 keywords when adding or revising rules so the intended requirement
  level is mechanically visible.
- A **MUST** or **MUST NOT** violation is a review blocker.
- A **SHOULD** or **SHOULD NOT** deviation remains a blocker until it is either
  corrected or supported by a case-specific reason showing that the full
  implications were understood and carefully weighed. A verified, documented
  exception is compliant and is not a finding.
- A **MAY** or **OPTIONAL** choice is compliant either way. The choice alone is
  not review feedback.
- Non-normative guidance MUST be explicitly labeled `Advice`. Advice can be
  offered when useful but is not a guideline violation.
- Determine applicability before severity. A more-specific rule can narrow a
  general rule's scope, but it cannot weaken the requirement level without an
  explicit BCP 14 keyword or documented exception.

## Cross-language requirements

- Treat an ordinary code-review request as authorization to run local,
  non-mutating verification within the review scope. Follow `SKILL.md`'s
  authority rules for check-only modes, state-changing commands, and skipped
  gates.
- When the user states code review preferences, record each one in the
  owning reference module of this skill (the language file, `testing.md`,
  `github-pr-comments.md`, `cli.md`, or this file) — never in a catch-all
  log.
- For setup docs, prefer CLI/API steps and capture the commands. Use the web UI
  only when no CLI/API exists.
- Keep setup docs, commit sequences, and guides linear so each step builds on
  the last. Reduce context switching unless it is necessary.
- Prefer polling as the primary mechanism for state changes. Use event
  notifications only as a latency-reduction layer, not as the single source of
  truth.

## Community-native code

- Code must be idiomatic for the target language. Follow the language
  community’s established terminology, organization, and tooling so the code
  reads naturally to an expert in that language.
- Treat community conventions as the informed baseline, not unquestionable
  authority. When starting a language or a project, learn and follow those
  conventions deeply enough to understand their purpose and tradeoffs before
  changing them.
- Do not write one language as though it were another. Borrow an organizational
  approach from another language only when it fits the target language and
  improves the design.
- Depart deliberately when a community convention conflicts with a project
  principle or is insufficiently mature for the problem. Each departure must
  have a concrete reason and improve correctness, clarity, maintainability, or
  another stated principle. Do not diverge merely for novelty or personal
  style.
- Evolve project style incrementally. Change one scoped convention at a time in
  response to concrete errors, inconsistencies, or confusion, then observe its
  effects before changing more. Do not wholesale transplant preferences from
  another language.
- Let the resulting style combine the target community’s idioms with deliberate
  project principles while remaining legible to experts in that language.
- Every rule in this skill is part of the user's personal review policy,
  including rules based on language semantics or community practice. A rule's
  stated basis explains its intent; it does not make the rule more or less part
  of the review lens.
- When a rubric rule conflicts with an established project convention,
  reviewers **MUST** report the conflict, its basis, and the affected scope
  instead of silently suppressing either rule.
- Reviewers **MUST NOT** migrate a project convention, apply a conflicting
  rubric rule inconsistently to new code, or widen the change beyond its
  requested scope without the user's explicit decision. Offer the choices to
  retain the project convention as a documented exception, apply the rubric
  rule in a defined local scope, or migrate every affected site.

## Intent and justification

- Every change must be intentional. For any diff, be able to explain why the
  code after the change is better than the code before it.
- Articulate the relevant goals, constraints, alternatives, tradeoffs, and how
  each factor was weighted. A decision is not justified merely because the
  result is different or preferred.
- Explain any non-idiomatic or otherwise surprising choice. If its rationale
  will not remain recoverable from the code, tests, and commit itself, record
  the reason in the nearest durable artifact.
- Use personal preference only to break a genuine tie after correctness,
  safety, clarity, maintainability, community convention, and stated project
  principles have been weighed. Do not use preference as the primary reason
  for a change.

## Constrain by default

- Within the known contract, default every degree of freedom to its narrowest
  form: private, immutable, closed, bounded, least-privileged, and unavailable
  unless required.
- Every widening **MUST** serve a concrete current use case. Widen only the
  smallest dimension needed to satisfy that requirement, and preserve all
  unrelated constraints.
- Speculative flexibility and hypothetical future consumers are not use cases.
  Let a real requirement force an explicit, reviewable widening later.
- A constraint **MUST NOT** exclude behavior the real contract already
  requires. When evidence reveals a valid new case, widen intentionally to the
  smallest form that admits it.

## Incremental ratchets

- Use a ratchet whenever moving an existing codebase along a chosen trajectory
  that a deterministic gate can enforce, such as lint adoption, coverage,
  warning elimination, stricter types, or tighter complexity limits.
- Define the intended destination, direction, measure, and enforcement gate.
  The measure **MUST** represent the desired property closely enough that
  improving the number or rule set cannot conceal a regression in that
  property.
- Evaluate the destination against the current code without committing a
  failing gate. Establish the current passing baseline, then immediately
  commit every strengthening of the gate that already passes.
- Partition the remaining distance into the smallest coherent classes. For
  each class, change the code and commit the improvement while the current
  gate still passes; then tighten and commit the gate in a separate following
  step. Every commit **MUST** pass the enforcement configuration committed at
  that point.
- Repeat the improve-then-enforce cycle until the destination is reached or
  every deliberate exception is recorded. Once tightened, a ratchet **MUST
  NOT** be weakened without an explicit reason explaining why the former
  trajectory or threshold is no longer correct.
- A greenfield project **SHOULD** begin at the intended destination rather than
  recreate an incremental migration with no legacy code to preserve.

## Automation first

- Apply the repository's relevant lint, formatter-check, test, docs-check, and
  coverage gates without mutating the reviewed files or external state. Use
  `SKILL.md` to select applicable gates and to reuse reliable evidence instead
  of rerunning a gate whose result is already known for the exact reviewed
  state.
- A verification gate adopted by the repository or an active profile **MUST**
  remain executable. An unverified required gate blocks approval even when the
  tooling failure predates the reviewed change.
- Reviewers **MUST NOT** invent a missing repository wrapper during a read-only
  review or treat wrapper availability alone as a blocker when equivalent
  native commands can verify the gate.
- **Advice:** Move repeated project-specific tool flags, environment, ordering,
  and multi-command gates into deterministic repository wrappers. Preserve
  efficiency by selecting only applicable work and reusing exact-state gate
  evidence.
- **Advice:** Mine this skill's repeated instructions and command sequences for
  wrapper candidates that preserve the complete check while reducing token
  usage, tool calls, and agent-managed steps. Keep the underlying native
  commands available for diagnosis.
- Capture review improvements in the owning rubric as soon as they are
  conceived or discovered. Do not leave reusable judgment only in review
  comments or memory.
- Bound "every available lint" to every non-removed rule exposed by the
  repository's already-adopted linters and plugins at their pinned versions.
  Adding another tool or plugin is a separate intentional decision. Inventory
  excluded, conflicting, and inapplicable rules with a reason.
- A greenfield project **SHOULD** start with every available lint enabled.
  Resolve immediately evident conflicts and record every exclusion, then keep
  the strictest mutually compatible set passing as code is introduced.
- For an existing project, evaluate every available lint. Treat the full lint
  set as a way to learn the community’s encoded understanding of good code.
  Temporarily enable the full set against real code and observe the changes
  each lint demands before deciding whether it belongs in the project. This
  evaluation configuration may fail; it is not the committed enforcement
  configuration.
- Apply the incremental-ratchet workflow to an existing project's evaluated
  lint set. Treat each retained lint or compatible violation class as a step:
  commit the fixes first, then enable and commit the lint. Continue until every
  agreed, applicable, non-conflicting lint is enabled and passing. Do not
  commit the failing temporary evaluation configuration as the project gate.
- When lints conflict, choose the rule that best expresses the project’s
  convention and disable the conflicting rules. When a lint’s demanded changes
  disagree with project conventions, tune it or disable it with a recorded
  reason. Converge on the strictest mutually compatible lint set that reflects
  both community knowledge and the project’s chosen conventions.
- Treat lint configuration as a ratchet: add useful rules as they become
  available and do not weaken existing settings without a specific reason.
- If a rule can be checked by a tool, remove it from human review and require
  the tool instead.
- Prefer partial automation to a wholly manual check when complete automation
  is not possible. Use deterministic tools such as `ast-grep` to find a
  high-recall set of candidates, then require an LLM or human to adjudicate the
  semantic cases the tool cannot decide. Report unadjudicated candidates as
  candidates, not confirmed violations, until that review occurs.
- When a new issue is found, ask: “Can this be automated?” If yes, log it as a
  tooling task.
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
- Use the repository's documented atomic commit workflow when it exists,
  including `just new-commit` and `just fixup-commit` when those recipes are
  provided. Otherwise, use `git commit` with the canonical `atomic-changes`
  commit form.
- Use Conventional Commit syntax for pull request titles, not git commit
  subjects.
- Apply review feedback, then commit again with the same workflow.
- Use an autoformatter for every language that has one. Prefer the community’s
  standard formatter and its standard output as the baseline. Change formatter
  configuration only when an observed result conflicts with a chosen project
  convention.
- Run formatters before linting. If lint fails, run the formatter again and
  re-run the linter.
- Re-open files after formatting to avoid stale edits.
- Goal: always trust the autoformatter. If it breaks output, fix the formatter
  so the next run is safe.

## Coverage ratchet

- Apply the incremental-ratchet workflow when increasing coverage: commit the
  tests or code changes first, then lower the enforced uncovered-item ceiling
  in a separate following commit.
- Use only absolute uncovered item counts; do not use coverage percentages for
  evaluation, enforcement, or reporting.
- For every available metric, configure the maximum allowed uncovered count at
  the exact current count. The gate is `actual <= threshold`; lower is stricter
  and the destination is a zero threshold.
- When an actual count decreases, lower its configured threshold to match.
  Raising an uncovered threshold is a regression and requires explicit user
  approval.
- Ceiling semantics: `state-space-minimization` `references/ratchet.md`
  (the ceiling only lowers; weakening only on explicit user request).

## Review quality

- Keep review findings ordered by severity.
- Make every finding reproducible for a low-context reviewer. State the
  applicable rule, observed evidence, concrete harm, idiomatic baseline, any
  deliberate project departure, the smallest safe fix, and the deterministic
  or partial check that can prevent recurrence. Do not promote an automated
  candidate to a finding until its semantic applicability is confirmed.
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
- Every input **MUST** have explicit bounds. Bound dynamically sized input by
  length or cardinality at the earliest boundary, before unbounded buffering or
  allocation; bound streaming input by an explicit total, record, or resource
  budget. Hardware exhaustion and implementation accidents are not bounds.
- Push constraints down to the lowest layer (primitive types, domains, DB
  constraints) and avoid redundant checks at higher layers when a more
  primitive guarantee exists.
- The database is the safety net, not the first line of defense. Application
  and domain layers **MUST** reject bad data before it reaches the database.
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
- Treat an estimated constraint as an intentionally strict, refutable
  hypothesis about the domain. Start with the strictest reasonable constraint
  consistent with known valid cases, run the system, and use explicit
  rejections to learn. A newly observed valid rejection refutes the hypothesis;
  weaken only the smallest dimension and amount needed to admit that case, then
  update the bound's provenance and boundary tests. Never silently truncate,
  discard, or normalize rejected input to hide the refutation.
- String types **MUST** be tightened toward the bounded form (min, max, and
  grammar). Treat `NonEmpty*` without a recorded upper-bound reason as
  an unfinished narrowing. Use `Option` when empty and missing are
  distinct, meaningful states.
- When a string has no known grammar, use printable characters only as the
  safe estimated default. Widen that grammar only when a known valid input
  refutes it. Default the minimum length to 0 unless a non-empty value is
  required to be meaningful.
- Prefer a parsed, specialized type over a raw string when one exists
  (URL, UUID, path, timestamp, semantic version).
- Domain values and plain data containers **MUST** be immutable after
  construction by default.
- A container **MAY** expose publicly readable fields only when the language or
  API prevents reassignment and reachable mutation, each field's type
  independently enforces every field invariant, and the aggregate has no
  cross-field invariant. Deserialization must preserve those field-level
  proofs.
- Otherwise, aggregate fields **MUST** remain private and every construction
  and deserialization path **MUST** enforce the aggregate invariants through
  its constructor or smart constructor.
- Mutation **MAY** be exposed only for a concrete requirement. Use the narrowest
  operation that expresses the required transition and preserves every
  invariant; do not expose general writable fields or unrestricted setters.
- If constraints tighten later, migrate or repair old data; do not keep
  invalid states around.
