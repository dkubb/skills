# Lean formalization of the state-space-minimization calculus

A Lean 4 + mathlib formalization of the core calculus. **This project
is a DERIVED, NON-NORMATIVE reflection** of the calculus that
`../SKILL.md` owns; the prose skill is normative.

## Determinant

`../SKILL.md` OWNS the calculus and is **normative**: it is the source
of truth for the inference rules and their semantics. This Lean project
is a **derived, non-normative reflection** of that calculus — a
machine-checked second representation. **If the two disagree,
`../SKILL.md` is right** and the Lean is stale. The
`state-space-minimization` skill's reference modules remain determinant
for *semantics* — what each concept means and when it applies — and the
Lean mirrors their *form*.

## Purpose

The reflection is **bidirectional**: it both tightens the skill and
proves it.

- **It tightens the skill.** Lean forces commitments prose can leave
  implicit — universe choices, embeddings, quantifier domains, whether
  a premise is dischargeable at all. Each place the formalization had
  to invent or disambiguate is a finding fed back into `../SKILL.md`.
- **It proves the skill.** The inference rules are Lean declarations;
  a green `lake build` is machine-checked evidence that the calculus
  is coherent, and `#print axioms` on every headline theorem shows it
  rests only on the allowlisted standard axioms. That allowlist is
  ratcheted to the minimal set the calculus actually uses: `propext`
  and `Quot.sound` only. `Classical.choice` was removed as dead
  codomain (no headline depends on it) — the skill's own
  shrink-codomain operation applied to the gate; if a future proof
  needs choice the gate flags the non-allowlisted axiom, which is the
  ratchet working.

## Scope — the full calculus

The **full** calculus is now reflected. Formalized: state sets and the
trust theorems; `Strict` and the refinement order; the selection
objective (`Sufficient` with a coverage-witnessed construction-path
set, eligibility, ⊆-minimality, rank, and the theorem that the chosen
mechanism is admissible) and the fallback; constructive dominance;
`Obl` and the proof-preservation rules (`Needs`/`Erases`/`Flows` ⇒
`ProofObligation`); reception semantics (denotation, support-subset
`AdmissibleRewrite`, `ReceptionNarrowing`, completeness);
self-similarity (`Req`/`K`, `MissingDistinction`, the gap analogy,
`SharpeningCandidate`); range/gap/preimage, the six operations, and the
Invariants (totality, single source, normalizer idempotence). Reception
semantics, self-similarity, and proof-preservation — once excluded —
are all included.

Allowlisted residuals (claims no command can enforce; see
`claims.toml` for the full justification each carries):

- `core.supp_residual` — the reception support is carried data, but
  its real value is empirical.
- `reception.operational_residual` — `Pr(q'|q,t,c)` is a
  probabilistic, reader-dependent measure; only its support shape is
  modeled. No probabilistic theorem is asserted.
- `selfsim.spans_repeated_use_residual` — "`E*` spans repeated use" is
  empirical; carried as an abstract hypothesis and proved
  load-bearing.
- `select.rank_metric_residual` — the encoding-order table is derived
  per architecture, not an axiom.
