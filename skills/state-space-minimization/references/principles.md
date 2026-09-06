# Principles

Foundational, language-agnostic rules. Other reference files extend these
techniques in depth or apply them to a specific language.

## Core principle

**Represent only valid states, using the lightest mechanism that
actually removes the invalid states.** The number of representable
states should match the number of valid states as closely as
possible. Each extra representable-but-invalid state is latent bug
surface that runtime checks must guard.

Narrowings compound. Each invalid state removed is a bug class
eliminated and a behavior tightened for every future change that
touches the artifact, so the value of one more narrowing is rarely
marginal — finding a single additional one justifies real effort.

This principle is language-agnostic:

- make illegal values impossible to construct
- make illegal transitions impossible to express
- make illegal outputs impossible to return
- make tests reject illegal shapes instead of accidentally accepting them

The mechanism constraint is the guard against type maximalism: a
representation with more capability than the invariant requires
adds surface area without proportional reduction in
representable-but-invalid states. The lightest mechanism also
minimizes the trusted surface that has to hold — less to drift,
fewer escape hatches, fewer places the invariant can quietly
weaken. (Falsifiability is a separate axis: a heavier mechanism
with mechanical verification can be more falsifiable than a
lightweight smart constructor. Lightness is about audit cost and
drift surface, not about the raw strength of the falsification.)
See `least-power.md` for the operational form.

### Preserve facts when reducing representations

Minimize invalid and redundant representations, not distinguishable
facts about the domain. A typed observation that an artifact exists but
fails validation is valid; it does not admit the invalid artifact as a
valid domain value. Recording that failure must remain possible.
Two classifications having the same current handling does not prove
they mean the same thing.

For example, `Missing` and `Invalid` artifacts both map to `Repair`
under a repair policy. Keep the authoritative classifications distinct:
replacing both with `NeedsRepair` hides the anomaly of an existing
artifact failing validation. Derive the shared policy result from the
classification; do not use that result to replace the facts it groups.

A handler's action schema may therefore have fewer cases than the
observation schema: four factual classifications may map to three
handler categories. This many-to-one projection is valid when the
original classification remains available through a linked authoritative
record. Each layer need not carry the same facts or have the same
cardinality; the system must preserve the authoritative distinctions.

When evidence cannot establish a classification, represent that
uncertainty explicitly. Do not invent a definite status to make the
state space smaller. This requires preserving distinctions supported
by the domain contract and evidence, not every raw input or speculative
category.

Before merging cases, ask: **What establishes semantic equivalence
beyond the same current action?** If the cases describe different facts,
keep them distinct in the authoritative representation and group only
in a derived view.

## Formal vocabulary

The symbols are shared with `state-space-minimization-formal`,
which owns the inference rules built on them; this section owns
their meaning. Every module in both skills uses these names for
these concepts — same letter, same meaning.

Artifact-state notation:

- **Representable states `S(A)`** — the states an artifact's shape
  permits.
- **Valid states `C(A)`** — the states the artifact's contract
  admits; `C(A) ⊆ S(A)`.
- **Representable-but-invalid states `I_repr(A) = S(A) \ C(A)`** —
  the latent bug surface; the set this skill minimizes. (`I_reach`,
  the subset reachable through a boundary constructor, is defined
  in `constructive-vs-predicative.md`.)

Function notation:

- **Domain `D(f)`** — the set of values a function accepts. Determined by
  the input types in the function's signature.
- **Codomain `K(f)`** — the set of values the function's return *type* can
  represent. Determined by the output type in the signature.
- **Range `R(f)`** — the set of values the function actually produces over
  its entire domain. Always a subset of the codomain; usually not
  expressible in the type system directly, but the *goal* is to make
  it expressible.
- **Gap `G(f) = K(f) \ R(f)`** — the codomain values the function
  never produces. "Closing the codomain-range gap" means driving
  `G(f)` toward empty.
- **Preimage `f^{-1}(F)`** — for a chosen set of outputs `F`, the set of
  inputs that produce it. Large preimage means many inputs collapse to the
  same output (information loss, as in hashing or masking). Small preimage
  means outputs are specific to particular inputs (information
  preservation).

