# Core Notation

- `S(A)` — representable states (structural; what the type's shape permits).
- `C(A)` — valid states (contract). ("Valid", matching
  `principles.md`; "admissible" is reserved for judgments about
  transformations and rewrites.)
- `R(b)` — reachable states under a boundary constructor
  `b: (u: U) × P(u) -> A`.
- `I_repr(A)` — representable invalid: `S(A) \ C(A)`.
- `I_reach(A,b)` — reachable invalid: `R(b) ∩ I_repr(A)`.
- `B(A)` — contract behavior: the artifact's observable behavior as a
  function of representable state `S(A)`. `B(A)|_C` restricts that
  function to the states in `C`, where it does real work.
- `D(f)`, `K(f)`, `R(f)`, `G(f)` — domain, codomain, range, gap.
- `cost(m)` — the audit burden of mechanism `m`: the trusted surface it
  adds plus its ergonomic and toolchain cost. Semantics:
  `state-space-minimization` `references/principles.md` § "Core
  principle" (audit cost and drift surface) and § "Encode invariants
  into types" (ergonomic and toolchain cost per rung).
- `supp(μ)` — the support of distribution `μ`: the outcomes with
  nonzero probability. (`μ`, not `P`: `P(u)` is the boundary
  evidence predicate.)
- `Needs(y, P(x))` — consumer `y`'s contract depends on evidence
  `P(x)`.
- `Erases(g, P(x))` — operation `g` maps `(x, p: P(x)) ↦ x`,
  discarding the evidence.
- `Flows(g, y)` — the output of `g` reaches consumer `y`.
- `[[t]]^D`, `[[t]]^O_{q,c}` — denotation, and operational reception
  from prior state `q` in context `c`.
- `c ∈ Ctx_0`, `q'` — a reception context, and the receiver state
  after reading `t`.

$$
\begin{array}{rcl}
S(A) &:=& \text{representable states (structural)} \\
C(A) &\subseteq& S(A) \\
R(b) &:=& \{\, b(u,p) \mid u \in U,\; p \in P(u) \,\} \subseteq S(A) \\
I_{\mathrm{repr}}(A) &:=& S(A) \setminus C(A) \\
I_{\mathrm{reach}}(A,b) &:=& R(b) \cap I_{\mathrm{repr}}(A)
\end{array}
$$

Always subscript the invalid set. `I_repr` and `I_reach` answer
different questions, and an unsubscripted `I` silently equates a
predicative encoding with a constructive one.

$$
\begin{array}{rcl}
R(f) &:=& \{\, f(x) \mid x \in D(f) \,\} \\
G(f) &:=& K(f) \setminus R(f) \\
f^{-1}(F) &:=& \{\, x \in D(f) \mid f(x) \in F \,\}
\end{array}
$$

## Constructive vs predicative

- **Constructive encoding**: `S(A) = C(A)`, so `I_repr(A) = ∅`. The proof
  lives in the type's shape and is preserved across every consumer.
- **Predicative encoding**: `S(A) ⊋ C(A)`, so `I_repr(A) ≠ ∅`. A trusted
  boundary `b` restricts `R(b) ⊆ C(A)`, so `I_reach(A,b) = ∅`, but `I_repr`
  remains and proof obligations must be reconstituted at every consumer
  that reasons about `S(A)`.

A smart constructor narrows `R(b)`, not `S`. Equating `I_reach = ∅` with
`I_repr = ∅` is the most common slip in informal SSM analysis.

Semantics: `state-space-minimization`
`references/constructive-vs-predicative.md` (intrinsic vs extrinsic
safety, the trusted-boundary audit, the hard-case fallbacks).

## Proof-preservation corollary

For predicative `A`, every consumer `f` with `D(f) = S(A)` whose
contractual behavior is specified only on `C(A)` carries an
obligation `Obl(f)` that the input came through `R(b)`. (`Obl(f)`, not
`P_f`: `P` is the boundary evidence predicate.) Adding a second
construction path `b' ≠ b` with
`R(b') ⊄ C(A)` voids `Obl(f)` globally.

For constructive `A`, `Obl(f)` is discharged by the type and survives
the addition of arbitrary new construction paths.

**The bad-lowerer test (is a claimed run output constructive or
predicative?).** When a property `Q` is asserted of an artifact's
*outputs* and one of the construction paths `b` is an authored lowering
— program data the reducer reads but does not compute (an occurrence id,
a template, a slot) — ask: *can a bad lowerer `b'` falsify `Q` while
every reducer rule behaves correctly?* If yes, `Q` is established only
through the trusted boundary `b`, so `R(b') ⊄ C(A)` voids it exactly as
above: `Q` is predicative, conditional on lowering well-formedness, not a
constructive (rule-level) theorem. State the required input property as a
hypothesis. For example,
if output labels must be unique and the reducer copies them from the
lowered input, two equal input labels refute an unconditional guarantee.
Discharge the hypothesis by proving that the lowering produces unique
labels. A rule-level theorem quantifies over all programs; a property a bad
program breaks is predicated on the program inhabiting `C`.
