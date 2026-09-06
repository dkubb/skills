# Normalization

Normalization is state-space minimization applied to *information
placement*: the same fact carried in two locations creates a state
space — the disagreement between the copies — that did not exist
when the fact lived in one place. Same vocabulary from
`principles.md` (domain, codomain, range, preimage), same six
operations, same audit questions — applied at the level of how
information is distributed across the artifacts that carry it,
rather than at the level of types or runtime state.

The same procedure applies whether the artifacts are database
tables, source files, or documentation modules. Each tradition
has its own vocabulary — Codd (1970) called it normal forms,
Parnas (1972) called it information hiding, Martin called it the
acyclic dependencies principle, DITA calls it topic-based
authoring — but the underlying frame is one DAG of atoms with
the redundant edges removed.

## Formal frame

A system carries information as **atoms** — the smallest units
that mean something to the reader. The structure of the system
is a relation between atoms expressing how each is determined.

| Term | Meaning |
|---|---|
| Atom | A smallest unit of information. A column value, a function, a doc claim. |
| Functional dependency (X → Y) | Knowing X determines exactly one Y. |
| Location | A place an atom may live — a table, a module, a function, a doc section. |
| Redundancy | Two locations encoding the same atom. The disagreement states between them are the state-space leak. |
| Passthrough | A node with one incoming and one outgoing dependency that does no transformation: it forwards an atom without owning one. |
| Transitive reduction | The unique smallest DAG whose transitive closure equals the original (Aho, Garey, Ullman 1972). Removes passthroughs and redundant edges. |

A system is **normalized** when:

1. No atom is encoded in more than one location.
2. No location carries atoms it does not own.
3. The dependency graph is a DAG, transitively reduced.
4. The DAG admits a topological sort that exposes each atom
   only after its dependencies.

This is the least common form of database normalization,
information hiding, acyclic dependencies, and topic-based
authoring.

## Formal underpinnings

The operational vocabulary above (atoms, functional
dependencies, passthroughs, transitive reduction) sits inside a
deeper formal frame the skill does not need to use directly,
but which a reader importing results from adjacent literatures
may want to retrieve.

- **Normal form is a canonical representative of an equivalence
  class.** The term originates in rewrite systems (β-normal
  form, η-normal form, weak head normal form) — the unique
  element of an equivalence class chosen to represent the
  class. Database normal forms (Codd, Fagin 1977), logical
  normal forms (DNF, CNF, NNF), and the canonical-form move
  above are all instances of the same idea. Confluence
  (Church-Rosser) is the property that says a normal form
  exists; strong normalization says every reduction reaches
  it.
- **The lattice of decompositions.** For a relation, the set of
  lossless decompositions forms a lattice ordered by
  refinement; project-join normal form (5NF, Fagin 1977) is
  the join. The same lattice structure recurs in type-system
  subtyping (Pierce 2002), the lattice of program abstractions
  in abstract interpretation (Cousot & Cousot 1977), and the
  partition lattice in information theory. Normalization is a
  walk up this lattice.
- **Schema and instance form a Galois connection.** Each schema
  constrains a set of instances; each set of instances
  determines a strongest schema. Normalization moves toward
  the schema half — the strongest constraints the instance
  space admits. The same connection underlies formal concept
  analysis, abstract interpretation, and the types-as-hypotheses
  frame in `principles.md` § "Types as hypotheses".
- **Transitive reduction as a closure-operator dual.** The
  transitive closure of a relation is the smallest
  composition-closed superrelation; transitive reduction is
  its inverse on DAGs (Aho, Garey, Ullman 1972). The operator
  is a closure operator in the lattice-theoretic sense; its
  fixpoints are the transitively-closed relations. "Every
  passthrough chain has a unique minimal representative" is
  the operational content of this fact.

The skill uses the operational vocabulary because that is what
the audit procedure runs on. The formal apparatus is the bridge
to import results from outside.

## The bilateral goal applied here

1. **Shrink the domain of valid system states.** A normalized
   system cannot occupy the disagreement state between two
   copies of one atom — that state has been removed from the
   space the system can represent.
2. **Close the codomain-range gap.** Each location's outputs
   match exactly the atoms it owns. Locations no longer
   return values that another location could equally claim.

## The other operations applied here

### Shrink the domain — eliminate redundancy

Each redundant copy of an atom adds the disagreement state. The
canonical eliminations:

