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
- Off hot paths, prefer a `.clone()` over clever borrowing when the borrow
  costs more human reasoning than the copy. No reflexive `Cow` or `Arc`;
  `Arc` is a concurrency primitive, not a sharing convenience.
- Use `LazyLock` for repeatedly computed constants.

## Conversions and typing

- Use `TryFrom` only when conversion can fail; otherwise `From`.
- Route every construction path through one shared private `parse` (used by
  `new`, `FromStr`, and `TryFrom<String>`) that returns the value, or its
  normalized form where the domain defines one, rather than a boolean check.
  Never provide `From<inner>` for a validated type; it bypasses the parse.
- Accept exactly what the system emits, nothing more. No trimming in
  `FromStr`, no alternate encodings, no tolerant parsing "just in case".
  Canonicalizing a domain-defined equivalence is different: where the domain
  says two spellings are one value (hex case in a sha or uuid, the
  case-insensitive domain part of an email), normalizing to one canonical
  form at the parse boundary is fine. What stays banned is tolerance the
  domain does not define.
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
- Bind deserialization to the validator: `#[serde(try_from = "String")]` for
  validated types, `#[serde(transparent)]` only for constructive ones.
  Derived serde that skips validation is a bug.
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

## Derives

- Every derived trait must earn its keep: a concrete use site, a supertrait
  obligation (`Clone` for `Copy`), or a denied-lint obligation
  (`derive_partial_eq_without_eq` forces `Eq` onto exported `pub` types that
  derive `PartialEq` where `Eq` is derivable; `pub(crate)` and private types
  are outside its reach). Reflexive full lists
  (`Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd`) widen a type's
  contract for free and hide which capabilities the code relies on.
- Marker structs used as type parameters are not an exception. A generic
  type's derives bound `K: Trait` lazily, so the marker needs only the traits
  behind capabilities callers exercise: a `HashMap` key needs `Hash + Eq`,
  an `assert_eq!` needs `PartialEq + Debug`, reuse after a move needs `Copy`.
- Tests count as use sites; speculative future needs do not. A dropped derive
  is a ratchet: the first real use fails the build and re-earns it visibly.
- To trim, remove the suspect traits and let the compiler and denied lints
  adjudicate — restore exactly what they demand, nothing more.
- Sort each derive list alphabetically (ASCII order, so path-qualified
  derives like `derive_more::Display` sort after the capitalized std traits).
- Omit `#[display("{_0}")]` on a single-field `derive_more::Display` newtype;
  forwarding to the sole field is the derive's default. Write a `#[display]`
  attribute only when the rendering differs from that default.
- Flag a hand-written trait impl that only forwards to a field (`Display`,
  `AsRef`, `Into`) when a `derive_more` derive expresses it; keep manual impls
  only where behavior must route through validation (`FromStr` and
  `Deserialize` through the smart constructor).

## Function structure

Guards first, produce last; the body between reads as paragraphs.

- **Guards first and flat.** Reject early with `let`-`else`, `?`, and early
  `return`; each guard un-nests everything after it. `return` appears only in
  guards — the happy path exits as the function's tail expression, un-nested.
  Anyone reading bottom-up knows the last expression is the answer.
- **One phase per stanza:** gather → decide → produce, blank lines between —
  the production mirror of AAA (arrange–act–assert).
- **Bind at stanza boundaries.** A chain is one sentence: chain freely within
  a stanza, but end each stanza in a named `let` — a binding is documentation
  a chain can't provide. Wanting a comment mid-chain marks the split point.
- **Name new meanings, shadow refined ones.** `let input = input.trim();`
  beats `trimmed_input`; distinct names are for distinct concepts, not steps
  of one value's pipeline.
- **Nesting past two levels means a guard, a name, or a new function is
  missing — in that order of preference.**
- **Prefer named destructuring** (`split_first`, `let`-`else`) over positional
  tricks (`split_at(1).1`, `index > 0` flags) when lints permit both.
- **Lines are free; stanzas aren't.** Complexity thresholds bound branching,
  not line count — but a fourth stanza is a second responsibility, and a
  stanza needing a comment header is a function asking for that comment as
  its name.
- The branching bound is `cognitive_complexity` at the configured threshold;
  this section explains its intent.

## Architecture and modules

- Traits are rare. Define one only when a real second implementation exists;
  the in-memory implementation is the mock (Rust does not have mocks).
  Associated types carry what varies per implementation (the lease, the
  transport error; `Infallible` where a mock cannot fail). Trait names are
  single-word domain roles: `Inbox`, `Sender`, `Database`, `Worker`.
- One module per type, with its `Error` enum beside it. No `types.rs` or
  `error.rs` dumping grounds. A validator with one call site lives in that
  type's impl, not a free function.
- Prefer modules over internal crates (crate boundaries are compile-time
  parallelism fences), one binary with subcommand dispatch, and no macros
  while the domain is young.
- No dead code, no speculative scaffolding, no future-proofing without a
  current consumer. Pre-name the escape hatch in a comment instead of
  building it.
- HTTP clients: prefer typed clients (for example, typed-reqwest) over raw
  reqwest, register the exact mime the endpoint emits, never blind-parse
  JSON, and prefer a narrow hand-rolled client over a third-party SDK for a
  handful of calls.

## Docs and comments

Doctests for happy and common paths on public items remain required; see the
testing references. The points below are the doc-comment (`///`, `//!`)
deltas.

