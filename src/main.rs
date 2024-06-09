use std::error::Error;

use args::Actions;
use args::Stageable;
use args::UnStageable;
use clap::Parser;

use crate::args::Args;
use crate::parser::GitStatus;

mod args;
mod git;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let git_status = GitStatus::new()?;
    match args.action {
        Actions::Stage { which } => match which {
            Stageable::Deleted => git::stage(git_status.unstaged_deletions, Stageable::Deleted)?,
            Stageable::Modified => git::stage(git_status.unstaged_modifications, Stageable::Modified)?,
            Stageable::New => git::stage(git_status.new, Stageable::New)?,
        },
        Actions::Unstage { which } => match which {
            UnStageable::Added => git::unstage(git_status.added, UnStageable::Added)?,
            UnStageable::Deleted => git::unstage(git_status.staged_deletions, UnStageable::Deleted)?,
            UnStageable::Modified => git::unstage(git_status.staged_modifications, UnStageable::Modified)?,
            UnStageable::Renamed => git::unstage(git_status.renamed, UnStageable::Renamed)?,
        },
        Actions::Print { which } => git_status.print(which),
    };
    Ok(())
}
