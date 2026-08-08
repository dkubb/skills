//! Find commits whose messages violate the Atomic Changes form.

use std::collections::HashSet;
use std::io::{self, Write};
use std::path::PathBuf;

use clap::Parser;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::invalid;
use crate::message::{has_atomic_subject, has_valid_action_lines, has_valid_format};
use crate::target_file::load as load_targets;

/// check commits for messages that violate the Atomic Changes form.
#[derive(Clone, Debug, Parser)]
pub(crate) struct CheckMessagesArgs {
    /// include all commits reachable from `BRANCH_REF`.
    #[arg(long)]
    root: bool,

    /// base ref for the revision range.
    #[arg(value_name = "BASE_REF", required_unless_present = "root")]
    base_ref: Option<String>,

    /// branch ref for the revision range.
    #[arg(value_name = "BRANCH_REF", default_value = "HEAD")]
    branch_ref: String,

    /// optional file with commit SHAs to include.
    #[arg(long = "commits-file")]
    commits_file: Option<PathBuf>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: String,
}

/// Resolve the ordered commits selected by `args`.
fn revisions(args: &CheckMessagesArgs) -> Result<String, SkillError> {
    if args.root {
        let root_ref = match args.base_ref.as_deref() {
            Some(value) if args.branch_ref == "HEAD" => value,
            Some(_) => {
                return Err(invalid(
                    "when --root is set, provide at most one ref (for example, --root HEAD)",
                ));
            }
            None => args.branch_ref.as_str(),
        };
        return run_git(
            &args.repo,
            &["rev-list", "--reverse", "--topo-order", root_ref],
        );
    }

    let base_ref = args
        .base_ref
        .as_deref()
        .ok_or_else(|| invalid("BASE_REF is required unless --root is provided"))?;
    let range = format!("{base_ref}..{}", args.branch_ref);
    run_git(
        &args.repo,
        &["rev-list", "--reverse", "--topo-order", &range],
    )
}

/// Return invalid commits as `(SHA, subject)` pairs.
fn find_invalid(args: &CheckMessagesArgs) -> Result<Vec<(String, String)>, SkillError> {
    let targets = args.commits_file.as_deref().map(load_targets).transpose()?;
    let commits = revisions(args)?;
    inspect_commits(args, targets.as_ref(), &commits)
}

/// Inspect a pre-resolved commit list.
fn inspect_commits(
    args: &CheckMessagesArgs,
    targets: Option<&HashSet<String>>,
    commits: &str,
) -> Result<Vec<(String, String)>, SkillError> {
    let mut invalid_commits = Vec::new();

    for commit in commits.lines() {
        if targets.is_some_and(|set| !set.contains(commit)) {
            continue;
        }

        let (subject, message) = read_message(&args.repo, commit)?;
        if !has_atomic_subject(&subject)
            || !has_valid_action_lines(&message)
            || !has_valid_format(&message)
        {
            invalid_commits.push((commit.to_owned(), subject));
        }
    }

    Ok(invalid_commits)
}

/// Read a subject and full message with one Git invocation.
fn read_message(repo: &str, commit: &str) -> Result<(String, String), SkillError> {
    let encoded = run_git(repo, &["show", "-s", "--format=%s%x00%B", commit])?;
    parse_message(&encoded)
}

/// Split the NUL-delimited output produced by `read_message`.
fn parse_message(encoded: &str) -> Result<(String, String), SkillError> {
    encoded
        .split_once('\0')
        .map(|(subject, message)| (subject.to_owned(), message.to_owned()))
        .ok_or_else(|| invalid("Git returned a message without a subject separator"))
}

/// Write invalid commits and return the range result.
fn run_with_writer(args: &CheckMessagesArgs, out: &mut dyn Write) -> Result<(), SkillError> {
    let invalid_commits = find_invalid(args)?;

    for (commit, subject) in &invalid_commits {
        let line = format!("{commit} {subject}\n");
        out.write_all(line.as_bytes())?;
    }

    if invalid_commits.is_empty() {
        Ok(())
    } else {
        Err(invalid(
            "found commit messages that violate the Atomic Changes form",
        ))
    }
}

/// Print each invalid commit as `<SHA> <subject>`.
///
/// # Errors
///
/// Returns `SkillError` when Git or file access fails, output cannot be
/// written, or an invalid message is found.
pub(crate) fn run(args: &CheckMessagesArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    run_with_writer(args, &mut out)
}

#[cfg(test)]
mod tests;
