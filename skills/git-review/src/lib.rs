//! Review Git commits against the canonical atomic changes contract.
//!
//! This crate implements deterministic evidence commands for `git-review`.

mod base_ref;
mod dates;
mod git;
mod message;
mod messages;
mod output;
mod target_file;
mod targets;
mod tree_hashes;

use clap::{CommandFactory as _, Parser, Subcommand};
use skill_core::SkillError;

/// Construct an invalid-input error with a caller-supplied reason.
fn invalid(message: impl Into<String>) -> SkillError {
    SkillError::InvalidInput {
        message: message.into(),
    }
}

/// review Git commits against the canonical atomic changes contract.
#[derive(Debug, Parser)]
pub struct Args {
    /// subcommand to execute; omit to print help.
    #[command(subcommand)]
    command: Option<Command>,
}

/// supported `git-review` evidence and repair subcommands.
#[derive(Clone, Debug, Subcommand)]
enum Command {
    /// check for date ordering violations.
    CheckDates(dates::CheckDatesArgs),
    /// check for invalid commit messages.
    CheckMessages(messages::CheckMessagesArgs),
    /// compose a canonical Atomic Changes commit message.
    Message(message::MessageArgs),
    /// resolve the base ref for review.
    ResolveBase(base_ref::ResolveBaseArgs),
    /// resolve commit targets for review.
    ResolveTargets(targets::ResolveTargetsArgs),
    /// emit tree hashes for idempotence verification.
    TreeHashes(tree_hashes::TreeHashesArgs),
}

/// Run the deterministic `git-review` command selected by `args`.
///
/// # Errors
///
/// Returns `SkillError` if the selected subcommand fails.
pub fn run(args: &Args) -> Result<(), SkillError> {
    match args.command.clone() {
        Some(Command::CheckDates(check_dates_args)) => dates::run(&check_dates_args),
        Some(Command::CheckMessages(check_messages_args)) => messages::run(&check_messages_args),
        Some(Command::Message(message_args)) => message::run(&message_args),
        Some(Command::ResolveBase(resolve_base_args)) => base_ref::run(&resolve_base_args),
        Some(Command::ResolveTargets(resolve_targets_args)) => targets::run(&resolve_targets_args),
        Some(Command::TreeHashes(tree_hashes_args)) => tree_hashes::run(&tree_hashes_args),
        None => {
            Args::command().print_help()?;
            Ok(())
        }
    }
}
