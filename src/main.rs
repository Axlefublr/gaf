mod parser;
mod git;

fn main() {
	let git_status = parser::Stats::new_from_git_status();
	println!("{:#?}", git_status)
}
