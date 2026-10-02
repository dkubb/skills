# Investigate disagreement and refine the understanding it challenges

Treat code and constraints as an explicit current understanding. Investigate
counterexamples, locate the mistaken understanding, and preserve useful
regression evidence. Disagreement is an opportunity for learning; it does not
pick the winner. Check the observation, interpretation, evaluator, model, and
environment in light of prior evidence and what changed. Mature knowledge earns
confidence without immunity from revision.

Repeated local patches can hide an incorrect model. Better constraints can
remove an entire class of mistakes and reduce future reasoning.

All code is a maintenance liability. Prioritize improvements by cost,
criticality, frequency, and what they unblock. Usually refactor where work
requires a change rather than searching unrelated working areas for cosmetic
improvements.

Good design compresses meaningful information and excludes invalid information;
compression that conflates different domain facts is not an improvement.

A discovered broken window deserves elevated priority. A quick repair can cost
less than recording, rediscovering, and scheduling it; keep that repair an
independent, appropriately classified change. Work already in progress has
priority from its context and restart cost: equal or slightly higher nominal
priority need not justify interruption. Switch when the benefit outweighs that
cost, rather than churning whenever another imperfection appears.

A specification models current knowledge; it is not a claim that reality has
been completely described. Constrain known use cases without inventing
hypothetical exceptions, then investigate legitimate counterexamples and revise
the model precisely. Changed requirements can expose conflicts with encoded
obligations earlier, but checks cannot reveal requirements never represented.
Use real operation to test the remaining gap. Before debating how to tolerate
inconsistent duplicate state, ask whether it should simply be derived instead.

Close the loop against the original problem and actual use. Review both the
specification and implementation when they drift; neither is immune to evidence
from the other or from reality.

The ordinary interruption trade-off above does not weaken the stop-the-line
response to unexpected verification failure in [test components and their
composition at the appropriate level][related-1].

Implementation creates information. An interview depends on what someone thinks
to ask; constructing types, attempting proofs, writing tests, and investigating
surviving mutants can expose questions neither participant anticipated. Treat
that friction as a signal to investigate, whatever mechanism produced it. Do not
dismiss a contradiction simply to restore apparent consistency. Resolve the
missing understanding and feed it back into the model.

**Friction is a signal, not inherently waste.** Investigate whether it comes
from a flawed rule, poor tooling, a communication or interface problem, or
necessary difficulty. Remove accidental friction and deliberately choose where
to pay unavoidable costs. If a strong diagnosis already exists, test it
directly; otherwise start with plausible hypotheses that are cheap to falsify.
Do not force every investigation through a ceremonial search procedure.

Keep the learning loop open. A strict model should make surprises visible so
legitimate counterexamples can refine the specific boundary that was wrong.
Neither permissive silence nor indiscriminate relaxation teaches the missing
distinction.

[related-1]: ./test-components-and-their-composition-at-the-appropriate-level.md
