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
fn skill_error_invalid_input_displays_reason() {
    let error = SkillError::InvalidInput {
        message: "BASE_REF is required".to_owned(),
    };

    let rendered = error.to_string();

    assert_eq!(rendered, "invalid input: BASE_REF is required");
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
    let invalid = SkillError::InvalidInput {
        message: "x".to_owned(),
    };

    let codes = (
        io.exit_code(),
        missing.exit_code(),
        command.exit_code(),
        invalid.exit_code(),
    );

    assert_eq!(codes, (3, 2, 1, 4));
}
