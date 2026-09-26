# Learn through bounded, reversible experiments

Plan at a high level, investigate risky or central pieces more deeply, and
alternate modeling with implementation to obtain real feedback. Prefer an easily
reversible choice when otherwise equivalent.

“Information creates information”: evidence from a real slice improves the next
decision. Detailed plans for everything can delay the evidence that would change
them. *Code is an experiment*: when a relevant attempt is cheap and cheap to
undo, trying it may cost less than further debate. Include the cost of reversal
in that judgment; reversibility alone does not make an experiment worthwhile.

Break experiments into the smallest coherent, verifiable steps. Decomposition
itself tests the mental model: a failed gate at an intended boundary can reveal
a dependency that a larger change concealed. Passing gates provide evidence
within their scope, not proof that no smaller coherent step exists.

Start from a meaningful central point, an important unknown, or a tractable
piece rather than always starting at the lowest dependency. Typed stubs can
expose missing dependencies without requiring the entire system to be designed
first. Harden an insight while its context is fresh when useful.

Choose experiments that address the current bottleneck, dependency, or material
risk; cheapness alone does not make work valuable. Comparing implementations,
synthesizing a candidate, or exploring in parallel can help when evaluation and
cost justify them. None is mandatory; use contextual stopping criteria.

Choose exploratory rigor for its information value. Strong types may expose a
false model early even in a tracer bullet; full mutation coverage may be
unnecessary for an explicitly disposable proof of concept. Demonstrate the
uncertain behavior to establish viability, then continue only while learning
useful design information. Rebuild when the experiment encodes assumptions you
would no longer choose. Retain it when the gap from the design you would now
build is small and it meets the required production bar; a prototype is neither
automatically disposable nor automatically production-ready.

Treat uncertainty and risk as bottlenecks alongside implementation dependencies.
Consider direct and transitive work that a decision blocks or could invalidate.
A familiar, low-risk dependency may be stubbed during exploration so attention
stays on the important unknown; a risky dependency may be the experiment itself.
This is a judgment heuristic, not a mandated numerical scoring system.

Evaluate additional assurance against the established rigorous baseline. Credit
contradictions resolved before implementation and rework avoided when
requirements change, as well as detected defects. Observe maintenance value over
time; an absence of failure alone establishes neither that the extra layer
caused success nor that it was unnecessary. Investigate observed friction before
discarding an approach or committing to it indefinitely.

Keep plans provisional while new information can improve them. Decompose only as
far as current understanding supports, then refine before action. Preserve what
actually happened when changing the future plan; implementation-specific freeze
boundaries belong in the workflow model.

Use **Theory of Constraints** to find what blocks useful learning. Deployment
and actual use often supply the most valuable feedback: put a sufficiently
assured system to work, with independent investigations in parallel when useful.
Do not delay that feedback for an unrelated experiment.

Reversibility has value and costs of its own. Commit when the expected benefit
justifies commitment and possible reversal, with stronger evidence as exposure
grows. Evaluate every tool, skill, and principle against demonstrated value,
expected future benefit, and continuing cost. Do not invent a stopping threshold
before experience provides one.
