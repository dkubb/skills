# Documentation as type signatures for readers

Code has formal type signatures that constrain the call site's state
space. Documentation does the same job for human readers: it
constrains the reader's mental model of what a function, module, or
system does. The reader codes against whatever the documentation
declares; if the declaration is wider than the real behavior, the
caller's call-site state space widens to match. Same vocabulary
from `principles.md` (domain, codomain, range), same six
operations, same audit questions — applied at the level of
human-inferred contracts rather than machine-checked types.

## Vocabulary applied to documentation

| Term | Meaning for documentation |
|---|---|
| Domain | the inputs the docs declare valid |
| Codomain | the behaviors the docs claim the code can produce |
| Range | what the code actually does |
| Drift | the gap between docs (codomain) and code (range), accumulated over time |

The bilateral goal: shrink the domain (declare what inputs are
valid, exclude the rest) and close the codomain-range gap (describe
exactly what the code produces — neither wider nor narrower than the
real behavior).

## The dominant failure mode is drift

Documentation drift is the documentation analog of a divergent
type signature (`principles.md` § "Types as hypotheses"), with a
delay. Code evolves; docs lag. Every commit that changes behavior
without updating the relevant docs widens the codomain-range gap.
The gap accumulates monotonically until a forcing function pulls
the docs back into sync.

Callers code against what the docs say. When the docs lag, callers
write defensively against both the documented behavior and the
behavior they observe — the call-site state space widens to the
union of "what docs claim" and "what code does." Same defect class
as *Test corpus as production specification* in `testing.md`, on
the consumer side.

The forcing function is what matters. Without one, drift is the
default.

A lightweight forcing function for any *derived representation* of
a determinant document (a formal rendering, a generated reference,
a mirror in another notation): stamp the derivative with the
determinant's version, and bump both in lockstep when they
correspond. A version mismatch is then a visible staleness flag
instead of silent drift — re-derive and re-stamp after determinant
changes, or leave the mismatch standing as an explicit "stale"
marker.

## Hierarchy: prefer executable forms over prose

Each rung is harder to drift than the previous because each carries
a forcing function (compiler, test runner) that fires when the
documentation no longer matches the code. Use the highest rung that
can carry the contract.

1. **Rich types that carry the invariant directly.** `NonEmpty<T>`,
   `EmailAddress`, `Email::parse() -> Result<Email, ParseError>` —
   the type *is* the documentation. The compiler verifies on every
   build. No prose claim is needed.
2. **Doctests.** Executable examples that fail when the code drifts
   away from the example's expected output. The test suite is the
   forcing function.
3. **Type signatures rich enough to imply the behavior.** A function
   `fn parse(input: &str) -> Result<Order, ParseError>` already
   tells the reader: takes a string, may fail with `ParseError`,
   succeeds with an `Order`. Less prose needed.
4. **Co-located rationale comments.** One line of prose next to the
   code, explaining the *why* the code cannot express — a hidden
   invariant, an upstream constraint, a workaround for a specific
   bug. Co-location means a future editor sees the comment when
   they touch the code.
5. **Architectural docs at boundaries.** Where the type system
   stops (system boundaries, ingress / egress, deployment,
   cross-team contracts), prose is unavoidable. Keep it close to
   the boundary it describes.

External documentation (wikis, separate docs sites, sphinx-style
auto-generated reference pages disconnected from the code) sits
below this hierarchy. Drift is structural: nothing breaks when the
code changes, so nothing forces the docs to follow.

## Applying the operations

### Shrink the domain

Declare which inputs are valid; exclude the rest by stating the
contract narrowly. "Takes any reasonable input" admits anything;
"Takes a non-empty UTF-8 string of at most 256 bytes matching the
RFC 5322 mailbox grammar" admits one set.

Anti-patterns at this axis: `any value`, `anything serializable`,
`as appropriate`, `for most cases`, `should work with most`. Each
is an unbounded codomain in three words.