- A one-line summary satisfies a `missing_docs` lint; add prose past that
  line only for what the code cannot say (for example, a bound counted in
  `char`s to match the database's `char_length`). Private items default to
  no doc.
- Flag the doc-comment shapes of restating-the-code: enumerating a
  validator's rules above the validator, restating `#[serde(...)]` or
  `#[derive(...)]` behavior, narrating an impl that sits adjacent, and
  paraphrasing an alias's right-hand side.
- Document deviations, not defaults. Norm behavior (rejecting unknown wire
  values, a derive's forwarding) is not a decision and earns no prose; only
  the permissive or surprising choice carries a why, and a posture the tests
  already pin needs no doc restating it.
- Module docs introduce the one concept the module adds, in a few lines.
- No roadmap in doc comments: "lands in a follow-up PR" is stale the day the
  work lands. Sequencing lives in the module's AGENTS.md or the PR
  description.

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
- Panic policy: avoid `unwrap`, `expect`, and panic-driven control flow on
  request, worker, or network paths. Crashing must be the explicit design.
- Where a failure is genuinely impossible, use `.expect("explain why failure
  is impossible")`, never `.unwrap()`. The message documents the invariant,
  not the operation.
- Enforce coverage requirements and the coverage ratchet. Record the exact
  coverage number and do not allow drops unless the user approves.
- Prefer exact `line.eq("...")` checks over trimming or fuzzy matching when
  matching fixed line literals.

## Errors and observability

- Preserve error context. Do not collapse distinct failures into generic
  strings or booleans.
- One error enum named `Error` per module, co-located with the type it
  serves. No crate-wide error sum, no `Box<dyn Error>` in APIs.
- Mark error enums `#[non_exhaustive]` so new variants can land across
  stacked PRs without breaking downstream matches; callers matching on one
  include a wildcard arm.
- Wrap external errors with `.map_err` and carry them as `#[source]`, never
  `#[from]` (it synthesizes global conversions that mis-route `?`). Display
  XOR source: never print an inner error that is also carried as
  `#[source]`, or reporters render it twice.
- Keep retry, timeout, and backoff behavior explicit where the code touches
  external systems.
- Make sure logs and metrics still identify the failing boundary or
  dependency. Walk the `#[source]` chain where a reporter prints only the
  top-level error.
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
- Check async code for accidental blocking work, long-held locks, and
  cancellation blind spots.
- Make resource cleanup explicit for streams, channels, spawned tasks, and
  shutdown paths. `Drop` is synchronous memory management only; never run
  teardown logic through RAII.
- Panics must terminate the process: `panic = "abort"` plus a panic handler.
  Tokio's default swallowing of task panics is banned; a supervised task
  that dies silently is an outage you cannot see.

## Configuration and secrets

- Every environment value parses into a `FromStr` newtype at boot. Match
  `VarError::NotPresent` explicitly; never `unwrap_or` a default over a
  malformed value (absence and corruption are different states). Wire data
  never defaults absence; operator config may.
- Secrets and PII get a hand-rolled redacting `Debug` with a comment saying
  why, pinned by an exact full-string `Debug` assertion test. Do not
  over-redact non-secrets: an opaque surrogate identifier is not a secret,
  so derive `Debug` for it.

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
- Group a file's unit tests into inner `mod` blocks by the contract under
  test (`mod length`, `mod from_str`), with shared fixtures in the outer
  module imported via `use super::{..}`; cross-cutting tests that fit no
  module get `// -- Theme --` banners in the outer module. Let the module
  path carry the theme: `length::rejects_empty`, not
  `length::empty_string_rejected`. Contract modules also let coverage and
  mutation runs scope to one public function's own tests, so incidental
  coverage cannot inflate the numbers.
- Apply the same contract grouping inside `mod proptests`
  (`proptests::try_new::accepts_every_valid_year`), with shared generators
  in the outer module as fixtures. This scopes the property coverage gate
  per method and makes per-function property-mutation kill rates
  measurable (`proptests::try_new::` as the test filter) — a measurement
  of which properties are load-bearing, not a gate; the mutation gate
  stays unit-tests-only.
- Mutation scope is two-tier. Public-function mutants run against only that
  function's contract module; incidental coverage is not permitted, so a
  mutant killed only by another function's tests is a miss. Private-function
  mutants run against all of the file's contract modules; incidental
  coverage is allowed by design, and the gate is reachability from at least
  one public function plus a kill by at least one of those public functions'
  tests. Do not test private functions directly; an unreachable or
  unkillable private function is dead code to delete, not a gap to cover.
- When a test chains two or more fallible steps whose success is plumbing
  rather than the subject, return `Result` from the test and use `?` (with a
  module-level `#[expect(clippy::panic_in_result_fn)]`); stacked `.expect`
  messages on setup are noise.
- Each test earns its place by failing for a reason no other test catches.
  Flag a test whose every failure mode is already pinned elsewhere. Pin a
  delegating path once, with the one input that discriminates the routing —
  do not re-test the delegate through the alias.
- Commit `proptest-regressions/` files so failing seeds replay
  deterministically.
- Resolve test data paths from `CARGO_MANIFEST_DIR`, never by walking the
  current directory.
- Integration tests live in the cargo-default `tests/` directory; do not add
  a `[[test]]` path entry to `Cargo.toml`, split tests into a sibling
  `<module>_test.rs` file, or wire them with `#[path]`.
- Database tests run against a real ephemeral database with the production
  schema loaded verbatim, never in-memory fakes or hand-rolled partial
  schemas (a fake schema can manufacture a wrong answer). Seed over a single
  connection, then move that connection into the struct under test; no pools
  in tests.
