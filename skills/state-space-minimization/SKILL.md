---
name: state-space-minimization
description: >-
  Minimize invalid and redundant states in types, boundaries, tests, and
  engineering artifacts while preserving domain facts. Use for parsing,
  normalization, proof preservation, strict matchers, and ratchets.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-06-v4"
triggers:
  - "state space minimization"
  - "make invalid states impossible"
  - "make illegal states unrepresentable"
  - "impossible states unrepresentable"
  - "parse don't validate"
  - "primitive obsession"
  - "smart constructor"
  - "boolean blindness"
  - "typestate"
  - "session types"
  - "phantom types"
  - "branded type"
  - "opaque type"
  - "ghosts of departed proofs"
  - "shotgun parsing"
  - "constructive data"
  - "constructive vs predicative"
  - "intrinsic type safety"
  - "refinement types"
  - "liquid types"
  - "pattern types"
  - "substructural types"
  - "Hyrum's law"
  - "serde validation bypass"
  - "domain primitive"
  - "domain modeling invariants"
  - "aggregate invariants"
  - "value object invariants"
  - "single case discriminated union"
  - "capability token"
  - "object capability"
  - "least authority"
  - "principle of least power"
  - "rule of least power"
  - "total functions"
  - "tighten types"
  - "narrow types"
  - "narrow domain"
  - "narrow codomain"
  - "narrow state"
  - "tighten matchers"
  - "strict matcher"
  - "tighten constraint"
  - "tighten regex"
  - "tighten regexp"
  - "anchor regex"
  - "bounded quantifier"
  - "simplest interface"
  - "fewest degrees of freedom"
  - "narrowest primitive"
  - "side effect coverage"
  - "preimage of failure"
  - "surviving mutant"
  - "alive mutation"
  - "equivalent mutation"
  - "commit state-space transition"
  - "commit message as type signature"
  - "atomic commit action verbs as closed set"
  - "gate trailer as typed proof"
  - "documentation drift"
  - "doc drift"
  - "doctest drift"
  - "executable documentation"
  - "documentation as type signature"
  - "natural-language type signature"
  - "defensive code"
  - "just-in-case code"
  - "overly defensive"
  - "redundant validation"
  - "redundant check"
  - "dead defensive check"
  - "unreachable branch"
  - "catch-all handler"
  - "ratchet"
  - "ratcheting"
  - "threshold as forcing function"
  - "aspirational threshold"
  - "tooling threshold"
  - "lock in improvements"
  - "perfect linter"
  - "perfect formatter"
  - "perfect mutation tester"
  - "imagined enforcer"
  - "imagined tool"
  - "consistency over optimality"
  - "as if enforced"
  - "project the perfect form"
  - "rubric as program"
  - "normalization as state-space minimization"
  - "database normalization"
  - "documentation normalization"
  - "code normalization"
  - "denormalization drift"
  - "single source of truth invariant"
  - "DRY as state-space"
  - "derive don't store"
  - "passthrough function"
  - "passthrough chain"
  - "sprawl as state-space"
  - "god module"
  - "god function"
  - "over-decomposition"
  - "under-decomposition"
  - "merge functions by determinant"
  - "split function by determinant"
  - "atomic decomposition"
  - "decompose then recompose"
  - "lattice of decompositions"
  - "topological sort as progressive disclosure"
  - "topological order for documentation"
  - "progressive disclosure invariant"
  - "progressive disclosure as topological sort"
---

# State Space Minimization

**Represent only valid states, using the lightest mechanism that
actually removes the invalid states.** Narrow both domain and
codomain — at the type level (data, signatures), at the boundary
(parsing, deserialization), at the system level (architectural
scopes), and in tests (matcher precision). Each
representable-but-invalid state is latent bug surface; each
unnecessarily heavyweight mechanism is added surface the invariant
did not buy.

Here, "state" means any accepted behavior of the artifact under review:
runtime values, wire shapes, tests, documentation claims, commit
boundaries, tooling thresholds, and agent instructions all have state
spaces.

Minimization is contract-preserving. Do not delete, reject, or make
awkward any state the real contract requires merely because a smaller
surface looks cleaner; strictness is a means to remove invalid states,
not an objective by itself. Preserve distinguishable facts and genuine
uncertainty. The same current handling does not make two classifications
semantically equivalent; a shared policy result can be derived without
discarding the authoritative classifications.

See `references/principles.md` for the full statement (search
algorithm, formal vocabulary, weaken-before-strengthen,
self-similarity).

## Progressive-disclosure invariant

This skill is effective only when the loaded material is the minimum
module set that can answer the task. Progressive disclosure is an
invariant, not a presentation preference:

1. Classify the task into one or more output profiles before loading
   technique modules.
2. Load `references/principles.md` first, then only the modules needed
   by those profiles, in router order.
