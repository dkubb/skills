---
name: state-space-minimization
description: Minimize representable states so invalid states are impossible.
compatibility: Unified agent skills CLI
metadata:
  author: dkubb
  version: "2026-05-v2"
triggers:
  - "state space minimization"
  - "state space"
  - "make invalid states impossible"
  - "make illegal states unrepresentable"
  - "impossible states unrepresentable"
  - "parse don't validate"
  - "primitive obsession"
  - "smart constructor"
  - "private constructor"
  - "tighten types"
  - "narrow domain"
  - "narrow codomain"
  - "narrow state"
  - "narrow types"
  - "tighten matchers"
  - "strict matcher"
  - "Hyrum's law"
  - "side effect coverage"
  - "typestate"
  - "session types"
  - "domain modeling"
  - "domain primitive"
  - "value object"
  - "aggregate root"
  - "bounded context"
  - "anti-corruption layer"
  - "constructive data"
  - "constructive vs predicative"
  - "intrinsic type safety"
  - "boolean blindness"
  - "total functions"
  - "ghosts of departed proofs"
  - "phantom types"
  - "branded type"
  - "opaque type"
  - "single case discriminated union"
  - "discriminated union"
  - "tagged union"
  - "sealed class"
  - "sealed interface"
  - "refinement types"
  - "liquid types"
  - "pattern types"
  - "dependent types"
  - "substructural types"
  - "linear types"
  - "affine types"
  - "capability token"
  - "object capability"
  - "effect system"
  - "schema first"
  - "serde validation bypass"
  - "shotgun parsing"
  - "create domain"
  - "check constraint"
  - "tighten constraint"
  - "tighten regex"
  - "tighten regexp"
  - "anchor regex"
  - "bounded quantifier"
  - "catastrophic backtracking"
  - "redos"
  - "principle of least power"
  - "rule of least power"
  - "principle of least privilege"
  - "least authority"
  - "mutation testing"
  - "alive mutation"
  - "surviving mutant"
  - "equivalent mutation"
  - "mutation coverage"
  - "simplest interface"
  - "fewest degrees of freedom"
  - "narrowest primitive"
  - "commit state-space transition"
  - "commit message as type signature"
  - "conventional commit as closed set"
  - "preimage of failure"
  - "transformation hierarchy"
  - "gate trailer as typed proof"
  - "documentation drift"
  - "doc drift"
  - "doctest"
  - "executable documentation"
  - "documentation as type signature"
  - "natural-language type signature"
  - "defensive code"
  - "just-in-case code"
  - "overly defensive"
  - "redundant validation"
  - "redundant check"
  - "unreachable branch"
  - "dead defensive check"
  - "catch-all"
  - "threshold as forcing function"
  - "ratchet"
  - "ratcheting"
  - "aspirational threshold"
  - "tooling threshold"
  - "lint configuration"
  - "ci gate"
  - "lock in improvements"
  - "no regression"
  - "forcing function"
  - "perfect tool"
  - "perfect linter"
  - "perfect formatter"
  - "perfect mutation tester"
  - "imagined enforcer"
  - "imagined tool"
  - "anticipatory design"
  - "consistency over optimality"
  - "canonical form"
  - "as if enforced"
  - "applying a rubric"
  - "rubric application"
  - "rubric as program"
  - "deterministic enforcement"
  - "project the perfect form"
  - "normalization as state-space minimization"
  - "database normalization"
  - "documentation normalization"
  - "code normalization"
  - "denormalization drift"
  - "third normal form"
  - "boyce-codd normal form"
  - "functional dependency"
  - "transitive reduction"
  - "transitive closure"
  - "topological sort as progressive disclosure"
  - "topological order for documentation"
  - "progressive disclosure invariant"
  - "progressive disclosure as topological sort"
  - "single source of truth invariant"
  - "DRY as state-space"
  - "derive don't store"
  - "passthrough"
  - "passthrough function"
  - "passthrough chain"
  - "sprawl as state-space"
  - "code sprawl state space"
  - "function sprawl state space"
  - "module sprawl state space"
  - "god module"
  - "god function"
  - "over-decomposition"
  - "under-decomposition"
  - "merge functions by determinant"
  - "split function by determinant"
  - "zettelkasten"
  - "atomic decomposition"
  - "decompose then recompose"
  - "lattice of decompositions"
  - "Galois connection"
  - "normal form"
  - "Church-Rosser"
  - "confluence"
  - "information hiding"
  - "acyclic dependencies"
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
§ "Self-similarity". Applying the skill to a concrete case
produces evidence about whether the skill's rules match the
actual situation. A skill-refinement audit is the forcing function that
converts that evidence into proposed improvements; the user is the gate
that approves or rejects them.

