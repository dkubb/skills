# Tests as state-space minimization

A test's accepted-behavior set is a state space. Same vocabulary
from `principles.md` (domain, codomain, range, preimage), same
four techniques, same audit questions — applied at the test level
rather than at production types or runtime state. The matcher is
the codomain; the system's actual behavior is the range; the goal
is to close the gap.

A weak matcher accepts many possible behaviors; a strong matcher
accepts one.

> **Matcher precision is the accepted-output state space.** Every weak
> matcher widens the set of behaviors that satisfy the test, which is
> the same defect the skill targets at the type level. Hyrum's Law
> guarantees that production code will eventually settle into whichever
> of those tolerated behaviors the matcher allowed.

## Strict matchers

Prefer the matcher that accepts the fewest behaviors while still matching
the contract.

### Identity contract vs structural contract

Two contract types, two matcher families:

- **Identity contract** — the function must return *this same instance*
  (memoization, deduplication, caching, returning a singleton, primitive
  comparison). Use an identity matcher: reference equality for objects,
  value equality for primitives. A structural matcher here is too loose
  — it would pass on a freshly constructed look-alike that the contract
  forbids.
- **Structural contract** — the function must return a value with
  *this shape* (a new instance each call, fields equal). Use a strict
  structural matcher: deep equality plus type / class identity plus own
  properties, rejecting class-tag mismatches and extra unexpected
  fields. A looser structural matcher (one that ignores type tags or
  extra `undefined` fields) widens the accepted set beyond the
  contract's range — the same defect a wide return type has on the
  production side.

Pick the matcher whose accepted set matches the contract's range:
identity when the contract is identity, strict structural otherwise.
Looser than that admits behaviors the contract forbids; tighter than
that produces spurious failures on valid behavior. Concrete matcher
names per ecosystem are in the `languages/` files.

### Return values: full structural equality

Spot-checking one field of a returned record means every other field is
unconstrained. If the function later adds a field, drops a field, or
silently changes one, the spot-check still passes.

```
weaker: checks one projection only
  assert user.id == expected_id

stronger: checks the full contractual shape
  assert user == User {
    id: expected_id,
    role: Role::Admin,
    email: expected_email,
    created_at: expected_time,
  }
```

For very large records, build the expected value with an explicit
constructor (a "test data builder") and assert exact equality, instead
of asserting fields one by one. The constructor sets the contract; the
test enforces it.

### Containers: assert presence *and* absence

A test that says "the result contains key X" leaves the rest of the
container's keys unconstrained. If a key was supposed to be removed
and was not, the test passes.

```
weaker: only checks one key projection
  assert value.get("description") == None

stronger: proves the object shape excludes the key entirely
  assert "description" not in value.keys()

stronger still: asserts the whole keyset
  assert sorted(value.keys()) == ["id", "name", "role"]
```

### Optional / sum types: match the case *and* the payload

`is_some` / `is_ok` / `is_present` are the testing equivalent of
boolean-blind APIs: they check the tag and discard the payload.

```
weaker: only proves the tag
  assert result.is_some()

stronger: proves both presence and exact payload
  assert result == Some(Expected::Value)
```

The same applies to `Result`, custom enums, and pattern matches with a
catch-all arm: a `_ => assert!(true)` arm accepts every case the test
forgot to enumerate.

### Exact call histories

Mocks and spies that assert "was called with X" do not constrain how
many times, in what order, or with what other arguments. Hyrum's-Law
behavior accumulates in the unconstrained dimensions.

```
weaker: was-called-with style; count, order, other args unconstrained
  mock.assert_called_with(user_id, "admin")

stronger: full call history with count, order, and arguments
  assert mock.calls == [
    Call(user_id, "admin"),
  ]
```

## Side effects: the Hyrum's-Law rule

Test every side effect every time. The general form:

> Every test asserts the **full Cartesian product** of (return-value
> shape × side-effect occurrence × side-effect content). Not just the
> return value.

For each function under test:

1. List every observable side effect the function can perform.
2. For each test case, assert return value, side-effect occurrence,
   payload, count, and order.
3. On paths where the side effect should not occur, assert the sink
   received nothing.
4. Add a regression test for any newly discovered incidental effect
   before changing the contract.

