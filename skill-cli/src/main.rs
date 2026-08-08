//! Unified agent skills CLI.
//!
//! Dispatches to individual skill crates registered as clap subcommands.

use std::process::ExitCode;

use clap::{CommandFactory as _, Parser, Subcommand};
use skill_core::SkillError;

/// unified agent skills.
#[derive(Debug, Parser)]
#[command(name = "skill", about = "unified agent skills.")]
struct Cli {
    /// skill to run; omit to print help.
    #[command(subcommand)]
    command: Option<SkillCommand>,
}

/// available skills.
///
/// intentional growth point: this registry holds one variant per skill.
#[derive(Debug, Subcommand)]
enum SkillCommand {
    /// run a clear code review across languages and change review rules.
    CodeReview(skill_code_review::Args),
    /// review Git commits against the canonical atomic changes contract.
    GitReview(skill_git_review::Args),
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(SkillCommand::CodeReview(args)) => skill_code_review::run(args),
        Some(SkillCommand::GitReview(args)) => skill_git_review::run(args),
        None => Cli::command().print_help().map_err(SkillError::from),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::from(err.exit_code())
        }
    }
}
