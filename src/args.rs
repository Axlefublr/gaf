use std::fmt::Display;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    #[command(visible_alias = "s")]
    Stage {
        #[command(subcommand)]
        which: Stageable,
    },
    #[command(visible_alias = "u")]
    Unstage {
        #[command(subcommand)]
        which: UnStageable,
    },
    /// Print all the file paths of a type of a change, separated by newlines.
    #[command(visible_alias = "p")]
    Print {
        #[command(subcommand)]
        which: Change,
    },
}

#[derive(Subcommand)]
pub enum Stageable {
    #[command(visible_alias = "n")]
    New,
    #[command(visible_alias = "m")]
    Modified,
    #[command(visible_alias = "d")]
    Deleted,
}

impl Display for Stageable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::New => "new",
            Self::Deleted => "unstaged deleted",
            Self::Modified => "unstaged modified",
        })
    }
}

#[derive(Subcommand)]
pub enum UnStageable {
    #[command(visible_alias = "a")]
    Added,
    #[command(visible_alias = "m")]
    Modified,
    #[command(visible_alias = "r")]
    Renamed,
    #[command(visible_alias = "d")]
    Deleted,
}

impl Display for UnStageable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match &self {
            Self::Added => "added",
            Self::Deleted => "staged deleted",
            Self::Modified => "staged modified",
            Self::Renamed => "renamed",
        })
    }
}

#[derive(Subcommand)]
pub enum Change {
    #[command(visible_alias = "n")]
    New,
    #[command(visible_alias = "m")]
    Modified,
    #[command(visible_alias = "d")]
    Deleted,
    #[command(visible_alias = "a")]
    Added,
    /// Staged modified
    #[command(visible_alias = "s")]
    Staged,
    #[command(visible_alias = "r")]
    Renamed,
    /// Staged deleted
    #[command(visible_alias = "t")]
    Trashed,
}
