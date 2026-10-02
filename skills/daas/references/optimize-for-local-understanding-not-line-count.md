# Optimize for local understanding, not line count

Write for human comprehension first, and LLM comprehension where it does not
conflict. Extract when a name, contract, smaller reasoning domain, or
independent test makes understanding easier.

Twenty tiny functions can obscure an algorithm; a long function can remain
understandable if its contract and intermediate states are constrained.

A 300-line SQL query with a clear input/output contract may be reasonable. A
short function with unconstrained interacting values can be harder to reason
about. Function length alone does not decide whether to extract.

A mechanical module-size limit can still be a useful default guardrail. An
exceptional module uses the visible, reasoned override from [be intentional
including when making exceptions][related-1]; this does not make function length
alone the extraction criterion.

Strong intermediate types can establish semantic checkpoints. Compiler inlining
may preserve execution efficiency without losing source-level verification
boundaries. Code must still be understandable by the agents expected to maintain
it.

Simplicity concerns the mental model loaded for the current task. A trusted
`UserId` can compress constructors, representation, and established guarantees
into one domain concept. More implementation can reduce downstream reasoning
when the contract lets callers safely forget those details. An interface with
many operations warrants an audit, not automatic splitting: look for
conditionals, distinct states, or operations that apply only to some values.
Decompose when those distinctions are real, rather than to meet a numerical
ideal.

**Idioms are compression.** Prefer concise, idiomatic expression when it
preserves correctness and maintainability. Write in a form an expert in the
language would recognize; expect the team to learn useful native idioms rather
than translating habits from another language. Introduce a better project idiom
when warranted and teach it explicitly. Familiarity informs comprehension, but
does not veto a demonstrably better approach.

[related-1]: ./be-intentional-including-when-making-exceptions.md
