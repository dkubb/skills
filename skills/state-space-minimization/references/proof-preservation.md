# Proof preservation

Once a value is parsed into a stronger type, downstream code should
consume that type, not unwrap to a primitive and re-validate. Each
unwrap-and-revalidate step risks losing or weakening the proof, and may
quietly drift out of sync with the original constructor.

This file covers the three techniques that preserve proofs across a
codebase, in increasing strength: phantom tags (Ghosts of Departed
Proofs), GADTs / witness types, and refinement / pattern / dependent
types.

## Ghosts of Departed Proofs (phantom tags)

When the same value flows through several stages with different
invariants, phantom-type tags record which checks have run without
changing the runtime representation. The technique is named after Edsko
de Vries / Matthew Noonan's *Ghosts of Departed Proofs* paper.

```
struct Checked<Tag, T>(T, marker: PhantomData<Tag>)
struct Sanitized
struct AuthorizedFor<U>(marker: PhantomData<U>)

fn sanitize(input: RawSql) -> Checked<Sanitized, Sql>

fn authorize<U>(q: Checked<Sanitized, Sql>, user: &U)
    -> Checked<(Sanitized, AuthorizedFor<U>), Sql>

fn run<U>(q: Checked<(Sanitized, AuthorizedFor<U>), Sql>) -> Rows
```

`run` is uncallable without proof of both checks. The compiler enforces
the order of operations: sanitize before authorize before run.

Reach for this pattern when:

- the call graph is large enough that "did we sanitize already?" or "is
  this query authorized for this user?" is hard to answer locally
- a proof must be carried across module boundaries without being re-checked
- the same data is touched by multiple subsystems with different
  authority requirements (sanitization, authorization, logging,
  encryption)

For small modules the cost outweighs the benefit — a single newtype is
enough.

## GADTs and witness types

Generalized algebraic data types let constructors refine the type
parameter they return, so the type carries proof at runtime.

```
intrinsically-typed AST: only well-typed expressions representable
  enum Expr[T]:
    LitInt(i32)            : Expr[i32]
    LitBool(bool)          : Expr[bool]
    Add(Expr[i32], Expr[i32]) : Expr[i32]
    If[T](Expr[bool], Expr[T], Expr[T]) : Expr[T]

  // there is no constructor for `If(LitInt(1), ..., ...)`
```

Witness types are the same idea applied to dispatch: a runtime tag
re-establishes a static type so downstream code can pattern-match
exhaustively.

GADTs are native in Haskell, OCaml, and Scala 3. In Rust the equivalent
is `PhantomData` plus sealed traits, or `enum_dispatch`-style patterns
with associated types.

This is the strongest constructive tool short of dependent types: the
type's inhabitants are exactly the valid trees, sequences, or programs.

## Refinement and pattern types

Refinement types attach predicates to base types and have those
predicates checked by the type checker. Liquid Haskell, Flux for Rust,
F\*, Dafny, RefinedRust, Verus, Creusot, and Aeneas all sit in this
family.

```
liquid-style refinement: type checker proves the predicate
  type Pos = { v: i32 | v > 0 }
  fn divide(numerator: i32, denominator: Pos) -> i32
```

Pattern types (a Rust proposal) restrict primitives to value patterns
directly in the type:

```
  fn day_of_month(month: u32 is 1..=12, day: u32 is 1..=31)
```

These are stronger than predicative smart constructors when available,
because the predicate travels with the type instead of living in a
trusted module, and the checker rejects callers that violate it.

The 2024 -- 2025 wave of Rust verification tools (RefinedRust, Verus,
Creusot, Aeneas, Thrust) all attempt to bring this guarantee to safe
Rust. None is yet stable in the core toolchain. Use them when your stack
supports them; until then, smart constructors plus tight modules are the
practical equivalent.

## Dependent types

Dependent types let types depend on values. The result is the strongest
practical guarantee currently shipped: the type system can express
arbitrary mathematical specifications. Languages: Idris 2, Lean 4, Agda,
Coq, F\*. Industry adoption is real but narrow (cryptography, kernel
verification, smart contracts).

Most working code does not need dependent types. The relevant point for
this skill is that the *progression* — runtime check → smart constructor
→ phantom tag → GADT → refinement → dependent — is a single ladder, and
moving up the ladder always moves runtime checks into the type system.

In languages without dependent types (Rust, TypeScript, Python, Go),
the practical approximations climb the ladder without reaching the
top: capability tokens for runtime-checked invariants that cannot be
statically encoded (see `ingress-and-boundaries.md` § "Capability
tokens"), substructural / affine types for one-shot proofs, smart
constructors plus tight modules for ergonomic refinement-like
guarantees. The composition of these mechanisms — capability tokens
issued by smart constructors and carried by affine values through
phantom-tagged pipelines — is the mainstream-language equivalent of
"the type expresses the specification."

## Substructural typing

Linear / affine / relevant / ordered type systems control *how often* a
value is used, not which values are valid. They are the proof-preservation
mechanism for *resources*:

- **Linear**: each value used exactly once (file handles, mutex tokens,
  one-shot capabilities, cryptographic nonces).
- **Affine**: each value used at most once (Rust ownership, move
  semantics).
- **Relevant**: each value used at least once (no silent dropping).
- **Ordered**: values used in a specific order (stack discipline).

Affine ownership in Rust means the type system tracks "this resource is
used once and dropped." Phantom tags compose with affine types to record
not just "this proof exists" but "this proof has not yet been spent."
Capability tokens are usually affine for this reason.

### Resource or fact: classify before imposing linearity

Substructural typing controls *how often* a value is used, so it is the
right mechanism only for actual **resources** (capabilities, handles,
nonces, budgets). A **fact** — a derived datum or single-assignment value
that is simply true once established, and readable by any number of
consumers — is not a resource. The classification is two-sided, and both
errors leak:

- **Fact modeled as affine deletes valid states.** A value that a later
  step *and* an audit/export step both legitimately read can be read only
  once under linearity, so the second reader starves — an artificial
  consumer race the contract never asked for. This breaks contract
  preservation (`B(A')|_C ≠ B(A)|_C`): strictness is a means to remove
  invalid states, not an objective, and linearity here removes states the
  contract requires (`references/principles.md` § "Core principle";
  the skill's own "do not delete or make awkward any state the contract
  requires").
- **Resource modeled non-linearly admits invalid states.** The familiar
  direction: a true one-shot capability used twice (double-spend, replayed
  nonce, reset-attack on a token) is a representable-invalid state that
  affine/linear typing exists to remove.

Decision: ask **"affine resource or fact?"** before reaching for
linearity. A fact is **persistent and owner-scoped**, with its coherence
expressed as a write-once cardinality bound (`≤ 1` value per
`(owner, binder)`) — uniqueness of the *fact*, not single *use*. A
resource is affine/linear with conservation under composition. Misreading
one for the other is the substructural analogue of a primitive-obsession
slip.

## Cross-references

- `principles.md` § "Encode invariants into types" — the full ladder.
- `constructive-vs-predicative.md` — refinement and dependent types
  bring constructive-grade guarantees to predicative-style code.
- `ingress-and-boundaries.md` — capability tokens as the affine
  realization of authority proofs.
- `history-and-lineage.md` — refinement-types lineage (Freeman & Pfenning
  1991), session types (Honda 1993), GDP (Noonan 2018).
