use std::fmt::Display;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser)]
#[command(author, about, next_line_help = true)]
pub struct Args {
	#[command(subcommand)]
	pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
	Stage {
		#[arg(value_enum)]
		which: Stageable,
	},
	Unstage {
		#[arg(value_enum)]
		which: UnStageable,
	},
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Stageable {
	New,
	Modified,
	Deleted,
}

impl Display for Stageable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match &self {
			Self::New => "new",
			Self::Deleted => "unstaged deleted",
			Self::Modified => "unstaged modified"
		})
	}
}

#[derive(ValueEnum, Clone, Copy)]
pub enum UnStageable {
	Added,
	Modified,
	Renamed,
	Deleted,
}

impl Display for UnStageable {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match &self {
			Self::Added => "added",
			Self::Deleted => "staged deleted",
			Self::Modified => "staged modified",
			Self::Renamed => "renamed"
		})
	}
}
