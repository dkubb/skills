# Rust Code Review Guidelines (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.
- Many rules below have automated-lint candidates documented in
  `../LINT-TODO.md`. Configure stock clippy lints where they cover a rule.

## Primitive obsession (review blockers)

- Go directly to the bounded form (bounded strings and collections with
  both bounds) when constraints exist. `NonEmpty*` proves only the lower
  bound — flag it unless the missing upper bound is recorded with a
  reason. Constructive forms: `state-space-minimization`
  `references/languages/rust.md`.
- Use `NonZero*` over numeric types when zero is invalid.

## Structural review (review blockers)

These patterns are state-space leaks. The `state-space-minimization` skill
covers the rationale; the bullets here are Rust-specific checklist items.

- **Single-variant `pub enum`.** An `enum X { OnlyVariant }` is a struct in
  disguise. Either mark `#[non_exhaustive]` with a doc comment naming the
  reserved future variant, or convert to `pub struct`.
- **Identity-passthrough methods.** Methods like `fn x(self) -> Self { self }`,
  `fn x(self) -> Self { *self }`, or `fn from_x(x: Self) -> Self { x }` are
  shims. They survive past their compatibility purpose. Delete and inline at
  call sites.
- **Constant-returning methods that ignore `self`.** A method that takes
  `self`/`&self` but returns a literal or constant is documentation in
  function form. Either rewrite as an exhaustive match over `self` (so new
  variants force the question) or delete and rely on the type-level proof.
- **Bit-identical type bodies under different names.** Two `enum`s or
  `struct`s with the same fields/variants encode the same state space. The
  distinguishing tag belongs on the success type, not on a parallel error
  or wrapper. Collapse via type alias, generic over the payload, or phantom
  tag.
- **Single-field newtype wrappers with delegated accessors.** A `struct X(Y)`
  or `struct X { f: Y }` whose `impl` only forwards to the inner type is
  documentation. Either carry a real invariant (smart constructor, phantom
  tag, ownership boundary) or replace with a type alias.

## API and ownership

- Avoid needless cloning. Prefer borrowing and explicit lifetimes.
- Use the simplest expression that satisfies the goal. Delegate to existing
  trait implementations rather than converting to a different type first
  (e.g., `self.inner.cmp(&other.inner)` over `self.inner.as_u128().cmp(…)`).
- Prefer `pub(crate)` shorthand over `pub(in crate)` for crate visibility.
- Prefer `use module::*` when importing multiple items from a module; prefer
  explicit imports when only one item is used.
- Keep public APIs minimal. Use the narrowest visibility that works.
- Keep method ordering consistent and easy to scan.
- Do not add `#[inline]` unless data shows it helps or a repo lint requires it.

## Conversions and typing

- Use `TryFrom` only when conversion can fail.
- Prefer established libraries for constrained values (bounded strings and
  collections; `non-empty-string` / `nonempty` for the lower-bound part).
  Avoid hand-rolled equivalents unless extra invariants are required, and
  treat a lower bound alone as an unfinished narrowing.
- Prefer `Option` for optional values unless empty and missing are
  distinct, meaningful states. Avoid bare `String` unless a strict
  validator enforces length and format.
- Use `NonZero*` types when zero is invalid, unsigned types for non-negative
  values, and the smallest integer width that covers all valid values.
- **Explicit integer widths at public boundaries.** Public error variants,
  wire-format types, and serialization boundaries use explicit-width
  integers (`u32`, `u16`, `i64`, etc.). Reserve `usize` for slice indexing,
  pointer arithmetic, `mem::size_of`, and FFI. A bounded count that fits
  `u32` is `u32`, not `usize`, in any public error field.
- When using Serde, require `#[serde(deny_unknown_fields)]` or an explicit
  deserializer that rejects unknown fields.
- For JSON parsing or generation, use `garde` with strong constraints that
  mirror the smart constructor.