The bilateral goal of state-space minimization:

1. **Shrink the domain** to the minimum set of valid inputs. The
   function accepts no values that are not valid.
2. **Close the gap between codomain and range** by tightening the
   return type until it can describe only values the function
   actually produces. The function returns no values the type cannot
   describe, and the type describes no values the function cannot
   return.

Both axes reduce the total state the system can represent at any
given moment.

## Search algorithm

State-space minimization is a search algorithm. The search space
is the set of (type, constraint, invariant) configurations the
codebase can be expressed in. The objective is to minimize the
count of representable-but-invalid states. The hard constraint is
that every genuinely valid behavior must remain representable.
The local moves are tightening, weakening, restructuring, and
lifting to a boundary; the six operations below name them
operationally. The oracle is the set of real-world tests, property
generators, mutation runs, type-checker errors, and production
traffic that reveals when a candidate configuration either rejects
a valid state or admits an invalid one.

The search direction is biased toward tightening — the ratchet
records the trajectory — but the goal is the **true minimum**:
the strictest configuration the oracle does not refute. Tightening
past the true minimum produces a type system that looks strong
and fails under real input. Looseness past the true minimum admits
invalid states and lets Hyrum's Law do the rest. The skill is the
structure of the search, not a target tightness.

### Types as hypotheses

A type is a hypothesis about the invariants of the values it
admits. The compiler refuses configurations that locally
contradict the hypothesis; the oracle refutes the hypothesis when
real-world behavior contradicts it. Strict types are deliberately
brittle so refutation happens fast — at compile time, at test
time, or in early production — rather than slowly, through silent
data corruption. A type the oracle refutes is a **divergent type
signature**: the type's claim and the code's behavior disagree.
No intent is implied — the author may have held an incomplete
model, the system may have evolved, or the type may have been
correct at first and drifted.

### Weaken before strengthen

The two failure modes of an invariant are not symmetric:

- **Too-strong invariant** (rejects valid states). Reality refutes
  the hypothesis directly: a valid input fails the type. The
  repair is to weaken — relax the constraint so the valid states
  pass through. The previously-admitted states were valid by
  construction, so nothing already in the system contradicts the
  relaxation. Cost: re-run tests, occasionally migrate data.
- **Too-weak invariant** (admits invalid states). Reality does not
  refute the hypothesis directly — the type admits silently. The
  repair is to strengthen, but existing data and existing call
  sites may already depend on the loose semantics. Tightening
  requires auditing every call site, proving no caller depends on
  the looser form, and often migrating data accepted under the
  loose constraint.

The asymmetry has a clear consequence: **start strict and weaken
when the oracle refutes, rather than start loose and strengthen
when bugs surface.** The ratchet only tightens because tightening
is the productive search direction; weakening, when forced by the
oracle, is correction of a refuted hypothesis, not regression.

### Classify the refutation before weakening

A refutation is not automatically a signal to relax the invariant.
Three cases, three repairs:

- **Too strong** — the invariant rejects values that are genuinely
  valid in the domain. The shape of the invariant is wrong. Weaken
  to the smallest form that admits the counter-example; do not
  weaken further than the counter-example forces.
- **Misplaced** — the invariant is correct but is enforced at the
  wrong layer. The repair is to *move* it, not weaken it: lift to
  the boundary parser, push to the aggregate, encode in a
  capability token, narrow the type that owns the constraint.
  Weakening here is a regression — it discards a correct claim
  because the claim was in the wrong place.
- **Wrong scope** — the invariant is correct for one context and
  incorrect for another, and the two contexts now share a type. The
  repair is to split the type so each context carries the invariant
  appropriate to it. Weakening the shared type to the lowest common
  denominator collapses two state spaces that should stay distinct.

After the refutation is resolved, record whether the skill itself
needs a sharpened rule, a counter-example, or a boundary note. The
refutation is evidence; the skill is the place evidence becomes
rule.

## Self-similarity

