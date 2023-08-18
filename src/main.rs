use std::process::ExitCode;
use crate::args::Args;
use crate::parser::GitStatus;
use args::Actions;
use args::Stageable;
use args::UnStageable;
use clap::Parser;

mod args;
mod git;
mod parser;

fn main() -> ExitCode {
	let args = Args::parse();
	let git_status = GitStatus::new();
	if git_status.is_none() {
		return ExitCode::FAILURE;
	};
	let git_status = git_status.unwrap();
	let result = match args.action {
		Actions::Stage { which } => { match which {
			Stageable::Deleted => git::stage(git_status.unstaged_deletions, Stageable::Deleted),
			Stageable::Modified => git::stage(git_status.unstaged_modifications, Stageable::Modified),
			Stageable::New => git::stage(git_status.new, Stageable::New),
		}},
		Actions::Unstage { which } => { match which {
			UnStageable::Added => git::unstage(git_status.added, UnStageable::Added),
			UnStageable::Deleted => git::unstage(git_status.staged_deletions, UnStageable::Deleted),
			UnStageable::Modified => git::unstage(git_status.staged_modifications, UnStageable::Modified),
			UnStageable::Renamed => git::unstage(git_status.renamed, UnStageable::Renamed),
		}}
	};
	match result {
		Err(m) => {
			eprintln!("{}", m);
			ExitCode::FAILURE
		},
		Ok(()) => ExitCode::SUCCESS
	}
}