- Deserialization must call the smart constructor.
- Goal: no instance exists without the smart constructor.
- Prefer private fields and smart constructors for domain types with
  invariants.
- For each smart constructor, require boundary tests and property tests over
  valid and invalid ranges.
- For SQLx row mapping, implement `FromRow` and delegate to the smart
  constructor.
- Use `garde` constraints for schema generation when possible.
- Use enums for restricted string fields so Rust and DB constraints overlap
  exactly. Treat mismatches as review blockers.
- For unstructured free-form text, enforce printable characters in both Rust
  and DB.
- When a string format is known, enforce it with a regex and bounded length.
- Prefer property tests that cover both valid and invalid ranges; include
  boundary tests for min/max.

## Review focus

- Focus on correctness, safety, API design, tests, docs, and style.
- Do not re-check what Clippy can catch. Require running Clippy instead.
- Lint suppression policy: do not use `#![allow(...)]`. Use
  `#![expect(..., reason = "...")]` instead, with a clear reason — `expect`
  forces suppressions to be removed once they are no longer needed, so
  improvements cannot silently backslide. Treat this as a cargo restriction
  for lint configuration.
- Arithmetic policy (Rust):
  Prefer `checked_*` math. Treat overflow/underflow as an error signal.
  If a clamp-to-zero behavior is required (for example, scanning a substring that may start mid-context), do it explicitly and document why.
- Enforce coverage requirements and the coverage ratchet. Record the exact
  coverage number and do not allow drops unless the user approves.
- Prefer exact `line.eq("...")` checks over trimming or fuzzy matching when
  matching fixed line literals.

## Boundary error handling

- **At correctness boundaries, distinguish I/O error kinds.**
  `unwrap_or(literal)` on `io::Result` swallows real failures
  (`ConnectionReset`, `BrokenPipe`, broken sockets, etc.) as success. At
  any boundary where the result drives a cleanup or disconnect decision,
  match on `error.kind()` and route unknown errors to the failure path,
  not the success path. The test-strictness rule against fallback helpers
  applies in spirit at every correctness boundary.
- **Bounded source, bounded return.** A function whose source is bounded
  (e.g., a fan-out over `ClientListenerSet` capped at N) must not return
  `Vec<T>` and erase the bound at the boundary. Return `impl
  ExactSizeIterator<Item = T>`, or a bounded vec type.

## Concurrency and OS interaction

- **Prefer `Child::kill()` over PID-based signaling.** `child.id()` followed
  by `nix::sys::signal::kill(Pid::from_raw(...), ...)` has a PID-reuse race:
  the child can exit and the OS can reuse the PID between the two calls.
  `tokio::process::Child::kill()` uses the OS handle and is race-free. On
  Linux, `pidfd_*` is the race-free path for explicit signaling.

## Test strictness (Rust-specific)

- Tests should fail on missing data; avoid fallback accessors in tests and test
  helpers.
- Avoid defaulting helpers in tests. Common offenders in std/lib APIs:
  - `Option`/`Result`: `unwrap_or`, `unwrap_or_default`, `unwrap_or_else`,
    `map_or`, `map_or_else`, `or`, `or_else`.
  - `Option`: `get_or_insert`, `get_or_insert_with`, `or_default`.
  - `HashMap::entry`: `or_insert`, `or_insert_with`, `or_default`.
  - `Iterator`: `next().unwrap_or(...)`, `next().unwrap_or_default()`.
  - `bool`/`Option<bool>` patterns like `unwrap_or(false)` that hide missing data.
- Be wary of `Default::default()` in tests when it hides missing data; use it
  only when the default value is the subject of the test.
- Prefer `expect(...)` with a clear message when a value is required.
- Avoid `#[cfg(test)]` outside test modules; when sharing helpers across
  modules, place them in a `test_support` module instead.
- Prefer separate `mod tests` and `mod proptests` modules, so unit and
  integration coverage can be measured independently from property-based
  tests.
