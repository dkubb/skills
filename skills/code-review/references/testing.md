# Testing Guidelines

These rules apply to tests across languages.

- When TDD workflow or commit sequencing for tests is in scope, load
  `../../tdd/references/tdd.md`.

## Structure

- One behavior per test, with Arrange → Act → Assert and full effect checks.
- Keep the act explicit in the test body; helpers should only arrange or
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
- Use exact comparisons. Avoid substring checks like `contains` or
  `starts_with`.
- Avoid fallback accessors in tests (for example, `unwrap_or_default`,
  `unwrap_or`, `unwrap_or_else`, `map_or`, `map_or_else`,
  `get(...).unwrap_or_default()`). These can mask missing values and hide
  behavior changes. Prefer explicit `expect(...)` or assertions that fail when
  data is absent.
- Tests must not normalize, fix, or transform inputs/outputs to “help” them
  pass. The observed behavior is the contract; any divergence should fail fast.
  Prefer brittle assertions (`expect`, exact matches) over defensive defaults.
- Avoid conditional logic in tests and test helpers. If behavior differs by
  branch, write separate tests or helpers so each test remains linear and
  explicit. Branches in tests often hide the very failure the test should
  surface.
- Avoid “soft” setup in tests (fallbacks, silent defaults, normalization, or
  conditional cleanup). Tests should make missing or unexpected state explode
  early so divergence is obvious.
- Avoid redundant assertions implied by earlier assertions.
- Assertion messages must name the thing under test, avoid generic terms, and
  use consistent phrasing within a test.
- Prefer test helpers over production environment switches when injecting
  failures.
- Ensure tests cannot write to real stdout or stderr.
- Treat anything under a `tests/` directory as an integration test; integration
  tests do not need madsim/DST coverage by default. Reserve DST requirements for
  unit tests and internal test helpers unless explicitly requested.

## Property test invariants

- 100% of valid inputs must be accepted. 100% of invalid inputs must be
  rejected. Property tests verify this on every run with fresh samples.
- Treat database constraints as part of the validity contract. Property tests
  for boundary parsers, validators, and smart constructors should match the
  database model so accepted application values are insertable and rejected
  values fail before they reach the database.
- When in doubt, bias toward rejection. "Too strong" is self-correcting
  (real inputs expose blocked valid cases). "Too weak" is silent until
  invalid data propagates downstream far from the entry point.
- The valid range is bounded; the invalid range is effectively unbounded.
  Broad invalid sampling is nearly worthless because the space is too vast.
  This asymmetry justifies 80% biased weighting for both sides.

## Smart constructors and property tests

- Prefer smart constructors for domain types with invariants.
- Treat constructors or public setters that can create invalid states as
  findings unless the change includes an explicit rationale.
- For each smart constructor, require both:
  - Boundary unit tests for all declared constraints.
  - Property tests that prove valid input acceptance and invalid input
    rejection.
- If the database encodes the same invariant, require tests that keep the
  application and database rules aligned. A non-uniqueness DB constraint
  failure in normal execution is a bug and should trigger test backfill.
- Keep fields private when possible and route parser/deserializer entry points
  through the smart constructor.

## Coverage

- Use boundary tests and property tests for smart constructors and parsers.
  - Boundary tests use the before/at/after model at each edge:
    - Before the boundary (valid interior near edge), at the boundary
      (valid edge), after the boundary (first invalid past the edge).
    - Apply at both min and max: below min (invalid), at min (valid),
      above min (valid), below max (valid), at max (valid), above max
      (invalid).
  - Do this even for simple rules like `>= 1` to catch spec drift.
  - Property tests explore the interior of the state space.
- For property tests, build four building-block generators per type:
  `*_valid_broad`, `*_valid_biased`, `*_invalid_broad`, `*_invalid_biased`.
  Compose them into two primary generators for use in tests:
  - `*_valid` = 20% broad, 80% biased.
  - `*_invalid` = 20% broad, 80% biased.
  - Both sides use the same 80/20 rule: 80% biased toward boundaries
    where bugs cluster, 20% broad for full-range sanity coverage.
  Tests use `*_valid()` and `*_invalid()` directly.
- Prefer an explicit inverse generator when possible; otherwise construct a
  complementary generator for invalid values.
- Invalid generators must be independent of the code under test. Do not use
  the constructor, validator, or parser in a `prop_filter` to create invalid
  cases. Use an inverse regex or explicit invalid strategy instead.
- Treat observable behavior as contractual (Hyrum’s Law). If behavior changes,
  tests should fail so the change is explicit.
- When a review finds a test issue, search for similar patterns and fix them.
- Backfill integration tests for user-visible behavior changes and I/O
  boundary contracts.
- Integration tests should assert full contracts where applicable:
  exit code, stdout, stderr, and side effects.

## Exploration vs validation

- Property tests are for **exploration**: they discover failures through
  combinatorial coverage across many inputs that no one would write by hand.
  Biased generators deliberately overlap with boundary unit tests because the
  value is in the combinations — boundary-adjacent values mixed with other
  inputs surface interactions that single-dimension unit tests miss.
- Unit tests are for **validation**: they pin known-good and known-bad
  behavior with deterministic, minimal reproductions.
- When a property test discovers a failure, reduce it to a minimal
  reproduction and encode it as a unit test. The unit test becomes the
  permanent regression guard; the property test continues exploring.
- When real data forces a constraint relaxation, add tests for the newly valid
  case before loosening the constructor, parser, or database constraint. Keep
  the feedback loop explicit.

## Complexity thresholds (strict defaults)

Use tool output when available. Otherwise, review against the intent.

- Tests (strict):
  - Apply the strongest limits here. Tests are reminders and should stay small.
  - Cyclomatic complexity per test: 1.
  - Cognitive complexity per test: ≤ 2.
  - NPath complexity per test: ≤ 2.
  - ABC counts per test: A, B, C ≤ 2 each.
  - Decision density per test: ≤ 0.15.
