# Principle of least power

Also known as the *rule of least power* (Tim Berners-Lee / W3C, 2006:
"the less powerful the language, the more you can do with the data
stored in that language") and closely adjacent to the *principle of
least privilege* (Saltzer & Schroeder, 1975) and *least authority* in
the object-capability tradition. The names emphasise different facets:

- **Least power** — choose the *least expressive* primitive that fits
  the task (capability surface).
- **Least privilege / least authority** — grant the *least access*
  required to perform the operation (security / capability tokens —
  see `ingress-and-boundaries.md` § "Capability tokens").
- **Principle of least astonishment** — a related but distinct rule
  about predictable surface behaviour, not capability narrowing.

This file treats them as one discipline at the design level — the call
site, primitive, and signature should accept only as many inputs and
emit only as many outputs as the task strictly requires.

When several primitives can accomplish the same task, choose the one
whose accepted state space is smallest. This is the dual, at the
implementation level, of "minimize representable states" at the type
level: the implementation should request only the power the contract
actually requires, and signal that fact in its types and call sites.

The state-space framing:

> When two or more primitives are equally capable for the task, choose
> the one that, in aggregate, accepts the **fewest input states**,
> returns the **fewest output states**, and uses the **fewest internal
> state transitions**.

The "for the task" qualifier is load-bearing in two directions, not
one.

**Over-power** is the canonical failure: a primitive that admits more
states than the task requires. Callers cannot tell from the type
which subset of those states was actually intended, and Hyrum's Law
fills the slack.

**Under-power with composition** is the symmetric failure: chaining
simpler primitives to imitate what a single more powerful primitive
already encodes. Each step is a new intermediate representation
between input and output — a loose seam where the contract can drift
(same defect as decomposition with loose boundaries in
`principles.md` § "Three roles a function can play").

The unified rule: **match the tool's state space to the operation's
state space.** Neither wider nor narrower. Wider lets the reader
infer the wrong intent; narrower forces the difference to be
re-implemented as composition, introducing loose intermediate states.

Examples of the symmetric failure:

- Chaining `.map(...).map(...).map(...)` when one transformation
  expresses the composition cleanly. Each intermediate collection is
  a representation that did not exist in the single-step form.
- Hand-composing `git write-tree && git commit-tree && git
  update-ref` when porcelain `git commit` already bundles them with
  hook execution and reflog update. The hand-rolled form admits
  states the porcelain forbids (e.g. skipping a hook).
- Re-implementing `flatMap` as `map` followed by manual flatten when
  `flatMap` is available — two operations, two chances to miss an
  edge case the single primitive handles.

Mutation testing is the operational verification of this principle —
see § "Mutation testing as the verifier" below and `testing.md` § "Mutation
testing".

## Three axes

Apply the principle on each axis independently. Tightening one axis
often tightens the others by knock-on effect — narrower input usually
means fewer internal branches; narrower internal branches usually
mean a narrower output type can be promised.

### Input domain

The narrowest type that supports the operation. Each accepted state
the function does not need is latent capability the consumer can
misuse and the implementation must defend against.

- **Reference, not owner**, when ownership is not required (`&str` over
  `String`, `&[T]` over `Vec<T>`, `readonly T[]` over `T[]`).
- **Iterator / generic bound**, not concrete container, when iteration
  is the only operation needed (`impl IntoIterator<Item = T>` over
  `Vec<T>`, `Iterable<T>` over `Array<T>`).
- **Refinement / brand**, not raw primitive, when a constraint exists
  (`NonNegativeF64` over `f64`, `Email` over `string`).
- **Trait object subset**, not the full type — request only the methods
  the operation actually uses.
- **Read-only view**, not mutable handle, when the operation does not
  mutate.
- **Single value**, not collection, when one is enough.

### Return codomain

The narrowest type the operation can actually guarantee. Returning a
wider type than necessary lets callers depend on accidental properties
of the implementation (Hyrum's Law) and forces every consumer to
re-narrow.

- **Concrete type**, not trait object, when only one shape is returned.
- **`NonEmpty<T>`** over `Vec<T>` when the result cannot be empty.
- **Specific tag**, not the full union, when the function only
  produces one variant under the call's preconditions.
- **`!` / `never`**, not `Result<T, E>`, when failure is impossible.
- **Total result**, not `Option<T>`, when the input domain has been
  narrowed to make the function total. See `total-functions.md`.
- **Branded output**, not raw primitive, when the function's output
  satisfies a domain invariant the caller will need.

### Internal state transitions

Once input is narrow, the body's branch count drops because impossible
cases no longer need handling. Read this in reverse: many internal
branches over input are an audit trigger for an over-wide input type
— either narrow the input or document the domain reason each branch
remains representable. Tightening the input collapses the branches.

- **Closed match without a wildcard arm** — input enum exhausted, no
  defensive `_ => ...`. If a wildcard arm exists, ask whether splitting
  the enum into live and terminal variants would delete it. See
  `principles.md` § "Encode invariants into types" and the dead-arm
  pattern in `proof-preservation.md`.
- **No null/empty/zero guards** at the top of the function — the input
  type already excluded those values.
- **No re-validation** of an input that was already parsed at the
  boundary. See `ingress-and-boundaries.md`.
- **Single return**, not multiple `if (X) return Y` early-exits, when
  the input is narrow enough that branching is not required.
- **Constructive sum dispatch** (`Match.valueTags` / `match` arms over
  the discriminant) instead of nested boolean conditionals
  reconstructing a sum type from scattered flags. See
  `boolean-blindness.md`.

A function whose body is all narrowing-then-rewrap is often a sign the
narrowing belongs at a boundary above (`ingress-and-boundaries.md`),
not in this function.

## Choosing between equally capable primitives

When two language or library primitives both accomplish the task,
prefer the one with the **smaller capability surface** — the one that
*cannot* do anything beyond what the call requires. This is the
"principle of least power" applied at the API level.

The mutation-testing literature surfaces these pairs explicitly: every
time a mutation testing tool replaces a more powerful primitive with a
less powerful one and the tests still pass, you did not need the extra
power. The simpler form is the correct form.

Examples (Ruby, drawn from `mutant`'s simplification axes — equivalents
exist in every language):

- `obj.method(:name)` → `obj.public_method(:name)` (restrict to public
  API; private methods can no longer leak).
- `obj.kind_of?(Klass)` → `obj.instance_of?(Klass)` (exact class only;
  subclasses no longer match).
- `==` → `equal?` only when object identity is the contract. Do not
  reach for `equal?` as a "no allocation" optimization — `==` does not
  generally allocate.
- `Array#include?` → `Set#include?` when membership is hot and the
  collection is built once.
- `each_with_index` → `each` when the index is not actually used.

Cross-language analogues:

- **Rust**: `clone()` → borrow when ownership is not needed;
  `Vec<T>` parameter → `&[T]`; `Box<dyn Trait>` → `&dyn Trait`;
  `String` → `&str`. For identity comparison, use `std::ptr::eq`,
  `Rc::ptr_eq`, or `Arc::ptr_eq` only when identity is explicitly the
  contract — `==` on collections is value equality, not identity.
- **TypeScript**: `T[]` → `readonly T[]`; `Map<K, V>` →
  `ReadonlyMap<K, V>`; `unknown` → narrowed type after schema decode;
  `any` → `unknown` then narrow; `(...args: any[]) => any` → typed
  signature; `Object.assign` → spread when only structural copy is
  intended.
- **SQL**: `SELECT *` → explicit column list; `WHERE col = ANY(array)`
  → `WHERE col IN (...)` when membership is finite; `JOIN` →
  `WHERE EXISTS` when only existence is checked.
- **Regex**: `.` → `[A-Za-z0-9_-]` when the alphabet is closed; `+` →
  `{1,N}` when the upper bound is known; alternation `(a|b|c)` →
  `Schema.Literal` / enum when the set is closed. See `languages/regex.md`.

In every case the principle is the same: the simpler form *cannot*
express the cases the original could, so callers cannot accidentally
rely on those cases.

## Eta-reduction

A wrapper that does nothing but forward its arguments to another
function is an over-power site. The wrapper body *could* contain
arbitrary logic but in fact does not. Reduce it to a direct function
reference — the canonical instance of the over-power → least-power
simplification.

```text
// Wrapped (over-power): the body could be anything
.map(x => f(x))

// Eta-reduced (least-power): cannot do anything beyond f
.map(f)
```

### Safety gate — verify before reducing

Eta-reduction is only safe when the receiver invokes the function
with the **same arity and argument order** that the wrapper passed.
The wrapper insulates the inner function from additional arguments
the receiver might supply; removing the wrapper exposes them.

Verify every condition before reducing. If any fails, keep the
wrapper.

1. **Arity match.** The receiver passes exactly the arguments the
   inner function accepts, in the same order. If the receiver
   passes `(value, index)` but the inner function declares only
   `(value)`, the wrapper was silently dropping `index`; reducing
   exposes it.
2. **No silent extra parameters.** The inner function does not
   accept optional, default, variadic, or keyword parameters that
   the receiver could supply by accident.
3. **Type system confirms the equivalence, or the call is verified
   by hand.** In strictly typed languages (Rust, Haskell,
   TypeScript with `strict`), the compiler rejects the unsafe
   reduction — the type system is the verifier, trust it. In
   dynamic or loosely typed contexts (JavaScript, Python without
   type hints, Ruby), the failure is silent at runtime — read the
   inner function's full signature manually before reducing.
4. **Binding semantics match.** A method reference is not always
   equivalent to a function reference. `obj.method` in JavaScript
   loses `this`-binding; in Python it preserves binding; in Ruby
   `method(:name)` returns a bound `Method` object; in Rust
   `Type::method` requires an explicit receiver. Confirm the
   reduction preserves the receiver / `self` the wrapper supplied.

The canonical failure is JavaScript's `[1,2,3].map(parseInt)`. The
wrapper `(x) => parseInt(x)` ignored `map`'s second argument (the
index); eta-reducing exposed `parseInt`'s second argument (radix),
so the call became `parseInt("2", 1)` and returned `NaN`. The bug
sat silently because JavaScript accepts extra positional arguments
without complaint.

### Per-language syntax and safety posture

| Language                          | Wrapped                       | Reduced       | Type system catches arity mismatch? |
|-----------------------------------|-------------------------------|---------------|-------------------------------------|
| Rust                              | `|x| f(x)`                    | `f`           | Yes                                 |
| Haskell                           | `\x -> f x`                   | `f`           | Yes                                 |
| OCaml                             | `fun x -> f x`                | `f`           | Yes                                 |
| TypeScript (strict)               | `(x) => f(x)`                 | `f`           | Yes                                 |
| Scala                             | `x => f(x)`                   | `f`           | Yes                                 |
| Kotlin                            | `{ x -> f(x) }`               | `::f`         | Yes                                 |
| Swift                             | `{ x in f(x) }`               | `f`           | Yes                                 |
| C#                                | `x => F(x)`                   | `F`           | Yes                                 |
| Java                              | `x -> f(x)`                   | `f::method`   | Yes                                 |
| Go                                | `func(x T) U { return f(x) }` | `f`           | Yes                                 |
| Python (mypy strict + annotations)| `lambda x: f(x)`              | `f`           | Yes                                 |
| Python (no type hints)            | `lambda x: f(x)`              | `f`           | No — silent at runtime              |
| TypeScript (non-strict)           | `(x) => f(x)`                 | `f`           | Partial — depends on flags          |
| JavaScript                        | `(x) => f(x)`                 | `f`           | No — silent at runtime              |
| Ruby                              | `{ \|x\| f(x) }`              | `method(:f)`  | No — silent at runtime              |

When the type system catches the mismatch, the compiler is the
verifier — apply the reduction and trust the gate. When it does
not, the safety gate is a manual review step; do not reduce without
reading the inner function's signature first.

### When the safety gate fails

If the receiver legitimately passes arguments the inner function
should not see, the wrapper is doing real work — argument filtering
or transformation. Keep it. Consider whether the filtering belongs
in a named adapter function rather than an inline lambda; a named
adapter documents the intent and gives the filter a single place to
live.

## Mutation testing as the verifier

Mutation testing tools (`mutant` for Ruby, `cargo-mutants` for Rust,
`stryker` for JS/TS, `mutmut` for Python) inject small mutations into
the code and re-run the test suite. A mutation that survives is one
of two things, both of which are state-space minimizations waiting to
happen:

- **Equivalent mutation**: both the original and the mutated code
  produce the same result for every input. The mutated form is
  usually narrower (less power). **This is the simplification — apply
  the mutation to the source code.** Then the more powerful primitive
  is gone and the call site reads precisely.
- **Killable but uncovered mutation**: the original and mutated code
  differ on some input, but no test exercises that input. **The test
  suite is accepting a wider behavior state space than the contract
  allows.** Add a test that fails on the mutant.

Both outcomes shrink representable behavior:

| Surviving mutant | Action            | What got narrowed                |
|------------------|-------------------|----------------------------------|
| Equivalent       | Simplify source   | Source code's capability surface |
| Killable         | Add / tighten test | Test suite's accepted-output set |

Mutation testing is therefore a *closed-loop* state-space discipline:
each iteration either deletes power from the source or narrows what
the test suite tolerates. A mutant-clean codebase has no representable
state in either direction that does not match the contract.

The simplification trap (from the `mutant` skill): do not rewrite the
syntax to hide the mutation. Changing `&method(:name)` to
`{ |x| name(x) }` may make the mutant disappear because the new form
is no longer syntactically mutatable, but the underlying expression
is still under-tested. The correct sequence is:

1. Add the test that exercises the expression.
2. Apply the principled simplification the mutant suggested.
3. Confirm 100% coverage on the subject.

## When more power is justified

Sometimes the wider primitive is the right one. Document the choice
in a comment so the next reviewer does not "fix" it.

- **Future-proofing for a known requirement**: the wider type is on
  the imminent roadmap and pre-narrowing would force a churn.
- **Genuine polymorphism**: the operation truly handles every
  inhabitant of the wider type (e.g. a logger formatting any
  serializable value).
- **Public-API stability**: tightening the input would break
  downstream callers; tighten the next major version instead.
- **Performance**: the simpler primitive is provably slower and the
  hot path matters (rare; verify with a profiler).

These are the same exceptions as for `constructive-vs-predicative.md`
§ "Hard cases for constructive modeling" — the principle is the same
shape, applied at the API rather than the data level.

## Cross-references

- `principles.md` § "Encode invariants into types" — narrowing input
  types is the type-level half of this principle; least power is the
  implementation-level half.
- `total-functions.md` — narrowing input until a partial function
  becomes total is the canonical input-axis simplification.
- `boolean-blindness.md` — replacing scattered booleans with a sum
  type narrows the internal branch count.
- `ingress-and-boundaries.md` — internal branches that re-validate
  belong at the boundary parser, not inside the function.
- `testing.md` § "Mutation testing" — the verification mechanism for
  this principle.
- `proof-preservation.md` — split live and terminal sums so the
  transition function's input excludes the terminal cases and the
  dead arms compile away.
- `languages/rust.md`, `languages/typescript.md`, `languages/sql.md`,
  `languages/regex.md` — concrete simplification pairs per language.
- `normalization.md` — under-power with composition is the
  function-level over-decomposition symptom; sprawl as
  passthrough chain; the dependency-graph treatment of
  merge / split decisions. η-reduction is the function-level
  instance of normalization's transitive-reduction move.
