# Overclaim & reception — defect class C (statement weaker than name/prose)

Load when auditing what a theorem's NAME, docstring, or surrounding prose
claims against what its STATEMENT actually says. This is the mutation-blind
class: the theorem can be true and every mutant can die while the reception —
what a reader takes away — is still wrong. Statement-surface defects need
statement-level audits, not more mutants.

## C4 Temporal-inversion test for bag / unordered state

When a theorem reads "A THEN B" but the state is an unordered bag / frame,
ask: *can a supposedly-LATER token sit in the INITIAL frame?* If yes, the
theorem proves COMPATIBILITY / REACHABILITY, not causal ARRIVAL — a
pre-supplied reply makes "the emit *sources* the reply" an overclaim (the
rule sources only what it actually mints; the reply is exogenous start-bag
input). Calibrate the claim to what the rule produces, and name the exogenous
tokens as environment input.

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
