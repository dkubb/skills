# Rust

Rust idioms for state-space minimization. Read together with the
language-agnostic principles files; this file gives the concrete shapes
and crate names.

## Newtype with smart constructor

The default Rust shape for primitive obsession and the predicative side
of `constructive-vs-predicative.md`.

```rust
pub struct Port(NonZeroU16);

impl Port {
    pub fn new(v: u16) -> Result<Self, PortError> {
        NonZeroU16::new(v).map(Self).ok_or(PortError::Zero)
    }

    pub fn get(&self) -> NonZeroU16 { self.0 }
}
```

Rules to keep the trusted boundary honest:

- field is private (no `pub`)
- module does not derive `Default` unless the default is provably valid
- `From<u16>` is *not* derived; only the fallible `TryFrom`/`new` exists
- `serde::Deserialize` is *not* derived directly — see below

## Prefer explicitly-sized integer types

For domain values — counts, IDs, bounded indices, serialized
numbers — use the narrowest explicitly-sized integer type the
range fits: `u8` / `u16` / `u32` / `u64` or their signed
counterparts. Reserve `usize` and `isize` for slice indexing,
pointer arithmetic, `mem::size_of`, and FFI — places where the
value genuinely is a machine word.

`usize` is platform-dependent (32 bits on a 32-bit target,
64 bits on a 64-bit target). For any value that is not a machine
address, that platform dependency is a state-space leak:

- The admitted value set differs across targets. A 32-bit build
  rejects values a 64-bit build accepts; cross-compilation drift
  becomes silent.
- Serialization is non-portable. The width choice belongs in the
  type, not in the codec.
- The type misnames the semantic. A retry count, a user ID, a
  rate-limit window — none of those are addresses. `usize`
  widens the codomain to a meaning the value does not carry.

In every signature that crosses a module boundary, an FFI
boundary, a serialization codec, or a public API, the integer
width should be explicit. `usize` appears only where the
underlying contract genuinely is "machine word." Apply
*Bounded primitives* below to narrow the explicit width further.

This rule is borrowed from TigerBeetle's TigerStyle, which states
it for the same reasons.

## Bounded primitives

For ranges with both ends bounded:

```rust
pub struct RetryCount(u8);

impl RetryCount {
    pub fn new(v: u8) -> Result<Self, RetryCountError> {
        match v {
            1..=5 => Ok(Self(v)),
            0 => Err(RetryCountError::Zero),
            _ => Err(RetryCountError::TooLarge),
        }
    }
}
```

Constructive alternative when the range is small:

```rust
pub enum RetryCount { One, Two, Three, Four, Five }
```

For wider ranges where an enum is impractical, `bounded-integer` gives
type-level integer ranges:

```rust
use bounded_integer::BoundedI32;
pub type RetryCount = BoundedI32<1, 5>;
```

## `nutype` for the full predicative package

`nutype` is the most direct way to get newtype + sanitization +
validation + serde-safe derives + private fields in one declaration:

```rust
use nutype::nutype;

#[nutype(
    sanitize(trim, lowercase),
    validate(not_empty, len_char_max = 254, regex = EMAIL_RE),
    derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Serialize, Deserialize),
)]
pub struct Email(String);
```

`nutype` emits `Email::new` and routes `Deserialize` through the same
validator, so the trusted-boundary audit shrinks to "do we use
`#[nutype]` or do we have a hand-rolled type?"

Alternatives by niche:

- `validator` / `validator_derive` — annotation-style validators on
  fields; weaker than `nutype` because the proof does not carry in the
  type, only the annotation.
- `derive_more` — `From`/`Into`/`Display`/`AsRef` on newtypes without
  losing the smart-constructor invariant; reduces friction that pushes
  callers off the newtype path.

## Bounded collections

- `[T; N]` — fixed-length array, length is part of the type.
- `arrayvec::ArrayVec<T, N>` — vec with compile-time max capacity, no
  heap allocation.
