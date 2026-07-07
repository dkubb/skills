# Overclaim & reception — defect class C (statement weaker than name/prose)

Load when auditing what a theorem's NAME, docstring, or surrounding prose
claims against what its STATEMENT actually says. This is the mutation-blind
class: the theorem can be true and every mutant can die while the reception —
what a reader takes away — is still wrong. Statement-surface defects need
statement-level audits, not more mutants.

## C1 Object-referent audit

FIRE on every theorem whose NAME references a concrete object or quantity
(`…Trace`, `…authority`, `…frontier`, `…spend`, `…schedule`): verify the
STATEMENT actually mentions/compares THAT object — not a DIFFERENT object,
at a different representation level, that happens to be the correct thing to
compare. The origin case: a guard named `…_not_fixedBlockReducerTrace`
proved inequality against the token block traces (the correct, necessary
level — the reducer trace is their lossy projection) and never mentioned the
reducer trace. True theorem, non-vacuous, every mutant dies — a pure
NAME↔referent reception defect the mutation gate structurally cannot catch.
Fix: rename to the actual referent, or add the docstring clause naming the
level. Three sibling lenses from the same review; apply together:

- **Run-independence of a transported `∃`** — when a `∀`-over-runs theorem
  reuses a lemma whose `∃`-witness must be run-independent to serve every
  run, confirm the reused lemma's ARGUMENT LIST omits the quantified run.
- **Lossy-projection comparison-level check** — comparing at the SOURCE
  level is CORRECT (not a gap) when the projection is lossy; a
  projected-level (in)equality is a DIFFERENT, weaker statement. Flip "why
  not compare the projected object?" into a correctness point.
- **Cosmetic-defeq-vs-genuine-read** — break the INPUT with a mutant to
  prove a `rfl`/defeq bridge genuinely READS its input, not an accidental
  defeq that would hold for any input.

## C2 N-distinct arity audit

FIRE on every `*_distinct` / `*_disjoint` name: N distinct objects need
exactly C(N,2) pairwise inequalities in the statement — `¬(A ∧ B)` does not
exclude A alone, and a two-of-three statement leaves one pair free to
collide. The tell: a docstring case taxonomy that silently omits a case
points at the missing conjunct. Decide by counting the inequalities against
the name's arity — this is arithmetic, not judgment.

## C3 Name-vs-semantic-load

FIRE on every named predicate: construct a value SATISFYING the predicate
that VIOLATES what the name implies. The canonical case: set-level
injectivity named as occurrence uniqueness is satisfied by `[e, e]` — the
set has one element, the name promises no duplicates. If the construction
succeeds, rename to the literal content (what the definition actually
says), never keep the aspirational name with a clarifying docstring alone —
the name is what readers carry. The deciding witness is the satisfying
violator; building it is cheap and settles the argument.

## C4 Frame honesty ("no X" claims in redex ++ frame systems)

FIRE on every "no X" / "contains no X" claim in a `redex ++ frame` rewrite
system: disambiguate THREE readings — (a) the RHS-PRODUCED block has no X;
(b) no NEW X appears; (c) the WHOLE output bag has no X. Reading (c) is
almost always FALSE without an explicit frame-absence hypothesis: the frame
may already carry an X, especially for PERSISTENT accumulating facts. The
honest forms: a frame-clean IMPLICATION (`frame has no X → output has no
X`) or a claim scoped to the produced residue. Prose for a bare local
rewrite says "residue block", never "in G′". [Anchor: small-footprint /
tight specifications — a rule's spec talks only about the footprint it
touches; whole-bag claims need the frame made explicit.] The
temporal-inversion test and the shadow-difference probe below are this
item's siblings — all three police what the frame smuggles in.

### Temporal-inversion test for bag / unordered state (C4 sibling)

When a theorem reads "A THEN B" but the state is an unordered bag / frame,
ask: *can a supposedly-LATER token sit in the INITIAL frame?* If yes, the
theorem proves COMPATIBILITY / REACHABILITY, not causal ARRIVAL — a
pre-supplied reply makes "the emit *sources* the reply" an overclaim (the
rule sources only what it actually mints; the reply is exogenous start-bag
input). Calibrate the claim to what the rule produces, and name the exogenous
tokens as environment input.

**The shadow-difference probe (the C4 sibling).** FIRE on every certified
distinguisher: NEUTRALIZE it (make the producer constant so the certified
difference disappears) and check whether another already-varying field still
proves the conclusion. If the theorem still goes through, the intended
premise is DEAD — the conclusion was riding a shadow difference in a field
the claim never mentions, and the certificate certifies nothing about the
named distinguisher. The deciding mutant is the constant-producer
neutralization; the fix pins the conclusion to the named difference
(equalize every other field in the witness pair).

## C5 Bystander / frame-generic overclaim

FIRE on every uniqueness / confluence headline stated over `∀ frame` plus
named participants: can the named parties be instantiated as BYSTANDERS
while other owners inside the frame do the real steps? A frame-generic
quantification admits frames containing other owners' tokens, so "the named
pair is confluent / unique" is satisfiable with the named parties idle — the
theorem is true of a run its name never intended. The deciding witness is
the bystander instantiation (named parties inert, a frame owner fires). Fix:
index the hypotheses to the ACTUAL firing step/owner, and demote the
frame-generic form to an internal engine lemma the indexed headline is
proved from.

## C7 Word-class closed-world doc sweep

Fire on every constructor/rule/arm addition: after adding a constructor /
rule / instruction, grep the ENTIRE module for every closed-world WORD-CLASS —
NOT a phrase list and NOT just the old declaration name (`Step :=`): "both",
"only producer", "the producer", "the only", "all N" / "two rules" / "N
rules", "the persistent token", "yet", "no rule … yet", "later increment",
"deferred". Each hit IN A COMMENT is a candidate stale closed-world claim a
new constructor may have falsified (a "two rules" comment next to a
now-six-arm def FALSELY NARROWS the case split a reviewer audits). The
lesson: a phrase-list or old-decl-name sweep MISSES whole clusters (it greps
`Step :=` and skips the "only producer" / "no routing yet" / token-taxonomy
comments) — the word-class sweep is exhaustive where the phrase list
re-commits the narrow-grep miss it was meant to fix.

## Obligation transfer to a durable successor

When an invariant protects a LIVE token later consumed into a DURABLE one
(`pending → observed`), ask whether the support obligation should transfer to
the successor. A co-presence clause (`pending has matching intent`) is
SUPPORT, not PROVENANCE — it cannot express authorship when the durable token
carries no owner (real provenance is a run/replay property, not a state
invariant). Name the clause for exactly what it discharges
("…-has-matching-… *support*"), and record the successor-obligation question
explicitly rather than letting the name imply the stronger property.
