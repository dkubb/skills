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
mod tests {
    use pretty_assertions::assert_eq;

    use super::{CommandOutcome, SkillError};
    use std::io;
    use std::path::PathBuf;

    #[test]
    fn command_outcome_exited_displays_code() {
        let outcome = CommandOutcome::Exited(2);

        let rendered = outcome.to_string();

        assert_eq!(rendered, "exited with code 2");
    }

    #[test]
    fn command_outcome_signaled_displays_signal_message() {
        let outcome = CommandOutcome::Signaled;

        let rendered = outcome.to_string();

        assert_eq!(rendered, "terminated by signal");
    }

    #[test]
    fn skill_error_io_displays_wrapped_error() {
        let error = SkillError::from(io::Error::new(io::ErrorKind::NotFound, "boom"));

        let rendered = error.to_string();

        assert_eq!(rendered, "IO error: boom");
    }

    #[test]
    fn skill_error_missing_file_displays_path() {
        let error = SkillError::MissingFile {
            path: PathBuf::from("/tmp/run-dylint"),
        };

        let rendered = error.to_string();

        assert_eq!(rendered, "required file not found: /tmp/run-dylint");
    }

    #[test]
    fn skill_error_command_displays_command_and_outcome() {
        let error = SkillError::Command {
            command: "code-review lint".to_owned(),
            outcome: CommandOutcome::Exited(2),
        };

        let rendered = error.to_string();

        assert_eq!(rendered, "command `code-review lint` exited with code 2");
    }

    #[test]
    fn exit_code_is_distinct_per_error_class() {
        let io = SkillError::from(io::Error::new(io::ErrorKind::NotFound, "x"));
        let missing = SkillError::MissingFile {
            path: PathBuf::from("/x"),
        };
        let command = SkillError::Command {
            command: "c".to_owned(),
            outcome: CommandOutcome::Exited(1),
        };

        let codes = (io.exit_code(), missing.exit_code(), command.exit_code());

        assert_eq!(codes, (3, 2, 1));
    }
}
