# Skill refinement

The skill is itself a state space — `principles.md`
§ "Self-similarity" states the principle; this module is the
operational discipline. Applying the skill to a concrete case
produces evidence about whether the skill's rules match the actual
situation. The skill-refinement audit is the forcing function that
converts that evidence into proposed improvements; the user is the
gate that approves or rejects them.

The formal frame: the distinctions made on the concrete case are
the new range; the skill's current rules are the codomain. Where
the codomain is wider than the range, or the case required a
distinction the codomain cannot express, surface a finding.

This module loads only when the skill-refinement profile is
active: the task asks to audit or improve this skill, repeated use
exposes drift, or a concrete case produces a finding worth user
attention. During ordinary application tasks, capture
counter-examples and tightening opportunities internally without
loading this module (see `SKILL.md` § "Active participation in
skill refinement").

## Finding categories

- **Sharpened rules** — existing rule fit loosely; the case
  revealed a tighter form
- **Counter-examples** — existing rule did not fit; document the
  boundary
- **New examples** — strong illustration of an existing principle
- **Candidate new modules** — recurring pattern the skill does
  not yet cover
- **Language patterns** — patterns specific to the language,
  belonging in `languages/<lang>.md`
- **Missing triggers** — terminology that should route here but
  did not
- **Cross-cutting concerns** — themes appearing across multiple
  modules
- **Threshold candidates** — metrics worth tracking at the
  project level

## Finding format

Each finding cites the concrete case that exposed it. A finding
without evidence is taste, not signal — do not propose a skill
edit from prose alone.

- **Category** — one of the above
- **Evidence** — the file, line, or interaction that surfaced the
  gap; quote or link the source so the user can re-derive the
  finding without you
- **Suggested change** — the specific rule or section to add,
  sharpen, move, or remove
- **Why this narrows the skill** — which state space the change
  closes (rule ambiguity, missing routing, language gap,
  inconsistent vocabulary, drift between modules)

## The report

The skill-refinement report is one of:

- **No material skill-refinement findings.** Stated explicitly,
  not by silence.
- **Findings:** one entry per finding, in the format above.

## Audit rules

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
  edits are atomic per `commits.md`, prose follows
  `documentation.md`, module introductions follow the
  self-similarity pattern in `ratchet.md`. The skill governs its
  own modifications.

## Validation probes

Self-use is one evidence source; independent probes catch classes
that self-audit structurally cannot. Each grades a different
property:

- **Unbiased audit loop** — fresh subagents audit the artifact
  repeatedly with this skill's discipline; the parent judges and
  applies findings, looping until finding quality drops below the
  apply bar. Grades internal coherence; later rounds audit the
  earlier rounds' fixes.
- **Cross-model one-shot** — a different model reviews once. Grades
  purpose ("does the rule do what the prose promises") — the gap
  class same-model loops converge away from.
- **Read-back** — a separate agent explains the artifact back from
  minimal context. Grades reception: what a consumer preserves,
  loses, or invents.
- **Translation probe** — reflect the artifact into a mechanically
  checkable form (for a calculus: Lean or TLA+) and grade with the
  checker. Grades commitment: translation forces bindings that
  paraphrase can fake, and the checker finds undischargeable rules
  prose review misses. The strongest probe; its findings flow back
  as ordinary skill edits.
- **Autonomous improvement loop** — an agent iterates on a derived
  artifact one change at a time behind a hard gate (build, tests),
  logging each step. Improvements that outrun the source are
  back-port candidates.

## Effectiveness audit

When the task is to audit or improve this skill, or when repeated
use exposes drift, record:

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