The vocabulary and techniques above apply at every level where
there is a domain, a codomain, and a gap between them — not only
at the type level. The same audit questions land on types,
function signatures, tests, commits, documentation, defensive
code, tooling thresholds, and the skill itself. Each topic module
in this skill is the same principle applied to a different level;
`SKILL.md` maps levels to modules.

The skill itself is one of those levels. Applying the skill to a
concrete case produces evidence about whether the skill's rules
match the actual situation; the audit after each use is the
forcing function that converts evidence into proposed
improvements. See `skill-refinement.md` for the operational
discipline.

When a new level is encountered that does not yet have a module,
derive guidance from this file directly. A candidate new module
is itself a finding worth surfacing.

## State space arithmetic

The cardinality of a domain or codomain is the number of values its
type can represent. The arithmetic of types is the arithmetic of
those cardinalities.

- Product types multiply state counts: `|A × B| = |A| · |B|`.
- Sum types add state counts: `|A + B| = |A| + |B|`.
- Optional types add one state: `|A?| = |A| + 1`.
- Waste ratio: `|I_repr(A)| / |S(A)|` — representable-but-invalid
  states over representable states.
- Lower waste ratio means less invalid behavior to guard at runtime.

A field of type "integer in 1..=5" represented as a 32-bit integer has a
waste ratio of `(2^32 − 5) / 2^32 ≈ 99.9999998%`. The same value as a
five-variant enum has waste ratio `0`.

Cardinality arithmetic sizes a single state space. Comparisons
*between* candidate encodings are ordered by inclusion, not
cardinality — see `state-space-minimization-formal` § "Artifact
Calculus".

## Six operations

### Shrink the domain

- Narrow input types to the smallest valid range.
- Every function parameter type is a domain decision.
- Use the strongest closed-set or constrained type the boundary supports.
- Bound both ends of every range. If the external domain does not publish an
  upper bound, choose an explicit operational hard limit.
- Bound the length of strings, byte arrays, and identifiers. If no protocol
  or storage limit is published, derive a cap from expected use plus headroom.
- Bound the cardinality of collections whenever empty, singleton, or maximum
  count matters.
- Use smart constructors with private fields when the language has no
  refinement-type checker.
- Treat every smart constructor as a *narrowing function*: it accepts the
  widest input that belongs at that boundary, checks all invariants
  available there, and returns the narrowest type the checks prove.
- Prefer allowlists over denylists.

### Bound ranges and cardinality

State-space minimization is not only about removing zero or empty values.
There is always an upper bound, even when the design has not named it.
For each primitive and collection, ask four questions:

- What is the lowest valid value?
- What is the highest valid value?
- How long may this string or byte sequence be?
- How many entries may this collection contain?

If a protocol, provider, database, file format, or user-facing contract
publishes a maximum, use that maximum as-is. If no external maximum
exists, choose an explicit bound anyway: estimate a plausible maximum
the system should accept, round it up to the nearest power of two, and
make that the hard limit. Adjust on evidence — a rejection at an
estimated bound is signal about the real domain, not noise. Do not
leave the type unbounded while waiting for perfect information: an
implicit hardware or runtime limit (memory, storage, a database
default) is a predicative situation — the bound exists, but it lives
outside the contract and varies by machine.

When the true limit is unknown, over-constraining is the cheap
direction to be wrong in. Loosening an estimated bound later is
contract-widening: every previously valid value stays valid and no
caller breaks. Tightening later is breaking: existing data may violate
the new constraint and needs remediation.

Record each bound's provenance alongside the bound:

- **Spec-derived** — published by a protocol, provider, or format
  (an RFC field size, a DNS label length). Fixed; not negotiable;
  excluded from the ratchet.
- **Estimated** — chosen by the heuristic above. Provisional; move it
  when evidence arrives (`ratchet.md` covers the direction rules).

Record the unit, too. A character count and a byte count are different
contracts: a 256-character UTF-8 string can be a kilobyte. The same
number enforced at two layers in different units agrees only on ASCII —
a silent `I_reach` gap between the layers (see
`constructive-vs-predicative.md` for the formal names).