Prefer:

- Exact input type with constraints stated, e.g. *"non-empty
  UTF-8 string, 1..=256 bytes, RFC 5322 mailbox grammar"*.
- The smallest set of valid examples that exhibits each shape
  — three or four cases, not twenty.
- An invalid example with the rejection mode named, e.g.
  *"empty string → `EmptyInputError`"*, *"non-UTF-8 bytes →
  `InvalidEncodingError`"*.

### Close the codomain-range gap

Describe exactly what the code produces, including each error
variant by name and every observable side effect.

Anti-patterns: `may throw`, `may return null`, `various
exceptions`, `generally returns the parsed value`. The codomain
implied is "anything"; the range is one thing.

Prefer:

- Exact return type, with each error variant by name
- Post-conditions on the returned value
- Side effects enumerated explicitly (disk writes, network calls,
  log lines, metrics emitted)

### Remove invalid intermediate representations

No section of the docs should be in an indeterminate state. Each
of the following is an invalid intermediate that pollutes the
docs' state space:

- `TBD` / `TODO` / `XXX` left in shipped documentation
- "This section is out of date" notes that remain shipped
- Aspirational sections describing behavior the code does not have
- Half-written examples that compile but do not exhibit the
  claimed behavior
- Documentation that contradicts itself across sections

If the documentation cannot describe the current behavior, fix it
before merging. The intermediate state where docs are wrong but
acknowledged-as-wrong is still wrong.

### Encode invariants into types

When the type system can carry the invariant, prefer that over a
prose claim. A function `fn process(o: PlacedOrder)` needs no doc
saying "the order must be placed before processing"; the type
guarantees it. A function `fn process(o: Order)` whose body runs
`assert!(o.status == Placed)` has shifted the defect from type to
runtime check — the type lies, the docs would have to lie too, and
the assertion catches it too late. Lift the narrowing to the type
and the documentation simplifies to nothing.

## Burndown priority

Apply the tier framing from `principles.md` § "Burndown priority:
infinities first" to documentation work. Address tier-1 first;
each completion exposes the next hotspot.

1. **Effectively unbounded prose claims.** Vague qualifiers (`may`,
   `might`, `sometimes`, `appropriately`, `as needed`, `where
   applicable`) admit any behavior. Highest priority to replace
   with specific claims.
2. **Out-of-tree documentation that describes code.** Wikis,
   external sites, separate API references for code that lives in
   this repo. Drift is structural — bring into the tree or accept
   the section as architectural-only and remove the code-level
   claims.
3. **Precise but unverified claims.** Prose docs that state exact
   behavior but have no executable proof. Promote to doctest or
   richer type when possible; keep as co-located prose otherwise.
4. **Verified docs.** Doctests, rich types, runnable examples. No
   action; these survive the lens.

## Documentation as consumer specification

Parallel to `testing.md` § "Test corpus as production
specification." Tests specify what production code should do; docs
specify what callers can rely on. Wrong docs shape *consumer code*
the same way wrong tests shape production code.

**When documentation overstates the contract:**

1. The docs claim a guarantee the code does not provide.
2. A caller codes against the claimed guarantee.
3. The caller's reliance becomes part of the API contract through
   Hyrum's Law — even if the original guarantee was a documentation
   bug.
4. The code can no longer be changed to match the original intent
   without breaking the caller.

**When documentation understates the contract:**

1. The docs omit a behavior the code provides.
2. A caller does not depend on the behavior (would have if known).
3. The behavior is later removed as "unused".
4. A future caller who would have used it cannot, because the
   feature is gone.

The discipline at every link:

- **Anything docs claim, the code must actually do.** Documentation
  is not aspirational. If the code does not implement the claim,
  the claim is wrong.
- **Anything the code does deliberately, docs should declare.**
  Behaviors callers should depend on are part of the contract;
  behaviors that should not be relied on either need explicit
  "do not depend on this" framing or should be made unobservable.