- **Single source of truth.** Pick one location per atom; others
  reference by name, not by value-copy.
- **Derive don't store.** When Y is functionally determined by X,
  store X and compute Y. Database `GENERATED ALWAYS AS ... STORED`,
  UI selectors over a single store, derived doc claims behind a
  link rather than restated.
- **Canonical form.** When multiple representations could carry
  the same atom (UTC vs local time, lowercase vs original-case
  identifier, sorted vs unsorted set), pick the canonical form
  and convert at the boundary. The duplicate representations
  collapse into one only when the domain contract establishes semantic
  equivalence. Shared handling is not that proof; see `principles.md`
  § "Preserve facts when reducing representations".

### Bound dependency depth — collapse passthrough chains

A chain "X → P₁ → P₂ → P₃ → Y" where each Pᵢ only forwards is a
passthrough chain. Each link is a state at which the value is
paused and re-interpreted; the chain's length is the count of
intermediate states a reader must traverse.

For each node in a chain, ask:

- **Does this node stipulate or eliminate?** (per `principles.md`
  § "Three roles a function can play"). If it only dictates the
  shape it received, it is a passthrough.
- **Does removing this node lose any information?** If no, the
  node is not load-bearing.
- **Would two readers, given the inputs, naturally inline this
  step?** If yes, the node exists for its author's convenience,
  not the reader's.

The transitive reduction is the formal version of this audit:
delete every edge implied by another path, delete every node
whose remaining role is only to carry an edge.

### Shrink the codomain — each location owns its atoms

A location's codomain is the set of atoms a reader can predict
from its name. If the name says one thing and the contents say
five, the location is **over-bundled** (god module, wide table,
catch-all doc page). If the name says one thing and the contents
say a fragment, the location is **under-bundled** (one-line
helper, single-attribute split table, doc topic that says one
sentence).

The balance point: the name and the contents agree on the same
set of atoms.

### Remove invalid intermediate representations

The passthrough is the canonical invalid intermediate at this
level — the function-boundary, table-row, or doc-section
equivalent of a half-validated struct. Delete or merge.

## Procedure

Whether the artifacts are tables, modules, or doc files, the
procedure is the same:

1. **Enumerate atoms.** The smallest units of meaning the
   system carries. For data: the facts. For code: the
   irreducible transformations. For docs: the load-bearing
   claims.
2. **Map dependencies.** For each atom, name the atoms it
   requires to be understood. The result is a directed graph.
3. **Detect redundancy.** Atoms carried in more than one
   location. Pick a single home; convert the others to
   references.
4. **Detect passthroughs.** Nodes that forward without
   stipulating or eliminating. Merge with neighbor or delete.
5. **Compute the transitive reduction.** The smallest DAG with
   the same transitive closure. Redundant edges drop out.
6. **Topologically sort.** The sort order is the reading order,
   the load order, the dependency-respecting order in which the
   artifacts can be presented.

If the graph cannot be made acyclic, you have not yet found the
right atoms. Cycles in dependency graphs are usually two atoms
mis-identified as one — split along a different axis until the
cycle breaks.

## Decompose, then recompose

The procedure above is a two-phase strategy: first atomize,
then group. The phases are not interchangeable. Trying to find
the right module boundaries before you have seen the atoms
locks in the boundaries you started with; an over-bundled
module never breaks itself apart by accumulation, and an
over-decomposed chain never collapses itself by inertia.

**Decompose (steps 1–2).** Atomize aggressively. The
zettelkasten — one atomic claim per note, references between
notes (Luhmann) — is the deepest practical decomposition. You
will not actually store one atom per file in most artifacts;
the point is to *see* the atoms, because only then is the
structure underneath visible. Atoms that turn out to share a
determinant can be recombined later; atoms hidden inside a
bundle cannot be rearranged at all.

**Analyze (steps 3–5).** Eliminate redundancy, eliminate
passthroughs, take the transitive reduction. The graph that
survives is the minimum dependency structure the system must
carry.

**Recompose (step 6 plus grouping).** Atoms that share a
determinant — same inputs determine all of them, same readers
need all of them, same change touches all of them — belong in
one location. The right module boundary is the one where atoms
inside agree on their determinants and atoms outside disagree.
Once grouped, sort topologically so each group's dependencies
appear before it.

The decomposition is mechanical; the recomposition is design.
The same strategy applies at every level:

- **Database normalization** atomizes columns, then groups by
  key.
