//! CLI behavior tests for the `skill` binary.
//!
//! `src/main.rs` is excluded from line coverage (see the `coverage` cargo
//! alias); its contract is gated here instead: no subcommand prints help to
//! stdout with exit 0, and unknown input goes to stderr with a non-zero exit.

#[cfg(test)]
mod tests {
    use std::process::Command;

    use pretty_assertions::assert_eq;

    fn skill() -> Command {
        Command::new(env!("CARGO_BIN_EXE_skill"))
    }

    #[test]
    fn no_subcommand_prints_help_to_stdout_with_exit_zero() {
        let output = skill().output().expect("run skill");

        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).lines().next(),
            Some("unified agent skills."),
        );
    }

    #[test]
    fn code_review_without_subcommand_prints_help_to_stdout_with_exit_zero() {
        let output = skill().arg("code-review").output().expect("run skill");

        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert!(!output.stdout.is_empty());
    }

    #[test]
    fn unknown_subcommand_errors_to_stderr_with_nonzero_exit() {
        let output = skill().arg("bogus").output().expect("run skill");

        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}
