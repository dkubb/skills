//! Find commits whose messages violate the Atomic Changes form.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::invalid;
use crate::message::{has_atomic_subject, has_valid_action_lines, has_valid_format};
use crate::output::{json_object, write_json_line};
use crate::target_file::{Targets, load as load_targets, select};

/// check commits for messages that violate the Atomic Changes form.
#[derive(Clone, Debug, Parser)]
pub(crate) struct CheckMessagesArgs {
    /// include all commits reachable from `BRANCH_REF`.
    #[arg(long)]
    root: bool,

    /// base and optional branch refs, or one root ref with `--root`.
    #[arg(value_names = ["BASE_REF", "BRANCH_REF"], num_args = 0..)]
    refs: Vec<String>,

    /// optional file with commit SHAs to include.
    #[arg(long = "commits-file")]
    commits_file: Option<PathBuf>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

/// Resolve the ordered commits selected by `args`.
fn revisions(args: &CheckMessagesArgs) -> Result<String, SkillError> {
    if args.root {
        let root_ref = match args.refs.as_slice() {
            [] => "HEAD",
            [value] => value,
            [_, _, ..] => {
                return Err(invalid(
                    "when --root is set, provide at most one ref (for example, --root HEAD)",
                ));
            }
        };
        return run_git(
            &args.repo,
            &["rev-list", "--reverse", "--topo-order", root_ref],
        );
    }

    let (base_ref, branch_ref) = match args.refs.as_slice() {
        [] => {
            return Err(invalid("BASE_REF is required unless --root is provided"));
        }
        [base_ref] => (base_ref.as_str(), "HEAD"),
        [base_ref, branch_ref] => (base_ref.as_str(), branch_ref.as_str()),
        [_, _, ..] => {
            return Err(invalid(
                "provide a base ref and at most one branch ref (for example, main HEAD)",
            ));
        }
    };
    let range = format!("{base_ref}..{branch_ref}");
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
    targets: Option<&Targets>,
    commits: &str,
) -> Result<Vec<(String, String)>, SkillError> {
    let mut invalid_commits = Vec::new();

    for commit in select(targets, commits)? {
        let (subject, message) = read_message(&args.repo, commit)?;
        let atomic_subject = has_atomic_subject(&subject);
        let valid_action_lines = has_valid_action_lines(&message);
        let valid_format = has_valid_format(&message);
        if !atomic_subject || !valid_action_lines || !valid_format {
            invalid_commits.push((commit.to_owned(), subject));
        }
    }

    Ok(invalid_commits)
}

/// Read a subject and full message from Git.
fn read_message(repo: &Path, commit: &str) -> Result<(String, String), SkillError> {
    let subject = run_git(repo, &["show", "-s", "--format=%s", commit])?;
    let message = run_git(repo, &["show", "-s", "--format=%B", commit])?;
    Ok((subject, message))
}

/// Write invalid commits and return the range result.
fn run_with_writer(args: &CheckMessagesArgs, out: &mut dyn Write) -> Result<(), SkillError> {
    let invalid_commits = find_invalid(args)?;

    for (commit, subject) in &invalid_commits {
        write_json_line(
            out,
            &json_object([
                ("commit", Value::String(commit.to_owned())),
                ("subject", Value::String(subject.to_owned())),
            ]),
        )?;
    }

    if invalid_commits.is_empty() {
        Ok(())
    } else {
        Err(invalid(
            "found commit messages that violate the Atomic Changes form",
        ))
    }
}

/// Emit each invalid commit as one JSONL record.
///
/// # Errors
///
/// Returns `SkillError` when Git or file access fails, output cannot be
/// written, or an invalid message is found.
pub(crate) fn run(args: &CheckMessagesArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    run_with_writer(args, &mut out)
}
