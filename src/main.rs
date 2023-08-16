use crate::args::Args;
use crate::parser::Stats;
use clap::Parser;

mod args;
mod git;
mod parser;

fn main() {
	let args = Args::parse();
	let git_status = Stats::new_from_git_status();
	println!("{:#?}", git_status)
}
