# Lint suppression style

- Project policy: do not use `#![allow(...)]`.
  Use `#![expect(..., reason = "...")]` instead, with a clear reason.
- Rationale: `expect(...)` forces us to remove suppressions once they’re no longer needed, so improvements can’t silently backslide.
- Treat this as a cargo restriction for lint configuration.
- Reiterated on 2026-01-25.
