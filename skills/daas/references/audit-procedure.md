# Audit using Dan's engineering principles

Evaluate the artifact from Dan's stated principles. Ground judgments in the
artifact, its intended use, and available evidence. Keep the audit read-only
unless changes are requested.

## Procedure

1. **Establish scope.** Identify the artifact or revision, intended behavior,
   criticality, maturity, and relevant project constraints. Inspect available
   contracts, callers, tests, and recorded decisions. Ask only when missing
   context materially changes the judgment; otherwise state the uncertainty.
2. **Select applicable principles.** Scan the [principle index](../SKILL.md),
   then read the relevant explanations and their linked qualifications. Include
   dependencies and boundaries the artifact can affect. Do not infer a rule from
   a summary alone or require every principle to produce a finding.
3. **Look for concrete evidence.** For each relevant principle, ask:
   - When does it apply here?
   - What observed behavior, representable state, dependency, or reasoning burden
     supports or challenges it?
   - Why does that distinction matter to correctness, comprehension, maintenance,
     or the required assurance?
   - Does an intentional trade-off or legitimate exception explain the choice?
   - What is the smallest useful improvement that preserves required behavior?
4. **Challenge the finding.** Read surrounding code and consumers before judging
   an isolated fragment. Seek evidence that could refute the concern. Use a
   counterexample or focused check when useful; for comprehension concerns,
   identify the facts a maintainer must reconstruct and why. Recognize guarantees
   already established by types or boundaries. Match verification effort to the
   consequence and uncertainty.
5. **Classify and prioritize.** Distinguish a demonstrated defect, a justified
   trade-off, missing evidence, and an unresolved preference. Missing evidence is
   not proof of a defect. A stated trade-off can still have unsupported premises
   or unacceptable consequences: explain those explicitly. Rank findings by
   consequence and scope; report confidence separately. Do not invent a universal
   ranking of principles or attribute generic preferences to Dan.
6. **Report actionable findings.** Lead with the most consequential findings.
   Combine observations with the same cause. Keep optional improvements separate,
   and state the scope reviewed, checks performed, and remaining uncertainty.
   If no actionable issue was found, say so without claiming completeness.

## Finding format

Use concise prose containing:

- **Observation:** exact location and concrete evidence; distinguish observation
  from inference.
- **Principle:** the applicable principle and why it applies, with a link to its
  explanation.
- **Consequence:** the failure, unnecessary state, reasoning burden, or maintenance
  cost; include the conditions under which it matters.
- **Improvement:** the smallest useful change, what it preserves, and how to check
  it; note relevant exceptions or unresolved assumptions.

Keep explanations proportional to the finding. Do not manufacture findings to
fill the format, prescribe implementation machinery without a demonstrated need,
or change the principles to make the artifact pass.
