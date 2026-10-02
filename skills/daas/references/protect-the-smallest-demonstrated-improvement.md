# Protect the smallest demonstrated improvement

Improve one narrow capability, reproduce the new baseline, and add a ratchet
that prevents losing that gain. Repeat through independently useful changes.
Thresholds protect measured results, not aspirations; include the artifact,
environment, measurement scope, and limitations behind the baseline.

Adopt stronger practice gradually without requiring an unrelated rewrite.
Prevent new violations at the frontier of change while keeping existing
violations discoverable. Discovery and enforcement must use the same rules.
Promote a broader scope to enforcement only after it demonstrably complies. A
newly introduced rule can trigger a codebase-wide audit: fix violations or
record explicit exceptions, prevent new violations, and burn down the existing
exceptions. This is a purposeful event, not a mandate for periodic polishing.

For a new lint, cheaply fixable violations can be repaired before enabling the
hard gate. Substantial inherited debt may instead use an explicit finite
allow-list while new violations fail. The shrinking list is the ratchet; each
entry need not have its own expiry. Burn it down opportunistically according to
available capacity, including small warm-up or blocked-time tasks, without
inventing a fixed cleanup schedule or priority algorithm. These are violations
to remove, unlike justified permanent applicability exceptions.

During a migration, continued backfill may be useful while an unexpected state
is investigated if it improves the baseline without another regression. A
cheaply reversible relaxation can sometimes restore service; information loss
requires stopping the harmful operation and fixing it. Prefer a forward fix when
the cause and correction are clear; consider reverting or relaxing the change
when they are not. These are intentional recovery decisions, not a license to
silently weaken a gate.

The tool can be prototyped first, but retained history must not depend on
uncommitted tooling. Combine improvement and enforcement only when they cannot
form independently valid steps. Preserve product and process changes as
separable atoms where possible.

Seek *antifragility*: use stress and discovered failures to strengthen the
system. Stress alone earns no improvement; capture its information in a
corrected model, test, constraint, proof, or tool that preserves the gain. Apply
this within the authority and risk limits of the experiment.

Make the ratchet remember improvements for you. Where a metric supports a
reproducible exact baseline, detect changes in both directions: regression
fails, and improvement requires tightening the declared baseline before it can
silently be lost. Likewise, detect when an exception is no longer needed and
require its removal. Preserve demonstrated gains mechanically under the accepted
meaning of improvement; do not let the mechanism redefine that goal.

A claimed process improvement should survive an experiment. After reproducing
and fixing an escape, ask what could realistically have detected that class of
failure earlier. Where feasible, apply the proposed guardrail to the historical
defective state, before the new regression test and fix, and demonstrate that it
exposes the defect. Preserve a validated lesson; do not manufacture hindsight or
insist every escape was preventable. The replay and commit procedure belongs in
a focused skill.

**Ratchet improvements, not immutable conclusions.** A ratchet guards against
accidental regression; sufficient contrary evidence and a reasoned decision can
justify reversing it. Established properties deserve a higher burden of change,
not eternal protection from learning. Intentionality remains the governing
principle.
