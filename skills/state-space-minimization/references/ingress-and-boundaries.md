# Ingress and boundaries

Type-system narrowing protects code in a closed world. The world's edge —
network sockets, files, subprocess output, FFI, env vars, CLI args, user
input, database rows, message queues — is open. Every byte that crosses
that edge has the wide type the network or filesystem hands you. The
ingress parser is where wide input becomes a narrow domain type, and it
is the single place where state-space discipline is non-negotiable.

This file covers four related techniques: parsing at the boundary,
restructuring data so the constraint disappears, capability tokens for
authority, and time-varying invariants.

## Closed world vs open world

Refinement types, dependent types, and smart constructors only protect
what is *inside* the closed world your type checker reasons about.
Anything reaching the program from outside arrives as bytes,
unstructured strings, or wire-shaped DTOs. Even a fully refinement-typed
codebase needs an explicit boundary parser; the type checker cannot
verify what it cannot see.

This is the strongest single argument for parse-at-the-boundary discipline:
the world will hand you invalid data eventually, and your only defense is
the ingress narrowing layer.

## Parse at the boundary

At every ingress, parse to the narrowest type the boundary can prove.
*Parse, don't validate*: parsing returns a typed proof; validation
returns a boolean and discards the proof.

Boundaries to audit:

- network input (HTTP requests, RPC calls, websocket messages)
- file and storage reads (config files, blob storage, database rows)
- environment variables and command-line arguments
- subprocess output and FFI return values
- message queue payloads and event-bus deliveries
- user input (form fields, CLI prompts, terminal sequences)
- timer and clock reads (large or unbounded values are a vector)

For each boundary, define a wire-shaped DTO that matches what the
external system actually sends, then convert through a narrowing parser
into the domain type. Never derive a deserializer directly on the domain
type — that bypasses the narrowing parser. See
`constructive-vs-predicative.md` § "Smart constructors are only as
strong as their trusted boundary".

## Restructure data to remove the constraint

When stacked refinements explode (`NonEmpty AND Sorted AND
AllPositive AND Unique`), the answer is often *not* more refinement
machinery. It is restructuring the representation so the constraint
disappears.

Examples:

- store **deltas** instead of absolutes when monotonicity matters: a
  sequence of `i32` deltas is monotonic by construction; a sequence of
  cumulative values needs an invariant
- store the **canonical** form and derive the others: keep radius, derive
  diameter; keep UTC instant, derive local time
- normalize **redundant copies**: if "user is admin" appears in three
  records, drift is inevitable. Pick one source of truth and derive the
  rest. This is database normalization applied to in-memory state.
- replace a **constraint between fields** with a single field of the
  derived type: `start_date: Date, end_date: Date, end_after_start: ()`
  becomes `range: DateRange` where `DateRange::new` enforces the
  constraint

The principle: most invalid states arise from the *ability to express
the same fact two different ways*. Eliminate the duplication and the
invariant becomes structural.

These four moves are the canonical normalization techniques applied
to runtime data, in `normalization.md` § "Shrink the domain — eliminate
redundancy" vocabulary: deltas-instead-of-absolutes is *derive don't
store*; canonical-form-and-derive is *canonical form*; deduplicating
across records is *single source of truth*; collapsing
constraint-between-fields into one derived field is the same
redundant-atom elimination at the field level. The point is the same
under either framing: the disagreement state between two copies of
one fact is the invalid combination, and removing the second copy
removes that state from the space the system can represent.

## Capability tokens

