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
fn git_review_without_subcommand_prints_help_to_stdout_with_exit_zero() {
    let output = skill().arg("git-review").output().expect("run skill");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert!(!output.stdout.is_empty());
}

#[test]
fn git_review_message_prints_canonical_message_with_exit_zero() {
    let output = skill()
        .args([
            "git-review",
            "message",
            "add",
            "--summary",
            "message composer",
        ])
        .output()
        .expect("run skill");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, b"Add message composer\n");
}

#[test]
fn unknown_subcommand_errors_to_stderr_with_nonzero_exit() {
    let output = skill().arg("bogus").output().expect("run skill");

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}
