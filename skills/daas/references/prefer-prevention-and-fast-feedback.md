# Prefer prevention and fast feedback

Use *poka-yoke*: make impossible states unrepresentable and impossible actions
impossible where practical. Otherwise detect violations as early as the relevant
information permits.

A compiler error at an encoded dependency is cheaper to understand than a
delayed failure whose cause is elsewhere. The aim is local reasoning with
mechanical enforcement of the dependencies that extend beyond that locality.

Programmer discipline can be a practical starting point for rules that lack
natural enforcement, but recurring mistakes justify stronger tooling. This
concerns additional enforcement machinery; known domain constraints belong in
the model immediately ([known invariants belong in the model
immediately][related-1]). Prefer an environment that constrains even its
experienced author.

A type system only enforces facts actually encoded in it; compilation is not
evidence for arbitrary runtime behavior.

Optimize the speed and breadth of useful feedback per unit of finite attention.
Upfront effort must earn its lifetime cost. Repair the instance, investigate its
cause, then ask whether the whole class can be prevented or detected earlier.

Minimize the interval between introducing a defect and detecting it. Delay loses
context and lets further work depend on a mistaken model. Seek both feedback
that the implementation is correct and feedback that it solves the real problem.
Preserve earlier decisions with mechanical guardrails; then improve the speed of
those checks without weakening what they establish.

Prefer the simplest reliable enforcement that fails in the safer direction. When
a conservative check creates cheap, visible extra work while a missed violation
permits silent degradation, slight over-enforcement can be a good trade. Add
precision only when observed harm earns its complexity. This is a contextual
strictness bias, not permission to ignore harmful false positives. Early failure
is often useful friction that avoids a larger downstream cost.

When a valuable check is slow, first seek a strict baseline and verification
proportional to the affected scope. Local checks can run on changed code;
dependency-sensitive checks must follow everything the change can invalidate.
Only after those options fail should moving the check later or running it
periodically become the alternative. Preserve an honest account of what each
gate establishes.

[related-1]: ./known-invariants-belong-in-the-model-immediately.md
