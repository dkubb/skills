---
name: state-space-minimization
description: >-
  Minimize representable states so invalid states are impossible.
  Parse, don't validate: replace primitive obsession with domain
  types, smart constructors, and boundary parsing; bound every range
  and cardinality; tighten test matchers; normalize so each fact has
  one determinant; ratchet tooling thresholds so the state space only
  narrows. Applies to types, signatures, tests, commits,
  documentation, and tooling configuration.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-05-v2"
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
  - "conventional commit as closed set"
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
not an objective by itself.

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
  formal skill for the requested form.

## How to use this skill

This skill uses **progressive disclosure**. SKILL.md is a router; load
only the reference files relevant to the current task. The router order
is a topological sort of the module dependency graph — every module is
loaded only after its dependencies. See `references/normalization.md` §
"Application: documentation" for the formal account.

**Presentation strategy.** Each module introduces its formal
definitions first, then applies them to practical cases and examples.
The formal frame is a dependency of the practical claim; placing the
dependency first gives every practical claim a frame to attach to. The
same dependency-before-dependent rule that orders the modules orders
the sections within each module.

### Foundation — always load

1. `references/principles.md` — vocabulary (domain, codomain, range,
   preimage), six operations, mechanism ladder, decision
   rubric, types as hypotheses, self-similarity.

### Profile → module map

Classify the task into output profile(s) first (Process step 2),
then load that profile's modules in router order.
`references/principles.md` is the foundation for every profile and
is omitted from the rows below. The "by symptom" column lists the
profile's optional modules: load one only when its entry in the
symptom router below matches evidence in the task.

| Profile | Always | By symptom |
|---|---|---|
| Core type / boundary | constructive-vs-predicative | primitive-obsession, boolean-blindness, total-functions, proof-preservation, architectural-scopes, ingress-and-boundaries, least-power, defensive-code |
| Testing | testing | defensive-code |
| Documentation | documentation, normalization | |
| Commit | commits, normalization | |
| Normalization | normalization | least-power |
| Tooling / ratchet | perfect-tool, ratchet | |
| Language | languages/&lt;lang&gt;.md per file type in scope, after the profile's other modules | |
| Skill-refinement | skill-refinement | |

### Cross-cutting lenses — load early when relevant

These modules are high-leverage, but not automatic. Load them right
after principles only when the selected profile needs their distinction.

2. `references/constructive-vs-predicative.md` — the deepest split
   for *how* to encode an invariant. Intrinsic vs extrinsic safety;
   smart-constructor trusted-boundary audit; four hard-case fallbacks.
   Load for representation, constructor, parser, proof, or boundary
   tasks.
3. `references/normalization.md` — the deepest split for *where* to
   place information. Formal frame (atoms, FDs, transitive reduction,
   lattice of decompositions, Galois connection); decompose-then-
   recompose strategy; applies to data, code, commits, and
   documentation alike. Load for placement, decomposition, drift,
   commits, documentation topology, or progressive-disclosure tasks.

If neither distinction is needed, skip both and continue to the specific
technique module.

### Technique modules — load by symptom

Entries below are ordered by dependency. A later module may reference
an earlier one; earlier modules do not depend on later ones.

- primitive parameters / weak domain types →
  `references/primitive-obsession.md`
- boolean returns or boolean parameters that throw away the reason →
  `references/boolean-blindness.md`
- partial functions, panics on bad input, sentinel returns →
  `references/total-functions.md`
- values are unwrapped back to primitives, revalidated downstream,
  passed through proof-losing conversions, or need phantom tags /
  GADTs / refinement / dependent / substructural proof preservation →
  `references/proof-preservation.md`
- value object / aggregate / bounded context / anti-corruption layer
  scope decisions →
  `references/architectural-scopes.md`
- raw external input, deserialization, env / CLI / config / database
  rows, or DTO translation →
  `references/ingress-and-boundaries.md`
- nonmonotonic or time-varying invariants such as not-banned,
  not-expired, lease-held, current-price →
  `references/ingress-and-boundaries.md`
