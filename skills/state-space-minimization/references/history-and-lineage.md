# History and lineage

State-space minimization is not new. The same idea has been rediscovered
in formal type theory, programming languages, software design, and
security, each with its own vocabulary. This file credits the lineage
and lists the canonical references.

## Origin of the slogans

- **"Make illegal states unrepresentable"** — coined by Yaron Minsky in
  *Effective ML* (Jane Street, 2010-11). The phrase is now used across
  Haskell, F#, Elm, Rust, and TypeScript without consistent
  attribution; Minsky is the source.
  https://blog.janestreet.com/effective-ml-revisited/
- **"Boolean blindness"** — coined by Bob Harper on his Existential
  Type blog (15 March 2011), attributing the term to Dan Licata.
  https://existentialtype.wordpress.com/2011/03/15/boolean-blindness/
- **"Parse, don't validate"** — Alexis King, 2019. The canonical
  modern statement of parsing as proof preservation.
  https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
- **"Names are not type safety"** — Alexis King, 2020. Why opaque
  newtypes are *extrinsically* safe, while constructive datatypes are
  *intrinsically* safe.
  https://lexi-lambda.github.io/blog/2020/11/01/names-are-not-type-safety/
- **"Constructive vs Predicative Data"** — Hillel Wayne, 2024. When
  each style fits, and the four failure modes for constructive
  modeling.
  https://www.hillelwayne.com/post/constructive/
- **Subrange types** — Pascal (Wirth 1970) and Ada (1983) both shipped
  `subtype X is Integer range 1..12` decades before the modern Rust
  pattern-types proposal. The idea is older than the modern revival.

## Foundational research

- Freeman & Pfenning, "Refinement Types for ML" (PLDI 1991) — origin
  of refinement types as a research line.
- Strom & Yemini, "Typestate: A Programming Language Concept for
  Enhancing Software Reliability" (IEEE TSE 1986) — origin of typestate.
- Honda, "Types for Dyadic Interaction" (CONCUR 1993) — origin of
  session types for protocol enforcement.
- Per Martin-Löf, "Constructive Mathematics and Computer Programming"
  (1982) — origin of dependent type theory and the constructive
  philosophy this skill builds on.
- Reynolds, "Types, Abstraction and Parametric Polymorphism" (IFIP 1983)
  — the abstraction theorem; foundation for parametric reasoning and
  Wadler's free theorems.
- Wadler, "Theorems for Free!" (FPCA 1989) — how parametric
  polymorphism narrows what a function *can possibly do* purely from
  its type.
- Wadler, "Propositions as Types" (CACM Dec 2015) — the modern
  statement of Curry-Howard.
- Pierce, *Types and Programming Languages* (MIT Press 2002) — the
  textbook reference.
- Cardelli, "Type Systems" (CRC Handbook 1996/2004) — concise survey.
- Liskov & Wing, "A Behavioral Notion of Subtyping" (TOPLAS 1994) —
  formal LSP, which is the variance underpinning of "shrink the
  domain / shrink the codomain".
- Reynolds, "Separation Logic" (LICS 2002); O'Hearn, "Resources,
  Concurrency, and Local Reasoning" (Gödel Prize 2016) — frame rule
  and locality of invariants.
- Meyer, *Object-Oriented Software Construction* (1988) — Design by
  Contract; runtime counterpart to refinement types.

## Normalization theory and modular structure

State-space minimization applied to information placement —
database normalization, code factoring, documentation structure.
The same DAG-of-atoms structure recurs under many names.

- Edgar F. Codd, "A Relational Model of Data for Large Shared Data
  Banks" (CACM 1970) — the founding paper of relational database
  theory and the first three normal forms.
- Ronald Fagin, "Multivalued Dependencies and a New Normal Form for
  Relational Databases" (ACM TODS 1977) — fourth normal form,
  multivalued dependencies, the lattice of lossless decompositions.
