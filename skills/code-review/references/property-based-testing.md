# Property-Based Testing Strategy

When backfilling tests, prefer boundary unit tests first, then property-based
tests in exploratory mode.

## Core invariants

Every property test suite must continuously verify these properties:

- **100% of valid inputs are accepted.** The valid range is well-defined and
  bounded. Every value in that range must pass the constructor or parser.
- **100% of invalid inputs are rejected.** The invalid range is effectively
  unbounded — within computer constraints it is vast compared to the valid
  range. Every value outside the valid range must be rejected.
- **Application and database rules stay aligned.** If the database encodes the
  same invariant, property tests at the application boundary should prove that
  accepted values fit the DB model and rejected values fail before they reach
  the database.
- **Invariants are reinforced on every run.** Each test suite execution
  re-verifies acceptance and rejection across fresh random samples, so
  regressions surface immediately.
- **When in doubt, bias toward rejection.** Constraints that are "too strong"
  are self-correcting — real inputs will eventually expose blocked valid
  cases, and the fix is to loosen. Constraints that are "too weak" are
  silent — invalid data propagates downstream and interacts with other
  components in unexpected ways, far from the entry point where it should
  have been rejected.

## Allowlist model

- Define the narrow range of valid values; reject everything else.
- This is an allowlist approach: specify what is in, and anything outside
  is invalid by default. The alternative — enumerating invalid cases to
  reject — is an unbounded game of whack-a-mole that can never be complete.
- The valid generator *is* the allowlist. The invalid generator is
  "everything else."
- This model is why the valid range is bounded and well-defined while the
  invalid range is effectively infinite within computer constraints.
  Random sampling from an unbounded invalid space almost never hits the
  interesting values, which justifies 80% biased weighting — concentrate
  on the narrow zone at the boundary where bugs actually live.

## Generators

- Use both valid and invalid generators.
- Valid generators: combine broad coverage with biased rare-but-valid cases
  using `prop_oneof!`.
- Invalid generators: combine broad invalid coverage with biased
  boundary-adjacent invalid cases using `prop_oneof!`.
- Keep four building-block generators per type:
  - `*_valid_broad` — broad coverage of the allowed range.
  - `*_valid_biased` — values before and at the boundary (still valid).
    Targets the valid edge: the exact boundary value and values just
    inside it.
  - `*_invalid_broad` — broad coverage of the disallowed range.
  - `*_invalid_biased` — values just after the boundary (first invalid).
    Targets off-by-one and fence-post errors right past the edge.
- Compose two primary generators for use in tests:
  - `*_valid` = `prop_oneof![80 => *_valid_biased, 20 => *_valid_broad]`.
  - `*_invalid` = `prop_oneof![80 => *_invalid_biased, 20 => *_invalid_broad]`.
  - Both sides use the same 80/20 rule: 80% biased, 20% broad. Bugs
    cluster at boundaries on both sides — boundary × boundary
    combinations across multiple fields are where the interesting
    interactions live. The 20% broad share still explores the full range
    for sanity coverage.
- Boundary model: before the boundary (valid interior near edge), at the
  boundary (valid edge), after the boundary (first invalid). The valid
  biased generator owns before+at; the invalid biased generator owns
  after.
- Tests use `*_valid()` and `*_invalid()`. The building-block generators
  exist for composition but are not typically used directly in tests.

## Serialization and deserialization properties

Always property-test serialization and deserialization. Three cases are
required:

1. **Valid string roundtrip**: `valid_string -> parse -> serialize -> string`.
   The output string must be equivalent to the input after normalization.
2. **Data roundtrip**: `data -> serialize -> parse -> data`. The output data
   must be equivalent to the input.
3. **Invalid string rejection**: `invalid_string -> parse -> error`. Every
   invalid string must produce an error.

These apply to every type that has both a string representation and a
structured representation (serde, `FromStr`/`Display`, database rows, API
payloads). Use the valid and invalid generators from the generator section
above.

## Roles: exploration vs validation

- Property tests are for **exploration**. They discover failures through
  combinatorial coverage that no one would write by hand.
- Unit tests are for **validation**. They pin known-good and known-bad
  behavior with deterministic, minimal reproductions.
- Biased generators overlap with boundary unit tests on purpose. The unit
  test pins the exact transition point; the biased generator puts
  boundary-adjacent values into combinatorial mixes with other inputs,
  surfacing interactions that single-dimension unit tests miss.
- When a property test discovers a failure, reduce it to a minimal
  reproduction and encode it as a unit test. The unit test becomes the
  permanent regression guard; the property test continues exploring.
- When real-world data proves a valid case was rejected, add a focused unit
  test for that input before relaxing the constructor, parser, or DB
  constraint. Then keep the property test exploring the updated boundary.

Also note MadSim is integrated; use it to reproduce rare cases.