If a function performs a side effect on 1% of inputs, every test
asserts:

- on the 99% case: the side effect **did not execute** (assert the sink
  received nothing)
- on the 1% case: the side effect executed, with an **exact match** to
  the expected payload, count, and order

The 99% case is the silent failure mode. A test that only asserts on the
return value lets a function that *also* writes a row, sends an email,
or logs a record pass — and Hyrum's Law guarantees that a downstream
caller will start depending on that incidental behavior, locking it in
forever.

Side effects to capture explicitly:

- writes to the database, file system, or shared memory
- network calls (URLs, headers, bodies, retry counts, timeouts)
- log lines (level, target, structured fields, count)
- metrics emitted (name, labels, value, count)
- subprocesses spawned (program, args, env, stdin)
- IPC sent (queue, payload, partition key)
- mutations to in-memory shared state (clocks, caches, registries)
- stdout / stderr writes
- panics, aborts, or signal handlers triggered

For each, the test records what *should* happen and asserts that exactly
that happened — including the implicit "nothing else happened on the
other path."

## Tests as code

Matcher precision narrows the *accepted-output* state space (covered
above). Test files themselves are also code, with their own input
domain (fixtures), structural shape (test-function bodies), and
per-test isolation invariant. The four techniques from
`principles.md` apply symmetrically.

Apply these rules to tests in the task scope, or to the whole suite only
when project policy already makes them universal. Do not turn a narrow
production change into unrelated test-suite cleanup just because nearby
tests violate this ideal.

- **Shrink input domain.** Builders and fixtures take narrow domain
  types, not raw primitives. Fixtures construct through production
  constructors. No `from_raw_for_tests` or equivalent escape hatches
  — see *Test corpus as production specification* below for why
  bypassing the production constructor is uniquely dangerous.
- **Bound cardinality.** One act per test — the test asserts exactly
  one observable transition. One test per behavior — duplicates
  create ambiguity about what the contract is. The number of
  assertions per test is bounded by the act's observable surface
  (return value × side effects × content), not by the author's
  energy. The number of canonical fixture states is bounded by the
  named factories that exist for the type.
- **Shrink codomain.** A test passes or fails. No `it.only` /
  `it.skip` / `xit` / `fdescribe` in committed code. No conditional
  logic that turns into pass-by-default when a branch is not taken.
- **Remove invalid intermediate representations.** Cyclomatic
  complexity = 1 inside the test body — no `if`, no `for` (except
  framework-level parametric tables that treat each row as its own
  test), no `while`, no `try/catch` unless the throw *is* the act
  being tested. No shared mutable state across tests; each test
  starts from a known canonical state and leaves no trace.

The deepest of these is **one act**. It forces the test name to
describe one observable behavior, the assertion set to cover that
behavior fully (the side-effect Cartesian above), and prevents the
"let me test a few extra things while I'm here" drift that produces
tests no one can name. AAA (arrange / act / assert) is the structural
template; the assertion block is the last thing in the test body, and
nothing follows it.

These rules are lint-enforceable; treat each violation as a CI
failure, not a review nit. See *Lints carry the matcher-tightening
principles* below.

## Property tests

Property tests are state-space tools by construction: the generator
defines the input space and the property defines the accepted-output
shape. Apply the same precision to both.

The canonical recipe (four building-block generators + 80/20 weighting
+ before/at/after boundary model) lives in the `code-review` skill at
`references/property-based-testing.md`. The state-space framing of
*why* it works lives here.

### Bug density across the input state space is non-uniform

Bugs do not distribute evenly over the input space. Density rises
sharply at boundaries and rises further at the corners where multiple
boundaries meet. There are three tiers:

1. **Single-boundary transition.** One dimension at before / on /
   after, pinned by deterministic unit tests. Off-by-one and
   fence-post bugs live here.
2. **Boundary × interior combinations.** One boundary-adjacent
   dimension combined with generated interior values in the other
   dimensions. The bug surface where a guard at the boundary fails to
   compose with the rest of the logic.
3. **Boundary × boundary combinations.** Two or more
   boundary-adjacent dimensions simultaneously. Uniform sampling
   hits this tier with probability roughly the *product* of each
   dimension's boundary mass, so the probability collapses quickly
   as independent dimensions grow.

