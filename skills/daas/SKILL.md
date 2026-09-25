---
name: daas
description: Apply Dan Kubb's engineering judgment to software design, implementation, review, testing, and process decisions. Use when asked for dkubb as a skill, Dan's principles, or a review against his engineering preferences.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-09-v1"
---

# dkubb as a skill

## When to Activate

- Apply Dan Kubb's engineering principles to design, implementation, review,
  testing, or process decisions.

## When Not to Use

- The task does not call for engineering judgment or Dan's preferences.

## Inputs

- The decision or artifact, its domain contract, constraints, and available
  evidence.

## Outputs

- A concrete judgment or improvement, with reasons, relevant trade-offs, and
  evidence limits.

## Utilities

- Load the linked explanations for principles relevant to the decision,
  including their qualifications and exceptions.

## Process

- Use this rubric where judgment has not been encoded as a rule. Surface
  contradictions so choices become intentional; respect explicit user decisions.
- Practice recursive self-improvement: make assumptions explicit, acquire
  feedback, encode learning, and test it against reality. No model is final.
- Apply the principles across fields, systems, workflows, and organizations.
  Optimize for useful software and affordable change throughout its life; tools
  must repay their construction, verification, and maintenance costs.
- Start with the best version current understanding permits, then refine it from
  feedback.
- Resolve competing principles intentionally in context; do not invent a
  universal ranking.
- Use the outline to find relevant principles, not as a mandatory sequence.
- Treat technology examples as illustrations; preserve their underlying
  obligations using mechanisms appropriate to the task. Apply the time-travel
  test: would the guidance still matter if every mechanism were replaced?

### Decide what matters

- [Make trade-offs and exceptions intentional, with visible reasons and costs.](./references/be-intentional-including-when-making-exceptions.md)
- [Choose assurance that meets the consequences of failure and remains maintainable.](./references/criticality-determines-required-assurance.md)
- [Prevent invalid states where practical and detect mistakes as early as possible.](./references/prefer-prevention-and-fast-feedback.md)

### Make correctness natural

- [Represent all required domain states and as few invalid states as possible before optimizing layout.](./references/model-the-domain-before-optimizing-its-representation.md)
- [Encode known invariants immediately and actively discover missing constraints.](./references/known-invariants-belong-in-the-model-immediately.md)
- [Parse untrusted input into values that carry enforceable guarantees across trust boundaries.](./references/parse-don-t-validate-carry-a-witness-across-trust-boundaries.md)
- [Admit known valid states, reject unexpected states, and revise mistaken domain assumptions.](./references/admit-known-valid-states-and-reject-unexpected-ones.md)
- [Separate initial admission guarantees from the stronger guarantees individual consumers require.](./references/separate-admission-from-consumer-specific-refinement.md)
- [Preserve distinct states despite equal outcomes, keep migrations valid, and record intent and observed results separately for consequential external work.](./references/preserve-distinct-states-even-when-outcomes-coincide.md)
- [Prefer total functions whose types express every supported input and outcome.](./references/prefer-total-functions-over-informal-preconditions.md)
- [Grant only the capabilities needed for the current task.](./references/grant-only-capabilities-that-are-actually-needed.md)

### Keep behavior understandable

- [Make code and planning dependencies explicit, placing operations at the lowest capable layer that owns their data.](./references/make-real-dependencies-explicit.md)
- [Abstract shared meaning and generalize only from demonstrated variation.](./references/abstract-known-meaning-generalize-from-demonstrated-variation.md)
- [Optimize for local understanding rather than minimizing line count.](./references/optimize-for-local-understanding-not-line-count.md)
- [Contain complexity in code, dependencies, and deployment; count the maintenance burden on both sides of each interface.](./references/contain-complexity-and-account-for-who-maintains-it.md)
- [Add performance complexity only to satisfy a demonstrated requirement.](./references/performance-complexity-must-answer-a-real-requirement.md)
- [Make behavior deterministic wherever possible and expose unavoidable variability.](./references/make-behavior-deterministic-wherever-possible.md)
- [Keep derived state visibly derived, investigate contradictions, and distinguish retained evidence from active context.](./references/derived-state-must-remain-visibly-derived.md)

### Establish confidence

- [State every claim at the strength and scope its evidence supports.](./references/state-claims-at-the-strength-of-their-evidence.md)
- [Normally keep direct unit and applicable property tests for public functions, mutation evidence for retained tests, and independent boundary defenses.](./references/keep-local-evidence-and-defense-in-depth.md)
- [Characterize insufficiently tested code and establish execution coverage before mutation testing or behavior changes.](./references/establish-what-was-exercised-before-judging-sensitivity-to-change.md)
- [Test components and composition at their own scopes; unexpected verification failures stop affected work.](./references/test-components-and-their-composition-at-the-appropriate-level.md)

### Deliver and learn

- [Preserve an unchanged behavioral oracle while refactoring.](./references/preserve-an-unchanged-oracle-during-refactoring.md)
- [Make published history an understandable sequence of independently valid commits.](./references/make-change-history-an-understandable-sequence-of-valid-states.md)
- [Distinguish exploratory checkpoints from work ready to publish with required assurance.](./references/distinguish-provisional-work-from-publishable-assurance.md)
- [Learn through bounded, reversible experiments without sacrificing the established baseline.](./references/learn-through-bounded-reversible-experiments.md)
- [Investigate disagreement and revise the understanding that evidence challenges.](./references/investigate-disagreement-and-refine-the-understanding-it-challenges.md)
- [Protect the smallest demonstrated improvement with a proportionate, revisable ratchet.](./references/protect-the-smallest-demonstrated-improvement.md)
- [Keep rules small, version changed meaning, and converge consumers while honoring compatibility commitments.](./references/keep-the-engineering-system-small-and-evolve-it-explicitly.md)
- [Encode useful learning in its narrowest durable owner and require process to earn its cost.](./references/make-improvement-compound-without-allowing-process-to-take-over.md)

## Validation Checklist

- Preserve required valid behavior and distinguish established constraints from
  hypotheses.
- Read the relevant full explanations before resolving a tension or applying an
  exception.
- Match claims to evidence and make unresolved assumptions explicit.
- Keep the improvement proportionate to the problem and its maintenance cost.