- Ronald Fagin, "A Normal Form for Relational Databases That Is
  Based on Domains and Keys" (ACM TODS 1981) — domain-key normal
  form (DKNF), the strongest classical form.
- David Parnas, "On the Criteria To Be Used in Decomposing Systems
  into Modules" (CACM 1972) — information hiding; modules
  organized by the decisions they encapsulate rather than by the
  steps of the computation.
- Robert C. Martin, "The Acyclic Dependencies Principle" (Object
  Mentor; later in *Clean Architecture*) — software-architecture
  formulation of the same DAG-of-modules discipline.
- Alfred V. Aho, Michael R. Garey, Jeffrey D. Ullman, "The
  Transitive Reduction of a Directed Graph" (SIAM J. Computing
  1972) — the unique minimal DAG with the same transitive closure;
  the formal account of collapsing redundant edges and passthrough
  chains.
- Patrick Cousot, Radhia Cousot, "Abstract Interpretation: A
  Unified Lattice Model for Static Analysis of Programs by
  Construction or Approximation of Fixpoints" (POPL 1977) — the
  lattice-of-program-abstractions frame; the cross-domain bridge
  from database normalization to static analysis.
- Bernhard Ganter, Rudolf Wille, *Formal Concept Analysis:
  Mathematical Foundations* (Springer 1999) — Galois connections
  between objects and attributes; the formal frame for
  schema/instance duality.
- Niklas Luhmann, *Kommunikation mit Zettelkästen* (1981) — the
  zettelkasten method as atomic decomposition for knowledge work;
  "atomize first, then recompose" applied to ideas.
- Donald Knuth, "Literate Programming" (Computer Journal 1984) —
  code as a graph of named chunks; the literate-programming
  ancestor of topic-based authoring.

## Modern Rust verification

The 2022-2025 wave of Rust verification tools brings refinement and
dependent types to mainstream Rust. None is yet stable in the core
toolchain.

- Lehmann, Geller, Vazou, Jhala, "Flux: Liquid Types for Rust" (PLDI
  2023). https://dl.acm.org/doi/10.1145/3591283
- Gäher et al., "RefinedRust: A Type System for High-Assurance
  Verification of Rust Programs" (PLDI 2024).
  https://plv.mpi-sws.org/refinedrust/paper-refinedrust.pdf
- Lattuada et al., "Verus: Verifying Rust Programs using Linear Ghost
  Types" (OOPSLA 2023).
- Ho & Protzenko, "Aeneas: Rust Verification by Functional Translation"
  (ICFP 2022). https://dl.acm.org/doi/10.1145/3547647
- Denis, Jourdan, Marché, "Creusot" (FMSE 2022).
- Unno et al., "Thrust: A Prophecy-based Refinement Type System for
  Rust" (PLDI 2025).
- Cutner, Yoshida, Vassor, "Deadlock-Free Asynchronous Message
  Reordering in Rust with Multiparty Session Types" — Rumpsteak
  (PPoPP 2022).
- Chen, Balzer, Toninho, "Ferrite: A Judgmental Embedding of Session
  Types in Rust" (ECOOP 2022). https://arxiv.org/abs/2205.06921

## Practitioner and design tradition

- Eric Evans, *Domain-Driven Design* (2003) — value object, aggregate,
  bounded context, anti-corruption layer (chs. 5, 6, 14).
- Vaughn Vernon, *Implementing Domain-Driven Design* (2013) — the
  practical companion (esp. ch. 6 Value Objects, ch. 10 Aggregates).
- Bergh Johnsson, Deogun, Sawano, *Secure by Design* (2019) — domain
  primitive as a security-carrying value object; bridges DDD to LangSec.
- Scott Wlaschin, *Designing with Types* series (8 parts) and *Domain
  Modeling Made Functional* (Pragmatic Bookshelf, 2018) — the F#
  curriculum on this topic; single-case discriminated union, choice
  type, constrained type.
  https://fsharpforfunandprofit.com/series/designing-with-types/
