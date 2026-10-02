# Keep local evidence and defense in depth

Use types and design to remove obligations, then test the behavior that remains.
Prefer meaningful boundaries, properties over valid domains, and invalid inputs
close to the admission boundary.

Every public function normally has direct unit tests and applicable property
tests. Exercise private behavior through its public callers, whose local tests
must also detect the relevant private-function mutations. Boundary examples and
property exploration complement one another over the admitted domain.
Integration tests do not discharge this local obligation, even when they kill
the same mutants: they establish composition at a different scope.

Judge redundancy *within a verification scope*. Tests of the same unit with the
same inputs, execution coverage, and mutant-kill set are candidates for
consolidation. Ordinary overlap is expected; overlapping evidence across unit,
integration, and system scopes is intentional defense in depth. Remove duplicate
evidence that adds no assurance within its scope, not a whole layer because
another layer exercises the same code.

Remove a downstream regression test when a stronger mechanism has eliminated its
obligation. Every retained test must exercise the code under test and kill at
least one mutant. If types or proofs reject all relevant mutations statically, a
test no longer needs to distinguish them at runtime. Do not manufacture
impossible internal values solely to preserve the old test.

**Unresolved:** what evidence satisfies the mutation requirement when current
operators cannot express a real defect a test detects? No exception is
established; surface the gap rather than treating it as resolved.

A test that kills no mutants may be ineffective rather than obsolete. First
investigate whether a valid starting state or different input restores its
purpose. Do not keep a test solely as documentation after its verification value
is gone; preserve that knowledge in the simplest appropriate prose. Mutation
testing can also expose needless expression complexity. Simplify while
preserving answers over the full input domain, using the unchanged tests as the
oracle under [preserve an unchanged oracle during refactoring][related-1].

A well-typed constructor can still store the wrong value of the right type. Keep
tests that distinguish those errors unless the stronger guarantee also
establishes the input/output relationship. Coverage and mutation evidence apply
to the remaining boundary and implementation, not just callers.

A surviving mutant is a question to investigate, not proof of dead code. The
state might be reachable and the test missing. Sampled property tests are not
exhaustive proofs; mutation scores do not establish behavior outside the
exercised scope. Speculative bit-flip defenses are not a default requirement.

Keep Lean and Rust assurance local to their artifacts. A theorem about the Lean
model does not kill a Rust mutant: Rust construction and types must make the
change impossible, or Rust tests must detect it. An executable Lean
implementation would need its own checks too. Differential agreement adds
another dimension of evidence and never replaces required unit, property,
mutation, or system checks. Properties derived from the model help check the
translation, but testing does not transfer a proof to Rust.

[related-1]: ./preserve-an-unchanged-oracle-during-refactoring.md
