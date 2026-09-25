# Grant only capabilities that are actually needed

Default to immutability, minimal visibility, and narrow scope. Add mutation or
public access for a concrete requirement.

Unused capabilities enlarge the space of possible actions and the amount of code
a maintainer must understand.

Use safe local mutation when the implementation needs it; it need not be
semantically unavoidable. Do not add `mut` for a hypothetical future change.
Borrowing versus synchronized sharing depends on the use case; widely spreading
shared mutable state remains a concern.
