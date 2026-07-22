# Rust Code Review Guidelines (Language-Specific)

- Use simple English.
- Use short bullets.
- Apply `../core-principles.md` first.
- Do not repeat core principles.
- Many rules below have automated-lint candidates documented in
  `../LINT-TODO.md`. Configure stock clippy lints where they cover a rule.

## Tooling

- Run the repository wrapper for `cargo fmt --all -- --check` and Clippy. In a
  conventional workspace, the Clippy gate is
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` when
  the all-features combination is valid; otherwise run the documented feature
  matrix.
- Encode the retained rustc and Clippy lint set in workspace lint tables and
  `clippy.toml`. Inventory every available lint when revising the policy; do
  not mistake `clippy::all` for every Clippy lint.
- Pin the Rust toolchain or MSRV used by CI. Run formatting and linting with
  that toolchain so local success and the review gate have one meaning.

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
  disguise. Convert it to `pub struct` unless the enum has a concrete current
  semantic purpose. A hypothetical future variant and `#[non_exhaustive]` are
  not sufficient reasons to retain the enum.
- **Identity-passthrough methods.** Methods like `fn x(self) -> Self { self }`,
  `fn x(self) -> Self { *self }`, or `fn from_x(x: Self) -> Self { x }` are
  shims. They survive past their compatibility purpose. Delete and inline at
  call sites.
- **Constant-returning methods that ignore `self`.** A method that takes
  `self`/`&self` but returns a literal or constant is documentation in
  function form. Either rewrite as an exhaustive match over `self` (so new
  variants force the question) or delete and rely on the type-level proof.
- **Bit-identical domain types.** Runtime representation does not determine
  domain identity. Keep separate newtypes when values are not interchangeable;
  their nominal distinction prevents accidental mixing even when their fields
  are identical. When the domain treats values as interchangeable, use one
  domain type directly rather than introducing parallel names.
- **Single-field newtype wrappers with delegated accessors.** A `struct X(Y)`
  or `struct X { f: Y }` can earn its existence through nominal domain
  identity, a smart-constructor invariant, a phantom tag, an ownership or API
  boundary, or a distinct trait contract. Prefer the newtype whenever callers
  must not substitute `Y` or another representation-identical domain value.
  Do not replace a domain newtype with a type alias: an alias does not preserve
  distinct type identity, and an LLM is fully capable of writing the newtype.

## API and ownership

- Avoid needless cloning. Prefer borrowing and explicit lifetimes.
- Use the simplest expression that satisfies the goal. Delegate to existing
  trait implementations rather than converting to a different type first
  (e.g., `self.inner.cmp(&other.inner)` over `self.inner.as_u128().cmp(…)`).
- Prefer `pub(crate)` shorthand over `pub(in crate)` for crate visibility.
- Prefer grouped explicit imports. Use glob imports only for an intentional
  prelude or an established repository convention where hidden provenance and
  future name collisions are controlled.
- Keep public APIs minimal. Use the narrowest visibility that works.
- Prefer exhaustive public types so adding a variant forces every dependent
  match site to be reviewed and updated. Applications and workspace-internal
  crates **MUST NOT** use `#[non_exhaustive]` merely to avoid coordinated
  updates. A publicly published library crate, such as a crates.io package,
  **MAY** use it when a documented compatibility commitment makes downstream
  source compatibility more important than exhaustive compiler enforcement.
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
- Decode external JSON with Serde into an untrusted boundary shape, then use a
  smart constructor or `garde` validation with constraints that mirror the
  domain type. Do not treat `garde` as a JSON parser or schema generator.
- Deserialization must call the smart constructor.
- Goal: no instance exists without the smart constructor.
- Prefer private fields and smart constructors for domain types with
  invariants.
- For each smart constructor, require boundary tests and property tests over
  valid and invalid ranges.
- For SQLx row mapping, implement `FromRow` and delegate to the smart
  constructor.
