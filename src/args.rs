use clap::Parser;
use clap::Subcommand;
use std::fmt::Display;

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
