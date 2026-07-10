# Lean 4 Review Guidelines (Language-Specific)

- Use simple English.
- Use short bullets.
- Do not repeat core principles.
- For adversarial review of the proof surface (oracle hunt, enforcement
  ranks, witnesses), use the `find-and-prove` skill. For pre-flight design
  of a new slice, use `formal-design`. This file is the mechanical per-diff
  checklist.

## Gate architecture

- `lake build` is not the proof gate. The strict gate is the check script
  (`just check`), which compiles with
  `-DautoImplicit=false -DrelaxedAutoImplicit=false -DwarningAsError=true`;
  every linter warning is a hard error.
- The module list in the check script is the compile order and the
  completeness roll. A diff that adds a `.lean` file must add it to the
  module list in dependency position; the gate fails on unlisted files, so
  no module is silently skipped.
- `just recheck` re-validates every theorem module through an external
  kernel (lean4checker) against the pinned toolchain. Meta-audit and
  claim-pin modules are excluded by design; nothing else is.
- Single-file `just compile <file>` (or `lake build <Module>`) is a fast
  inner-loop check, NOT the gate. Some linters — unused hypothesis, unused
  `variable`/typeclass binder — and the module-roll completeness fire only
  on the whole-library `just check`/`just lint`. Never conclude lint-clean
  from a per-file compile.
- The toolchain is pinned (`lean-toolchain`), and dependencies pin the same
  release tag. Flag any dependency or toolchain drift.

## Banned constructs

The gate bans these tokens in proof modules; treat any occurrence in a
diff as a blocker:

- `sorry`, `axiom`, `unsafe`, `partial`, `opaque`, `native_decide`,
  `@[implemented_by]`, `@[extern]`, and `admit` in tactic position.
- `decide` is fine (kernel-checked); `native_decide` is forbidden
  (compiler-trust boundary; its axioms are rejected by the audit).
- No `Mathlib.*` or `Batteries.*` imports in proof modules. Community
  dependencies are lint-only.

## Axiom discipline

- Allowlist is `propext` and `Quot.sound`. `Classical.choice`, `sorryAx`,
  and the `native_decide` axioms are rejected.
- Two enforcement layers, both required:
  - Headline theorems: `#print axioms` for every entry in the headline
    list, checked by the gate.
  - Environment-wide: a `run_cmd` meta-audit sweeps every constant in the
    library namespace (including mangled `_private` names) and errors on
    any axiom outside the allowlist. New modules must be imported by the
    audit module, each import justified with a comment; a module the audit
    cannot reach escapes the sweep, and that is a review defect.

## Module structure

- One module per concept; module groups are subdirectories with their own
  roll-up import file. The library root imports every submodule.
- Every module opens with, in order: a `/- ... -/` module header comment
  giving the rationale, then the pair
  `set_option autoImplicit false` / `set_option relaxedAutoImplicit false`,
  then the `namespace`. A missing pair is a defect even though the gate
  flags also enforce it.
- Scope any other `set_option` with `... in` to a single declaration and
  treat it as an exception needing a reason. No `maxHeartbeats` raises, no
  `set_option linter.* false`; lint exceptions use the documented two-line
  comment form (`-- lint exception: <name>` / `-- reason: <why>`).
- Prefer selective `open Foo (name1 name2)` over blanket `open Foo`.

## Naming and docs

- `theorem` exclusively; never the `lemma` keyword.
- Names are `snake_case`, dot-namespaced on the type
  (`Grant.sub_trans`, `Intent.no_self_escalation`); iff-lemmas end `_iff`.
- Every new `Prop`-valued `def` or relation gets a companion shape pin
  `Foo_iff : Foo … ↔ <body> := Iff.rfl`. It locks the definitional surface,
  so a later hidden extra conjunct is a compile break, not a silent
  widening. A relation with no shape pin is a finding.
- Name to exactly what is proved: a fixture theorem is not `_universal`, a
  forward refinement is not `_equiv`, and "consume" is for linear removal
  only. A name that overclaims the statement is a reception finding.
- Every declaration carries a `/-- ... -/` docstring; `/-! ## ... -/`
  section markers organize modules.
- Load-bearing theorems carry a `/-- HEADLINE ... -/` docstring and an
  entry in the headline list. Keep the two in sync — a marker without an
  entry (or entry without marker) is drift between two determinants of the
  same fact.
- Prose claims about types and signatures (README, design docs) get an
  anonymous `example`/`#check` pin in the claims module, citing the doc
  source. New load-bearing prose claims without a pin are findings.
