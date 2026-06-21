# Reflection Improvement Iterations

## Iteration 1

Change: `Boundary.b` now returns a representable-state subtype
`{x : State // x in A.S}` instead of returning a raw `State` with a
separate `lands_in_S` proof field.

Why: this moves the `R(b) subset S(A)` obligation from an extrinsic
side condition into the boundary result shape. Per the calculus, the
ordinary boundary now makes impossible the invalid encoding where a
boundary value exists without evidence that it is representable.

Build result: `PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 2

Change: `TrustedBoundary` now returns a contract-state subtype
`{x : State // x in A.C}` directly and derives its ordinary
`Boundary` view through `TrustedBoundary.toBoundary`.

Why: this removes the extrinsic `lands_in_C` side-condition field.
Trusted construction now makes the proof of `b(u,p) in C(A)` part of
the result shape, so a trusted boundary value without contract evidence
is not representable in the Lean encoding.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 3

Change: `BehaviorOK` now requires `ContractPinned A (m.apply A)` in
addition to behavior preservation on `C(A)`.

Why: the strictness rule forbids shrinking invalidity by changing the
contract. This closes the encoding gap where a mechanism could satisfy
the behavior predicate while silently widening or replacing `C(A)`.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 4

Change: added `EligibleMechanisms` as the single determinant for
candidate membership, sufficiency, and `BehaviorOK`, then rewrote
`EarliestSufficient` and `CostMinimalAmongTies` to quantify over that
set.

Why: eligibility is one fact in the calculus, not three independent
premises to repeat. This normalization removes disagreement states where
one selector ranges over all candidates while another ranges over only
sufficient, contract-preserving candidates.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 5

Change: introduced `Rank` with a positive `index` proof and changed
`Mechanism.rank` from `Nat` to `Rank`; rank comparisons now use
`rank.index`.

Why: `Nat` admitted rank `0`, but the encoding order starts at one.
The positive subtype narrows the codomain without freezing the
architecture-derived rank table into a closed enum.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 6

Change: `ConstructiveDominance` now carries
`contract_pinned : Ap.C = Ac.C`.

Why: constructive dominance compares representations of the same
contract. Without this field, the Lean witness could relate artifacts
whose behavior happened to agree on `Ac.C` while the predicative artifact
used a different contract.

Build result:
`LAKE_ARTIFACT_CACHE=false PATH="$HOME/.elan/bin:$PATH" lake build Reflection`
succeeded.

## Iteration 7 — close the SufficientWithPaths vacuity (2026-06-v4)

Change: `SufficientWithPaths` no longer takes a caller-supplied
`Paths` type and path function. It takes a `ConstructionPaths`
structure whose `covers` field requires every representable state of
`m(A)` to be reachable through some enumerated path. Added the theorem
`sufficientWithPaths_imp_irepr_empty`: over a covering path set,
`I_reach = ∅` on every path forces `I_repr(m(A)) = ∅`.

Why: the old definition quantified over a caller-supplied `Paths`, so
`Paths = Empty` discharged sufficiency vacuously for any mechanism and
artifact (Codex-confirmed). Coverage ties the path set to the
artifact's actual constructors; an empty/subset path set can no longer
discharge sufficiency unless `S(m(A))` is itself empty.

Mutant: `mutants/mut_sufficient_empty_paths.lean` builds the old
`Paths = Empty` witness for an artifact with `S = univ` and fails on
the `covers` obligation (Type mismatch). The old vacuous proof no
longer typechecks.

Build result: `lake build` succeeded (357 jobs).

## Iteration 8 — expand to the full calculus

Change: added `ProofPreservation`, `Reception`, `SelfSimilarity`, and
`Operations` modules, plus the root module `Ssm` importing all of
them; switched the lakefile default target to `Ssm`.

Why: the original reflection omitted proof-preservation, reception
semantics, and self-similarity, and did not model the six operations,
range/gap/preimage, or the named Invariants. The skill is now
canonical, so the Lean must cover the whole calculus.

Build result: `lake build` succeeded (357 jobs).

## Iteration 9 — claims inventory, checker, and canonicalization

Change: added `claims.toml` (73 rows) and `check_claims`; tagged every
guarantee-bearing sentence in `../SKILL.md` with its claim id;
replaced the `$$` display blocks with normative Lean citations; flipped
`README.md` to "Lean is normative"; bumped all version stamps to v4.

Why: Phase 0 single source of truth and the mechanical anti-vacuity
gate; Phase 3 canonicalization (Lean defines, prose annotates).

Build + gate result: `lake build` succeeded; `check_claims` PASS
(rows=73, kernel-theorems=20, headlines-with-failing-mutant=11,
residuals=9).

## Iteration 10 — adversarial-audit fixes (2026-06-v5)

An independent adversarial audit found gate-enforcement holes and
overclaims (the proofs themselves verified sound). Fixes, each its own
commit:

Change (HIGH-2): `check_claims` now forbids `kernel-theorem` /
`kernel-definition` ranks on any `residual=true` row (allowed residual
ranks: evaluator-judgment, operator-policy, honest-doc).
Why: a provable theorem (e.g. `core.gap_iff`) could be dodged by
emptying `lean_decl` and flipping `residual=true` with kernel rank.
Build result: `check_claims` PASS; probe (gap_iff as kernel residual)
now FAILS the gate.

Change (HIGH-1): added `StatementBinding.lean`, a guard that applies
every headline decl to fresh hypotheses and consumes its real
conclusion; `check_claims` requires it to COMPILE and to reference
every headline id (orphan check exempts it).
Why: mutants live in separate files and never reference the real decl,
so gutting a headline to `: True := trivial` left every mutant red and
the gate green. The guard binds the real STATEMENT.
Build result: `lake build` green; probe (gut `admissibleRewrite_trans`
to `: True := trivial`) now FAILS the gate via the guard.

Change (MEDIUM-1): renamed `Confluent`/`confluent_unique_nf` to
`Idempotent`/`idempotent_fixes_image`; dropped all order/uniqueness
language from the claim and SKILL.md.
Why: the def is mere idempotence and the theorem restated its
hypothesis; it never proved "normalization order does not affect
normal form". The honest content is the fixed-point property.

Change (MEDIUM-2): demoted `proofObligation_needs_holds` (the field
projection `ob.needs x hx`) to a non-headline corollary and promoted
`proofObligation_reconstitute` (load-bearing on all three obligation
fields) to the headline, retargeting the drop-needs mutant.

Change (SIMPLIFY-1): dropped the three duplicate Invariants rows
(`inv.contract_preservation`, `inv.single_source`,
`inv.boundary_monotonicity`) — they re-indexed existing theorems
against the same decls — and rewrote the SKILL.md bullets as
cross-references. rows 73→70.

Change (SIMPLIFY-2 + SIMPLIFY-3): removed `Mechanism.cost`,
`CostMinimalAmongTies`, `ObjectiveChoice`, and the
`core.cost_residual`/`select.cost_tiebreak`/`select.objective` rows
(an unused tiebreak whose value was an admitted residual). With cost
gone `ObjectiveChoice` collapsed to `EarliestSufficient`. Added
`earliestSufficient_admissible`: the selected mechanism is Sufficient
and behavior-preserving — the one kernel theorem that justifies the
selection lattice's normative presence. rows 70→68.

Net surface change: rows 73→68, residuals 9→8, headlines unchanged at
11, kernel-theorems 20→19. Build green, gate PASS, axioms clean
(below).

## Axiom baseline (Invariant 3) — 2026-06-v5

"Clean" = no `sorryAx`, no project-defined axioms, only the
allowlisted standard axioms `propext`, `Quot.sound`,
`Classical.choice`. `check_claims` re-derives this on every run by
`#print axioms` over every non-residual `lean_decl` and failing on any
non-allowlisted axiom. The per-headline baseline:

```text
trusted_boundary_reaches_valid             (no axioms)
trusted_boundary_no_reachable_invalid      propext, Quot.sound
boundary_introduction                      (no axioms)
sufficientWithPaths_imp_irepr_empty        propext, Quot.sound
earliestSufficient_admissible              (no axioms)
proofObligation_intro                      (no axioms)
proofObligation_needs_holds                (no axioms)
proofObligation_reconstitute               (no axioms)
constructive_discharges_obl                (no axioms)
Reception.admissibleRewrite_refl           (no axioms)
Reception.narrowing_imp_admissible         propext, Quot.sound
Reception.complete_imp_no_unintended       propext, Quot.sound
Reception.admissibleRewrite_trans          (no axioms)
SelfSimilarity.missing_not_gap             (no axioms)
SelfSimilarity.gap_not_missing             (no axioms)
SelfSimilarity.sharpening_requires_span    (no axioms)
Fn.mem_gap_iff                             (no axioms)
Fn.shrink_codomain_keeps_range             (no axioms)
Fn.shrink_domain_range_mono                (no axioms)
Fn.singleSource_functional                 (no axioms)
Fn.idempotent_fixes_image                  (no axioms)
```

No `sorryAx`, no `Classical.choice`, no project axioms anywhere. The
only standard axioms used are `propext` and `Quot.sound`, both on the
Invariant-3 allowlist; they enter through mathlib `Set` extensionality
in the four set-equality theorems above.

## Phase 4 — Hardening (find-and-prove oracle hunt)

One solid round. The four required artifacts follow. Outcome: no
unresolved high/medium finding; one informational finding adopted as a
reception-narrowing on `Erases`'s doc.

### Artifact 1 — Oracle table (public observations ↦ hidden predicate)

This formalization is a SPECIFICATION artifact (Prop obligations and
modeled set-shapes), not a capability handle hiding a secret, so the
"oracle" framing classifies each exported observation against its
intended spec content rather than a confidentiality threat model.

| Public observation | Hidden predicate it computes | Allowed? |
| --- | --- | --- |
| `ConstructionPaths.covers` projection | `∀ x∈S(m A), ∃p, x∈R(path p)` (the coverage obligation) | yes — this IS the spec |
| `ConstructionPaths.Paths` projection | the enumerated path index type | yes — intended |
| `ProofObligation.{erased,flows,needs}` | the three premises | yes — Prop, proof-irrelevant |
| `Text.den` / `Text.supp` | denotation / reception support shape | yes — numeric distribution deliberately unmodeled |
| `#synth Repr/BEq/DecidableEq/Inhabited` on Text/Rank | derived observer oracle | NONE synthesize — clean |
| `T.rec` / `T.casesOn` (all structures) | field exposure | only the intended fields; `ProofObligation` is `Prop` so no large-elim data leak |

### Artifact 2 — Ranked target list (score = claim_strength × control × proof_gap)

1. `sufficientWithPaths_imp_irepr_empty` + `covers` — the just-closed
   vacuity; strongest claim ("sufficient for EVERY construction
   path"). Attacked hardest. RESULT: sound (see Artifact 3).
2. `ConstructiveDominance` / `Strict` premises — "pinned contract"
   strong word. RESULT: behavior-preservation premise load-bearing.
3. `AdmissibleRewrite` support-subset — "no new unintended reading".
   RESULT: drop-support mutant RED; trans depends on it.
4. `Erases`/`Needs`/`Flows` — proof-preservation. RESULT: value
   preservation load-bearing; `_P` decorative (finding F1).
5. `SingleSource` / `Confluent` — normalization invariants. RESULT:
   non-functional / non-idempotent mutants RED.
6. Export surface of all structures — RESULT: clean (Artifact 1).

### Artifact 3 — Mutation list with command outputs

All 11 headline mutants re-run under the independent review; every one
FAILS to compile with a genuine type/proof error (not a lint):

```text
mut_admissible_drop_support.lean:31:2:   error: Type mismatch
mut_confluent_nonidempotent.lean:21:2:   error: Tactic `rfl` failed
mut_constructive_discharges_obl.lean:29:2: error: Type mismatch
mut_no_reachable_invalid_ordinary.lean:32:2: error: Tactic `rfl` failed
mut_proofobl_drop_needs.lean:28:2:       error: Type mismatch
mut_selfsim_conflate.lean:26:38:         error: Type mismatch
mut_shrink_codomain_drop_value.lean:26:2: error: Type mismatch
mut_singlesource_second_determinant.lean:27:2: error: Tactic `rfl` failed
mut_strict_drop_pin.lean:36:2:           error: Tactic `rfl` failed
mut_sufficient_empty_paths.lean:37:33:   error: Type mismatch
mut_trusted_reaches_valid.lean:34:2:     error: Type mismatch
```

Coverage sweep (lower-power mutations recompiled against the whole
theorem set):

- WEAK COVERAGE (covers over C instead of S): RED — the coverage call
  `coversC x (by exact hx.1)` fails Type mismatch (x∈S supplied where
  x∈C needed). The S-coverage is load-bearing; the vacuity fix is
  minimal.
- STRICT behavior changed on C: RED — `ContractBehaviorPreserved`
  unprovable.
- Honest green degeneracies (NOT missing theorems; informational):
  `Erases g P ↔ Erases g Q` (def independent of `P`); `Flows` with
  `D=univ` trivially true; `AdmissibleRewrite univ` trivially true;
  constant-val `SingleSource` and `Confluent id` GREEN. All are
  semantically correct degenerate instances, not unpinned behavior.

### Artifact 4 — Independent subagent review transcript (verdict)

A general-purpose subagent ran the same hunt independently. Verdict:
"I could not break any kernel claim. Every headline is non-vacuous,
every rejecting mutant fails on a real type/proof error, axioms are
clean, the export surface leaks nothing beyond intended spec content,
and the SufficientWithPaths coverage fix is sound under direct
adversarial attack." Two non-soundness findings:

- **F1 (adopted):** `Erases`'s `_P` binder is decorative — the def is
  independent of `P`. Honest but over-readable. ADOPTED as a
  reception-narrowing: the doc comment now states explicitly that `P`
  NAMES which evidence is dropped (not a constraint), the value-
  preservation is the formal content pinned by
  `proofObligation_reconstitute`, and `P`'s role is pinned via `Needs`
  in `proofObligation_needs_holds`. claims.toml `pp.erases` statement
  updated to match.
- **F2 (accepted as-is):** the mutants are hand-built counter-model
  witnesses rather than literal premise-deletions of the named decl.
  This is a sound (arguably stronger) witness style — it exhibits a
  concrete model where the dropped premise is required — so no change.

### Remaining / left for a future round

- The mutants could additionally be expressed as literal
  premise-deletion re-declarations of each headline for a second,
  independent kill signal (F2). Deferred; the current counter-model
  mutants already kill every headline.
- `find-and-prove` recommends a `LowEq`/two-run theorem family for
  confidentiality artifacts; not applicable here (no role/secret —
  this is a state-validity calculus, not an information-flow system),
  so it is intentionally out of scope.

## Iteration 11 — HIGH-1 round-2: give the statement-binding gate teeth (2026-06-v6)

A round-2 audit proved the HIGH-1 fix from Iteration 10 had NO TEETH:
gutting a headline (`admissibleRewrite_trans` → `: True := trivial`)
kept `lake build` GREEN and `check_claims` could still PASS. Two
compounding holes:

1. `StatementBinding.lean` was NOT in the build graph (not imported by
   `Ssm`, an explicit orphan-check exception), so `lake build` never
   compiled the guard — a gutted headline kept the build green.
2. `check_claims` verified the guard and mutants with `lake env lean
   <file>`, which loads PREBUILT oleans and does not recompile changed
   sources; with the cross-worktree artifact cache restoring stale
   project oleans even across `lake build`, the gate judged STALE
   artifacts (the old, real theorem) after a source edit.

Fix (make the gate self-contained and cache-safe):

- **Build-integrate the guard.** `Ssm.lean` now `import StatementBinding`
  and the lakefile lists it as a `lean_lib` root; the orphan check no
  longer exempts it (it is reachable from `Ssm` like any module). The
  guard applies each headline decl to fresh hypotheses and consumes its
  real conclusion, so a gutted/weakened headline now fails `lake build`
  itself at the `StatementBinding` target.
- **Cache-safe rebuild before judging.** `check_claims` now, before any
  Lean-invoking check, deletes the PROJECT module oleans (mathlib lives
  in a separate package dir and is untouched) and runs `lake build`
  with `LAKE_ARTIFACT_CACHE=false`. A failed build IS a gate failure.
  The decl probe, mutant gate, and guard-id checks run only when that
  build succeeds, and the probe/mutant `lake env lean` invocations carry
  the SAME `LAKE_ARTIFACT_CACHE=false` so they compile against fresh
  oleans, never stale cached ones. The gate sets its own env — it does
  not depend on the caller setting any variable.

Verification (clean tree, NO manual env vars):

- a) `./check_claims` → PASS (rows=68, kernel-theorems=19, headlines=11,
  residuals=8; version 2026-06-v6 / lakefile 2026.6.6).
- b–d) Gut `admissibleRewrite_trans` (Reception.lean) → `lake build`
  FAILS at the `StatementBinding` target ("Function expected at
  `Reception.admissibleRewrite_trans` but this term has type `True`"),
  and `check_claims` FAILS ("cache-disabled `lake build` FAILED …
  StatementBinding.lean:46:2 … has type True"). Restore → PASS.
- e) Repeated for a SECOND headline in a different module —
  `shrink_codomain_keeps_range` (Operations.lean): `lake build` and
  `check_claims` both FAIL pointing at `StatementBinding.lean:106:2`
  / `Fn.shrink_codomain_keeps_range … has type True`. Restore → PASS.
  Proves the bite is general, not special-cased to one decl.
- f) HIGH-2 residual-rank probe still bites: flipping `core.gap_iff`
  (kernel-theorem) to `residual=true` → FAIL ("a residual may only be
  evaluator-judgment | operator-policy | honest-doc").
- g) Final `lake build` green (358 jobs), `check_claims` PASS, axiom
  baseline unchanged (only `propext`/`Quot.sound` on the four
  set-equality theorems; no `sorryAx`, no `Classical.choice`, no
  project axioms).