- When schemas must be generated, use a schema tool whose output is derived
  from the same constraints and verify it stays aligned with runtime
  validation.
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
- Lint suppression policy: do not use `allow` attributes at any scope. Use the
  smallest-scoped `expect(..., reason = "...")` instead, with a clear reason —
  `expect`
  forces suppressions to be removed once they are no longer needed, so
  improvements cannot silently backslide. Treat this as a cargo restriction
  for lint configuration. Use `expect` only when the repository's pinned Rust
  toolchain supports it. An older toolchain is not permission to substitute
  `allow`: refactor the code to satisfy the lint, or upgrade the pinned
  toolchain in a separate atomic change. If neither is currently valid, report
  the conflict rather than suppressing the warning.
- Arithmetic policy (Rust):
  Prefer `checked_*` math. Treat overflow/underflow as an error signal.
  If a clamp-to-zero behavior is required (for example, scanning a substring
  that may start mid-context), do it explicitly and document why.
- Panic policy: avoid `unwrap`, `expect`, and panic-driven control flow on
  request, worker, or network paths. Crashing must be the explicit design.
- Where a failure is genuinely impossible, use `.expect("explain why failure
  is impossible")`, never `.unwrap()`. The message documents the invariant,
  not the operation.
- Enforce the shared coverage ratchet. Record every absolute uncovered item
  count, keep each configured maximum at the exact current count, and treat any
  threshold increase as a regression requiring user approval. Do not use
  percentages.
- Prefer exact `line.eq("...")` checks over trimming or fuzzy matching when
  matching fixed line literals.

## Errors and observability

- Preserve error context. Do not collapse distinct failures into generic
  strings or booleans.
- One error enum named `Error` per module, co-located with the type it
  serves. No crate-wide error sum, no `Box<dyn Error>` in APIs.
- Keep error enums exhaustive by default so new failure modes force dependent
  matches to be updated. Apply the API-level `#[non_exhaustive]` exception only
  to publicly published library crates, such as crates.io packages, with a
  documented compatibility requirement; stacked PR convenience is not a
  sufficient reason.
- Use `.map_err` when a boundary needs context or when one source error can map
  to different domain variants. Permit `#[from]` only when the source-to-variant
  conversion is unique, lossless, and correct everywhere `?` can invoke it.
  Display XOR source: never print an inner error that is also carried as
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

- **Prefer operations on an owned `Child` over reconstructing a target from a
  numeric PID.** `child.id()` followed by PID-based signaling can race with
  process exit and PID reuse. Do not claim `Child::kill()` is identity-safe on
  every platform; when PID-reuse safety is a correctness requirement, use a
  platform API with that guarantee, such as Linux `pidfd_*` or an owned process
  handle.
- Check async code for accidental blocking work, long-held locks, and
  cancellation blind spots.
- Use RAII and `Drop` for infallible synchronous resource release. Make
  asynchronous, fallible, or protocol-level shutdown explicit for streams,
  channels, spawned tasks, and services; `Drop` cannot await or report failure.
- Choose panic strategy for the deployment context. Long-running supervised
  services **SHOULD** abort or propagate task panics to a supervisor rather than
  silently losing a Tokio task; libraries must not impose a process-wide panic
  strategy on their callers.

## Configuration and secrets

- Every environment value parses at boot into an existing standard type or a
  `FromStr` domain newtype. Match `VarError::NotPresent` explicitly; never
  `unwrap_or` a default over a malformed value (absence and corruption are
  different states). Wire data never defaults absence; operator config may.
- Secrets and PII get a hand-rolled redacting `Debug` with a comment saying
  why, pinned by an exact full-string `Debug` assertion test. Do not
  over-redact non-secrets: an opaque surrogate identifier is not a secret,
  so derive `Debug` for it.

## Test strictness (Rust-specific)

- Tests **MUST** fail on missing data; avoid fallback accessors in tests and test
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
- The contract module owns its contract prefix. A leaf test name must state
  only the behavior and must not repeat that prefix: use
  `proptests::discover::rejects_unknown_keys`, not
  `proptests::discover::discover_rejects_unknown_keys`.
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
  rather than the subject, return `Result` from the test and use `?`. If this
  requires suppressing `clippy::panic_in_result_fn`, place
  `#[expect(clippy::panic_in_result_fn, reason = "...")]` on each affected test,
  never on the module; stacked `.expect` messages on setup are noise.
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