- authority should be passed explicitly instead of being ambiently
  available →
  `references/ingress-and-boundaries.md`
- duplicate fields or cross-field constraints suggest changing
  representation so the constraint disappears →
  `references/architectural-scopes.md` when the owning aggregate /
  context is in question; plus `references/ingress-and-boundaries.md`
  for the boundary-parsing technique; plus
  `references/normalization.md` for the placement frame
- choosing between equally capable primitives; mutation-driven
  simplification; rule of least power / least privilege; η-reduction →
  `references/least-power.md`
- function-decomposition sprawl, tiny passthrough functions, god
  functions, deciding whether to merge or split, code organization,
  module-level normalization, recompose into the right grain →
  `references/normalization.md` + `references/least-power.md`
  (under-power with composition)
- auditing defensive checks against impossible-by-type or
  impossible-by-flow inputs; catch-all exception handlers without a
  known failure mode; redundant validation after upstream parsing;
  deleting just-in-case branches that have nothing left to defend →
  `references/defensive-code.md`
- tighter test matchers, side-effect coverage, Hyrum's-Law testing,
  mutation testing as state-space verification →
  `references/testing.md`
- writing or auditing documentation; choosing between rich type /
  doctest / co-located rationale / boundary prose for an invariant;
  removing vague qualifiers from prose claims; documentation drift
  as a state-space leak; progressive disclosure as topological sort →
  `references/documentation.md` + `references/normalization.md`
- writing or auditing commit messages; sequencing commits by
  transformation priority (Remove → Fix → Move/Rename → Refactor →
  Change → Add → Upgrade/Downgrade);
  deciding commit granularity for small preimage of failure;
  encoding gate results as commit trailers; atomic-commits as
  decompose-then-recompose →
  `references/commits.md` + `references/normalization.md`
- designing code, tests, or tooling against an imagined perfect
  enforcer (perfect mutation tester, perfect formatter, perfect
  linter); choosing consistency over local optimality so
  project-wide transforms stay cheap; building a real tool with
  few special cases and re-applying as it improves; applying a
  rubric as a deterministic program (the LLM runs the program,
  output matches what the program would produce) →
  `references/perfect-tool.md`
- setting project thresholds for tooling-enforced state-space
  rules; ratcheting current values toward aspirational targets;
  choosing forcing functions; weakening thresholds only on
  explicit user request →
  `references/ratchet.md`
- auditing or improving this skill; converting use evidence into
  skill findings →
  `references/skill-refinement.md`
- origin of the slogans, canonical citations, further reading →
  `references/history-and-lineage.md`

### Language modules — load by file type in scope

- Rust files → `references/languages/rust.md`
- TypeScript / JavaScript files → `references/languages/typescript.md`
- SQL / database migrations / `CREATE DOMAIN` / `CHECK` constraints,
  database normalization concrete forms → `references/languages/sql.md`
- Regex patterns in any language (string validation, lexer, route
  matchers, denylists) → `references/languages/regex.md`

If the work spans multiple symptoms, load multiple technique files. If
the work spans multiple languages, load each language file.

### End-to-end linear read order

For full skill loading (rather than targeted use), the topological
sort of the module DAG is:

1. `references/principles.md`
2. `references/constructive-vs-predicative.md`
3. `references/normalization.md`
4. `references/primitive-obsession.md`
5. `references/boolean-blindness.md`
6. `references/total-functions.md`
7. `references/proof-preservation.md`
8. `references/architectural-scopes.md`
9. `references/ingress-and-boundaries.md`
10. `references/least-power.md`
11. `references/defensive-code.md`
12. `references/testing.md`
13. `references/documentation.md`
14. `references/commits.md`
15. `references/perfect-tool.md`
16. `references/ratchet.md`
17. `references/skill-refinement.md`
18. `references/history-and-lineage.md`
19. `references/languages/<lang>.md` for each language in scope

Each module loads only after its dependencies. A reader following this
order encounters every concept after the concepts it references.

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
- **Tooling / ratchet profile** — floor and aspirational thresholds,
  forcing functions, gap between imagined and real enforcer, weakening
  notes only when explicitly requested.
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
