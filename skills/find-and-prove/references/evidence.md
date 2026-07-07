# Evidence mechanics — defect class H (how to actually decide)

Load when generating or judging the evidence for any finding or adequacy
claim: mutation mechanics and operators, witness discipline, and the
mechanical checks that run before any judgment lens. The Lean export-surface
catalog and robust-writing habits stay in `references/lean-robustness.md`.

## H1 Mutation discipline

**Mutate intentionally — execute, don't argue.** Reasoning "a bad mutant would
surely fail" is how vacuity *survives* review; apply the mutant and recompile
(`just compile`), let the compiler rule. *Vacuity* (is each stated theorem
real?): break ONE def *under* a headline (consume doesn't decrement; remaining
is always original; canonical args ignore authority context; error returns the
recorded head; child debits full allocation not used share); if it still proves,
it wasn't the theorem you thought. *Coverage* (is there behavior NO theorem
pins?): mutate each load-bearing def across its behavior space and recompile
against the WHOLE theorem set — a mutant that compiles green is an *unspecified*
behavior, a MISSING theorem (not a weak one), so pin it. Per-theorem vacuity can
all pass while a whole behavior dimension ("does this preserve the rest of the
structure?") has no theorem at all; sweep the full set, not one def. For the
operators and how to derive them, see "Mutation operators" below. A kill counts
only if SEMANTIC: a mutant dying on a `warningAsError` unused-binder lint is
incidental and overstates adequacy — rename `_x` and rerun, requiring a
type/proof failure.

## Mutation operators — derive by least-power simplification

Don't memorize a list; *generate* it. Every operator replaces an operation with
one that has a **smaller state space** — a strictly less-powerful form the types
still accept (`mutant`'s principle of least power: "use the most constrained
primitive that satisfies the requirement"; `kind_of?`→`instance_of?` narrows to
one exact class, `method`→`public_method` drops private access). When a mutant
**survives** (every proof/test still passes), it forces a binary — and *both*
answers improve the artifact:

- **Lower-power operator is sufficient** — nothing needed the extra power;
  adopt the simpler form as the new source (shrink the state space; the code
  was over-powered). Mutant's own disposition: accept the survivor.
- **It is NOT sufficient** — the dropped behavior matters but no proof/test
  forced it; **add the theorem/test that forces it** (pin it; our siblings case).

There is no third option (rewriting to mask the mutant without coverage is mutant's
explicit trap — same rule as the proof ladder). The catalog below is the common
cross-language core + Lean specializations; the generator (smaller-state-space
substitution) finds the artifact-specific operators a fixed list misses — survivors
hide where the types DON'T already constrain (type-compatible value choices).
Floor, not ceiling. Under budget, rank mutation targets the way oracle targets
are ranked: defs under strong-word headlines and defs feeding public
observations first — and report the unswept remainder explicitly (a silent
partial sweep reads as full adequacy).

**Universal core (every language; specialized to Lean):**

- Relational / order swap — `≤ < = ≠ ≥ >`; `⊆`↔`=`.
- Boundary / off-by-one — `n ± 1`; `<`↔`≤`.
- Literal / constant — `0`↔`1`; `some k`↔`none`↔`some 0`; `true`↔`false`.
- Logical / arithmetic operator — `∧`↔`∨`; `+`↔`-`; `min`↔`max`.
- Condition negation / branch swap — flip `if`/`match` arms; negate a guard.
- Deletion — drop a conjunct, a function argument, or a list element.
- Return replacement — identity (return an argument unchanged); a default.

**Lean / type-theory / proof-specific (the smaller-state-space form of a proof
construct):**

- Weaken a carried witness — `g'.sub g`→`g'.sub g'` (`sub_refl`); `restrict_sub`→
  `sub_refl`; drop a `scope`/`drop` narrowing proof.
- Constructor swap / collapse — `ordinary`↔`authoring`; `cons`→`nil` or its
  tail; `some`↔`none`.
- Identity a structural transform — `map f`→identity; `restrict s g`→`g`;
  `consume c`→ no-op.
- Hypothesis mutation — drop or generalize a theorem antecedent; if the proof still
  closes, the hypothesis was unused (over-claim).
- Conclusion weakening — replace the goal with a strictly weaker one; if it still
  needs the full proof, suspect misattribution / vacuity.
- Type-index loosening — relax an index or an instance constraint.

## H2 Witness discipline

**Load-bearing-hypothesis witness audit** — to show hypothesis C is
necessary for a theorem `{A, B, C} → G`, the necessity-witness must
satisfy ALL the OTHER hypotheses (A, B) and fail *only* when C is
dropped — else a critic says "your counterexample also violates B." A
witness proving merely `A ∧ ¬C → ¬G` is "a nearby bad state," not a
clean load-bearing witness; discharge every sibling hypothesis in the
witness (often cheap, and the proof is what makes the necessity claim
adversary-binding).

## H3 Mechanical floor (run before judging)

- **Statement authenticity (Pollack-inconsistency)** — the reviewer
  audits the *rendered* statement; local `notation` / `macro_rules` / `infix`
  can shadow core symbols so a headline reads as one claim and elaborates as
  another (Wiedijk's Pollack-inconsistency: the printer/parser as attack
  surface — the system prints a statement that reads as a different claim
  than it elaborates to). Re-elaborate every headline under `set_option pp.all true` (or
  `#print`) and confirm no in-scope notation shadows `=`, `¬`, `→`, `∀`, `∃`.
  When the artifact author is untrusted (LLM-generated proofs included), add
  an external kernel pass (`lean4checker`) — elaborator exploits are outside
  the reach of `#print axioms`. Re-elaboration does NOT catch homoglyphic
  identifiers or bidi-reordered rendering (Trojan Source): a lookalike code
  point makes two names render identically while naming different
  declarations. Add a confusables / non-ASCII scan over headline statements
  and exported names.
- **Kernel conservativity** (did a def/quotient/axiom enlarge what the *kernel*
  proves?) vs **elaboration-surface stability** (did public instances/simp/
  reducibility change what downstream constructs or what proofs *mean*?).
- **Axiom / TCB budget** — `#print axioms`; `Classical.choice` is debt only
  under a constructive/extraction gate; `sorryAx`/`native_decide`/unsafe are
  the real red flags. `#print axioms` is NOT the whole TCB: `@[implemented_by]`
  and `@[extern]` swap the *compiled* code out from under the verified
  definition and appear in no axiom report — grep for them (and `opaque`)
  whenever anything executes or extracts; each hit is at best a runtime-bridge
  obligation.

## Evidence generation

- **Small-scope / property-based** (Alloy small-scope; QuickChick) — instantiate
  finite parameters to small bounds and search
  (`decide`/`#eval`/`plausible`/enumerators) BEFORE proving; require a
  non-degenerate witness per headline. **Mutation ·
  metamorphic · differential · CEGAR · Hughes property taxonomy.**
- **Coverage-guided fuzzing** (AFL/libFuzzer lineage) — for anything the rank
  classifier sends to *runtime bridge*: round-trip fuzz codecs and
  canonicalizers (`decode ∘ encode = id`, canon idempotent), differential-fuzz
  the implementation against the model interpreter on shared inputs.
  Property-based testing searches the spec's input space; coverage guidance
  searches the IMPLEMENTATION's branch space — they find different bugs.