- Richard Feldman, "Making Impossible States Impossible" (ElmConf 2016)
  — the canonical Elm/UI angle. https://www.youtube.com/watch?v=IcgmSRJHu_8
- Edwin Brady, *Type-Driven Development with Idris* (Manning 2017) —
  the dependent-types practitioner reference.
- Khalil Stemmler, "Make Illegal States Unrepresentable in TypeScript".
  https://khalilstemmler.com/articles/typescript-domain-driven-design/make-illegal-states-unrepresentable/
- Chris Krycho, "Making Illegal States Unrepresentable — In TypeScript".
  https://v5.chriskrycho.com/journal/making-illegal-states-unrepresentable-in-ts/
- Noonan, "Ghosts of Departed Proofs" (Haskell Symposium 2018) —
  phantom-tag preservation of proofs across pipelines.
- Mark S. Miller, *Robust Composition* (PhD dissertation, 2006) —
  object-capability model; canonical reference for capability tokens
  as authority.

## Security-flavored lineage

- Bratus, Patterson et al., "From Shotgun Parsers to More Secure Stacks"
  (LangSec). http://langsec.org/ShotgunParsersShmoo.pdf
- Sassaman, Patterson, Bratus, "The Halting Problems of Network Stack
  Insecurity" (USENIX ;login: 2011) — LangSec foundational paper.
  https://langsec.org/papers/Sassaman.pdf
- Bratus et al., "The Seven Turrets of Babel" (LangSec) — the
  shotgun-parsing antipattern in context.

## Adjacent traditions worth a search

The same problem under different names. Searching for any of these
phrases will surface complementary material.

- refinement types, liquid types, bounded refinement types, Flux, Liquid
  Haskell
- correct by construction, intrinsic vs extrinsic safety
- constructive vs predicative data, parse don't validate, names are not
  type safety
- ghosts of departed proofs, phantom tags, witness types, GADTs
- typestate, session types, multiparty session types, linear types,
  affine types, substructural types
- dependent types, sized types, finite types, singleton types, sigma /
  pi types
- value object, domain primitive, aggregate root, bounded context,
  anti-corruption layer
- branded type, opaque type, nominal brand, single-case discriminated
  union, sealed class, sealed interface, value class, inline class
- choice type, workflow type, narrow type, constrained type
- success typing (Erlang/Dialyzer)
- object-capability model, capability token, ocap, principle of least
  authority
- effect system, algebraic effects, capability-passing style
- Hyrum's Law, Postel's Law (and its security inversion in LangSec)
- design by contract, Hoare logic, separation logic, frame rule
- Liskov substitution principle, behavioral subtyping, variance
- shotgun parsing, language-theoretic security, parse at the boundary
- normal form (rewrite systems, β/η, Church-Rosser, confluence,
  strong normalization)
- database normalization, 3NF, BCNF, 4NF, 5NF, DKNF, functional
  dependency, multivalued dependency, join dependency
- information hiding, acyclic dependencies principle, stable
  dependencies principle, common closure principle
- transitive reduction, transitive closure, Hasse diagram,
  topological sort, DAG algorithms
- lattice theory, lattice of decompositions, partition lattice,
  Galois connection, formal concept analysis
- abstract interpretation, fixpoint computation, Knaster-Tarski,
  Cousot framework
- zettelkasten, atomic note taking, topic-based authoring, DITA,
  literate programming

## Cross-references

- `principles.md` — the foundation built on this lineage.
- `constructive-vs-predicative.md` — King's intrinsic/extrinsic split.
- `boolean-blindness.md` — Harper / Licata coinage.
- `architectural-scopes.md` — DDD vocabulary.
- `proof-preservation.md` — refinement types, GDP, GADTs, dependent
  types.
- `normalization.md` — Codd, Fagin, Parnas, Martin's acyclic
  dependencies, Aho-Garey-Ullman, Cousot, Luhmann's zettelkasten.
