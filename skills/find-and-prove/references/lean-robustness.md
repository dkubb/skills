# Lean robustness — the export-surface attack catalog + writing habits

Load when reviewing or writing Lean. **The theorem is not the artifact; the
exported environment is.** A Lean module exports types, constructors, recursors,
projections, instances, simp lemmas, coercions, notation, macros, theorem
statements, and axioms. A "security review" that starts inside proof terms is
already too late — the public API may already be an oracle.

## The public elimination-surface catalog (how a sealed handle leaks)

The precise name: **public eliminator/destructor exposure breaking the ADT
abstraction barrier** — *not* a parametricity violation; the public type was
simply never abstract in the ML-module-signature sense. Structures are
single-constructor inductives; fields are constructor parameters; the recursor
eliminates by giving a branch that receives those parameters. So:

1. **Recursors.** `T.rec`/`T.recOn`/`T.casesOn`/`T.brecOn` are PUBLIC even under
   `private mk ::` / `private field`. `T.rec (motive := fun _ => Secret)
   (fun s => s) h` recovers the field downstream (often `noncomputable`; exact
   generated names vary by version). A `def T := PrivateStruct` alias does NOT
   seal — it can be `unfold`ed to the recursor.
2. **Projections.** A projection per field, plus positional `.1`/`.2`, parent
   projections (`toParent` under inheritance, in field order), structure update,
   anonymous-constructor syntax.
3. **`noConfusion` / injectivity.** Exported constructor injectivity/disjointness
   leaks whether two sealed handles share a constructor and can recover parameter
   equality.
4. **Deriving** — each derived instance is an observer or minter:
   `Repr`/`ToString`/`ToJson` → prints hidden structure; `BEq`/`DecidableEq`/`Ord`
   → equality/order oracle; `Hashable` → compressed-representation oracle;
   `Inhabited`/`Nonempty` → default/witness constructor path; `SizeOf` →
   size/shape leak.
5. **Typeclass instances & coercions.** Synthesis is recursive program
   construction (priorities + declaration order break ties). A public
   `Repr`/`DecidableEq`/`Coe T Raw`/`Membership`/`ForIn`/`GetElem` is an
   *exported eliminator* even if it doesn't look like one.
6. **Unlawful equality.** `BEq` is only boolean equality with no axioms;
   `LawfulBEq` connects `==` to `=`. If replay/identity uses `BEq`, require
   lawfulness or prove the exact relation you mean.
7. **`import all` / same-package import.** Can expose `private` definitions by
   name. "Hostile downstream module" is therefore TWO attacks — ordinary consumer
   import *and* same-package/test import. Test-only privacy is not a security
   boundary.
8. **Private decls in public defaults/proofs.** Private values can appear in
   public default arguments and proof-generated public values. Audit default
   args, instance-method bodies, proof-produced public values.
9. **Transparent aliases & reducibility.** A public `abbrev` or reducible `def`
   makes the representation definitional (instance search unfolds reducibles).
   Add `abbrev`, reducible wrappers, and reducibility attributes to the "does not
   seal" list.
10. **`autoImplicit`.** Lean inserts auto-implicit parameters *per field*; two
    fields each mentioning `n` become two *separate* fresh implicits — a
    degenerate-instantiation hazard and a source of "the theorem proved the wrong
    thing." Use `set_option autoImplicit false` for serious work.
11. **Public metaprograms / macros / elaborators.** Run during term construction
    (stronger coupling than ordinary defs); if they touch private state or emit
    public terms with private reductions, they belong in the TCB/export audit.
12. **`@[implemented_by]` / `@[extern]` / `opaque` — kernel/runtime
    divergence.** The compiled program runs the substitute, not the verified
    definition, and NONE of these appear in `#print axioms` — the axiom audit
    passes clean while the executable semantics are unverified. Grep the
    module and its dependencies for these attributes whenever anything is
    executed or extracted; each hit is at best a runtime-bridge obligation.
13. **Shadowed notation / Pollack-inconsistency.** Local `notation` /
    `macro_rules` / `infix` can shadow core symbols so a headline *reads* as
    one claim and *elaborates* as another — an attack on the review itself,
    not on downstream code (the canonical name: Pollack-inconsistency,
    Wiedijk). The sibling channel `pp.all` cannot catch: **Trojan Source /
    homoglyphs** (Boucher–Anderson) — bidi controls and confusable code
    points make source render as a different statement than elaborates.
    Scan headline statements and exported names for non-ASCII/confusables. Re-elaborate headlines under
    `set_option pp.all true` / `#print` before trusting what they say.

## The adversary-import drill (write it BEFORE the implementation is finished)