3. Do not emit outputs for modules that were not loaded. If evidence
   crosses into another module's scope, load that module before using
   its vocabulary or producing its outputs.
4. Prefer a precise module-specific answer over a full-skill checklist.
5. Use the end-to-end linear read order only when the user explicitly
   asks for a full load, a whole-skill audit, or a cross-module
   consistency pass.

The progressive-disclosure audit question is: **Could a smaller loaded
module set have produced the same correct answer?** If yes, the pass was
too wide.

"Smallest module set" means smallest sufficient set, not smallest
possible read. Under-reading evidence is also a progressive-disclosure
failure.

## Active participation in skill refinement

The skill is itself a state space — see `references/principles.md`
§ "Self-similarity". When this skill loads, apply its rules to code
already seen this session, not just code reached after load; for a
language without a `references/languages/<lang>.md` file, derive
guidance from `references/principles.md` only. Capture
counter-examples (rule did not fit) and tightening opportunities
(rule could be sharper) as you work; for ordinary tasks keep this
capture internal unless it produces a material finding. Load
`references/skill-refinement.md` — the finding categories, finding
format, report shape, audit rules, and effectiveness audit — only
when the task asks to audit or improve this skill, repeated use
exposes drift, or a concrete case produces a finding worth user
attention.

## When to Activate

- Reducing invalid states in a design.
- Parsing instead of validating at a boundary.
- Replacing primitive obsession with domain types.
- Adding smart constructors or tighter type boundaries.
- Encoding an invariant via typestate, phantom tags, capability tokens,
  or refinement types.
- Tightening function signatures (input domain, output codomain).
- Tightening test matchers so they accept fewer behaviors.
- Writing or auditing commit messages and ordering commits in a
  branch by transformation priority.
- Writing or auditing documentation; deciding whether an invariant
  belongs in a rich type, a doctest, a co-located comment, or
  boundary prose.
- Auditing defensive checks; deleting just-in-case code; narrowing
  catch-all exception handlers to specific failure modes.
- Configuring lints, CI gates, or other forcing functions; defining
  or ratcheting project thresholds; deciding the starting position
  for a new or existing project.
- Designing code, tests, or tooling against an imagined perfect
  enforcer; choosing a canonical form for an idiom; trading local
  optimality for project-wide consistency so future sweeping
  transforms stay cheap.

Broad module terms route here only when paired with state-space intent:
invalid states, invariants, proof preservation, boundary parsing,
codomain-range gaps, forcing functions, Hyrum's Law, drift, or
progressive-disclosure topology. Otherwise use the narrower skill for
the task (for example commit workflow, generic documentation editing, or
ordinary refactoring).

## When Not to Use

- The task is only formatting or naming.
- The task does not change domain invariants, data boundaries, or test
  precision.
- The user wants a short summary only, not design guidance.
- The required output is formal, proof-like, or definition-only — use
  `state-space-minimization-formal`, the notation and inference-rule
  layer over this skill's modules. Semantics stay here; load the
  formal skill for the requested form. (For using it as a reasoning
  instrument when the output is informal, see § "Formal escalation".)

For a new formal design, use [formal-design](../formal-design/SKILL.md)
to choose an identity, representation, or interface before writing proofs.
For an adversarial review of an existing specification or implementation,
use [find-and-prove](../find-and-prove/SKILL.md) to test what a caller
can observe and collect a witness.

## How to use this skill

Read [principles](references/principles.md) for the vocabulary, six
operations, and search procedure. Choose profiles by the result the
user needs. The Outputs section describes what each profile delivers.
The file names below are relative to `references/`.

| Profile | Load first | Additional materials |
|---|---|---|
| Core type / boundary | constructive-vs-predicative.md | Choose by the evidence below. |
| Testing | testing.md | defensive-code.md for branches a test cannot reach |
| Documentation | normalization.md, documentation.md | |
| Commit | normalization.md, commits.md | |
| Normalization | normalization.md | least-power.md for passthroughs or sprawl |
| Tooling / ratchet | perfect-tool.md, ratchet.md | |
| Language | Applicable profiles above | languages/rust.md, languages/typescript.md (also JavaScript), languages/sql.md, languages/regex.md |
| Skill-refinement | skill-refinement.md | history-and-lineage.md for origins and citations |

For a core type or boundary task, follow the evidence:

| Evidence | Module |
|---|---|
| Primitive parameters stand for domain values | primitive-obsession.md |
| A Boolean discards the reason for an outcome | boolean-blindness.md |
| Panic, sentinel, or partial function | total-functions.md |
| Unwrapping a value discards a proof, tag, or refinement | proof-preservation.md |
| Choosing the aggregate, value object, or bounded context that owns an invariant | architectural-scopes.md |
| External input, deserialization, expiring guarantees, or ambient authority | ingress-and-boundaries.md |
| A stored fact repeats another field or creates a cross-field constraint | normalization.md; also the scope or ingress module if ownership or parsing changes |
| Several primitives can express the same operation | least-power.md |
| An upstream guarantee makes a defensive check or handler unreachable | defensive-code.md |