Artifact-cache / runtime note: the robust freshness guarantee comes
from deleting the ~7 project oleans (a few KB each) and disabling the
artifact cache for the build, NOT from `lake clean` of the whole
workspace. Mathlib's 6.7G of oleans stay cached in their own package
dir, so the cache-disabled rebuild recompiles only the project modules.
End-to-end `check_claims` runtime is ~18s (vs ~17s before): the
cache-disabled project rebuild adds ~1–4s, dominated by the unchanged
mutant/probe Lean invocations. A full cold `lake clean` rebuild would
restore mathlib oleans from cache in ~35s; the targeted-olean approach
avoids that without sacrificing freshness.

Net surface change: rows/residuals/headlines/kernel-theorems unchanged
(68 / 8 / 11 / 19). All three version stamps bumped v5 → v6 (SKILL.md
metadata.version, lakefile, Reflection.lean header / claims.toml),
because gate semantics changed. Build green, gate PASS, axioms clean.

## Iteration 12 — type pin replaces the guard; lean simplification (2026-06-v7)

A third independent audit DEFEATED the Iteration 11 statement-binding
gate AGAIN. Three compounding links all failed under one co-mutation:

1. The mutants test the DEFINITION, not the theorem, so gutting a
   def-headline left its mutant red (mutant check passes).
