//! Shared Git command execution.

use std::io;
use std::path::Path;
use std::process::Command;

use skill_core::{CommandOutcome, SkillError};

/// Run `git` in `repo` and return trimmed standard output.
pub(crate) fn run(repo: &Path, args: &[&str]) -> Result<String, SkillError> {
    run_program("git", repo, args)
}

/// Run `program` in `repo` so command-start failures can be tested directly.
fn run_program(program: &str, repo: &Path, args: &[&str]) -> Result<String, SkillError> {
    let output = Command::new(program)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let command = if stderr.is_empty() {
            format!("git {}", args.join(" "))
        } else {
            format!("git {}: {stderr}", args.join(" "))
        };
        let outcome = outcome(output.status.code());

        return Err(SkillError::Command { command, outcome });
    }

    decode_stdout(output.stdout).map(|stdout| stdout.trim_end().to_owned())
}

/// Decode command output without accepting malformed UTF-8.
fn decode_stdout(stdout: Vec<u8>) -> Result<String, SkillError> {
    String::from_utf8(stdout)
        .map_err(|error| SkillError::Io(io::Error::new(io::ErrorKind::InvalidData, error)))
}

/// Preserve whether a failed process exited or was terminated by a signal.
fn outcome(code: Option<i32>) -> CommandOutcome {
    code.map_or(CommandOutcome::Signaled, CommandOutcome::Exited)
}
