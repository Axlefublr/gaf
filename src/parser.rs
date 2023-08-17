use regex::Regex;

use crate::git;

#[derive(Debug)]
pub struct Stats {
	pub unstaged: Vec<String>,
	pub added: Vec<String>,
	pub staged_modifications: Vec<String>,
	pub unstaged_modifications: Vec<String>,
	pub renamed: Vec<String>,
	pub unstaged_deletions: Vec<String>,
	pub staged_deletions: Vec<String>,
}

impl Stats {
	fn new() -> Self {
		Self {
			unstaged: vec![],
			added: vec![],
			staged_modifications: vec![],
			unstaged_modifications: vec![],
			renamed: vec![],
			unstaged_deletions: vec![],
			staged_deletions: vec![],
		}
	}
	pub fn new_from_git_status() -> Option<Self> {
		let mut stats = Stats::new();
		parse_status(&mut stats)?;
		if are_all_empty(&stats) {
			None
		} else {
			Some(stats)
		}
	}
}

fn parse_status(stats: &mut Stats) -> Option<()> {
	let status = git::status()?;
	let regex = Regex::new(r"^(.{2}) ([^(?: \->)\n]*)(?: -> )?(.*)?$").unwrap();
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
			"?" => stats.unstaged.push(path.clone()),
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

fn are_all_empty(stats: &Stats) -> bool {
	stats.added.is_empty()
		&& stats.unstaged_deletions.is_empty()
		&& stats.unstaged_modifications.is_empty()
		&& stats.renamed.is_empty()
		&& stats.staged_modifications.is_empty()
		&& stats.unstaged.is_empty()
		&& stats.staged_deletions.is_empty()
}