2. The guard's per-headline id check was just `guard_src.contains(id)`,
   and each id lived in a `-- comment`, so deleting/no-opping the guard
   `example` still passed the substring check.
3. The guard `example` was ATTACKER-EDITABLE: co-mutating theorem +
   guard (or deleting the guard example while keeping the id comment)
   made `check_claims` PASS with a gutted headline.

Fix — replace the editable guard with a NON-EDITABLE TYPE PIN. Each
non-residual row now records `expected_type` in claims.toml.
`check_claims` rebuilds cache-disabled, prints every decl's type with
`#check @<decl>` under `pp.fullNames`, collapses whitespace, and
compares to `expected_type`. A theorem's type IS its full statement, so
gutting a headline to `: True := trivial` changes the printed type and
FAILS the pin — with NO editable guard in the loop. The only way to
make a gutted headline pass is to ALSO edit `expected_type` in
claims.toml, a visible reviewable change (fail-closed on a lone source
edit). Normalization is deterministic (`pp.fullNames` + stable `u_N`
universes + whitespace collapse), so an honest rebuild round-trips and
the gate does not flap.

Deletions / simplifications (zero guarantee loss, all interderivable):
- Deleted `StatementBinding.lean` (122 lines) and all its checker
  special-casing (orphan exemption, guard-compile step, id-substring
  check). Removed `import StatementBinding` from `Ssm` and its lakefile
  root. The type pin subsumes it.
