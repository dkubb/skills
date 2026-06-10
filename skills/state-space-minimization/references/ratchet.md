# The ratchet

The project's threshold configuration is a state space, and the
ratcheting process is the state-space-minimization process applied
to that state space. Same vocabulary from `principles.md`, same
six operations, same audit questions — applied at the level of
project tooling configuration rather than at the level of types or
runtime behavior.

The rest of this module is the operational specifics derived from
that frame: how to set thresholds, how to ratchet them, how to use
tooling as the forcing function, and how to handle the new-project
and existing-project starting positions.

The ratchet pairs with `perfect-tool.md`: that file is the
*generative* move (imagine the perfect enforcer, design code and
tests as if it existed); this one is the *preservative* move (lock
in what the imperfect enforcer catches so the codebase only
tightens). The perfect tool defines the target; the ratchet keeps
the gains.

## Vocabulary at this level

| Term | Meaning when the codomain is the project's tooling configuration |
|---|---|
| Domain | Configurations the ratchet accepts as input — codebases passing the current floors |
| Codomain | Configurations the ratchet permits as output — codebases passing tightened floors |
| Range | The configurations the project actually occupies over its trajectory |
| Preimage of failure | The set of changes that could have caused a regression when CI trips |
| Type signature | The threshold set; the project's declared invariants |

## The bilateral goal applied here

1. **Shrink the domain.** Only narrowing changes pass through the
   ratchet. CI rejects anything that would regress a current floor.
2. **Close the codomain-range gap.** The floor sits at the exact
   edge of current state — what the project actually achieves
   today. The aspirational target names where the codomain is
   heading; new code aims for that target so the range can tighten
   and the floor follows.

## The other operations applied here

- **Shrink the input domain.** The ratchet rejects regressions;
  only narrowing changes are accepted at all. Same move as
  narrowing a function parameter to forbid invalid inputs.
- **Bound cardinality.** Each change is atomic and from the closed
  transformation set (see `commits.md`). One narrowing per commit.
  Each commit passes every gate.
- **Shrink the codomain.** Each threshold defines exactly what the
  project permits at that metric. As the codebase narrows, the
  threshold's codomain follows.
- **Remove invalid intermediate representations.** Every commit
  passes every gate; no half-ratcheted state is representable in
  the history.

## The dual threshold

Every metric carries two values, both required:

- **Floor** — the maximum value currently tolerated. Lint or CI
  enforces it. Set at the *exact edge of current state* so any
  regression trips the check immediately. The floor is never
  loose; "100 lines max when current worst is 70" is wrong because
  it permits 30 lines of slack to backslide into.
- **Aspirational** — a target tighter than the floor. New code
  aims here; existing code burns down toward it over time.
  Aspirational becomes the new floor once the codebase reaches it.

The floor is `max(current_project_state, community_baseline)` —
whichever is stronger. We never relax to a community baseline that
is looser than the project's current state. If the project already
exceeds the community baseline, the floor stays at the stronger
value; the community baseline becomes irrelevant for that metric.

## The ratchet

The ratchet is what turns state-space minimization from a one-time
audit into a continuous process. Without it, drift is the default —
the same lesson `documentation.md` draws about prose claims,
applied to project configuration.

- Capture the current value of each metric. That is the floor.
- The floor only ever tightens. Improvements update it; regressions
  cannot pass CI.
- Keep the floor at the exact edge of current state — not loose,
  not arbitrary — so any backslide trips the check immediately.
- As individual hotspots are addressed and the range narrows, the
  floor ratchets down in lockstep. A PR that brings the worst
  function from 70 lines to 50 lines updates the floor to 50.
- Aspirational targets are promoted to floors over time as the
  codebase reaches them.

The mental model: the floor is a vise. Each turn tightens it; no
mechanism loosens it. Hotspots get burned down then locked in by
the next turn of the vise.

## What to threshold

Every metric that admits a quantitative bound is a candidate. The
discipline is to **measure each metric in isolation** so each axis
can be optimized and ratcheted independently. Multidimensional
tradeoffs become tractable when each dimension has its own number.

Common metrics worth tracking:

- Function length, line length, file length
- Cyclomatic and cognitive complexity
- Argument count, indentation depth
- Test coverage (statement, line, region, branch, MCDC)
- Mutation score
- Assertion density
- Diff size per commit (see `commits.md`)
- Dependency count and depth
- Type-system strictness flags (e.g., `noImplicitAny`,
  `strictNullChecks`, `clippy::pedantic`)
