//! CLI behavior tests for the `skill` binary.
//!
//! `src/main.rs` is excluded from line coverage (see the `coverage` cargo
//! alias); its contract is gated here instead: no subcommand prints help to
//! stdout with exit 0, and unknown input goes to stderr with a non-zero exit.

#[cfg(test)]
mod tests;