When this skill loads:

1. Apply principles to code already seen this session, not just
   code reached after load. Re-evaluate earlier files if a loaded
   rule would change a finding.
2. For code in a language without a `references/languages/<lang>.md`
   file, derive guidance from `references/principles.md` only —
   never invent rules from outside that file. Flag genuinely
   unfamiliar territory; note derived rules as candidates for a
   future module.
3. Capture counter-examples (rule did not fit) and tightening
   opportunities (rule could be sharper) as you work.

For ordinary application tasks, keep this capture internal unless it
produces a material finding. Load the skill-refinement profile only when
the task asks to audit or improve this skill, repeated use exposes
drift, or the concrete case produces a finding worth user attention.

When the skill-refinement profile is loaded, apply the SSM audit to the
skill itself. The distinctions made on the concrete case are the new
range; the skill's current rules are the codomain. Where the codomain is
wider than the range, surface a finding.

Finding categories:

- **Sharpened rules** — existing rule fit loosely; the case
  revealed a tighter form
- **Counter-examples** — existing rule did not fit; document the
  boundary
- **New examples** — strong illustration of an existing principle
- **Candidate new modules** — recurring pattern the skill does
  not yet cover
- **Language patterns** — patterns specific to the language,
  belonging in `references/languages/<lang>.md`
- **Missing triggers** — terminology that should route here but
  did not
- **Cross-cutting concerns** — themes appearing across multiple
  modules
- **Threshold candidates** — metrics worth tracking at the
  project level

Finding format. Each finding cites the concrete case that exposed
it. A finding without evidence is taste, not signal — do not
propose a skill edit from prose alone.

- **Category** — one of the above
- **Evidence** — the file, line, or interaction that surfaced the
  gap; quote or link the source so the user can re-derive the
  finding without you
- **Suggested change** — the specific rule or section to add,
  sharpen, move, or remove
- **Why this narrows the skill** — which state space the change
  closes (rule ambiguity, missing routing, language gap,
  inconsistent vocabulary, drift between modules)

The skill-refinement report is one of:

- **No material skill-refinement findings.** Stated explicitly,
  not by silence.
- **Findings:** one entry per finding, in the format above.

Audit rules:

- **Task result first.** Answer the user's actual request, then
  append the skill-refinement report. The meta-audit is an
  addendum to the work product, never a replacement for it. A
  finding worth surfacing is still worth surfacing after the task
  result; a finding only worth surfacing if it crowds out the task
  result is not material.
- **Verify before surfacing.** Read the cited file. LLM audits
  are not infallible; the gate against fabricated findings is to
  verify each one against the source before reporting.
- **Gist over detail.** Short, scannable findings.
- **Skip when nothing material.** Do not pad the output with weak
  findings.
- **Do not auto-edit.** Surface findings only; the user decides
  which to apply.
- **Modifications follow the skill's own rules.** When approved,
  edits are atomic per `references/commits.md`, prose follows
  `references/documentation.md`, module introductions follow the
  self-similarity pattern in `references/ratchet.md`. The
  skill governs its own modifications.

Effectiveness audit. When the task is to audit or improve this skill,
or when repeated use exposes drift, record:

- **Activation precision** — did the trigger select a true
  state-space-minimization task?
- **Module-load precision** — which modules were loaded, which were
  needed, and which were unused?
- **Output precision** — which emitted outputs were actionable, false
  positives, or noise?
- **Finding fate** — accepted, rejected, deferred, or superseded by a
  narrower rule.
- **Missed routing** — terms that should have loaded this skill or a
  specific module but did not.
- **Progressive-disclosure fit** — whether a smaller module set could
  have answered the task.

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
   preimage), four narrowing techniques, encoding ladder, decision
   rubric, types as hypotheses, self-similarity.

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
  transformation priority (remove → fix → refactor → change → add);
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
17. `references/history-and-lineage.md`
18. `references/languages/<lang>.md` for each language in scope

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
- **Skill-refinement profile** — material findings in the format above,
  or "No material skill-refinement findings."

## Utilities

- No mandatory scripts. Use `SKILL.md` as the router and the
  `references/` files as progressively loaded modules.
- For skill edits, run focused frontmatter and markdown checks on this
  skill. Treat repo-wide findings outside this skill as separate audit
  debt unless the user expands scope.

## Process

1. Read `references/principles.md`. Use the four-technique frame
   (shrink domain, bound ranges, shrink codomain, remove intermediate)
   as the universal checklist.
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
