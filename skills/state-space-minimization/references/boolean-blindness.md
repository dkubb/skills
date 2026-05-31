# Boolean blindness

A boolean return or parameter discards the reason behind the answer.
The function had to distinguish multiple input conditions to produce
its result; the return type erases those distinctions back to one bit.
In `principles.md` vocabulary, a boolean output has a large *preimage*
— many different input classes map to the same `true` or the same
`false` — and the call site cannot recover which class it received.
Two callers handling different domain conditions look identical in
code, and the missing distinction gets reconstructed downstream with
`if`/`else` chains over scattered state.

The term comes from Bob Harper's blog post "Boolean Blindness" (2011),
attributing the coinage to Dan Licata: "the only thing you can do
with a bit is to branch on it, and pretty soon you're lost in a
thicket of if-then-else's."

## Symptoms

- Predicate functions named `is_*`, `has_*`, `can_*` whose callers
  immediately branch on the result and need to know *why*.
- Boolean parameters whose names do not appear at the call site
  (`send(msg, true, false)`).
- Pairs or trios of boolean fields that should be a single sum type
  (`is_loading`, `has_error`, `is_complete`).
- Long chains of `if x { ... } else if y { ... } else { ... }` that
  reconstruct a sum type from scattered booleans.
- Booleans returned from a function whose name suggests a richer answer
  (`fn lookup_user_state() -> bool`).

## Replace with richer types

Replace the boolean with the answer it was projecting away.

```
boolean blind: caller cannot tell why the user is unavailable
  fn is_available(user) -> bool

richer: caller pattern-matches the reason
  enum Availability {
    Available,
    Offline,
    Busy { until: timestamp },
    Banned,
  }
  fn availability(user) -> Availability
```

```
boolean blind: call site reads send(msg, true, false) with no context
  fn send(msg, urgent: bool, sign: bool)

richer: each axis is its own type and shows up at the call site
  enum Priority { Normal, Urgent }
  enum Signing { Unsigned, Signed }
  fn send(msg, priority: Priority, signing: Signing)
```

The richer return type is *constructive* state-space minimization on the
codomain: the function returns proof of the case, not a one-bit
projection of it.

## When a boolean is fine

A `bool` is fine when:

- the answer is genuinely binary and the call site never needs the reason
- the predicate is a derived view over data the caller already has
  (`xs.is_empty()`, `n > 0`)
- the boolean is an input to a generic combinator that does not care about
  the reason (`xs.filter(|x| ...)`)

Audit every other boolean. Most "is this X?" predicates in domain code
deserve to return the typed answer instead of a one-bit projection.

## Field-level boolean blindness

Boolean fields are the same defect at rest. If a record contains
`is_verified: bool` plus `verified_at: Option<Timestamp>`, the type
admits the invalid state `is_verified = true, verified_at = None`. The
constructive fix is a sum type:

```
loose: invalid combinations representable
  type Email = {
    address: String,
    is_verified: bool,
    verified_at: Option<Timestamp>,
  }

constructive: only valid combinations representable
  enum Email {
    Unverified { address: String },
    Verified   { address: String, at: Timestamp },
  }
```

This is the canonical Wlaschin "Designing with Types" move and the
Feldman "Making Impossible States Impossible" move applied at the field
level.

In `normalization.md` vocabulary, the record-with-bool form carries a
*redundant atom*: verification status appears in two places (the
`is_verified` flag and the presence of `verified_at`), and the
disagreement state between the two copies is the invalid combination
the type admits. The sum-type form eliminates the redundancy —
verification status lives in exactly one place (the variant tag), and
the constraint between flag and timestamp becomes structural rather
than enforced by a separate check.

## Cross-references

- `principles.md` § "Shrink the codomain" — boolean blindness is the
  most common codomain-shrinking opportunity.
- `constructive-vs-predicative.md` — replacing a record-with-bool with
  a sum type is constructive modeling.
- `architectural-scopes.md` — DDD calls the resulting sum type a
  *choice type* or value object.
- `history-and-lineage.md` — Harper's blog, Licata's coinage,
  Wlaschin's series, Feldman's talk.