- Demoted three bare-projection headlines to `headline=false`
  corollaries (still proven, decls kept): `cvp.no_reachable_invalid`
  (`trusted_boundary_no_reachable_invalid`),
  `ops.normalize_single_source` (`singleSource_functional`, body
  `h a b hk`), `inv.idempotent_normalizer` (`idempotent_fixes_image`,
  body `h x`). Removed their now-unneeded mutants (~79 lines).
- Merged the self-similarity dual: dropped the `selfsim.gap_not_missing`
  claim row + SKILL tag; `gap_not_missing` stays in Lean as a one-line
  corollary of `missing_not_gap` (each proves the other in one line).
- README "Version parity" prose corrected from v5/2026.6.5 to the live
  v7/2026.6.7, and `check_claims` now SCANS README for both stamps so
  the prose cannot silently drift again.

`strict.contract_pinned` (the one kernel-DEFINITION headline) is kept:
the type pin guards a def's TYPE but not its BODY, so its mutant
(`mut_strict_drop_pin`, red because `ContractPinned`'s body
`A'.C = A.C` is required) is the load-bearing body guard. Every other
headline is a kernel-theorem whose statement (hence body content) is in
its type and thus pinned.

Verification (clean tree, NO manual env vars):

- a) `./check_claims` → PASS (rows=67, kernel-theorems=18, headlines=8,
  residuals=8; version 2026-06-v7 / lakefile 2026.6.7).
