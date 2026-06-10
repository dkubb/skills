# Constructive vs predicative representations

The deepest split in state-space minimization. Two ways to make invalid
states impossible:

- **Predicative**: keep a wider type and exclude invalid values with a
  predicate (smart constructor, validator, refinement). The invariant
  lives in the trusted module that owns the constructor. Alexis King calls
  this *extrinsic* safety: the type's name says "valid", but the type
  itself cannot enforce that without help.
- **Constructive**: choose a representation whose inhabitants are exactly
  the valid values. The invariant lives in the shape of the type, not in a
  check. King calls this *intrinsic* safety.

Prefer constructive when feasible. The proof carries through every
consumer without any trusted module to audit, exhaustive pattern matching
becomes possible without `_ => unreachable!()` arms, and refactors cannot
weaken the invariant by accident.

## Examples

```
predicative: smart constructor enforces 1..=5
  type RetryCount(value)
    new(v): require 1 ≤ v ≤ 5 then RetryCount(v) else error

constructive: only five inhabitants exist; no constructor to trust
  enum RetryCount = One | Two | Three | Four | Five
```

```
predicative: a non-empty list via runtime check
  type NonEmptyList[T](Vec[T])
    new(xs): require xs.len() ≥ 1 then NonEmptyList(xs) else error

constructive: head + tail; emptiness is unrepresentable by shape
  type NonEmptyList[T] = { head: T, tail: Vec[T] }
```

When the constructive shape is right, the predicative version starts to
feel silly: there is nothing for the smart constructor to check.

The split has a precise formal counterpart in the shared
vocabulary (`principles.md` § "Formal vocabulary"). The
reachable-invalid set under a boundary constructor `b` is
`I_reach(A,b) = R(b) ∩ I_repr(A)`. A predicative encoding has
`S(A) ⊋ C(A)`: the smart constructor narrows `R(b) ⊆ C(A)`, so
`I_reach` is empty, but `I_repr` remains representable. A
constructive encoding has `S(A) = C(A)`, so `I_repr` is empty by
shape. A mechanism swap from one predicative encoding to another
(hand-written smart constructor to refinement-library wrapper, say)
holds `S` and `R(b)` constant and is therefore not a state-space
narrowing — only a move toward `S(A) = C(A)` is. See
`state-space-minimization-formal` for the full calculus, including
§ "Constructive vs predicative" and § "Constructive dominance at
rank 1".

## Smart constructors are only as strong as their trusted boundary

A predicative newtype proves the invariant *only* if no other path
constructs the type. The newtype's name does not enforce anything; the
module's API does. Audit every construction path:

- public field access — fields must be private
- pattern destructuring with rest-binding — keep visibility tight
- default constructors, copy/clone, conversion traits, derive macros —
  derived impls can bypass the constructor
- deserialization — by default constructs by field, not by constructor
- database row mapping, FFI, raw byte casts, unchecked casts
- builder structs whose `.build()` skips the constructor
- test-only constructors that exist for convenience and leak

Treat the module that owns the type as the audit unit. Test the trusted
boundary directly — feed it adversarial input, deserialize hostile
payloads, check that derived impls preserve the invariant — not just the
happy callers.

For deserialization specifically, prefer one of:

- a "try-from-wire" hook so deserialization always goes through the
  constructor
- a separate DTO struct that mirrors the wire format, then a fallible
  conversion that calls the smart constructor
- never derive a deserializer on the domain type at all and parse
  explicitly at the boundary

The same warning applies to any other deserializer in the call graph: env
readers, CLI parsers, database row mappers, RPC bindings, FFI shims. A
single field-by-field bypass invalidates the proof for every downstream
consumer. See `languages/rust.md` and `languages/typescript.md` for the
concrete patterns in each ecosystem.

## Hard cases for constructive modeling

Before choosing a predicative type, run this audit and record the first
failing condition. The recorded failure becomes the documented reason
for the fallback.

- **Ergonomics**: separate sub-types per case would force callers to
  write one branch per shape even when the operation is uniform, or
  the encoded type would push callers toward escape hatches.
  Fallback: predicative newtype with smart constructor.
- **Overlapping categories**: valid values fall into multiple categories
  that interact and a sum type would explode combinatorially.
  Fallback: predicative newtype or capability token.
- **Nonmonotonic predicates**: validity depends on absence ("no overlap
  between favorites and blocked", "not banned and not rate-limited") so
  adding information could invalidate the value.
  Fallback: encode the smaller monotonic facts and combine them at the
  operation site with a capability token or typestate. See
  `ingress-and-boundaries.md` § "Capability tokens".
- **Invalid vs undesirable**: the constraint is product policy,
  performance budget, or stylistic preference, or the program needs to
  represent transient invalid states (half-filled form, draft order) on
  the way to validity.
  Fallback: enforce in lints, tests, or runtime configuration; or
  represent the in-progress shape as a separate draft / workflow type.

When the audit forces a predicative type, document the failing condition
in a comment next to the type. The trusted module still owns the
invariant; the comment is what tells the future reader why the
constructive shape was rejected.

## Cross-references

- `principles.md` § "Encode invariants into types" — the full ladder.
- `proof-preservation.md` — refinement, pattern, and dependent types
  bring constructive-grade guarantees to predicative-style code by making
  the type checker enforce the predicate.
- `ingress-and-boundaries.md` — capability tokens for nonmonotonic
  predicates.
- `architectural-scopes.md` — value object as the DDD-tradition name for
  the predicative newtype.