- An EVIDENCE claim in a docstring or comment ("compiled fact", "proven
  below", "see witness X") must resolve to a declaration in the SHIPPED
  module (or a tracked build file), never a review-only or scratch witness.
  A reviewer who authored or ran the witness sees it compile and blesses
  the claim, but a scratch file deleted in cleanup leaves the claim
  dangling. Grep the shipped tree for the referent.

## Proof style

- No naked `simp`. Every call is `simp only [...]` or carries an explicit
  lemma list. Prefer `simp only`: `simp [foo]` still draws from the
  ambient `@[simp]` set, so it is not closed-world; treat new
  `simp [...]` call sites as ratchet-down targets.
- Prefer term-mode (pattern-matching equations) for structural proofs;
  tactic blocks where case work needs them.
- `@[simp]` only on rewrite lemmas that earn it; the mathlib standard
  linter set (simpNF among them) runs in the lint gate.
- `example`, `#check`, and `#print` belong only in the claims and audit
  modules. No `#eval` or `#guard` in proof modules.
- `deriving DecidableEq, Repr` is the standard clause. When the deriving
  handler cannot reach through recursion, use a standalone
  `deriving instance Repr for X` with a docstring saying why.
- `private` fields make smart constructors the sole producers of validated
  types — same goal as the Rust rule: no instance without the constructor.
- Shared binders live in section `variable` blocks
  (`variable {κ : Type} [DecidableEq κ]`), not repeated per declaration.
- Every binder a declaration names must be used by its statement or its
  proof. An unused hypothesis or `[Inst]`/`variable` binder is a linter
  finding — and one that surfaces only on the full gate, so a per-file
  compile will not catch it.
- A hypothesis the proof reads only through one projection (`h.1`), or that
  still proves when weakened to that projection, is stronger than used.
  Weaken it, or keep the richer antecedent deliberately as consumer-facing
  vocabulary and say so in the docstring (see find-and-prove H1).

## Lint gate and baselines

- The mathlib syntax linters run against a shadow tree and diff findings
  against a committed baseline; new or changed findings fail. Baselines
  are burndown debt, not policy — they only shrink. Regenerating the
  baseline (`--update`) requires triage first.
- The style-lint and env-lint nolints files follow the same rule: entries
  are burndown, additions are findings.
- The statement-surface rubric (find-and-prove's syntactically decidable
  subset) runs via ast-grep with a pinned grammar hash and a ratcheted
  baseline.
- Line length is capped at 100 CODEPOINTS, not bytes. Unicode math (`∈`,
  `≤`, `⟨⟩`, subscripts like `₀`) is multi-byte; measure with `wc -m` or
  Python `len()`, never `awk`/byte counts, or wrapping decisions are wrong.

## Mutation testing

- `just mutate <file>` runs single-span mutants against the strict gate as
  oracle. Classifications: SURVIVED means no theorem pins the mutated
  behavior (a vacuity finding — the actionable one); KILLED is healthy;
  INVALID does not count as a kill; DIVERGENCE (local-scope survivor
  killed downstream) is a self-pinning smell.
- Non-advisory survivors and divergences fail the run. `delete-headline`
  survivors mean a redundant theorem — prune it.
- Run mutation on the modules a diff touches and record the numbers, the
  same per-change discipline as cargo-mutants in Rust.
- Co-locate each definition's separating-witness theorem and each
  headline's non-vacuity witness (a concrete `∃`-witness or `example`) in
  the SAME module. Then `--scope local` reports 0 divergence and
  modified-files-only mutation is complete — a witness that lives one module
  away leaves a local survivor that only a downstream module kills.

## Review checklist (per diff)

- New `.lean` file: module list entry in dependency position, audit-module
  import with justification, module header + `set_option` pair, namespace.
- New theorem: docstring; HEADLINE marker and list entry if load-bearing;
  axiom audit still green; every binder used; name matches the statement.
- New `Prop` def/relation: companion `_iff := Iff.rfl` shape pin.
- New proof: no banned tokens, no naked `simp`, `simp only` preferred,
  term-mode where structural; no hypothesis consumed only at a projection
  without a reason.
- Formatting: every line ≤100 codepoints (measured as codepoints).
- New type: docstring, standard deriving clause, private fields with a
  smart constructor where an invariant exists.
- Prose changed: claims module pins still match; update pins with the
  prose.
- Baselines: no baseline grew; any `--update` has triage evidence.
- Mutation: touched modules mutated; survivors triaged to new theorems,
  pruned redundancy, or documented advisories.
