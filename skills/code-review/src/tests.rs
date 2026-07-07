use std::env;
use std::ffi::OsStr;
use std::os::unix::process::ExitStatusExt as _;
use std::path::Path;
use std::process::ExitStatus;

use pretty_assertions::assert_eq;

use super::{
    Args, CommandOutcome, SkillError, failure_outcome, lints_dir, report, require_runner,
    resolve_cwd, run,
};
use clap::Parser as _;

#[test]
fn run_without_subcommand_returns_unit_success() {
    let args = Args::parse_from(["code-review"]);

    let result = run(&args);

    assert_eq!(result.ok(), Some(()));
}

#[test]
fn lints_dir_ends_with_lints() {
    let dir = lints_dir();

    assert_eq!(dir.file_name(), Some(OsStr::new("lints")));
}

#[test]
fn require_runner_finds_bundled_runner() {
    let runner = require_runner(&lints_dir());

    assert_eq!(runner.ok(), Some(lints_dir().join("run-dylint")));
}

#[test]
fn require_runner_missing_returns_missing_file_error() {
    let result = require_runner(Path::new("/nonexistent-xyz"));

    assert!(matches!(result, Err(SkillError::MissingFile { .. })));
}

#[test]
fn resolve_cwd_some_returns_that_dir() {
    let result = resolve_cwd(Some(Path::new("/tmp")));

    assert_eq!(result.ok(), Some(Path::new("/tmp").to_path_buf()));
}

#[test]
fn resolve_cwd_none_returns_current_dir() {
    let result = resolve_cwd(None);

    assert_eq!(result.ok(), env::current_dir().ok());
}

#[test]
fn failure_outcome_some_is_exited() {
    let code: i32 = 1;
    let outcome = failure_outcome(Some(code));

    assert_eq!(outcome, CommandOutcome::Exited(code));
}

#[test]
fn failure_outcome_none_is_signaled() {
    let outcome = failure_outcome(None);

    assert_eq!(outcome, CommandOutcome::Signaled);
}

#[test]
fn report_success_returns_unit() {
    let result = report(ExitStatus::from_raw(0), Path::new("/tmp"));

    assert_eq!(result.ok(), Some(()));
}

#[test]
fn report_nonzero_exit_returns_command_error() {
    let code: i32 = 1;
    let result = report(ExitStatus::from_raw(256), Path::new("/tmp"));

    assert!(matches!(
        result,
        Err(SkillError::Command {
            outcome: CommandOutcome::Exited(c),
            ..
        }) if c == code
    ));
}

#[test]
fn report_signal_returns_command_error() {
    let result = report(ExitStatus::from_raw(15), Path::new("/tmp"));

    assert!(matches!(
        result,
        Err(SkillError::Command {
            outcome: CommandOutcome::Signaled,
            ..
        })
    ));
}
