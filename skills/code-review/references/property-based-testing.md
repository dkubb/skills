# Property-Based Testing Strategy

Every smart constructor, serializer, emitter, output generator, deserializer,
and parser requires focused boundary unit tests and property-based tests. Write
the boundary unit tests first, then add property tests in exploratory mode.

## Core invariants

Every property test suite must specify these properties over the complete
support of its generators:

- **Every valid input is accepted.** The valid generator's support **MUST** span
  every representable state in the declared valid domain, including both
  boundaries and all interior states.
- **Every invalid input is rejected.** The invalid generator's support
  **MUST** span every representable state outside the valid domain within the
  operation's declared representation and operational bounds.
- **Application and database rules stay aligned.** If the database encodes the
  same invariant, property tests at the application boundary **MUST** prove that
  accepted values fit the DB model and rejected values fail before they reach
  the database.
- **Invariants are exercised on every run.** Each randomized execution samples
  fresh values from the complete generator support. Sampling does not execute
  every supported value in one run unless the finite domain is exhaustively
  enumerated.
- **When in doubt, bias toward rejection.** Constraints that are "too strong"
  are self-correcting — real inputs will eventually expose blocked valid
  cases, and the fix is to loosen. Constraints that are "too weak" are
  silent — invalid data propagates downstream and interacts with other
  components in unexpected ways, far from the entry point that was responsible
  for rejecting them.

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
  - `*_invalid_biased` — near-misses: take a valid value and diverge in
    exactly one small way (the first value past a bound, one character
    outside the allowed class, one element over the cardinality limit,
    one missing or extra field). Targets off-by-one and fence-post
    errors, and more generally the inputs a buggy validator is most
    likely to wrongly accept.
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
- Prefer an explicit inverse generator when possible; otherwise construct a
  complementary generator for invalid values.
- Invalid generators must be independent of the code under test. Deriving
  invalid cases by filtering random data through the constructor,
  validator, or parser (for example a `prop_filter` on rejection) makes
  the property circular: "rejects all invalid inputs" becomes "rejects
  what it rejects", which holds for any implementation and can never
  fail. Write the invalid generator as an independent statement of the
  spec — an inverse regex, an explicit invalid strategy, or
  boundary-adjacent construction — so a disagreement between the
  generator and the code surfaces as a test failure instead of a
  silently narrowed sample. The same circularity poisons downstream
  uses: a filtered generator inherits the constructor's current notion
  of invalid, never produces the values a too-loose constructor wrongly
  accepts, and mutates along with the constructor so mutants survive.
- Filtered-random generation also has the wrong distribution: a uniformly
  random input is almost never *near*-valid, so its rejects are obviously
  invalid and exercise almost nothing. An invalid case's value is
  proportional to how close it sits to the valid boundary — a buggy
  validator wrongly accepts near-misses, not noise. Construct near-misses
  deliberately by mutating a valid value one dimension at a time (the
  `*_invalid_biased` recipe); a rejection filter essentially never finds
  them.

## Producer and consumer properties

Always property-test paired serializers, emitters, and output generators with
their deserializers or parsers. Three cases are required:

1. **Valid representation round trip**:
   `representation -> consume -> produce -> representation`. Require exact
   output unless the domain contract defines normalization; then require the
   declared equivalence and exact canonical output. This is a narrow exception:
   the property must name the normalization rule and must not perform ad hoc
   cleanup merely to make the comparison pass.
2. **Structured-data round trip**:
   `data -> produce -> consume -> data`. The output data must equal the input.
3. **Invalid representation rejection**:
   `invalid_representation -> consume -> error`. Every sampled invalid
   representation must produce an error.

These apply to every type that has both a produced representation and a
consumer (serde, `FromStr`/`Display`, parsers/emitters, code or configuration
generators, database rows, and API payloads). Use the valid and invalid
generators from the generator section above.

- **JSON example:** For `string -> data -> string`, parse the input and output
  strings and compare the JSON data values when the contract follows JSON
  semantics. Object-member order is insignificant; array order remains
  significant. Do not use this normalization when duplicate-member behavior,
  number spelling, whitespace, or another textual distinction is part of the
  contract. When the emitter promises canonical JSON, also assert the exact
  canonical output string.

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

When deterministic simulation (madsim-style) is available, use it to
reproduce rare cases (see `determinism.md`).
