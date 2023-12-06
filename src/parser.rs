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

    pub fn new() -> Result<Self, String> {
        let mut model = GitStatus::blank();
        let git_status = git::status()?;
        parse_status(&mut model, &git_status);
        Ok(model)
    }
}

fn parse_status(model: &mut GitStatus, git_status: &str) {
    for line in git_status.lines() {
        let mut chars = line.chars();
        let staged_char = chars.next().unwrap();
        let unstaged_char = chars.next().unwrap();
        let path: String = chars.skip(1).collect();
        let mut is_renamed = false;
        match staged_char {
            'M' => model.staged_modifications.push(path.clone()),
            'A' => model.added.push(path.clone()),
            'D' => model.staged_deletions.push(path.clone()),
            '?' => model.new.push(path.clone()),
            'R' => {
                let captures = Regex::new(r"(.+) -> (.+)").unwrap().captures(&path).unwrap();
                let staged_deleted = captures[1].to_owned();
                let renamed_file = captures[2].to_owned();
                model.renamed.push(staged_deleted);
                model.renamed.push(renamed_file);
                is_renamed = true;
            }
            _ => (),
        }
        let path = match is_renamed {
            true => Regex::new(".* -> (.+)").unwrap().captures(&path).unwrap()[1].to_owned(),
            false => path,
        };
        match unstaged_char {
            'M' => model.unstaged_modifications.push(path),
            'D' => model.unstaged_deletions.push(path),
            _ => (),
        };
    }
}
