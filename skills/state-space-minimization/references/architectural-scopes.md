# Architectural scopes

Type-level state-space minimization is the core of this skill, but it
applies at four nested scopes. Each scope is the unit at which
"representable states ≈ valid states" is measured. The Domain-Driven
Design tradition (Evans 2003, Vernon 2013) names these scopes; pulling
the names in lets the skill cross over to the OO/enterprise tradition
and keeps reviews from arguing about which level the invariant lives
at.

## Value object — the field-level scope

A **value object** is a primitive-replacement type with no identity:
equality is by value, the type is immutable, and the constructor enforces
the invariant. `EmailAddress`, `MoneyAmount`, `DateRange`, `UserId` are
all value objects.

Every newtype-with-smart-constructor in this skill is a value object
under DDD vocabulary. Bergh Johnsson's *Secure by Design* sharpens the
term to **domain primitive**: a value object whose explicit purpose is
to carry a security-critical invariant.

The state space is "valid values of this single concept."

## Aggregate — the multi-field scope

An **aggregate** groups one or more value objects under a single
**aggregate root** that owns the cross-field invariants. The root is
the only entry point for modification; all internal invariants are
re-checked atomically when the root accepts a command.

The state space is "valid combinations of these fields, considering
constraints between them." A field-level value object cannot enforce
"start_date < end_date" on its own; the aggregate root can.

When this skill talks about "remove invalid intermediate
representations" or "restructure to remove the constraint between
fields", the aggregate root is the natural place to put the enforcement.

## Bounded context — the system-level scope

A **bounded context** is the linguistic and architectural region within
which a domain term has one consistent meaning. "Customer" in the
billing context and "Customer" in the support context are usually
different types with different valid-state spaces; trying to share the
type erases the narrowing each context needs.

The state space is "what the domain term means here." A single global
"User" type that handles every concern accumulates fields, optional
flags, and "valid only in some contexts" invariants — a textbook
state-space explosion.

State-space minimization is per-bounded-context. Cross-context flow goes
through translation, not type sharing.

## Anti-corruption layer — the boundary scope

An **anti-corruption layer (ACL)** is the translation surface between
two bounded contexts (or between your domain and an external system
whose model you do not control). The ACL parses the foreign DTO, runs
all narrowing checks the boundary can prove, and emits the domain type.

This skill calls the same idea "boundary parsing" or "ingress parsing"
in `ingress-and-boundaries.md`. The ACL is its DDD name.

The agent-skills repo has a separate `external-integration` skill that
treats ACL design in depth. For state-space purposes the rule is
simple:

- the ACL owns the wire-shaped DTO
- the ACL is the only place that knows the foreign vocabulary
- the ACL produces narrow domain types and rejects everything else
- never let foreign types leak past the ACL

## Choosing the scope to narrow

For each invariant, ask which scope owns it:

- **Single field** → value object with smart constructor or constructive
  type
- **Cross-field within one concept** → aggregate root with constructor
  that takes the whole tuple
- **Cross-aggregate** → either lift the invariant into the aggregate by
  redrawing the boundary, or accept that it is a *workflow* invariant
  enforced by the operation, not the data
- **Cross-context** → translation through an ACL, with each context's
  type carrying only its own invariants
- **Across the world's edge** → ingress parser at the boundary; see
  `ingress-and-boundaries.md`

Putting the invariant at the wrong scope (e.g. enforcing a workflow rule
inside a value object) leads to types that lie or callers that have to
re-check at every use.

## Choice types and workflow types

Wlaschin's "Designing with Types" series adds two terms worth borrowing:

- **Choice type**: an aggregate expressed as a sum (`VerifiedEmail |
  UnverifiedEmail`) instead of a record-with-flags
  (`Email { is_verified, verified_at }`). The choice-type form makes
  invalid combinations unrepresentable; see `boolean-blindness.md`.
- **Workflow type**: distinct input and output types per workflow step,
  so each step has its own minimized state and the type tells the order
  of operations. Maps onto typestate when the order is strict and onto
  separate command/event types when the workflow is event-driven.

## Cross-references

- `principles.md` § "Encode invariants into types" — value object and
  aggregate root sit on the smart-constructor and typestate rungs
  respectively.
- `ingress-and-boundaries.md` — ACL is the parse-at-the-boundary
  technique under its DDD name.
- `external-integration` skill — deeper treatment of ACL/Gateway
  design.
- `history-and-lineage.md` — Evans 2003, Vernon 2013, Bergh Johnsson
  2019, Wlaschin 2018.