- b) Gut `admissibleRewrite_trans` (Reception) → FAIL with a type-pin
  mismatch ("expected: ∀ … AdmissibleRewrite …; actual: True").
  Restore → PASS.
- c) Old defeat is dead: no editable guard exists to co-mutate
  (StatementBinding gone). A lone source edit FAILS; making a gutted
  headline pass REQUIRES also editing `expected_type` in claims.toml
  (demonstrated: gut + edit expected_type → PASS) — a visible
  reviewable change, the correct bar.
- d) Repeated (b) for a SECOND module: `shrink_codomain_keeps_range`
  (Operations) → type-pin FAIL ("actual: True"). Restore → PASS.
- e) Residual-rank probe still bites: flipping `select.sufficient_paths`
  (kernel-theorem) to `residual=true` → FAIL (rank guard + empty
  lean_decl/expected_type + missing justification + headline forbidden).
- f) No-flap: `./check_claims` 3× on the clean tree → PASS all three
  (type normalization is stable).
- g) Final `lake build` green (357 jobs); axioms clean (only
  `propext`/`Quot.sound` on the two set-equality theorems; no `sorryAx`,
  no `Classical.choice`, no project axioms).

Net surface deleted: StatementBinding.lean (122) + 3 mutants (~79) =
~201 Lean lines; checker net -65 deletions amid the type-pin rewrite.
Counts: rows 68→67, headlines 11→8, kernel-theorems 19→18, residuals
unchanged at 8. All stamps bumped v6 → v7 because gate semantics
changed.

