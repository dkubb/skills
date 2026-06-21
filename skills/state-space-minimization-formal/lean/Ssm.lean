/-
Root module of the state-space-minimization-formal Lean
formalization. Calculus version: 2026-06-v8.

This module imports EVERY calculus module and is the sole default
target in `lakefile.toml`. The orphan check in `check_claims` fails if
any `*.lean` under `lean/` (except this root, the checker, and the
`mutants/` witnesses) is not transitively imported here — so adding an
unimported module cannot keep the build green.

Each headline's real statement is bound NOT by an editable guard file
but by `check_claims`'s type pin: it rebuilds the env cache-disabled,
prints every decl's type with `#check @decl` under `pp.fullNames`, and
compares the whitespace-normalized result to the `expected_type`
recorded in `claims.toml`. Gutting a headline (e.g. to
`: True := trivial`) changes its printed type, so the gate fails —
and the only way to make it pass is to also edit `expected_type` in
`claims.toml`, a visible reviewable change.
-/

import Reflection
import ProofPreservation
import Reception
import SelfSimilarity
import Operations
