# The perfect tool

Design code, tests, and tooling against an *imagined* perfect
enforcer — a mutation tester, formatter, linter, type checker, or
verifier whose rules are simple, whose coverage is total, and
whose enforcement cannot be circumvented. Pretending such a tool
exists is state-space minimization applied to design decisions:
the imagined enforcer fixes the shape of the code, the tests, and
the *real* tool you eventually build to approximate it. Same
vocabulary from `principles.md`, same four techniques, same audit
questions — applied to the design loop rather than to runtime
types or project configuration.

The rest of this module is the operational specifics: how to
apply the move during a real review (§ "Active application"), the
three classic instances of the technique, the consistency-over-
optimality tradeoff that makes it pay off, the constraint on the
imagined tool itself, and how the loop pairs with `ratchet.md`.

## Active application

When this file is loaded for a real review or audit, do not just
describe the rule. **Project the perfect-tool form of the code
being reviewed**, then surface the gap. The diff between the
actual code and its perfect-tool form *is* the finding.

The move:

1. Read the code (or rubric item) as written.
2. Imagine the perfect tool enforcing the rule at full power.
   Name the canonical form concretely — the test that would
   exist, the capability token the call site would carry, the
   operator that would mutate this branch, the canonical
   formatting of the expression.
3. Diff actual against imagined. Each difference is a finding.

### Rubrics are deterministic programs

This applies *especially* to rubrics. A rubric is the perfect
tool expressed in prose; the LLM applying it is interpreting that
program.

- The output is what the program would produce, using the logic
  the program would use. No judgment by taste; no selection of
  which findings "feel important" — that is the LLM suppressing
  findings the program would have emitted.
- Enumerate every input the program would visit (every public
  method, every match arm, every test, every boundary parser).
  Emit a finding for every input that fails the rule.
- The output must be reproducible. Applying the same rubric to
  the same code twice should produce the same finding set. If it
  would not, the LLM is not running the program — it is
  improvising.

Without this frame, "apply rubric X to code Y" degrades into
skim-and-summarize. With it, the output is mechanically
determined by the rubric and the code — which is what makes the
imagined perfect tool worth imagining.

Mutation testing (§ "The perfect mutation tester") is this frame
at runtime: the operator set is the program, the surviving
mutants are its deterministic output. Every applied rubric is a
mutation tester in prose.

### Two stages: high-recall rubric, high-precision filter

The deterministic-program framing produces findings at full
recall — every failing input emits a flag, including some that
turn out to be false positives. The pattern that works is two
stages, not one:

1. **Rubric stage (high recall).** Apply the rubric naively;
   emit every candidate. Do not pre-filter for "important" —
   the LLM running the rubric lacks the project context to
   judge that, and silent suppressions are invisible to the
   consumer.
2. **Filter stage (high precision).** A separate reviewer
   (stronger model, parent agent, human) classifies each
   candidate as real, false positive, or non-actionable. The
   filter holds the project-level context the rubric does not.

Both stages are load-bearing. Pruning at the rubric stage
silently drops findings the operators caught — the consumer
has no way to recover them, and the rubric becomes
under-powered without anyone noticing. Acting on every flag at
the filter stage produces churn and trains reviewers to ignore
the rubric.

Mutation testing has the same shape: killable + equivalent +
spurious mutants at full recall, triage decides which to act
on. The `read-back` skill realizes this pattern as a workflow:
a lower-power subagent applies the rubric, a stronger parent
agent filters.

## The pattern

Three classic instances. Same move, different artifact.

### The perfect mutation tester

Imagine a mutation tester whose operator set covers every
semantically meaningful change to your code and whose runtime is
free. Write tests not against the implementation you happen to
have, but against the operators that imagined tool would apply.
For every mutation the perfect tester *would* generate, a test
*should* fail.

This inverts the usual test-design question. Instead of "did I
cover the code I wrote?", the question becomes "did I leave any
equivalent implementation reachable that my spec would consider
wrong?" The implementation is whatever survives. Real mutation
testers (`mutant`, `Stryker`, `cargo-mutants`, `mutmut`,
`pitest`) are the approximation; the imagined operator set is the
target.

A surviving mutant in real mutation testing is the same finding
as "a test the perfect tester would have demanded that we forgot
to write." Treat surviving mutants as bug reports against the
test suite, not against the tool.

### The perfect formatter

Imagine a formatter that knows the single canonical form of every
expression in your language. Two expressions that compute the
same thing print identically; two expressions that differ in any
semantic way print differently. Write code in that canonical form
even when the real formatter does not enforce it yet.

