# Primitive obsession

Primitive obsession is the entry symptom this skill targets. The term comes
from Kent Beck's smell catalogue and Martin Fowler's *Refactoring*: code
that uses primitives (`int`, `String`, `bool`, raw arrays) where a small
domain type would carry the invariant.

Every primitive in a domain signature is a missed opportunity to shrink the
state space. A `String` parameter accepts every Unicode sequence; a
`UserEmail` parameter accepts only what the parser proved is an email.

## Symptoms

- Function signatures whose parameter types do not name the domain concept:
  `fn charge(user: u64, amount: i64, currency: String)`.
- The same primitive used for unrelated concepts: `i64` for `UserId`,
  `OrderId`, `Cents`, `Milliseconds`. Mixups type-check.
- Validation duplicated across call sites: every function that takes a
  `String` re-runs the same regex or length check.
- Constants gathered in one module that callers must remember to consult
  (`MAX_NAME_LENGTH = 64`, `VALID_CURRENCIES = ["USD", "EUR", ...]`).
- Nullable / optional primitives that drop the reason for absence: a
  `Option<String>` for "email if verified" should be a sum type that
  distinguishes verified-and-present from unverified-because-no-attempt
  from unverified-because-bounced.
- Boolean flags whose true/false meaning is policy: `is_admin: bool` is
  primitive obsession around a richer authority model.

## The progression

For each primitive in a signature, walk the ladder:

1. **Primitive** (`String`, `i64`, `bool`) — accepts the entire universe of
   its base type.
2. **Type alias** (`type UserId = i64`) — readable but type-system-weak;
   the alias and the base type unify.
3. **Specialized parsed type from a standard library** when one exactly
   matches the domain — URL, UUID, timestamp, semantic version, money,
   locale.
4. **Predicative newtype with private fields and smart constructor**
   when no exact parsed type exists. Narrows by predicate at
   construction; the invariant lives in the trusted module. See
   `constructive-vs-predicative.md`.
5. **Constructive datatype** when the representation can make invalid
   states impossible by shape. The strongest rung; no constructor to
   trust.

Move directly to the strongest rung the boundary can prove without
harming ergonomics. Do not stop at an alias or generic newtype when a
closed enum, standard parsed type, or constructive shape is available.
Primitive obsession is not solved by adding `type FooId = i64` aliases —
the type system still treats them as the underlying primitive in most
languages.

## Cost of stopping early

Each level the type stays at incurs a tax:

- **Stays at primitive**: every consumer must re-check, mixups go
  undetected, and refactors that change the invariant must touch every call
  site.
- **Stops at alias**: the name is a comment for humans, not a constraint
  for the type checker. Hyrum's Law guarantees callers will come to depend
  on the alias being interchangeable with the underlying primitive.
- **Stops at predicative newtype**: invariant is real but only as strong as
  the trusted module. See `constructive-vs-predicative.md` § "Smart
  constructors are only as strong as their trusted boundary".

## When to leave a primitive alone

Not every primitive is obsession. Leave the primitive when:

- The value has no domain meaning beyond its representation (loop
  counters, array indices over generic data, hash buckets).
- The function is generic over the primitive type and the domain meaning
  belongs to the caller (`fn sort<T: Ord>(xs: &mut [T])`).
- The primitive is at a serialization boundary and a separate DTO is the
  cleaner split (see `ingress-and-boundaries.md`).

The test: can a domain expert read the signature and name what the
primitive represents? If yes, the type should say what the expert says.

## Cross-references

- `constructive-vs-predicative.md` — choosing between newtype and
  constructive datatype for the next rung.
- `boolean-blindness.md` — primitive obsession around `bool` specifically.
- `ingress-and-boundaries.md` — narrowing primitives at the parse step.
- `architectural-scopes.md` — value object as the DDD name for a
  primitive-obsession remedy.
- `languages/rust.md`, `languages/typescript.md` — concrete idioms.
