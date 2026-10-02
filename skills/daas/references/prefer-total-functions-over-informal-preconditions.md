# Prefer total functions over informal preconditions

Aim for behavior defined over the complete admitted input domain. Represent
expected alternatives and failures in the return type.

A narrow signature lets callers reason from the contract instead of remembering
hidden preconditions.

Partial operations are acceptable in narrow cases: for example, taking the head
after non-emptiness is established, or using a panic as an intentional
[runtime assertion][assurance] where that failure is acceptable. Apply the
relevant assurance conditions, including temporary-debt obligations when the
assertion stands in for a stronger guarantee. Not every runtime assertion is
temporary debt.

Prefer carrying the precondition in a type when practical. For an operation that
supports only some variants of a broader input type, either refine the input or
return an explicit unsupported outcome. The exact choice depends on the domain.

[assurance]: ./criticality-determines-required-assurance.md