Uniform sampling badly under-samples tiers 2 and 3 because the joint
boundary regions are a vanishingly small fraction of the input
distribution. Biased generators keep the per-dimension boundary mass
roughly constant under biasing, so tier-3 hit rate falls only
linearly with N instead of multiplicatively. They spend 80% of their
draws inside the narrow zones where bug density is highest, and 20%
on the broad interior for sanity coverage.

This is the state-space justification for the four-generator pattern:

- `*_valid` and `*_invalid` are independent allowlist / outside-allowlist
  generators. Each is built from a `*_broad` (full range) and `*_biased`
  (boundary-adjacent) building block, mixed 20% / 80%.
- Build `*_valid` and `*_invalid` independently from the constructors
  and validators they exercise — otherwise the generator and the
  parser drift in lockstep and the test cannot catch the parser
  loosening.
- Assert canonical outputs, not just acceptance or rejection. A
  property test that says "the parser does not crash on this input" is
  weaker than "the parser produces this exact AST" or "the round-trip
  parse-then-print returns the original."
- Push property coverage as close to 100% as practical for
  constructors, parsers, serializers, and explicitly property-tested
  methods.

### Coverage is structural; biasing is the draw distribution

Two separate generator rules apply at once, and they are easy to
confuse:

- **Coverage rule (structural).** The valid generator's reachable
  set must equal the full *valid* input space. The invalid
  generator's reachable set must equal the full *invalid* input
  space (the complement). Together they reach 100% of the input
  domain — every value the type can represent is producible by one
  of the two generators.
- **Biasing rule (distribution).** Within each generator, draws are
  weighted 80% toward boundary-adjacent values and 20% toward the
  broad interior. This is the *sampling distribution* within the
  generator's coverage, not a restriction on what the generator can
  produce.

Both rules hold simultaneously. A generator that draws 80% from
boundaries but cannot produce some valid input is broken on the
coverage rule. A generator that covers the full space but draws
uniformly is broken on the biasing rule.

The downstream proof of the coverage rule is target-function
coverage. When the property test runs against its target, measured
coverage of that target should reach 100% on every dimension the
toolchain supports — statement, line, region, branch, MCDC. An
uncovered section is one of two findings:

- a **generator gap** — some valid or invalid input shape the
  generator cannot produce, leaving a code path the property test
  never exercises
- a **dead code path** — code that no real input can reach, which
  should be deleted (and is often a defensive branch against an
  impossible-by-type state — see *Test corpus as production
  specification* below)

Neither is acceptable as "uncovered but fine." Treat every uncovered
region as a finding to classify.

Biased generators **overlap with boundary unit tests on purpose**.
The unit test pins the tier-1 transition exactly; the biased
generator combines boundary-adjacent values across multiple fields to
surface the tier-2 and tier-3 interactions a single-dimension unit
test cannot reach.

When a coverage gap requires constructing a state the type system now
makes impossible, keep the state impossible. Record the gap as
*eliminated* state space, not as a coverage hole that needs a test-only
escape hatch.

## Integration tests

Integration tests should exercise public APIs, binaries, CLI behavior,
files, environment, network/database boundaries, and supported I/O as a
user or downstream caller would.

- enumerate public entry points and supported I/O paths; cover each
  real user-visible contract at least once through public APIs;
  document any uncovered branch as either unreachable because
  narrowing removed it, covered by unit/property tests, or an
  explicit residual risk
- assert full contracts: inputs, outputs, side effects, error surfaces
- do not add mocks, stubs, fake adapters, test-only switches, or
  artificial seams only to reach branches the real system never visits
- do not recreate invalid states that type narrowing made
  unrepresentable

If a branch cannot be reached by real integration behavior, document
why. The right outcome is often to cover the invariant locally with
unit/property tests, or to delete the branch because the narrowed type
makes it impossible.

Integration gaps are acceptable when they prove the design is tighter.
They are not acceptable when they hide untested real user behavior.

## Doctests

Doctests are examples, not exhaustive coverage machinery. Use them for
happy paths and common paths only. Common paths can include frequent
non-happy outcomes; rare edges, defensive branches, and exhaustive
rejection cases belong in unit, property, or integration tests.

