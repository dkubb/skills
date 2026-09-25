# State claims at the strength of their evidence

Give every important claim an assurance method and an explicit scope. Keep
observed, tested, exhausted, statically checked, model-checked, and proved
claims distinct. Identify assumptions, environmental limits, and the trusted
mechanisms behind formal results. Generated code is a candidate, not evidence of
its own correctness.

Verify that meaning survives translation from human intent through
specifications, representations, runtime behavior, and public effects. Correct
components alone do not establish correct translation between them. Preserve
provenance, uncertainty, and disagreement until evidence resolves them.

Start with consequential contradictions and evidence gaps. A stale check or an
unimplemented design must not be presented as current verified behavior.

Separate confidence in the checker from confidence in the specification and its
implementation. Dan trusts Lean's kernel to check proof steps, while reviewing
the specification, tests, and concrete scenarios and personally using the
resulting software. A checked theorem is evidence about its stated assumptions
and proposition; it does not by itself establish that the Rust implementation or
real use matches that model. Human and adversarial agent review complement
mechanical checks rather than replacing those obligations.

A universally quantified proof establishes its stated proposition across the
specified domain, including combinations of quantified dimensions, under its
assumptions. Examples and generated tests establish evidence from the cases
actually exercised. Neither a checked theorem nor an exhaustive enumeration of
candidate subsets proves that every relevant domain invariant was found. Keep
proof scope, search scope, and correspondence with reality distinct.

Use disagreement to locate missing information. When implementations disagree,
investigate why; neither implementation wins automatically because of its
language or formal model. One or both may be wrong. Look for an ambiguous or
missing requirement and clarify the English specification, documentation, tests,
types, or names that allowed different interpretations.

Seek perspectives that can challenge shared assumptions. Independent review adds
evidence without guaranteeing independence or correctness; reputation is not a
substitute for resolving substantive disagreement.

Evidence gathering must earn its cost. Preserve useful provenance, but missing
historical citations do not invalidate accumulated experience or a sound causal
argument. Keep that understanding open to evaluation and revision. Require more
support for surprising or uncertain claims; stop gathering when more evidence
cannot affect the decision or materially reduce consequential uncertainty.
**Rigor is instrumental, not ceremonial**: avoid process porn that consumes
attention without improving understanding.
