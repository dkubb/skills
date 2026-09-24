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

## Copilot reviews

- When requesting a new Copilot review, hide all previous Copilot reviews
  (including the "Pull request overview") since new reviews supersede them.
  Use API dismissal with reason/message "Outdated" when available.
- When Copilot reports "generated no new comments" after a review, stop
  requesting further Copilot reviews for that PR.