For a whole-skill audit, read the principles and both cross-cutting
lenses (constructive vs predicative, normalization), each applicable
technique, then the language references. A prerequisite introduces a
concept needed for the current task. Links to examples and companion
procedures are optional. Load another module when its guidance becomes
necessary. For a language without a module, derive the answer from
principles. Load history-and-lineage.md directly for a citation request.

### Formal escalation

Use [state-space-minimization-formal](../state-space-minimization-formal/SKILL.md)
to reason about a hard invariant or compare candidate encodings. Write
down the representable states, the contract, and each construction
path's range. Check strictness, proof preservation, and dominance. For
a mechanical check, translate those obligations into Lean or TLA+.
Translate the result back into the form the user requested.

## Inputs

- Domain invariants and invalid cases.
- Ingress and egress boundaries.
- Current types, constructors, deserializers, and tests.

## Outputs

Emit only the outputs for the loaded profile(s). If multiple profiles
apply, compose their outputs without adding unrelated checklist items.

- **Core type / boundary profile** — minimized design for changed
  boundaries; constructive-vs-predicative recommendation per type;
  lower-bound, upper-bound, length, and cardinality constraints;
  trusted-boundary audit for smart constructors; boolean-blindness
  audit; totality notes; boundary-parser or capability-token plan;
  scope choice per invariant.
- **Testing profile** — matcher-tightening findings; exact call-history
  and side-effect assertions; property-test plan; integration coverage
  plan; eliminated-state notes with no test-only escape hatches.
- **Documentation profile** — narrowed prose contracts; drift findings;
  executable-documentation opportunities; placement decision among rich
  type, doctest, co-located rationale, and boundary prose.
- **Commit profile** — commit-message/diff codomain audit;
  transformation-priority ordering; atomic split recommendations;
  gate-trailer proof notes.
- **Normalization profile** — atoms, functional dependencies,
  duplicated facts, passthrough nodes, determinant-based grouping, and
  topological presentation order.
- **Tooling / ratchet profile** — enforcement direction, current and
  aspirational bounds, forcing functions, gap between imagined and real
  enforcer, and weakening notes only when explicitly requested.
- **Language profile** — concrete idioms for the language files in
  scope, loaded after the relevant technique modules.
- **Skill-refinement profile** — material findings in the format in
  `references/skill-refinement.md`, or "No material skill-refinement
  findings."

## Utilities

- No mandatory scripts. Use `SKILL.md` as the router and the
  `references/` files as progressively loaded modules.
- For skill edits, run focused frontmatter and markdown checks on this
  skill. Treat repo-wide findings outside this skill as separate audit
  debt unless the user expands scope.

## Process

1. Read `references/principles.md`. Use the six-operation frame
   (shrink domain, bound ranges, shrink codomain, remove intermediate,
   normalize, ratchet) as the universal checklist.
2. Classify the task into output profile(s). If the task is only
   formatting, naming, or summary, stop without loading more modules.
3. Load only the profile's dependency modules, in router order. Load
   language modules only for file types in scope.
4. Apply the loaded module rules. Do not run audits from unloaded
   profiles. If evidence crosses into another profile, load the needed
   module first; suppressing that evidence is under-reading, not
   progressive disclosure.
5. For the core type / boundary profile, list invariants and trust
   boundaries; choose constructive when feasible; otherwise use the
   lightest predicative mechanism; bound every range and cardinality;
   audit construction paths; narrow bools, partial functions, temporal
   invariants, and architectural scope.
6. For the testing profile, tighten matchers, call histories, and side
   effects; plan property and integration coverage; record eliminated
   state space instead of adding test-only bypasses.
7. For documentation, commit, normalization, tooling, or language
   profiles, follow the loaded module's local procedure and emit only
   that profile's outputs.
8. End with the skill-refinement report only when the
   skill-refinement profile is loaded. State "No material
   skill-refinement findings" if no evidence-backed findings remain.
9. Stop here unless the user asks for further follow-up.

## Validation Checklist

- This skill is the canonical home for state-space minimization
  guidance. Other skills should reference it for deep rules.
- Progressive disclosure is enforced as an invariant: classify the
  profile first, load only needed modules, and emit only loaded-profile
  outputs.
- Broad triggers are qualified by state-space intent, not bare
  formatting, naming, or generic refactoring requests.
- Output profiles are route-specific and do not require unrelated
  checklist items.
- Project-specific examples are generic or explicitly non-normative.
- Apply the same state-space thinking to tests: tighter matchers mean
  fewer accidentally-accepted behaviors.
- When work spans languages or symptoms, load multiple files. The
  router structure is intentional.
