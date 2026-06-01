//! run a clear code review across languages and change review rules.
//!
//! the skill is primarily guidance (see SKILL.md). the `lint` subcommand
//! runs the bundled Dylint lint library against the caller's project.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

use clap::{CommandFactory as _, Parser, Subcommand};
use skill_core::{CommandOutcome, SkillError};

/// run a clear code review across languages and change review rules.
#[derive(Debug, Parser)]
#[command(name = "code-review")]
pub struct Args {
    /// subcommand to execute; omit to print help.
    #[command(subcommand)]
    command: Option<SkillCommand>,
}

/// code-review subcommands.
#[derive(Clone, Debug, Subcommand)]
enum SkillCommand {
    /// run bundled Dylint lints against the caller's project.
    Lint(LintArgs),
}

/// arguments for the `lint` subcommand.
#[derive(Clone, Debug, Parser)]
struct LintArgs {
    /// working directory to run the lints against. defaults to the current
    /// directory.
    #[arg(long, value_name = "DIR")]
    cwd: Option<PathBuf>,

    /// extra arguments forwarded verbatim to `cargo dylint -- ...`.
    /// anything after `--` lands here.
    #[arg(last = true, value_name = "DYLINT_ARGS")]
    forward: Vec<String>,
}

/// run the code-review skill.
///
/// # Errors
///
/// returns `SkillError` when the `lint` subcommand fails to invoke the
/// bundled runner or the runner exits non-zero.
pub fn run(args: &Args) -> Result<(), SkillError> {
    match &args.command {
        Some(SkillCommand::Lint(lint_args)) => run_lint(lint_args),
        None => {
            Args::command().print_help()?;
            Ok(())
        }
    }
}

/// execute the bundled `lints/run-dylint` against the caller's project.
fn run_lint(args: &LintArgs) -> Result<(), SkillError> {
    let runner = require_runner(&lints_dir())?;
    let caller_cwd = resolve_cwd(args.cwd.as_deref())?;

    let mut cmd = Command::new(&runner);
    cmd.env("DYLINT_CALLER_CWD", &caller_cwd);
    if !args.forward.is_empty() {
        cmd.args(&args.forward);
    }

    let status = cmd.status()?;

    report(status, &caller_cwd)
}

/// resolve the bundled lint runner, requiring it to exist on disk.
fn require_runner(lints_dir: &Path) -> Result<PathBuf, SkillError> {
    let path = lints_dir.join("run-dylint");
    if path.is_file() {
        Ok(path)
    } else {
        Err(SkillError::MissingFile { path })
    }
}

/// resolve the working directory to run the lints against.
fn resolve_cwd(cwd: Option<&Path>) -> Result<PathBuf, SkillError> {
    match cwd {
        Some(dir) => Ok(dir.to_path_buf()),
        None => Ok(env::current_dir()?),
    }
}

/// translate an exit code into a non-success command outcome.
fn failure_outcome(code: Option<i32>) -> CommandOutcome {
    code.map_or(CommandOutcome::Signaled, CommandOutcome::Exited)
}

/// report the lint run result: print success, or describe the failure.
fn report(status: ExitStatus, cwd: &Path) -> Result<(), SkillError> {
    if status.success() {
        println!(
            "code-review lint: bundled dylint checks passed, no findings in {}",
            cwd.display()
        );
        Ok(())
    } else {
        Err(SkillError::Command {
            command: "code-review lint".to_owned(),
            outcome: failure_outcome(status.code()),
        })
    }
}

/// resolve the absolute path to the skill's bundled lints directory.
///
/// uses the compile-time `CARGO_MANIFEST_DIR` on purpose: the lint runner
/// shells out to `lints/run-dylint`, which needs the on-disk lints source
/// tree (it clones and builds dylint there). this skill therefore only
/// works run from its source checkout, not relocated to an arbitrary path.
fn lints_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("lints")
}

#[cfg(test)]
mod tests {
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
}