- `select.rank_vocabularies_residual` — the encoding-order rank
  (target-system mechanism class) and the enforcement rank (how the
  skill's own claim is discharged) are two distinct axes; their
  mapping is meta-documentation about the gate, not a target theorem.
- `ops.bound_ranges_residual`, `ops.remove_intermediate_residual`,
  `ops.ratchet_residual` — operation templates / operator policies
  whose load-bearing premise is a per-artifact judgment.
- `inv.exhaustiveness_residual` — enforced structurally by the Lean
  kernel (an omitted constructor does not typecheck), not by a
  project theorem.
- `selfsim.type_pin_body_mutant_residual` — the type-pin guards a
  theorem's statement but not a Prop definition's body; body-carried
  content needs a red body-mutant. A property of the gate, not a
  target theorem.
- `boundary.freshness_residual` — the gate's cache-safe rebuild is a
  freshness boundary: derived oleans are trusted only when rebuilt
  from current sources; a stale read is the verifier reaching an
  invalid verdict. A property of the gate, not a target theorem.

## Hardening — the machine-checked gate

`check_claims` is a **Lean-internal** mechanical gate. It never reads
`../SKILL.md`; it proves the reflection is internally sound and
anti-vacuous. It fails (nonzero exit) on any of:

- **Cache-safe rebuild.** It deletes the project oleans and runs a
  cache-disabled (`LAKE_ARTIFACT_CACHE=false`) `lake build`, so every
  downstream check judges artifacts that match the current sources, not
  stale cached oleans.
- **Type pin.** For every non-residual headline it prints the real
  decl's type (`#check @decl` under `pp.fullNames`) and compares it to
  the recorded `expected_type`; gutting a headline to `: True := trivial`
  changes its printed type and fails. The only way to pass a gutted
  headline is to also edit `expected_type` — a visible, reviewable
  change.
- **Per-headline mutants.** Every headline claim carries a rejecting
  mutant in `mutants/` that is compiled and confirmed to FAIL; a
  headline whose mutant compiles is presumed vacuous.
- **Residual-rank restriction.** A `residual=true` row may carry only an
  `evaluator-judgment`, `operator-policy`, or `honest-doc` rank — never
  a kernel rank, which would let a provable theorem be dodged as prose.
- **Orphan-module check.** Every `*.lean` under `lean/` (except the root
  and the checker) must be transitively imported by the root module
  `Ssm`.
- **Axiom allowlist.** Every headline theorem rests only on `propext`
  and `Quot.sound`.

All of these checks operate on the Lean only.

## Enforcement-rank taxonomy

`check_claims` knows seven enforcement ranks (`KNOWN_RANKS`). Two of
them — `export-seal` and `runtime-bridge` — belong to the general
`find-and-prove` taxonomy but are used by zero rows in this artifact,
which has no C-export drill and no runtime bridge. They are kept as
**deliberately reserved** values (not deleted) so the taxonomy stays
whole and a future export-seal or runtime-bridge claim has a home; a
code comment in `check_claims` marks them reserved-but-unused so they
are not silently carried as dead values.

## Naming parity

Lean identifiers mirror the calculus names wherever the calculus names
a thing (`I_repr`, `I_reach`, `R`, `Strict`, `Sufficient`, `Obl`,
`Needs`, `Erases`, `Flows`, `Req`, `K`). Where Lean coins a name for
an implicit calculus concept (`ContractPinned`, `ConstructionPaths`,
`ReceptionComplete`), the coined name is back-ported into `../SKILL.md`
prose. Renames on either side are vocabulary drift and must land on
both sides or not at all.

## Version parity

The reflection is **versioned independently** of `../SKILL.md`. Its
formalization version (currently `2026-06-v8`, semver `2026.6.8`) is a
Lean-internal stamp: it tracks `../SKILL.md`'s calculus by **content
and review**, not by a shared stamp the gate enforces. `check_claims`
cross-checks only the Lean-internal stamps — the `lakefile.toml`
`version` (`2026.6.8`), the `Reflection.lean` header (`2026-06-v8`),
`claims.toml`, and these README literals — and it **does not read
`../SKILL.md`**. Matching internal stamps are the signal that the Lean
files are connected to each other; the connection to the skill is by
review, not by a stamp.

## Provenance

Originally a Codex (GPT-class) reflection probe of `../SKILL.md` on
2026-06-12, graded by compilation under Lean 4.30.0 + mathlib.
Subsequently expanded to the full calculus and adversarially hardened;
the refinement and hardening iterations are logged in `ITERATIONS.md`.

## Building and checking

This directory is a self-contained lake project:

```sh
lake exe cache get   # once; downloads the mathlib build cache
lake build           # type-checks the root module Ssm (all modules)
./check_claims       # the mechanical gate (needs elan on PATH)
```

`Ssm.lean` is the root module: it imports every calculus module and is
the sole default target. `check_claims` enforces that no module is an
orphan, that every claim maps to a real declaration with a failing
mutant or an allowlisted residual, and that the Lean-internal version
stamps agree.
