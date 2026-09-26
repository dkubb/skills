# Be intentional, including when making exceptions

Apply one coherent set of principles to humans and agents. A departure must be
visible at the relevant decision and explain its reason.

An explicit trade-off can be reviewed. An accidental exception silently changes
the system's effective rules.

The principles are guidelines, not laws. Intentionality resolves their tensions:
state which consideration takes precedence here, why, and what is sacrificed.
Dan has no known stable ordering of the remaining principles; precedence is
contextual. A useful ranking may emerge from actual decisions and corrections,
but do not invent one in advance. Compare against an understood baseline so
taste and judgment make an explicit trade-off. Explain changed precedence when
new information changes the decision; admit when a choice was wrong and preserve
what it taught you. Deliberately pushing a constraint can reveal where the
guideline needs a qualification, rather than merely producing an exception to
tolerate.

Reconsider exceptions when normal work exposes a failure, changed requirement,
unclear code, or new applicable rule. A new guideline or lint can justify a
broader audit and [ratchet][related-1]. Revisit trade-offs as understanding
changes; do not impose a fixed audit cadence or hunt for unrelated polish.

This is a design principle, not permission to override user authority or safety
requirements.

Distinguish an **applicability exception** from a **temporary violation**. A
justified permanent lint suppression says the rule is wrong for this case; keep
the reason beside it, without a routine expiry or automatic invalidation when
nearby code changes. A temporary quarantine says the rule applies but cannot yet
be satisfied: give it an explicit deadline that mechanically restores hard
failure. Extending that deadline is another deliberate decision, not the default
outcome of silence. A material change to a lint can justify a targeted review of
its exceptions; Dan considers that plausible, not an established personal
workflow. Grandfathered migration debt has its own [shrinking-list
ratchet][related-1].

[related-1]: ./protect-the-smallest-demonstrated-improvement.md