**When replacing `String` or `Vec`, go directly to the bounded form.**
Prefer `BoundedString` / `BoundedVec` over `NonEmptyString` / `NonEmpty`
unless the upper bound is genuinely unresolved *and* the gap is recorded
(comment, ticket, or doc) with the reason. Bottom bound and top bound,
always — `NonEmpty*` proves only the lower bound and is a placeholder,
not a destination.

For strings, prefer this progression:

1. unbounded string — never leave a domain field here
2. non-empty string — placeholder; record why the upper bound is
   unresolved before committing
3. bounded string with min length, max length, and grammar rules
4. specialized parsed type when one exists (URL, path, UUID, timestamp,
   semantic version, JSON)

For collections, prefer this progression:

1. unbounded collection — never leave a domain field here
2. non-empty collection — placeholder; record why the upper bound is
   unresolved before committing
3. bounded collection with min and max cardinality
4. stricter collection type when needed (sorted, unique, keyed,
   order-preserving)

If a code review finds a `NonEmpty*` type without a recorded reason for
the missing upper bound, treat it as an unfinished narrowing and either
add the bound or add the explicit note. The audit question for every
`NonEmpty*` site is the same: what is the upper bound, and is there a
stricter grammar, element count, ordering, uniqueness, or
domain-specific invariant to encode?

### Shrink the codomain

Close the gap between codomain and range: tighten the return type
until it describes only values the function actually produces. A
function whose codomain matches its range cannot return a
representable-but-impossible value, and downstream callers do not
have to handle any.

- Return the narrowest type the function can guarantee.
- Return non-empty when empty output is impossible.
- Avoid optional return when success always has a value.
- Avoid result return when failure is impossible.
- Return typed errors, not raw strings.
- Return result for expected failure paths, not panics.
- Preserve distinct failure classifications and genuine uncertainty;
  a smaller result type must not erase facts (§ "Preserve facts when
  reducing representations").

### Remove invalid intermediate representations

- Parse into validated domain types at boundaries.
- Narrow every time a new type is created. Constructors, parsers,
  deserializer adapters, database row mappers, CLI/env readers, and API DTO
  translators are all opportunities to replace wide input with stronger
  proof-carrying types.
- Preserve existing proofs instead of round-tripping through primitives.
- Avoid temporary structs that are "partially valid".
- Avoid builders that allow `.build()` before required fields exist.
- Prefer direct construction from validated parts.
- Use typestate for multi-step construction when needed.

*Parse, don't validate* (canonical statement in
`ingress-and-boundaries.md` § "Parse at the boundary"): parsing
returns a typed proof of validity; validation returns a boolean
and discards the proof. Prefer parse APIs so callers cannot
forget to enforce checks.

### Normalize

- Decompose facts into atoms; give each fact exactly one
  determinant; remove transitively derivable copies; recompose
  along use.
- Each redundant copy of a fact is state: a copy that can disagree
  with its determinant is a representable-but-invalid state of the
  artifact.
- Applies to data schemas, code, commits, and documentation alike.

`normalization.md` is the deep module.

### Ratchet

- Replace a threshold `θ` with a stricter `θ'` once current
  evidence satisfies `θ'`.
- Each ratchet step records the search trajectory and prevents
  silent regression past an already-achieved tightness.

`ratchet.md` is the deep module, including the rules for when a
threshold may move the other way.

## Match representation to use pattern

A program is a set of transformations between values. Each
transformation creates an intermediate state, and the runtime
state space is the union of every state the program passes
through. The six operations above narrow the **static** state
space — what each type can represent at any given point. This
section narrows the **trajectory** — what the program actually
passes through during execution.

Choose the representation whose operations match the program's
use pattern. Each transformation needed to make the data usable
is one intermediate state in the trajectory; eliminating the
transformation eliminates the state.

The heuristic:

1. List every operation the program performs on the data —
   membership test, ordered iteration, indexed lookup, length
   check, parsing, comparison, mutation, serialization.
2. Pick the representation that supports all of those operations
   natively, without an intermediate conversion.

Wide source → narrow source, by operation needed:

| Operation | Wide source | Narrow source |
|---|---|---|
| Membership tests | `Vec<T>` + `.contains()` | `Set<T>` / `HashSet<T>` |
| Ordered iteration | `Vec<T>` + `.sort()` | sorted structure (`BTreeSet`, sorted newtype) |
| Indexed lookup by key | `Vec<T>` + linear scan | `HashMap<K, V>` |
| Length-bounded | `Vec<T>` + size check | `BoundedVec<T, MAX>` |
| Non-empty | `Vec<T>` + length check | `NonEmpty<T>` |
| Closed tag set | `String` + match | enum / sum type |
| Unique + sorted | `Vec<T>` + sort + dedupe | `BTreeSet<T>` |

In every row, the wide source admits values the operation does
not allow; the runtime trajectory passes through those values
during construction, transformation, and validation; the narrow
source skips those states entirely.

### Fuse what cannot be eliminated

External input arrives in the wide form (network bytes, JSON
arrays, file contents) and must be parsed at the boundary. When
a transformation cannot be eliminated, fuse it: combine N passes
into one. `parse → validate → normalize → index` as four passes
holds three intermediate values; a fused `parseAndIndex` returns
the narrow form directly with zero intermediates. Same idea as
Haskell stream fusion, Rust iterator chaining (which fuses by
design), or any "build it in one pass" idiom.

The composition: **match representation to use pattern first;
fuse whatever transformations remain.**

The same insight appears under other names in adjacent
literatures — *data-oriented design* (memory layout matched to
access pattern), *purely functional data structures* (Okasaki's
"the structure whose operations are the operations you need"),
*stream fusion* (collapse pipelines into single passes). All
converge on the same rule: representation choice is driven by
use pattern, not by what the data looked like at ingress or what
is idiomatic in isolation.

## Burndown priority: infinities first

Not every narrowing has the same payoff. Classify the system's types
into three tiers and address them in priority order:

1. **Effectively unbounded** — raw `String`, unconstrained
   collections, raw integers used as identifiers, anything whose
   representable size is bounded only by hardware. Highest priority.
   Within the tier, precise ordering does not matter; pick any one
   and burn it down to the next tier.
2. **Very large but bounded** — types with a concrete upper bound
   but a large interior (a 100-character string, a 10^6-element
   collection). The narrowing here is to add grammar, ordering,
   uniqueness, or domain-specific predicates that close the gap
   between codomain and range.
3. **Tightly bounded** — closed sets (enums, small numeric ranges,
   constrained structs). Further narrowing yields diminishing
   returns; revisit only when a specific invariant demands it.

Each completed narrowing demotes a type out of its tier and exposes
the next hotspot. The system behaves as a layered sieve: clear the
worst tier first, re-evaluate, then continue. Spending effort on a
tier-3 narrowing while tier-1 hotspots remain is misallocated work.

This burndown recurs at every level the skill addresses. The
type-level form lives here; `documentation.md` § "Burndown
priority" applies the same framework to prose claims;
`ratchet.md` describes the project-tooling instance as a
one-way tightening. Self-similar restatements are deliberate
denormalization for local readability — readers landing in
each module need the tier framing without chasing a reference
elsewhere.

## Encode invariants into types

This is the **mechanism ladder**: concrete mechanisms ordered from
lowest to highest guarantee. It is distinct from the **encoding
order** in `state-space-minimization-formal`, which ranks mechanism
classes (type, constructor, boundary adapter, schema, …) by
construction paths governed and detection phase; this ladder's
upper rungs refine that order's rank 1, and its lowest rungs
(runtime checks, smart constructors) correspond to its later ranks.

Progression ladder, from lowest to highest guarantee:

1. **Runtime checks at use sites** — a single boundary, easy to forget.
2. **Predicative newtypes with private fields and smart constructors** —
   distinguish values that share a representation and keep the invariant
   inside the trusted module. See `constructive-vs-predicative.md` for
   the audit checklist that keeps this layer honest.
3. **Enums for closed sets** — exhaustive matching becomes possible.
4. **Constructive datatypes** — the type's inhabitants are exactly the
   valid values. See `constructive-vs-predicative.md`.