## Mutation testing

Mutation testing tools (`mutant` for Ruby, `cargo-mutants` for Rust,
`stryker` for JS/TS, `mutmut` for Python) inject small mutations into
the production code and re-run the test suite. A surviving mutant is
direct evidence that the test suite accepts a *wider* behavior state
space than the contract allows: the original and mutated code differ
on some input, and no test catches the difference.

This makes mutation testing the operational verification of every rule
in this file. A test that relied on a weak matcher, a missing
side-effect assertion, or a forgotten boundary case will be exposed by
a surviving mutant — the mutant is the input the test forgot.

For each surviving mutant, take exactly one of two actions (in order):

1. **The mutated code is acceptable for every valid input.** This is an
   *equivalent mutation*. The mutated form is usually narrower (less
   power) than the original. **Apply the mutation to the source.**
   This is the simplification — see `least-power.md`. The source now
   uses the smallest primitive that satisfies the contract, and the
   mutation can no longer be generated.
2. **The mutated code is wrong on some input.** Add a test that fails
   on the mutant and passes on the original. The new test must
   distinguish the two forms — assert the precise output, not a
   property both forms happen to satisfy. This tightens the test
   suite's accepted-output state space until the mutant cannot
   survive.

The trap (from the `mutant` skill): do not rewrite the syntax to make
the mutant disappear without tightening the test suite. Replacing
`xs.map(f)` with a hand-written for-loop may stop the mutation from
being generated, but it does not prove the original behavior is
covered. The correct sequence is always test-first, then source
simplification.

Run mutation testing on every commit that touches:

- a smart constructor or any predicative validator
- a parser, codec, or schema
- a state-machine transition function
- any function whose body is a `match` / `switch` over a sum type

These are the hot zones for state-space defects, and surviving mutants
in these files are the highest-leverage signal. The skill commits to
mutation testing as a CI gate for these areas, not just a periodic
quality check.

Track surviving mutants alongside any state-space audit in a
project-local improvements document — each surviving mutant is a
finding waiting to be classified as either a simplification or a missing
test.

## Test corpus as production specification

Tests do not only verify; they specify by example. The test corpus
documents what production code is expected to handle, and production
code grows to fit that specification. **Wrong tests do not just fail
to catch bugs; they shape production code into the wrong contract.**

The failure chain when a test constructs an impossible state via a
bypass constructor:

1. Test constructs an impossible state via `from_raw_for_tests` or
   an equivalent backdoor.
2. Test asserts behavior on that state — for example, "if `email`
   is null, the function returns `Err(MissingEmail)`."
3. Production code grows a defensive branch to satisfy the test.
4. The defensive branch becomes part of the contract — Hyrum's Law
   on the production side.
5. If the production boundary ever loosens (or a new code path is
   added that *can* deliver that state), the defensive branch is now
   reachable behavior — locking the loose boundary in forever.
6. The function design now reflects the impossible-state input
   space, not the real one.

The discipline at every link:

- **Do not write the bypass constructor.** Anything tests can
  construct, production must be able to construct via real code
  paths. If the production constructor is too awkward to use in
  tests, fix the constructor — see the named-factory pattern in
  *Test-data builders and named factories* below.
- **Treat defensive branches against impossible inputs as smells.**
  Each `if (x == null)` on a non-nullable type, each `try`/`catch`
  around code that cannot throw, each "just in case" guard widens
  the call-site state space to cover inputs the type already
  rejects. Delete the branch; lean on the type.
- **When the type system says a state is impossible, lean on it.**
  The right test for "what if `email` is null" is not "construct
  that state and verify the error" — it is "verify the parser
  rejects the upstream input that would produce it." Push the test
  to the boundary where the narrowing actually happens.

Symmetric form of the matcher rule: a wide test *input* surface
produces a wide production *code* surface. Tests that construct only
states production can construct via real paths produce production
code with no defensive branches against impossible inputs.

## Test-data builders and named factories

When the production type tightens, the test-data builder must tighten
with it. A builder that takes raw primitives and constructs the domain
type via internal `unwrap` defeats the narrowing for tests, and tests
written against it will accept production code that violates the
invariant.

