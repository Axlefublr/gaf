use regex::Regex;

use crate::git;

#[derive(Debug)]
pub struct GitStatus {
	pub new: Vec<String>,
	pub added: Vec<String>,
	pub staged_modifications: Vec<String>,
	pub unstaged_modifications: Vec<String>,
	pub renamed: Vec<String>,
	pub unstaged_deletions: Vec<String>,
	pub staged_deletions: Vec<String>,
}

impl GitStatus {
	fn blank() -> Self {
		Self {
			new: vec![],
			added: vec![],
			staged_modifications: vec![],
			unstaged_modifications: vec![],
			renamed: vec![],
			unstaged_deletions: vec![],
			staged_deletions: vec![],
		}
	}

	pub fn new() -> Option<Self> {
		let mut stats = GitStatus::blank();
		parse_status(&mut stats)?;
		if are_all_empty(&stats) {
			None
		} else {
			Some(stats)
		}
	}
}

fn parse_status(stats: &mut GitStatus) -> Option<()> {
	let status = git::status()?;
	let regex = Regex::new(r"(.)(.) ([^(?: \-> )\n]+)(?: -> )?(.*)?($|\n)").unwrap();
	let captures = regex.captures_iter(&status);
	for capture in captures {
		let path = capture[3].to_owned();
		match &capture[1] {
			"M" => stats.staged_modifications.push(path.clone()),
			"A" => stats.added.push(path.clone()),
			"D" => stats.staged_deletions.push(path.clone()),
			"R" => {
				stats.renamed.push(path.clone());
				let staged_deletion = capture[4].to_owned();
				stats.renamed.push(staged_deletion)
			}
			"?" => stats.new.push(path.clone()),
			&_ => (),
		}
		match &capture[2] {
			"M" => stats.unstaged_modifications.push(path),
			"D" => stats.unstaged_deletions.push(path),
			&_ => ()
		}
	}
	Some(())
}

fn are_all_empty(stats: &GitStatus) -> bool {
	stats.added.is_empty()
		&& stats.unstaged_deletions.is_empty()
		&& stats.unstaged_modifications.is_empty()
		&& stats.renamed.is_empty()
		&& stats.staged_modifications.is_empty()
		&& stats.new.is_empty()
		&& stats.staged_deletions.is_empty()
}
