//! Review Git commits against the canonical atomic changes contract.
//!
//! This crate implements deterministic evidence commands for `git-review`.

mod message;

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
    /// compose a canonical Atomic Changes commit message.
    Message(message::MessageArgs),
}

/// Run the deterministic `git-review` command selected by `args`.
///
/// # Errors
///
/// Returns `SkillError` if the selected subcommand fails.
#[inline]
pub fn run(args: &Args) -> Result<(), SkillError> {
    match args.command.clone() {
        Some(Command::Message(message_args)) => message::run(&message_args),
        None => {
            Args::command().print_help()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;
