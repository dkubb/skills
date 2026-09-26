# Contain complexity and account for who maintains it

Minimize owned complexity and the surface on which consumers depend. Apply the
same design discipline recursively inside software you own.

Hiding complexity behind a thin API helps callers but does not erase the
maintainer's liability. Some complexity is necessary; allocate it where it can
be understood and maintained most effectively. Criticality and the burden on
both sides matter. An unusually complex area warrants investigation, not an
automatic rule that all complexity must move to producers or consumers.

A large third-party tool can be preferable to a short script when a stable,
useful, narrow interface removes repeated work. For owned tooling, the internal
maintenance cost still counts.

Dependencies need quality and dependency audits. A trivial operation may not
justify bringing in a library. A small wrapper can narrow external results and
create a test boundary, but extensive massaging can signal that the dependency
is a poor fit.

Include specialized expertise, license fit, resource use, transitive
dependencies, maintenance, and security-patch responsiveness in that decision.
Fifty familiar lines and fifty lines of cryptography do not carry the same
ownership burden. Prefer retaining a proven, suitable dependency over replacing
it merely because extraction is now cheap. An agent-built spike and differential
testing can inform a replacement decision, but agreement on tested inputs is not
proof of equivalence over every input.

Start explicit, understand and polish the mechanism, then package demonstrated
patterns into an abstraction or convention. Each layer should be coherent at its
own level of detail. Dropping below a clean interface should reveal another
understandable layer, not machinery excused because it was hidden.

A convention must preserve the guarantees it packages. Do not trade established
type safety and assurance for convenience that depends on remembering an
implicit rule. First seek an abstraction that provides both convenience and
mechanical enforcement.

Prefer one deployable unit unless a substantial demonstrated benefit pays for
the ongoing cost of distribution. A domain boundary can remain a module
boundary. Independent scaling is a candidate justification, not an automatic
reason for separate services: include release coordination, compatibility,
network failures, and the team's capacity in the decision.
