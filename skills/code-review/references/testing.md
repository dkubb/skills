# Testing Guidelines

These rules apply to tests across languages.

- When TDD workflow or commit sequencing for tests is in scope, use a TDD
  workflow skill if one is available (such as `tdd`).

## Structure

- One behavior per test, with Arrange → Act → Assert and full effect checks.
- Keep the act explicit in the test body; helpers **MUST** only arrange or
  configure.
- Separate Arrange, Act, and Assert sections with a blank line to make test
  structure obvious. Do not add section header comments like `// Arrange`,
  `// Act`, or `// Assert` — the blank-line separation and the pervasive AAA
  pattern make them redundant.
- Use one set up + one step per test.
- Use one check for the result value. Use more checks only for side effects or
  no side effects.
- Use tight checks. Cover all possible result values and changes from the step.
- If only assertions differ, merge into one test with all required assertions.
- Always assert the return value. If the act has observable side effects,
  assert all potential side effects for that act in every test that uses it.
- Tests **MUST** compare the exact observed output by default. Avoid substring
  checks like `contains` or `starts_with`.
- Avoid fallback accessors in tests (for example, `unwrap_or_default`,
  `unwrap_or`, `unwrap_or_else`, `map_or`, `map_or_else`,
  `get(...).unwrap_or_default()`). These can mask missing values and hide
  behavior changes. Prefer explicit `expect(...)` or assertions that fail when
  data is absent.
- Tests **MUST NOT** normalize, sort, fix, or transform inputs or outputs to
  help them pass. The observed behavior is the contract; any divergence
  **MUST** fail fast. Prefer brittle assertions (`expect`, exact matches) over defensive
  defaults.
- A test **MAY** compare under normalization or order-insensitive equivalence
  only when the domain contract explicitly defines that equivalence. The test
  **MUST** name the rule, preserve distinctions the contract still observes
  (such as duplicates), and assert canonical output exactly when the system
  defines one. Ad hoc cleanup for comparison is not a valid exception.
- Avoid conditional logic in tests and test helpers. If behavior differs by
  branch, write separate tests or helpers so each test remains linear and
  explicit. Branches in tests often hide the failure the test is meant to
  surface.
- Avoid “soft” setup in tests (fallbacks, silent defaults, normalization, or
  conditional cleanup). Tests **MUST** make missing or unexpected state fail
  immediately so divergence is obvious.
- Avoid redundant assertions implied by earlier assertions.
- Assertion messages must name the thing under test, avoid generic terms, and
  use consistent phrasing within a test.
- Prefer test helpers over production environment switches when injecting
  failures.
- Ensure tests cannot write to real stdout or stderr.

## Test classification

- Classify a test by the boundary it exercises, not its directory or filename.
  A unit test exercises one unit through in-process interfaces. An integration
  test exercises collaboration across real component, process, storage,
  network, or system boundaries.
- Language and framework references own test directory, filename, module, and
  runner conventions. Follow the repository's adopted convention when several
  ecosystems are valid for the language.
- Determine deterministic-simulation requirements from the behavior under
  test and the project's adopted requirements, not from the test's path or
  unit/integration label.

## Bug-fix testing discipline

- For bug fixes, reproduce the exact failure with a test or deterministic
  harness before changing production code. Fix only after the failure is
  reproducible; do not commit the failing intermediate state.
- Assume failures are non-deterministic until verified by re-running the seed
  before investigating. After applying the fix, run the cheapest decisive
  verification first: the focused regression test and the same seed,
  fast-forwarded to the same iteration.
- After the decisive checks pass, run every applicable verification gate whose
  result is unknown for the exact fixed state, following `SKILL.md`'s evidence
  reuse rules. Commit only after the decisive checks and all applicable gates
  pass.
- Creating a commit does not by itself invalidate exact-working-tree evidence.
  Do not rerun the seed or gates after committing unless relevant content,
  configuration, dependencies, or toolchain state changed, or the earlier
  evidence was incomplete.
- Refactor either the test or the fix, but not both at once — the
  code-or-tests rule in `atomic-changes` `references/commits.md` § "Ordering".
- Use fixed unit tests for known-bad inputs and boundary crossings around
  fixed points; reserve property tests and fuzzing for exploration.

The property-test strategy — core invariants, the allowlist model,
generator construction and 80/20 bias, round-trip properties, and
exploration vs validation — is owned by `property-based-testing.md`.
This file owns test structure and the review requirements.

## Boundary transformations and property tests

- Prefer smart constructors for domain types with invariants.
- Treat constructors or public setters that can create invalid states as
  findings unless the change includes an explicit rationale.
- Every smart constructor, serializer, emitter, output generator,
  deserializer, and parser **MUST** have both:
  - Focused unit tests at each lower and upper boundary, including the first
    invalid value outside each boundary when the operation can reject input.
  - Property tests whose generator support spans the boundaries and every
    representable interior state in the declared domain. Include the full
    representable invalid domain when the operation rejects input.
- Serializer and deserializer unit tests **MUST** pin round trips at the domain
  boundaries. Parser and smart-constructor unit tests **MUST** pin acceptance
  at each valid edge and rejection immediately outside it.
- Every paired producer and consumer **MUST** have unit and property round-trip
  tests: structured value to serialized, emitted, or generated representation
  and back to the same value; and accepted representation through parsing or
  deserialization and back to the same representation. Compare exactly unless
  the domain contract defines normalization, in which case compare the
  declared equivalence and assert the canonical output.
- If the database encodes the same invariant, require tests that keep the
  application and database rules aligned. A non-uniqueness DB constraint
  failure in normal execution is a bug and **MUST** trigger test backfill.
- Keep aggregates immutable and fields private by default. Publicly readable
  fields are acceptable only when the language or API prevents reassignment
  and reachable mutation, their types independently enforce every invariant,
  and the aggregate has no cross-field invariant.
- Route parser and deserializer entry points through the owning field or
  aggregate smart constructors so they preserve the same proofs. When a
  concrete requirement needs mutation, test the narrow transition operation
  across its boundaries and properties; do not add a general writable-field
  test surface.

## Coverage

- Use boundary tests and property tests for smart constructors, serializers,
  emitters, output generators, deserializers, and parsers.
  - Boundary tests use the before/at/after model at each edge:
    - Before the boundary (valid interior near edge), at the boundary
      (valid edge), after the boundary (first invalid past the edge).
    - Apply at both min and max: below min (invalid), at min (valid),
      above min (valid), below max (valid), at max (valid), above max
      (invalid).
  - Do this even for simple rules like `>= 1` to catch spec drift.
  - Property tests explore the interior of the state space; generator
    construction is owned by `property-based-testing.md`.
- Treat observable behavior as contractual (Hyrum’s Law). If behavior changes,
  tests **MUST** fail so the change is explicit.
- When a review finds a test issue, search for similar patterns and fix them.
- Backfill integration tests for user-visible behavior changes and I/O
  boundary contracts.
- Integration tests **MUST** assert full contracts where applicable:
  exit code, stdout, stderr, and side effects.

## Complexity thresholds (strict defaults)

Use tool output when available. Otherwise, review against the intent.

- Tests (strict):
  - Apply the strongest limits here. Tests are reminders and **MUST** stay
    small.
  - Cyclomatic complexity per test: 1.
  - Cognitive complexity per test: ≤ 2.
  - NPath complexity per test: ≤ 2.
  - ABC counts per test: A, B, C ≤ 2 each.
  - Decision density per test: ≤ 0.15.