Preferred shapes for any builder that exists:

- builders take the same narrow types the public constructor takes
- builders that need defaults derive them from a single source of truth
  shared with production code
- builders never expose `from_raw_for_tests` or similar bypasses; if
  tests need a state the type forbids, the test is wrong or the type
  is wrong
- builders return values; persistence (DB insert, network I/O,
  filesystem write) is a separate explicit step

The last bullet generalizes to a production rule: **no I/O during
construction.** A constructor that touches the network, file system,
or shared state is doing two things and cannot be tested
independently. Lift the I/O to a separate phase; the constructor
returns a value.

### The destination: named factories on the production type

A separate test builder is itself often a smell. It usually means the
production constructor is awkward enough that tests grew a parallel
construction path. The parallel path drifts from production,
accumulates `unwrap`s and primitive parameters, and becomes the place
where invariants quietly relax — directly enabling the failure chain
in *Test corpus as production specification* above.

The destination is to ergonomically fix the production constructor
and have tests use it directly, with **named factory functions** for
canonical states living on the production type:

```
User::factory().verified_admin()
User::factory().banned_for_abuse()
Order::factory().shipped_domestic()
```

Three properties:

- factories use production constructors only — no bypasses
- factory names document the canonical state in domain language
- factories compose for related aggregates:
  `Order::factory().shipped_domestic({ user: User::factory().verified_admin() })`

The factory module is the canonical vocabulary. Seed code, onboarding
flows, and tests all reference the same factories. When the
production type tightens, the factories tighten with it; when a new
canonical state emerges, it is added to the factory module, not
invented per-test. One-off "make a user" helpers in test files are
lint-banned unless they call through the factory.

## Lints carry the matcher-tightening principles

The rules in this file — full structural equality, presence and
absence, sum case plus payload, exact call histories, the side-effect
Cartesian, CC = 1 in tests, no `it.only` / `it.skip`, named factories
only — are most durable when they are mechanically enforced by lint,
not by review convention.

Lint enforcement is what stops a weak matcher from sneaking in once
the principle is agreed. Review-only conventions decay because
reviewers forget; lint failures do not. In LLM-assisted codebases the
case is sharper: pattern drift propagates faster, because LLMs
reproduce whatever shapes appear in the existing corpus and reinforce
them into their own future suggestions. A bad pattern present in a
small fraction of files still gets picked up if it appears in the
file the model just read.

The split:

- The principle (what shape is contractually acceptable) lives in
  this file and in any project-level reference document.
- The enforcement (mechanical rejection of forbidden shapes) lives
  in lint configuration applied to test files.

Both are needed. The principle without enforcement decays; the lint
without the principle produces failures the reader cannot interpret.

## Cross-references

- `least-power.md` — surviving mutants point either to source-side
  simplifications (apply the mutation; the original used too much
  power) or to test-side tightenings (add the test that distinguishes
  the two forms).
- `boolean-blindness.md` — `is_some` / `is_ok` matchers are the
  test-side boolean-blindness defect.
- `constructive-vs-predicative.md` § "Smart constructors are only as
  strong as their trusted boundary" — test the trusted boundary
  directly with adversarial input.
- `total-functions.md` — defensive branches against impossible inputs
  (the failure mode in *Test corpus as production specification*) are
  the partial-function pattern on the production side; lift the
  narrowing into the type instead.
- `principles.md` — the same "minimize representable states" rule, now
  applied to test outcomes and to the structure of test code itself.
- `commits.md` § "Trailers as typed proof of validity" — gate
  trailers are the commit-level forcing function; lints on test
  code are the same idea at the test level.
- `documentation.md` § "The dominant failure mode is drift" — drift
  in test claims is the same defect class as drift in prose claims;
  same forcing-function discipline applies.
- `ratchet.md` — 100% target-function coverage is the
  aspirational target at the project level; the project's current
  coverage is the floor that the ratchet only ever tightens.
- `perfect-tool.md` — mutation testing is the canonical instance
  of the imagined-perfect-tool design move; the operator set is
  the imagined tool, surviving mutants are the gap between
  imagined and real.
- `languages/rust.md`, `languages/typescript.md` — concrete matcher
  patterns per ecosystem.