- `heapless::Vec<T, N>` — same shape, embedded-friendly.
- `tinyvec` — small-vec optimization with bounded variant.

For lower-bound enforcement (non-empty), Rust has no canonical type in
std; `nonempty` and `nonempty-collections` cover most needs. Combine an
upper-bounded type with a lower-bound smart constructor when both
matter; do not lose the non-empty proof while adding the maximum.

For length-indexed types (vector whose length is in the type), const
generics + `[T; N]` is the simplest path; `generic-array` and
`typenum`-based crates exist for finer control.

## Boundary parsing: serde

Default `#[derive(Deserialize)]` on a domain type bypasses the smart
constructor by setting fields directly. Prefer one of:

```rust
// option 1: try_from a wire shape; constructor runs on every parse
#[derive(Deserialize)]
#[serde(try_from = "EmailWire")]
pub struct Email(String);

#[derive(Deserialize)]
struct EmailWire { address: String }

impl TryFrom<EmailWire> for Email {
    type Error = EmailError;
    fn try_from(w: EmailWire) -> Result<Self, Self::Error> {
        Email::new(w.address)
    }
}
```

```rust
// option 2: separate DTO at the boundary, never derive on the domain type
#[derive(Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub age: u32,
}

pub struct CreateUser {
    pub email: Email,
    pub age: AdultAge,
}

impl TryFrom<CreateUserDto> for CreateUser {
    type Error = ValidationError;
    fn try_from(d: CreateUserDto) -> Result<Self, Self::Error> {
        Ok(Self {
            email: Email::new(d.email)?,
            age: AdultAge::new(d.age)?,
        })
    }
}
```

`serde_with` provides `#[serde_as(as = "TryFromInto<Wire>")]` and
per-field validation hooks for the same pattern with less boilerplate.

The same warning applies to: `clap` derive, `envy`/`figment` env
parsers, `sqlx::FromRow`, protobuf code generators, `bincode`,
`rmp-serde`, FFI shims. Audit each binding for field-by-field bypass.

## Closed sums with `enum`

Rust enums are the workhorse for the constructive side. Exhaustiveness
checking gives the compiler-enforced "every case covered" invariant; an
`_ => unreachable!()` arm is a state-space leak.

```rust
pub enum Availability {
    Available,
    Offline,
    Busy { until: Instant },
    Banned { since: Instant, reason: BanReason },
}
```

For interop with foreign data where the value set may grow, mark the
enum `#[non_exhaustive]` so external matches force a wildcard arm —
that is a deliberate widening for forward compatibility.

## Phantom tags (Ghosts of Departed Proofs)

```rust
use std::marker::PhantomData;

pub struct Checked<Tag, T>(T, PhantomData<Tag>);

pub struct Sanitized;
pub struct AuthorizedFor<U>(PhantomData<U>);

pub fn sanitize(input: RawSql) -> Checked<Sanitized, Sql> { todo!() }

pub fn authorize<U>(q: Checked<Sanitized, Sql>, user: &U)
    -> Checked<(Sanitized, AuthorizedFor<U>), Sql> { todo!() }

pub fn run<U>(q: Checked<(Sanitized, AuthorizedFor<U>), Sql>) -> Rows {
    todo!()
}
```

For multiple proof tags on one value, prefer a tuple of zero-sized
markers over a custom marker enum: composition is cheap and the type
reads as the conjunction of proofs.

## Typestate (legal call orders)

Hand-rolled with phantom types when the API surface is small:

```rust
pub struct Connection<S>(PhantomData<S>);
pub struct Disconnected;
pub struct Connected;

impl Connection<Disconnected> {
    pub fn new() -> Self { Self(PhantomData) }
    pub fn connect(self) -> Connection<Connected> { Connection(PhantomData) }
}

impl Connection<Connected> {
    pub fn send(&self, msg: &[u8]) { todo!() }
    pub fn close(self) -> Connection<Disconnected> { Connection(PhantomData) }
}
```

