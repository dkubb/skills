use skill_core::{CommandOutcome, SkillError};

use super::{decode_stdout, outcome, run, run_program};
use crate::test_repo::TestRepo;

#[test]
fn run_returns_trimmed_stdout() {
    let repo = TestRepo::new();
    let commit = repo.commit("Add foundation");

    assert_eq!(
        run(repo.path(), &["rev-parse", "HEAD"]).expect("resolve HEAD"),
        commit
    );
}

#[test]
fn run_reports_stderr_and_exit_code() {
    let repo = TestRepo::new();

    let error = run(repo.path(), &["rev-parse", "--verify", "missing"])
        .expect_err("missing revision must be rejected");
    let actual = match error {
        SkillError::Command { command, outcome } => Some((command, outcome)),
        SkillError::Io(_) | SkillError::InvalidInput { .. } | SkillError::MissingFile { .. } => {
            None
        }
        _ => None,
    };

    assert_eq!(
        actual,
        Some((
            "git rev-parse --verify missing: fatal: Needed a single revision".to_owned(),
            CommandOutcome::Exited(128)
        ))
    );
}

#[test]
fn run_reports_failure_without_stderr() {
    let repo = TestRepo::new();
    let _base = repo.commit("Add foundation");
    let _path = repo.write("untracked", "content");

    assert_eq!(
        run(
            repo.path(),
            &["diff", "--no-index", "/dev/null", "untracked"]
        )
        .expect_err("different files must report exit one")
        .to_string(),
        "command `git diff --no-index /dev/null untracked` exited with code 1"
    );
}

#[test]
fn run_program_reports_command_start_failure() {
    let repo = TestRepo::new();

    let error = run_program("missing-git-review-command", repo.path(), &[])
        .expect_err("missing command must fail");

    assert_eq!(error.exit_code(), 3);
}

#[test]
fn decode_stdout_rejects_invalid_utf8() {
    let error = decode_stdout(vec![0xff]).expect_err("invalid UTF-8 must fail");

    assert_eq!(error.exit_code(), 3);
}

#[test]
fn outcome_preserves_process_state() {
    let exit_code: i32 = 7;

    assert_eq!(
        [outcome(Some(exit_code)), outcome(None)],
        [CommandOutcome::Exited(7), CommandOutcome::Signaled]
    );
}
