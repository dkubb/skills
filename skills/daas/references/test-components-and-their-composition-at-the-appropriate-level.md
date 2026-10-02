# Test components and their composition at the appropriate level

Isolate I/O behind a concrete test seam when useful. Test the adapter and core
separately over their typed contracts, then use integration and deterministic
simulation to investigate composed behavior and faults.

Local input exploration and system sequences answer different questions. I/O
boundaries have repeatedly been important sources of failures in Dan's
experience.

Strong types and unit mutation coverage do not make integration tests
unnecessary. When a system-level check exposes a local contract defect,
reproduce and strengthen the smaller boundary too.

Stateful property-based tests can cover sequences. The useful distinction is the
modeled scope and observer, not a rigid division between tool names. Agreement
between implementations is also weaker than proof of correctness.

Use higher-level exploration to find gaps in the verification process itself.
When it exposes a defect, shrink the reproducer to the narrowest scope where the
failure remains real; an integration or targeted system test is appropriate when
unit scope cannot express it. Fix against that regression, then replay the
original discovery to verify the diagnosis. **DST is for exploration**: retain
the seed as review provenance, rather than automatically making that trajectory
a permanent CI regression. Keep exploring; do not suppress a legitimate state to
hide a failure. Investigate whether the implementation, handling, asserted
invariant, or simulation model is wrong.

Low discovery frequency alone does not retire cheap DST insurance. Allocate
exploration by criticality and cost: continuous spare compute is attractive;
merge-triggered, nightly, or dedicated capacity may fit other systems. A costly
suite that adds no distinct evidence over a long period can be reconsidered, but
backfilled regressions do not automatically replace its exploratory role. Keep
flakiness an investigated defect, not accepted noise.

Value a different exploratory technique when it exposes faults the existing
methods miss, even when their coverage and mutation metrics are already strong.
Reduce discovered failures to the smallest meaningful, fast regression check;
retain the exploratory method while it continues to earn its discovery value.
This distinction extends beyond DST to other ways of probing the system.

## Stop the line; relax deliberately

An unexpected verification failure initially stops affected change progression
and gives investigation priority, even if the defect looks trivial or lies in
the harness. Tolerated failures train people to ignore the signal. Restore trust
in the product and assurance machinery before resuming. A contained third-party
defect may justify a narrow, visible quarantine when other needs warrant it;
this is a reluctant exception, not automatic permission to continue. Temporary
relief expires back to hard failure under [be intentional including when making
exceptions][related-1]. Determine the stop scope from actual dependencies:
independent work can continue while affected work pauses. A shared harness
failure can invalidate confidence across many dependents; apparent separation is
not enough if they rely on the same broken evidence.

[related-1]: ./be-intentional-including-when-making-exceptions.md
