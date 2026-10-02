# Criticality determines required assurance

Decide what failure the component may tolerate, then choose assurance that meets
that requirement. Cost informs the mechanism; it does not erase a critical
requirement.

The right trade-off differs between code that absolutely must not fail and a
component whose failure the system can safely handle.

Critical code may justify an expensive type-level invariant. For less critical
code, an understandable runtime assertion may be preferable to a type encoding
the team cannot maintain. First try to simplify the encoding and push the
constraint toward the relevant boundary.

Ordinary known constraints remain the default under [known invariants belong in
the model immediately][related-1]. Types have practical limits; weigh
compilation cost, readability, and criticality in context rather than imposing a
universal formula.

**Temporary runtime assertions are executable debt.** Push validation toward
ingress so internal values carry the guarantee. An assertion can remain when
time or a higher priority prevents stronger enforcement. Publishing that debt is
a last resort: ordinary tests and observed use must not trigger it, and a
deliberate negative test must exercise the assertion and kill its mutant. An
unexpected trigger becomes a blocker. When stronger enforcement makes the
failure unreachable, remove the redundant assertion and test. Repeated checks
inside a component, after its boundary has established the guarantee, need their
own cost or criticality justification. This does not relax [carry validation
guarantees across trust boundaries][related-2]: separate representation and
trust boundaries still express their enforceable constraints.

Include maintenance and future engineering capacity in delivery decisions. User
acceptance establishes feature value, but does not price the work a shortcut
will consume later or the more important work it may displace. Make that
liability explicit when a business deadline overrides the preferred bar. A
shortcut can be justified by the whole business trade-off. Aim for a stable
base: useful, fast enough, understandable, and supported by trustworthy
regression evidence that makes further change affordable. More rigor must
continue earning its cost; there is no universal stopping threshold.

[related-1]: ./known-invariants-belong-in-the-model-immediately.md
[related-2]: ./parse-don-t-validate-carry-a-witness-across-trust-boundaries.md