The canonical form is a design artifact, not just an aesthetic
one: any *deviation* from canonical is a question the reader has
to answer ("does this idiom mean something here, or did the
author just feel like it today?"). Canonical-by-default is the
form that frees the reader from that question.

Languages with built-in canonical formatters (`gofmt`, `rustfmt`,
`black`, `prettier`) demonstrate the benefit by making the choice
unforgeable. Languages without one make the discipline a personal
or team-level practice — written as if the formatter existed —
until tooling catches up.

### The perfect linter

Imagine a linter that sees every expression in every file at
once, holds a complete catalogue of every idiom the project has
chosen, and flags anything inconsistent or suboptimal anywhere in
the codebase. Write code that would survive that linter's pass.

The imagined linter forces decisions to be made *once* per
pattern rather than per-occurrence. If two files solve the same
problem differently, the imagined linter would pick one and
rewrite the other; until then, the team writes only the chosen
form.

Real linters approximate this badly — they see one file at a
time, ship with rule sets tuned for the median project, and miss
project-specific consistency entirely. Custom linter rules,
project-scoped `ast-grep` patterns, and review checklists are the
practical fill-in. The discipline is written as if the perfect
linter were running.

## Consistency over optimality

The reason this move pays off is not aesthetic. It is a
state-space argument about the codebase as a whole.

Each local idiosyncrasy a codebase tolerates multiplies the cost
of any project-wide transformation. *N* inconsistencies create up
to 2^*N* combinations a sweeping migration must handle. A
codebase with one canonical form per pattern can be migrated with
a single rewrite; a codebase with five variations per pattern
needs five rewrites and a discovery pass to know which is which.

The trade is: prefer the move that keeps automated global
transformation cheap, even when it costs local optimality. Local
optimality is fragile — what counts as "optimal" changes as
language idioms evolve, as new tools arrive, as the team's
priorities shift. Consistency-but-suboptimal preserves the option
to do one global correction later; locally-optimal-but-
inconsistent forecloses on that option.

The asymmetry is the same one that justifies the ratchet (see
`ratchet.md` § "The ratchet"): a one-way tightening preserves
optionality. A consistent codebase can be re-optimized with a
single mechanical sweep; an inconsistent codebase cannot.

## Few special cases in the tool

The imagined tool is itself a state space; the same minimization
rules apply to it. A linter with fifty special-case rules cannot
be reasoned about, cannot be evolved coherently, and produces
findings users learn to ignore. A linter with three rules and a
canonical form can be improved continuously and trusted as it
grows.

The discipline on the tool side mirrors the discipline on the
code side: few rules, no exceptions, predictable application. If
the real tool grows enough special cases that you can no longer
predict its output, the codebase's state space has leaked into
the tool's state space and the loop breaks. The fix is the same
as for code: narrow the rule set; remove the exception by
changing the codebase; or split the tool.

This is `least-power.md` applied to the enforcer: pick the
smallest rule set that captures the invariant, no more.

## The design loop

1. **Imagine** the perfect enforcer with minimal special cases.
   What invariant does it enforce? What is its canonical form?
   What does its rule set look like, in three bullets or fewer?
2. **Design** the code and tests as if that enforcer existed.
   Write code in canonical form; write tests against the
   operators the imagined tool would generate; treat any
   deviation as a deliberate exception worth naming.
3. **Build** the strongest checkable approximation of the
   imagined tool. Custom lints, `ast-grep` rules, mutation
   operators, type wrappers, review checklists. Keep the rule
   set small; reject features that would introduce special
   cases.
4. **Target the gap.** Where the discipline is unenforced by the
   real tool, write a test or a checklist item that catches the
   specific failure mode the imagined tool would have caught.
   The test does not duplicate the imagined tool's coverage; it
   covers exactly the gap between imagined and real.
5. **Ratchet.** As the real tool improves, raise the floor (see
   `ratchet.md`). Gap-targeting tests retire as the underlying
   tool subsumes them. The codebase tightens monotonically.

The loop is generative on one side and preservative on the other:
the perfect tool defines the target; the ratchet keeps the gains.

## When not to activate

- The invariant has exactly one occurrence in the codebase and
  no planned feature would introduce a second instance. A
  single-occurrence invariant cannot benefit from
  consistency-over-optimality — there is nothing to keep
  consistent with.
- No real-tool approximation can be built: no test, lint,
  review checklist, or human audit could catch the failure the
  imagined tool would catch. The gap is unfillable, so the
  imagined-tool framing becomes aspiration without a forcing
  function. Pick a weaker imagined tool whose gap *can* be
  tested.
- The code will be deleted before the next release. Apply this
  discipline only to code the project commits to keeping.

## Cross-references

- `principles.md` § "Self-similarity" — the imagined-tool move is
  the design-time instance of the same SSM frame.
- `ratchet.md` — locks in what the perfect tool would have
  caught; the preservative half of the loop.
- `testing.md` § "Mutation testing" — the canonical
  anticipatory-design instance; tests written against imagined
  operators.
- `least-power.md` — the same minimal-capability-surface
  principle applied to the tool itself; few rules, no exceptions.
- `defensive-code.md` — what becomes deletable once the
  perfect-tool discipline holds upstream; the discipline removes
  the need for the guard.
- `proof-preservation.md` — predicative smart constructors and
  capability tokens are the practical approximation of imagined
  constructive guarantees; the same "design as if the strong form
  held" move applied to types.
- `commits.md` — the imagined commit-message linter and the
  perfect formatter for commit metadata are this module's pattern
  applied to history.
