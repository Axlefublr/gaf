use std::process::ExitCode;
use crate::args::Args;
use crate::parser::GitStatus;
use clap::Parser;

mod args;
mod git;
mod parser;

fn main() -> ExitCode {
	let args = Args::parse();
	let git_status = GitStatus::new_from_git_status();
	println!("{:#?}", git_status);
	ExitCode::SUCCESS
}
