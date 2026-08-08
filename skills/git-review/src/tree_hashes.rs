//! Emit tree hashes for commits in a revision range.

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::Parser;
use serde_json::Value;
use skill_core::SkillError;

use crate::git::run as run_git;
use crate::output::{json_object, write_json_line};
use crate::target_file::{Targets, load as load_targets, select};

/// emit tree hashes for a revision range.
#[derive(Clone, Debug, Parser)]
pub(crate) struct TreeHashesArgs {
    /// base ref for the revision range.
    #[arg(value_name = "BASE_REF")]
    base_ref: String,

    /// branch ref for the revision range.
    #[arg(value_name = "BRANCH_REF", default_value = "HEAD")]
    branch_ref: String,

    /// optional file with commit SHAs to include.
    #[arg(long = "commits-file")]
    commits_file: Option<PathBuf>,

    /// path to the Git repository.
    #[arg(long, default_value = ".")]
    repo: PathBuf,
}

/// Resolve the selected revision range.
fn revisions(args: &TreeHashesArgs) -> Result<String, SkillError> {
    let range = format!("{}..{}", args.base_ref, args.branch_ref);
    run_git(
        &args.repo,
        &["rev-list", "--reverse", "--topo-order", &range],
    )
}

/// Read one commit tree hash.
fn read_tree(repo: &Path, commit: &str) -> Result<String, SkillError> {
    run_git(repo, &["show", "-s", "--format=%T", commit])
}

/// Inspect a pre-resolved commit list.
fn inspect_commits(
    args: &TreeHashesArgs,
    targets: Option<&Targets>,
    commits: &str,
) -> Result<Vec<(String, String)>, SkillError> {
    let mut hashes = Vec::new();
    for commit in select(targets, commits)? {
        hashes.push((commit.to_owned(), read_tree(&args.repo, commit)?));
    }
    Ok(hashes)
}

/// Find tree hashes selected by `args`.
fn find_hashes(args: &TreeHashesArgs) -> Result<Vec<(String, String)>, SkillError> {
    let targets = args.commits_file.as_deref().map(load_targets).transpose()?;
    let commits = revisions(args)?;
    inspect_commits(args, targets.as_ref(), &commits)
}

/// Write each selected commit and tree hash.
fn run_with_writer(args: &TreeHashesArgs, out: &mut dyn Write) -> Result<(), SkillError> {
    let hashes = find_hashes(args)?;
    for (commit, tree) in hashes {
        write_json_line(
            out,
            &json_object([
                ("commit", Value::String(commit)),
                ("tree", Value::String(tree)),
            ]),
        )?;
    }
    Ok(())
}

/// Emit tree hashes for the requested range.
///
/// # Errors
///
/// Returns `SkillError` when Git, selection-file access, or output fails.
pub(crate) fn run(args: &TreeHashesArgs) -> Result<(), SkillError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    run_with_writer(args, &mut out)
}
