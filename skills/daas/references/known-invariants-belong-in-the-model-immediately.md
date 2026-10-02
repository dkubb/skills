# Known invariants belong in the model immediately

Express known constraints using the strongest natural mechanism that faithfully
represents them: types, smart constructors, database constraints, proofs where
practical, and checks or tests for the remainder.

A known invariant is part of the specification. Making it impossible to violate
removes defensive work from every downstream consumer.

Do not wait for a mistake before encoding a known constraint. If a particular
domain requires names of 1–100 characters, represent that contract with a
constrained type and constructor rather than an unconstrained string.

The example bounds illustrate the principle; they are not a universal policy for
people's names. Substantial custom enforcement, such as a large bespoke lint,
has a separate maintenance burden and needs justification. That burden must not
become an excuse to omit ordinary domain modeling.

Constrain the representation the current program needs; defer capabilities with
no current consumer. Even an approved feature expected next week need not
dictate today's generalization: priorities change and real consumers supply
better information. Make the decision when current work cannot proceed correctly
without it. A known prerequisite belongs first in the dependency graph;
speculative future structure does not. Strong modeling removes invalid choices,
while YAGNI avoids adding unsupported ones.

Actively discover invariants rather than waiting for a complete specification to
enumerate them. Examine individual facts and their relationships, express each
genuine constraint over its necessary dependencies, and test the resulting model
against real data and use. A rejection may expose an incorrect model or invalid
data admitted by earlier enforcement; investigate before choosing which to
change. Do not invent relationships merely to constrain more states or exclude
valid cases. Use `state-space-minimization` for representation design. Search
individual fields, then pairs and larger subsets for genuine predicates;
minimize each predicate to its necessary fields. Independence is a valid result,
and pairwise independence does not rule out higher-order constraints. Separate
independent constraints from derived consequences. Exhausting subsets does not
prove that every relevant invariant was found.
