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

	pub fn new() -> Result<Self, &'static str> {
		let mut stats = GitStatus::blank();
		parse_status(&mut stats)?;
		Ok(stats)
	}
}

fn parse_status(stats: &mut GitStatus) -> Result<(), &'static str> {
	let status = git::status()?;
	for line in status.lines() {
		let mut chars = line.chars();
		let staged_char = chars.next().unwrap();
		let unstaged_char = chars.next().unwrap();
		let path: String = chars.skip(1).collect();
		let mut is_renamed = false;
		match staged_char {
			'M' => stats.staged_modifications.push(path.clone()),
			'A' => stats.added.push(path.clone()),
			'D' => stats.staged_deletions.push(path.clone()),
			'?' => stats.new.push(path.clone()),
			'R' => {
				let captures = Regex::new(r"(.+) -> (.+)")
					.unwrap()
					.captures(&path)
					.unwrap();
				let staged_deleted = captures[1].to_owned();
				let renamed_file = captures[2].to_owned();
				stats.renamed.push(staged_deleted);
				stats.renamed.push(renamed_file);
				is_renamed = true;
			}
			_ => (),
		}
		let path = match is_renamed {
			true => Regex::new(".* -> (.+)").unwrap().captures(&path).unwrap()[1].to_owned(),
			false => path,
		};
		match unstaged_char {
			'M' => stats.unstaged_modifications.push(path),
			'D' => stats.unstaged_deletions.push(path),
			_ => (),
		};
	}
	Ok(())
}
