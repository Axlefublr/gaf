use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;

#[derive(Parser)]
#[command(author, about, next_line_help = true)]
pub struct Args {
	#[command(subcommand)]
	action: Actions
}

#[derive(Subcommand)]
pub enum Actions {
	Stage {
		#[arg(value_enum)]
		which: Stageable
	},
	Unstage {
		#[arg(value_enum)]
		which: UnStageable
	}
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Stageable {
	Unstaged,
	Modified,
	Deleted,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum UnStageable {
	Added,
	Modified,
	Renamed,
	Deleted
}