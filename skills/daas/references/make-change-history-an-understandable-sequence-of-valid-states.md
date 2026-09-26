# Make change history an understandable sequence of valid states

Published commits should each change one thing, explain what and why, and pass
their applicable gates independently. Arrange them as an understandable
transformation whose valid prefixes can be integrated.

Reviewers should not need to mentally simulate repeated repairs to code
introduced earlier in the same branch to discover its final meaning. Each
verified state lets the reader move to the next without holding every previous
implementation detail in mind.

While a branch is still being shaped, fold a discovered flaw into its owning
commit and revalidate that commit and its descendants under the corrected
constraint. Once the flaw is on main, repair it through new atomic Fix commits.
Establish a valid baseline before introducing a stronger gate: repair existing
violations or use the explicit finite allow-list permitted by [protect the
smallest demonstrated improvement][related-1]. The introducing commit must
satisfy the declared gate; an exception cannot silently become an unrestricted
bypass.

Deliberately layering complexity for explanation is an explicit exception.
Subdivide until further subdivision loses the required validity.

The commit vocabulary and mechanics belong to the owning `atomic-changes` skill.

[related-1]: ./protect-the-smallest-demonstrated-improvement.md