---

## Current state (authoritative)

Each iteration above logs its own delta, so an intermediate "Net surface
change" line (e.g. the v5 `rows 73→68, headlines 11`) is a snapshot of
that iteration, not the present total. The authoritative counts are
whatever `./check_claims` prints; as of v8 that is **rows=70,
kernel-theorems=18, headlines=8, residuals=11** (version `2026-06-v8` /
lakefile `2026.6.8`).

---

## Experiment outcome — the canonicalization trial was reverted

Part of this work trialed making the **Lean project canonical**: rewriting
`../SKILL.md` to cite the Lean declarations (prose as the gloss), flipping
the README to "Lean is normative", and coupling the gate to SKILL.md via a
claim-id↔row bijection. That trial was **reverted**. The decision:

- `../SKILL.md` stays the **normative prose calculus**, byte-identical to
  `main`. The Lean project is a **non-normative, full, hardened reflection**
  of it, and `check_claims` is **Lean-internal** (it does not read SKILL.md;
  the bijection is gone, the anti-vacuity machinery — type pin, per-headline
  mutants, residual-rank, orphan, axioms, cache-safe rebuild — stays).

- Why: an A/B (operational skill vs formal skill, and old `$$` formal vs new
  Lean formal, isolated subagents on the same task) found the formal
  *calculus* adds concentrated value when paired with the operational skill,
  but the *Lean grounding specifically* does not change task-time
  effectiveness — its payoff is the formalization's own machine-checked
  correctness, not sharper per-task output. So the readable prose stays the
  primary artifact; the Lean is kept because it is genuinely improved
  (full-calculus, vacuity closed, adversarially hardened) and worth locking
  in.

The canonicalization is reproducible if retried: the driving prompt is
`CANONICALIZE_PROMPT.md` (kept on disk, gitignored). This log is retained as
the honest record of how the formalization was built and hardened, even
though some of its steps (the SKILL.md canonicalization, the README "Lean is
normative" flip) were subsequently undone.