5. **Substructural types** (linear, affine, relevant, ordered) — restrict
   how often a value is used. Affine types power Rust ownership.
6. **Typestate** — encode legal transition paths between states.
7. **Phantom types and proof tags** — record which checks have run
   without changing runtime representation. See `proof-preservation.md`.
8. **GADTs and witness types** — runtime tags that re-establish a static
   type. See `proof-preservation.md`.
9. **Refinement and pattern types** — predicates checked by the type
   checker. See `proof-preservation.md`.
10. **Dependent types** — types may depend on values; the strongest
    practical guarantee available today (Idris, Lean, Agda, F\*).

Use the lightest level that removes the invalid state in practice. Each
rung adds either ergonomic cost, toolchain cost, or both.

## Function signatures as state-space boundaries

Each function signature defines the states that can pass through the call.
Tightening parameter types shrinks the domain at input boundaries.
Tightening return types shrinks the codomain at output boundaries. Read
each signature as a contract: it both promises and demands.

A function's domain narrowing also turns *partial* functions *total* — see
`total-functions.md` for why that matters and where to push the narrowing.

### Three roles a function can play

Every function does one or more of the following at the type boundary
between its inputs and its outputs:

- **Dictates** — imposes structure on the output. Establishes the
  shape that downstream code can rely on.
- **Stipulates** — narrows the set of possible output values by
  adding constraints (length, format, ordering, uniqueness, the
  closing of a sum to a single variant).
- **Eliminates** — makes invalid states unrepresentable past the
  boundary. The strongest of the three: invalid states cannot exist
  in the output type, so no downstream code has to guard against
  them.

A function that does none of these — a passthrough that accepts a
wide type and returns a value of the same wide type — is not
contributing to state-space minimization. Use this lens when
evaluating a decomposition: each new boundary should at least
stipulate, ideally eliminate. A new boundary that only dictates
risks *increasing* the total state space, because it exposes an
intermediate representation broader than the relationships the
original function maintained implicitly.

See `normalization.md` for the dependency-graph treatment of
this rule: passthroughs are the nodes a transitive reduction
collapses, sprawl is the over-decomposition failure, the god
function is the under-decomposition failure.

## Decision rubric

For each invariant, ask:

- How many boundaries cross this invariant?
- What is the cost if invalid data escapes?
- How often is this invariant checked?
- Is the value reused widely across modules?
- Can a simple newtype solve it before typestate is needed?
- Is the invariant stable enough to encode, or is it speculative product
  policy that may change next quarter?

Default guidance:

- First ask whether a constructive representation makes invalid states
  unrepresentable by shape. If yes, use it. If not, use a predicative
  newtype with private fields and a smart constructor, then document
  which hard case forced the fallback.
- Move to enum when the value set is closed.
- Use typestate only when construction or call-order rules are strict.
- Use phantom types or proof tags when marker capabilities are required.
- Reach for refinement, pattern, or dependent types when the toolchain
  supports them and the invariant is rich enough to justify the cost.

## When narrowing is the wrong move

State-space minimization has costs. The goal is constraints that
*eliminate illegal states*, not narrowing for its own sake. Choose a
wider type and document why when:

- the invariant is product policy that may change, not a domain fact
- the invariant requires negation or absence (nonmonotonic predicates) —
  see `ingress-and-boundaries.md` for the capability-token alternative
- the encoded type would be so awkward callers reach for escape hatches
- categories overlap such that a sum type would explode combinatorially
- the value crosses a serialization boundary where the format is
  primitive-only and a separate DTO is the cleaner split
- the narrowing would push complexity into conversion layers without
  eliminating a corresponding region of invalid state
- the narrowing would add boilerplate without reducing real ambiguity
  at any boundary

Reward narrowings that demote a type out of its tier (see *Burndown
priority* above) or that close the codomain-range gap on a function
whose output type is currently broader than its actual range. Do not
reward narrowings that only relabel the same state space under a new
name.

See `constructive-vs-predicative.md` § "Hard cases" for full treatment.
