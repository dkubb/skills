# GitHub PR Comment Style

Use this reference when review feedback will be written as GitHub pull request
comments.

## Labels

- Use conventional comment labels with bold markdown:
  `__suggestion:__`, `__comment:__`, or `__question:__`.
- Use `__suggestion: (nitpick)__` only when the user explicitly wants to mark a
  point as non-blocking. Otherwise, assume a `suggestion` is expected to be
  followed.

## Structure

- Write comments in 2 parts:
  1. State the suggestion, comment, or question clearly.
  2. Add reasoning when it is not obvious.
- Keep comments high signal. Do not add praise unless the user explicitly asks
  for that style.

## Commit-by-commit fixup protocol

When reviewing a pull request with the user, proceed one commit at a time.

1. Discuss a suspected issue before leaving a comment. Do not create a review
   comment merely to preserve an observation the user has not accepted.
2. Once the user agrees the issue is real, leave one inline conventional comment
   on the exact affected line or contiguous affected range.
3. Make a visible `fixup!` commit that targets the commit that introduced the
   issue. Give the fixup body a short explanation of the intended correction;
   that body is useful while the fixup remains visible and is intentionally
   discarded by autosquash.
4. After the fixup is pushed, edit the original review comment to include a
   direct URL to that fixup commit. Keep the original finding and rationale; add
   the link as the resolution record.
5. Continue with the next review location. Do not replace an inline comment
   with a general PR comment merely because its line becomes hidden in the
   aggregate diff after the fixup; that hiding is intentional.

## Inline scope

- Prefer inline comments on the exact changed lines.
- If one point applies to multiple contiguous lines, comment on the full line
  range instead of anchoring only the first line.
- Keep each inline comment scoped to one distinct issue. Split unrelated points
  into separate comments.

## Resolving comments

- Resolve each review flag with the smallest sufficient change that addresses
  the exact concern. Do not turn a feedback response into a broader audit,
  refactor, hardening pass, or cleanup unless that extra work is required to
  resolve the flag correctly.
- Keep adjacent issues discovered while resolving feedback out of the current
  response and PR scope. Record or propose them separately; include one only
  when the user explicitly requests the expansion.
- If the smallest correct resolution is no code change, explain why and close
  the flag without manufacturing work.
- When resolving review comments, add 👍 to helpful feedback and 👎 to
  feedback you will ignore, to reinforce reviewer preferences.

## Portfolio audits

- When open pull requests are growing faster than they can be reviewed and
  merged, bias the audit toward aggressive scope reduction. Each retained PR
  must justify its carrying cost with a current concrete bug, required
  capability, or necessary dependency.
- Recommend closing speculative hardening, obsolete or duplicated work,
  low-priority cleanup, and changes with no concrete witness. Do not preserve a
  PR because of sunk effort; important work can return when a real requirement
  makes its value and minimum scope clear.
- Prefer the smallest independently reviewable retained diff. Report scope that
  should be removed, deferred, or extracted instead of treating every valid
  change as a reason to keep the current PR open.

## Copilot reviews

- When requesting a new Copilot review, hide all previous Copilot reviews
  (including the "Pull request overview") since new reviews supersede them.
  Use API dismissal with reason/message "Outdated" when available.
- When Copilot reports "generated no new comments" after a review, stop
  requesting further Copilot reviews for that PR.