- Public-API surface area

This module does not name specific numeric defaults. The right
number depends on language ecosystem and project domain. When
numeric anchors are useful, they belong in `languages/*.md` files
where the ecosystem context is explicit.

## Tooling as the forcing function

Every threshold worth enforcing needs a tool that fires when the
threshold is crossed. The forcing function is what makes the
ratchet operational; without one, drift is the default and the
threshold becomes prose that decays.

Where automated tooling exists, use it:

- Static analysis (linters, type checkers, complexity tools)
- Mutation testing
- Property-based testing
- Coverage tools with diff-coverage modes
- Pre-commit hooks and CI gates

Where automated tooling does not exist for a metric, the threshold
still lives in a project document — a `THRESHOLDS.md`, a
`CLAUDE.md`, or equivalent — so the LLM has a frame of reference
and human reviewers have an audit checklist. A written threshold
without a forcing function will drift; the discipline is to write
the threshold down anyway so the gap is visible and a forcing
function can be built later.

## Weakening a threshold

Not encouraged. Acceptable only on explicit user request — the LLM
never relaxes a threshold on its own initiative. If a threshold
blocks legitimate work, surface the conflict and let the user
decide.

When a weakening is approved, document the reason. The reason is
usually a constraint that forced the relaxation (a specific
legacy file, an external integration that the threshold cannot
yet accommodate, a deliberate trade-off). When the constraint no
longer applies, the threshold can be re-tightened. Each
documented weakening is an outstanding ratchet opportunity, not a
permanent state.

This is the project-tooling instance of the weaken-before-
strengthen asymmetry in `principles.md` § "Weaken before
strengthen": a too-strong threshold is refuted directly (CI fails
on a legitimate change); the weakening that lets the change pass
is correction of a refuted hypothesis, not regression. A too-weak
threshold admits invalid configurations silently; strengthening it
later requires auditing every state that slipped through under the
loose value.

## Estimated bounds: the ratchet's mirror

A bound chosen without a spec (`principles.md` § "Bound ranges and
cardinality": a plausible limit rounded up to the nearest power of
two) runs the ratchet in reverse. Start strict, then widen on
evidence: a rejection at an estimated bound is signal about the
real domain, and widening is contract-preserving — every previously
accepted value stays accepted. This is the same
weaken-before-strengthen asymmetry as above, applied to a bound
whose true value is unknown rather than a threshold whose value is
chosen.

Provenance decides which regime a bound is in. A spec-derived bound
is fixed: it never ratchets and never widens. An estimated bound is
provisional in both directions: widen it when real valid traffic
hits it; tighten it (ordinary ratchet direction) when evidence
shows the estimate was too generous. Widening an estimated bound on
concrete evidence is correction of a refuted estimate, not a
regression — record the evidence and the new value where the bound
is enforced.

## New project vs existing project

- **New project.** Set the floor at the strongest available value
  from day one — community baseline, or stronger. Set the
  aspirational target tighter still. Write all code to the
  aspirational target. There is no legacy to migrate; the ratchet
  starts at the destination.
- **Existing project.** Measure current state per metric. Floor =
  `max(current_state, community_baseline)`. Aspirational = a
  chosen stronger target (often the community baseline when the
  current state is below it; a deliberate tighter target when the
  current state already exceeds the community baseline). Ratchet
  from current toward aspirational over time.

In both cases the floor sits at the edge of what the codebase
already achieves; the aspirational target names the next narrowing
the project is committed to reaching.

## Cross-references

- `principles.md` § "Burndown priority: infinities first" — the
  burndown-priority sieve is the type-tier instance of the ratchet
  this module describes at the project level.
- `commits.md` § "Atomicity" — the per-commit diff-size thresholds
  are one instance of this module's pattern, applied to the
  size-of-a-state-transition metric.
- `testing.md` § "Coverage is structural; biasing is the draw
  distribution" — the 100% target-function coverage rule is the
  aspirational target at this level; the project's current
  coverage is the floor.
- `documentation.md` § "The dominant failure mode is drift" — the
  same forcing-function thread, applied to prose claims rather
  than tooling configuration.
- `defensive-code.md` — once a threshold catches regressions of a
  given invariant, defensive code that guarded against that
  regression becomes deletable; the threshold is the upstream
  narrowing.