Some invariants are nonmonotonic: validity depends on absence ("user is
not banned", "rate limit not exceeded", "session not yet expired").
These cannot be encoded into a single static type because adding
information can invalidate them. Encoding nonmonotonic predicates as
data leads to combinatorial explosions or types that lie.

The standard answer is a **capability token**: a one-shot proof that all
checks passed at the moment of issuance. The operation accepts the
token, not the underlying facts. Make capability tokens affine by default — used at most once — so they
cannot be cached past their validity window. A reusable token requires
an explicit validity model: expiration, revocation, scope, replay
semantics, and tests proving stale or replayed tokens fail.

```
fn try_post(user, content) -> Result<CanPost, RejectReason> {
    require !user.banned
    require !user.rate_limited
    require user.session_active
    Ok(CanPost { ... })
}

fn post(token: CanPost, content) -> PostId
// post is uncallable without a fresh CanPost
```

Capability tokens are the bridge between predicative checks (which
return booleans) and constructive type discipline (which carries the
proof in the type). The check happens once; downstream code consumes
the token.

This pattern generalizes to:

- authorization tokens (`AuthorizedFor<Resource>`)
- transaction handles (`InTransaction`)
- rate-limit grants (`RateLimitGrant`)
- request-scoped capabilities (logging context, request ID, tenant ID)
- resource handles (file descriptors, connections, leases)

The object-capability model in security uses the same mechanism at the
language level: authority equals possession of an unforgeable reference,
so absence of a reference makes a whole branch of behavior
unrepresentable.

## Temporal validity

Some invariants are *not statically true* — they were true at the time
of the check but may become false before the value is used. Examples:

- "this address is valid for this basket" — true at checkout, may
  expire before payment
- "this session token is live" — true at issue, expires at TTL
- "this lease is held" — true at acquire, lost on timeout
- "this price is current" — true at quote, stale after seconds

A static parse cannot capture this; the proof must be re-validated at
each use, or carried by an affine token whose lifetime matches the
validity window.

Patterns:

- short-lived capability tokens with explicit deadlines
- re-parse at use rather than caching a stale narrowed value
- typestate transitions that "consume" the proof when the invariant is
  spent
- versioned references (a typed `Version<T>` whose use re-checks against
  the current version)

The defect to watch for: caching a parsed value past its temporal
validity window. The type still says "valid", but the world has changed.

## Effect systems

State-space minimization in the *codomain of behavior*, not just the
codomain of return values. An effect system tracks which side effects a
function may perform.

In an enforced effect system, absence of an effect annotation means the
function *cannot* perform that effect. In TypeScript and Rust this only
holds inside disciplined APIs — Effect-TS, capability-passing services,
Rust ownership and capability wrappers, or explicit trait boundaries.
Plain functions in those languages do not carry that proof; the absence
of an effect annotation in an idiomatic Rust or TS signature is silence,
not a guarantee.

```
// Koka-style: function declares its effects
  fn compute(x: Int) : <total>            Int     // pure
  fn read_file(p: Path) : <io>            String  // may do I/O
  fn maybe_fail(x: Int) : <exn>           Int     // may throw
```

The function's *behavior* state space is bounded by its effect signature.
A function typed `<total>` cannot do I/O at all — that prunes an enormous
behavioral state space at the type level.

Languages with effect tracking: Koka, Eff, Frank, OCaml 5 (handlers),
Scala 3 (capabilities), Roc. Rust has informal effect tracking through
trait bounds (`Send`, `Sync`, `?Send`) and `async`/`unsafe`. Effect-TS
brings effect tracking to TypeScript via the `Effect<A, E, R>` type —
see `languages/typescript.md`.

Capability-passing styles (object capabilities, Effect-TS service
injection, `cap-std`'s capability-safe filesystem) are the practical
realization of effect typing in mainstream languages today.

## Cross-references

- `principles.md` § "Remove invalid intermediate representations" —
  parsing as the boundary technique.
- `constructive-vs-predicative.md` — the trusted-boundary audit applies
  per ingress point.
- `proof-preservation.md` — affine types and phantom tags as the
  capability-token machinery.
- `architectural-scopes.md` — anti-corruption layer as the DDD name for
  the boundary parser.
- `languages/rust.md` § "Boundary parsing" — `serde(try_from)`, `nutype`,
  `cap-std`.
- `languages/typescript.md` § "Schema-first parsing" — Zod, Effect
  Schema, ArkType.
- `normalization.md` § "Shrink the domain — eliminate redundancy"
  — restructure-to-remove-the-constraint is one of the canonical
  normalization moves; the dependency-graph treatment of the
  same rule.