Crates when the state machine is large enough to warrant generation:

- `typestate` — proc-macro DSL that derives the phantom-type encoding
  from a state-machine declaration.
- `state_machine_future` — older, async-flavored.
- `sm` — declarative state-machine macro.

For protocols (two-party communication), prefer session-types crates:

- `ferrite-session` — judgmental embedding of binary session types in
  stable Rust.
- `rumpsteak` — multiparty session types in async Rust.
- `sesh` — alternative binary-session embedding.

## Capability tokens

Affine ownership makes capability tokens natural in Rust: a token is
just a value the caller owns, and the type system enforces "used at
most once" without any extra machinery.

```rust
pub struct CanPost { /* unforgeable */ }

impl CanPost {
    fn issue(user: &User) -> Result<Self, RejectReason> {
        if user.banned { return Err(RejectReason::Banned); }
        if user.rate_limited { return Err(RejectReason::RateLimited); }
        Ok(Self {})
    }
}

pub fn post(_token: CanPost, content: Post) -> PostId {
    // post is uncallable without a fresh CanPost
    todo!()
}
```

For ambient-authority elimination at the OS level, `cap-std` replaces
the default filesystem and network APIs with capability-typed versions.

## Refinement and verification

The 2022-2025 wave of refinement-type and verification tools for Rust:

- **Flux** — liquid types layered on Rust; PLDI 2023.
- **RefinedRust** — Coq-foundationally-verified refinement type system
  including unsafe; PLDI 2024.
- **Verus** — SMT-backed lightweight verification with
  `requires`/`ensures` clauses and ghost code; OOPSLA 2023. Currently
  the most production-leaning entry.
- **Aeneas** — Rust-to-functional translation for proof in F\*, Coq,
  HOL4, Lean; ICFP 2022.
- **Creusot** — deductive verification through translation to WhyML;
  FMSE 2022.
- **Thrust** — prophecy-based refinement types; PLDI 2025.

None is yet stable in stock Rust. Use them when the problem domain
justifies the toolchain cost; otherwise smart constructors plus tight
modules are the practical equivalent.

## Test matchers in Rust

See `references/testing.md` for the language-agnostic principle.
Rust-specific matcher patterns:

```rust
// weak: spot-check
assert_eq!(user.id(), expected_id);

// strong: full structural equality
assert_eq!(
    user,
    User {
        id: expected_id,
        role: Role::Admin,
        email: expected_email,
    }
);
```

```rust
// weak: tag check only
assert!(result.is_some());

// strong: tag and exact payload
assert_eq!(result, Some(Expected::Value));
```

```rust
// weak: contains_key
assert!(value.as_object().unwrap().contains_key("id"));

// strong: full key set
assert_eq!(
    value.as_object().unwrap().keys().collect::<BTreeSet<_>>(),
    BTreeSet::from(["id", "name", "role"]),
);
```

For mocks, `mockall`'s `expect_*().times(n).withf(|...| ...)` lets you
constrain count, order, and arguments. Prefer `times(1)` and
`with(eq(...))` over `withf` predicates; predicates re-introduce
matcher slack.

For property tests, use `proptest` or `quickcheck`. Build `*_valid` and
`*_invalid` strategies independently; assert canonical output, not just
acceptance/rejection.

For mutation testing, `cargo-mutants` measures whether the test suite
catches injected mutations. A test that passes against the mutant is a
test that accepts a wider state space than the contract allows — the
matcher is not strict enough.

## Cross-references

- `principles.md`, `constructive-vs-predicative.md`,
  `proof-preservation.md` — the principles these idioms apply.
- `ingress-and-boundaries.md` — boundary parsing applied across all
  Rust deserializers.
- `testing.md` — strict-matcher principle, applied here in Rust.
- `external-integration` skill — anti-corruption layer construction in
  Rust.
