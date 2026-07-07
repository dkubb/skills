//! Shared types for the unified agent skills CLI.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Shared error type for all skills.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum SkillError {
    /// IO operation failed.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// A required bundled file was not present on disk.
    #[error("required file not found: {path}")]
    MissingFile {
        /// The path that was expected to exist.
        path: PathBuf,
    },

    /// A spawned command did not exit successfully.
    #[error("command `{command}` {outcome}")]
    Command {
        /// The command that was run, for diagnostics.
        command: String,
        /// How the command terminated.
        outcome: CommandOutcome,
    },
}

impl SkillError {
    /// Exit code for this error, distinct per class so an embedding script
    /// can branch on the failure mode.
    #[must_use]
    pub const fn exit_code(&self) -> u8 {
        match self {
            Self::Command { .. } => 1,
            Self::MissingFile { .. } => 2,
            Self::Io(_) => 3,
        }
    }
}

/// How a spawned command terminated when it did not succeed.
///
/// ```
/// use pretty_assertions::assert_eq;
/// use skill_core::CommandOutcome;
///
/// assert_eq!(CommandOutcome::Exited(2).to_string(), "exited with code 2");
/// assert_eq!(CommandOutcome::Signaled.to_string(), "terminated by signal");
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandOutcome {
    /// Exited with a non-zero status code.
    Exited(i32),
    /// Terminated by a signal, with no exit code.
    Signaled,
}

impl fmt::Display for CommandOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Exited(code) => write!(f, "exited with code {code}"),
            Self::Signaled => write!(f, "terminated by signal"),
        }
    }
}

#[cfg(test)]
mod tests;