```lean
-- A hostile downstream file. If any of these returns more than the role should
-- learn, the representation is wrong.
#check @T.rec
#check @T.recOn
#check @T.casesOn
#check @T.noConfusion
#check @T.mk
#check @T.field
#synth Repr T
#synth BEq T
#synth DecidableEq T
#synth Hashable T
#synth Inhabited T
-- also try: .1, .2, pattern matching, structure update, parent projections
-- (toParent), coercions, inferInstance, and any serializers (ToJson/Encodable).
-- also grep: @[implemented_by], @[extern], opaque, partial, native_decide —
-- none of these show in #print axioms.
```

## Writing robust Lean — habits

- **Seal by capability, not by data.** Not `structure H where private mk ::
  private tape : …`. Closer to `structure H where step : EffectIntent ->
  StepResult`, with the trusted constructor closing over the secret. A recursor
  can expose `step`, but `step` is exactly the authority the caller was meant to
  have. If `step` is reusable, the seal is not done — make the API **terminal on
  divergence** and enforce single-use by an indexed transition / runtime
  ownership / an explicit bridge theorem.
- **`Subtype` is not a seal — it's a pair.** Don't hide a sensitive payload
  in a public subtype and hope the invariant hides it. Don't put a sensitive
  proof in `Type` unless it is meant to be inspectable computational data.
- **`set_option autoImplicit false`**, then make every index/role/universe
  explicit.
- **Public theorem before private helper.** The public theorem mentions only
  public constructors, smart constructors, public observations, and public
  relations. If the *statement* needs a private field, the API is not ready. (A
  private *lemma* in the proof is fine.)
- **Four theorem families per operation:** `ok` (what changes on success),
  `error` (what changes/reveals on failure), `frame` (what does NOT change),
  `coherence` (all returned fields tell one story).
- **Resource systems add:** `conservation`
  (`parent_after + child_used + residual = parent_before`), `frame` (unrelated
  capabilities unchanged), `validity` (nothing goes invalid), `disjointness`
  (distinct keys ↦ distinct physical resources, or a *named* minting boundary
  obligation).
- **Search before proving.** `plausible` (né `slim_check`) is the
  in-ecosystem QuickChick: run it on every headline before attempting the
  proof; a counterexample now is cheaper than a failed induction later.
- **Don't trust `simp` as a spec.** `[simp]` only for canonical API facts; at
  seams use `simp only [public_def, theorem_name, h]`. A proof that survives only
  because `simp [Internal.secretRep]` ran is not an API proof.
- **Mutate intentionally — execute, don't argue.** Apply the mutant and
  recompile; let the compiler rule. The full discipline (vacuity vs coverage
  sweeps, the semantic-kill rule) and the operator catalog are
  `references/evidence.md`.
- **Define `LowEq role : State → State → Prop` early; prove two-run theorems**
  (`LowEq s₁ s₂` ∧ low-equal inputs → low-equal observations ∧ `LowEq`
  next states) by self-composition/product — the Lean form of unwinding.
  Prevents "each run is safe" missing "two runs are distinguishable."
- **Audit derived observers as hand-written functions.** `Repr`/`BEq`/`Hashable`/
  `Ord`/`ToJson`/`GetElem`/`Membership`/`ForIn`/coercions each get an explicit
  allow/deny. If equality is allowed, prove what it means; if hash is allowed,
  prove it excludes the secret or admit a declassification.
- **Separate `Prop` from `Type` deliberately** — invariants in `Prop`
  (proof-irrelevant, noncomputational); evidence in `Type` only when downstream
  may inspect it. Don't move a secret from erased proof-land into executable
  data-land.
- **Re-elaborate before you read.** Audit headline statements under
  `set_option pp.all true` / `#print`; confirm no local notation or macro
  shadows a core symbol (`=`, `¬`, `→`, `∀`, `∃`). For untrusted authors
  (LLM-generated proofs included), finish with an external kernel pass
  (`lean4checker`) — elaborator exploits do not show in `#print axioms`.
- **Keep a `DocClaims`-style module.** Every prose phrase — "unrepresentable",
  "exact", "complete", "only", "faithful", "no fabricated" — gets a declaration
  whose *type* says the same thing, compile-pinned.
- **Runtime bridges need their own theorem/law:** lowering preserves/narrows
  authority; codec round-trips; canonicalization idempotent & complete; request
  identity includes every semantically relevant field; replay never fabricates
  observations; out-of-envelope observation halts. "Lean proved it" never means
  "Rust does it."

> **The non-negotiable rule:** before proving a constructor safe, prove that
> every exported eliminator is intended, harmless, or impossible to call by the
> adversarial role. This one habit catches the multi-iteration seal problem
> immediately.