- **The doc set should be auditable.** Doctest failures, undocumented
  public APIs, broken example builds, dead internal links — these
  are the forcing functions that keep docs in sync.

## Anti-patterns

- **Boilerplate that duplicates the signature.** `param x: the x
  parameter`. Pure waste — no narrowing.
- **Comments that explain *what*.** The code already shows the
  what. Useful comments narrow by saying *why* — non-obvious
  constraints, workarounds, hidden invariants — not by restating
  the operation.
- **References to the current task, fix, or PR.** "Added for the
  OAuth migration", "fix for issue #1234", "used by the rate-limit
  flow". These rot as the codebase evolves and become misleading.
- **Speculative or aspirational documentation.** Claims about what
  the code *will* do rather than what it does. Future-state docs
  are invalid intermediate representations until the code catches
  up.
- **Out-of-tree documentation that describes code.** Reference
  pages on a wiki, function descriptions in an external doc site
  — guaranteed to drift, with no forcing function.
- **`Note:`, `BTW:`, `Heads up:` framings.** Colloquial framings
  signal "this is not part of the formal contract" — but readers
  treat them as contract anyway. Either it is a contract claim
  (make it formal) or it is not (delete it).
- **Examples that do not run.** Code blocks that look like code but
  were never compiled or executed. First to drift, with no signal.
- **Hidden warnings buried in long prose.** Critical constraints
  embedded in paragraphs. Readers skim and miss the constraint.
  Lift critical constraints to bullets or required preconditions.

## Patterns that survive

- **Doctests.** Executable examples that the test suite runs. An
  outdated doctest fails CI; the forcing function is automatic.
- **Rich types that carry the invariant.** No doc claim needed; the
  type is the documentation. Reviewed by the compiler on every
  build.
- **One-line rationale comments next to non-obvious code.** Short,
  located, specific. Explains the *why* the code cannot express.
- **Architectural docs at boundaries.** Where the type system
  stops, prose is unavoidable. Keep it close to the boundary; treat
  it as part of the boundary's maintenance burden.
- **Tested examples.** Code examples that are linted, compiled, or
  run as part of the build. Same forcing function as doctests.
- **Generated reference docs from type signatures.** When the type
  system is rich, the generated reference reflects reality by
  construction.

## When prose documentation is unavoidable

Prose is necessary when:

- The invariant cannot be expressed in the type system (capability
  tokens, time-varying properties, cross-system protocols).
- The *why* of a non-obvious decision needs to be preserved (a
  workaround for a specific bug, an upstream constraint, a
  deliberate trade-off).
- The documentation describes a boundary where the type system
  stops (ingress / egress, deployment, cross-team contracts,
  system architecture).
- The audience is not the language's type checker (humans
  onboarding, external API consumers, downstream teams).

In these cases:

- Keep the prose co-located with the boundary or code it describes.
- State the contract narrowly and exactly — same rules as for
  return-type narrowing.
- Update the prose whenever the boundary's behavior changes; treat
  it as code under change.
- Audit periodically — the forcing function here must be human,
  since no compiler will check.

## Cross-references

- `principles.md` — the vocabulary (domain, codomain, range) and
  the six operations this file applies; the burndown-priority
  framing.
- `testing.md` § "Test corpus as production specification" — the
  parallel failure on the test side; same chain, different actor.
- `commits.md` — commit messages are documentation of state
  transitions; same drift rules apply.
- `ratchet.md` — tooling as the forcing function for project
  configuration drift; the same anti-drift discipline applied to
  the project's allowed-configuration state space.
- `ingress-and-boundaries.md` — where prose is unavoidable because
  the type system stops at the boundary.
- `total-functions.md` — narrowing inputs until a function is total
  removes the need for prose documentation about preconditions.
- `proof-preservation.md` — rich types carry invariants the docs
  would otherwise have to state.
