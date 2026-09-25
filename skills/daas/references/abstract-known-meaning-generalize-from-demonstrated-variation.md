# Abstract known meaning; generalize from demonstrated variation

Name a domain operation or introduce a useful type even with one use. Wait for
concrete variation before adding interchangeable implementations or general
configuration.

**Abstraction is knowledge compression.** Reuse and comprehension are distinct
reasons to extract. One consumer does not justify a layer whose only benefit
would be hypothetical reuse. It can justify a named, independently verified unit
that isolates necessary complexity and reduces simultaneous understanding.
Indirection is not inherently harmful; concealing relevant details, distorting
the domain, or imposing an unjustified performance cost is.

Naming can compress understanding without adding new possibilities. Speculative
generalization adds possibilities that must be maintained and may encode the
wrong common concept.

Do not add an override or extension point merely because it is cheap to write.
Wait for a concrete consumer that needs different behavior; the supported
configurations and their interactions remain a maintenance cost after the
implementation is finished.

A PostgreSQL-only application usually does not need a generic storage trait just
in case its database changes. Its real dependency may include specific database
semantics. Accept some duplication while learning what actually varies, usually
from three or more concrete cases.

The count is a heuristic, not a quota: two cases that evolve in lockstep over
time may provide strong evidence. First *reduce the delta* between candidate
implementations, then live with their alignment while they remain independent.
If identical names, fields, and operations make one domain unnatural, the
apparent duplication may encode a real distinction. Generalize only when the
shared meaning earns confidence and improves understanding. Keep the extraction
small enough that reverting it is an easy recovery from a mistaken abstraction.

Prefer a standard or portable form when behavior, performance, clarity, and cost
are otherwise equal. This tiebreaker does not justify extra complexity or
discarding useful vendor semantics for a hypothetical migration.

An I/O testing seam is already a concrete need: it can preserve
PostgreSQL-specific semantics without inventing a general database abstraction.
A deliberately general protocol client has different requirements from a narrow
application interface; established protocol behavior can itself justify
generality.

A declarative language must buy back the cost of its constrained representation.
Start with a clear ordinary API or builder; a configuration parser should
construct the same model through that interface. Prefer an existing
understandable format, such as YAML, when it meets the need. A custom DSL has a
high bar because its grammar, parser, diagnostics, tests, and extra indirection
become owned machinery. Different users can justify different interfaces;
repetition alone does not justify a new language.

**Design an application as a library that happens to be assembled into an
application.** Preserve useful composition boundaries without assuming the
system is the final consumer. Composability does not require unlimited
configurability: embedding a capability differs from expanding what it means.
Support demonstrated variation within a coherent domain; some workflows can
legitimately remain outside it.

Apply the same abstraction discipline to these principles. Split when concepts
have diverged in applicability or consequences, not merely when prose is long.
Keep apparent duplicates separate until shared meaning earns confidence through
independent cases or sustained evolution in lockstep. Repeated outcomes alone do
not establish one reason. Preserve cheap reversal if an abstraction fails.