- **Code refactoring** atomizes functions, then groups by
  determinant.
- **Documentation** atomizes to the zettel level, then groups
  by audience and dependency.
- **Atomic commits** decompose changes into the smallest coherent
  transformation that compiles and passes every gate relevant to its affected
  surfaces and transitive consumers; recompose into a branch or PR; split the
  branch further into multiple branches when its commits do not share a
  determinant (see `commits.md`).

The strategy is recursive: the recomposition at one level
becomes the decomposition at the next. Atoms compose into
modules and modules into systems; commits compose into
branches and branches into release lines; sentences compose
into sections and sections into documents. At each level the
same procedure runs — atomize, analyze, group by shared
determinant, sort topologically — and the same failure modes
appear (god-modules, passthrough chains, duplicated atoms,
unsorted DAGs).

The strategy generalizes beyond information placement: any
refactor that needs the right grain benefits from
atomize-first, then recompose, because the wrong starting
granularity is invisible until atomization exposes it.

## Application: databases

The classical case. Normal forms 1NF through 5NF are
progressively stronger statements that the schema's dependency
graph satisfies the properties above:

- **1NF** — atoms are atomic.
- **2NF** — every non-key atom depends on the whole key.
- **3NF** — every non-key atom depends only on the key.
- **BCNF** — every dependency's determinant is a superkey.
- **4NF / 5NF** — multi-valued and join dependencies normalized
  (Fagin 1977).

See `languages/sql.md` § "Domains as branded primitives",
"Generated columns", and "Restructure data to remove the
constraint" for the concrete PostgreSQL idioms. The
state-space-minimization framing makes explicit *why*
normalization wins: each denormalized fact is a state where the
copies can disagree.

## Application: code

A function boundary is an intermediate representation. The
computation crosses the boundary in a paused state,
re-interpreted at the next call site. Each boundary that does
not stipulate or eliminate is a passthrough.

### Sprawl: the over-decomposition failure

Sprawl is one concept fragmented across many small functions,
each holding a sliver. The reader reconstructs the behavior by
chasing a call chain that should have been a single function.
Each tiny function is a passthrough node; the chain is a
passthrough chain. A passthrough function `(x) => f(x)` is an
η-redex; η-reduction is the function-level instance of the
transitive-reduction move on the dependency graph (see
`least-power.md` § "Eta-reduction" for the safety gate).

The audit question for any small function:

> **What does this boundary stipulate or eliminate?**

If the answer is "nothing — it renames a sub-expression," the
boundary is a passthrough. Inline it. The boundary buys nothing;
the call site re-acquires understanding for free.

Concrete signatures of sprawl:

- A function with one caller, one expression in the body, and
  no type narrowing at the boundary.
- A chain of `with*`, `do*`, `handle*`, `process*` wrappers
  where each calls exactly the next.
- A class whose methods are pure forwarders to fields.
- Adapter / facade / wrapper layers where the source and target
  shapes are equal.

Each is a node that fails the passthrough test. Each adds one
intermediate state to the trajectory the reader must follow.

### The god function: the under-decomposition failure

The symmetric failure. The same audit question, inverted:

> **Do the atoms in this location share a determinant?**

If the body carries five unrelated atoms with five unrelated
input sets, split: each atom belongs in its own location. If
the body carries five atoms determined by one input set, the
function is already at the right grain.

This is `least-power.md` § "Under-power with composition"
applied at the function-decomposition level: sprawl
(over-decomposition) and god functions (under-decomposition)
are the two failure modes around the correct grain. Mutation
testing surfaces both — survivors against an inlined version of
a passthrough chain show the chain added no testable behavior;
survivors against a split of a god function show the splits
were always independent.

### Existing seeds in this skill

The principles file already carries the seed rules under
different names:

- `principles.md` § "Match representation to use pattern" — the
  data-level statement of the same shape-vs-operation choice.
- `principles.md` § "Three roles a function can play" — a
  passthrough does none of dictate / stipulate / eliminate.
- `ingress-and-boundaries.md` § "Restructure data to remove the
  constraint" — three canonical normalization moves at the data
  level (deltas vs absolutes, canonical form, single source of
  truth).
- `least-power.md` § "Under-power with composition" — the
  function-level statement of the over-decomposition failure.

This module is the unified frame for those rules.

## Application: documentation

Each documentation module is a location; each load-bearing
claim is an atom. The same procedure applies:

- A claim stated in two modules is redundant; pick one home and
  cross-link.
- A module whose entire content is "see X" is a passthrough;
  merge into the referring module or fold into X.
- A long chain of cross-links the reader must follow to reach a
  claim is a passthrough chain; bring the claim closer to its
  reader.
- The topological sort of the module DAG is the reading order —
  the linear sequence in which each module's dependencies have
  already been introduced when the reader reaches it.

### Progressive disclosure is the topological sort

A skill's SKILL.md is the router; the reference modules are the
nodes; the reader's optimal load order is the topological sort
of the dependency DAG.

When a reader loads modules in topological order, every concept
is primed by its dependencies. When the load order violates
topology — a module references a concept not yet introduced —
the reader absorbs a forward reference as a black box and
back-fills the meaning later. The black-box state is the
state-space leak: the reader's understanding is in an
indeterminate state until the back-fill completes.

The skill applies this rule to its own reference files. The
skill *is* a normalized DAG of concepts; the SKILL.md router is
the topological sort applied to a load order. A reference that
loads forward (module A cites module B before B has been
introduced) is the documentation analog of a passthrough chain
the reader cannot yet shorten.

### Formal definitions are dependencies of practical applications

The same topological-sort rule applies *within* each module.
A formal definition is a dependency; a practical example built
on that definition is a dependent. Introduce formal definitions
first so each practical claim lands against the formal frame
the reader already holds. This is the section-level instance
of the rule the router applies between modules. The skill
adopts this as a presentation invariant: each module opens with
its formal frame (vocabulary, definitions, the deeper apparatus
when load-bearing) and only then applies the frame to
operational rules and concrete examples.

## Anti-patterns

- **The god module.** One file carrying many unrelated atoms.
  Over-bundled codomain; readers cannot predict which atom they
  will find.
- **The passthrough.** A node forwarding without transformation.
  Adds an intermediate state with no atom of its own.
- **The duplicated atom.** The same fact restated in two
  locations. The disagreement state is the leak.
- **The cyclic dependency.** Two atoms that each require the
  other. Usually evidence of one atom miscut into two — split
  along a different axis until the cycle breaks.
- **The unsorted DAG.** A correctly-decomposed system whose
  presentation order does not respect topology. Readers
  encounter forward references as black boxes.

## When normalization is the wrong move

The state-space rule still applies: the goal is to eliminate
*invalid* states (redundancy, drift, passthroughs), not to
normalize for its own sake. Choose a denormalized form when:

- **Read performance forces a fused representation.** Cached
  views, materialized aggregates, single-pass parsers — same
  rationale as `principles.md` § "Fuse what cannot be
  eliminated".
- **The atoms are genuinely co-determined.** Two facts that
  always change together belong in one location, not two
  cross-linked locations.
- **Splitting would introduce a cycle.** Two atoms that require
  each other are not separable atoms; keep them together until
  the cut becomes clear.
- **The decomposition costs ergonomics more than the redundancy
  costs drift.** A single-attribute table that adds a JOIN to
  every read may be normalized but unwieldy.

The criterion is always: does this move reduce the number of
representable-but-invalid states, or merely relabel them?

## Cross-references

- `principles.md` § "Match representation to use pattern" — the
  shape-vs-operation choice that drives both normalization and
  fusion.
- `principles.md` § "Three roles a function can play" — a
  passthrough function does none of dictate / stipulate /
  eliminate.
- `least-power.md` § "Under-power with composition" — the
  function-level statement of the over-decomposition failure.
- `ingress-and-boundaries.md` § "Restructure data to remove the
  constraint" — three canonical normalization moves at the data
  level.
- `documentation.md` — drift between docs and code is the
  redundancy failure mode at the documentation level.
- `commits.md` — atomic commits as one decompose-then-recompose
  instance; the transformation priority (Remove → Fix →
  Refactor → … → Add) is the topological sort of the commit
  dependency graph.
- `languages/sql.md` — concrete PostgreSQL idioms for database
  normalization (`CREATE DOMAIN`, `GENERATED ALWAYS AS`,
  `EXCLUDE USING gist`).
- `history-and-lineage.md` — Codd 1970, Parnas 1972, Fagin 1977,
  Aho-Garey-Ullman 1972, Cousot & Cousot 1977, Martin's acyclic
  dependencies principle, Luhmann's zettelkasten.
