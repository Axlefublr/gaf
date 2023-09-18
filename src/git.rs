use crate::args::Stageable;
use crate::args::UnStageable;
use std::process::Command;

pub fn status() -> Result<String, String> {
	let output = match Command::new("git")
		.arg("status")
		.arg("--porcelain")
		.output()
	{
		Err(_) => return Err("`git` is not in your $PATH".to_owned()),
		Ok(v) => v,
	};
	if !output.status.success() {
		return Err(String::from_utf8(output.stderr)
			.expect("git status' stderr should convert to a string"));
	}
	let output = String::from_utf8(output.stdout)
		.expect("git status --porcelain failed to convert to a string")
		.trim_end()
		.to_owned();
	if output.is_empty() {
		Err("there are no git changes in this directory".to_owned())
	} else {
		Ok(output)
	}
}

pub fn stage(what: Vec<String>, unstage_type: Stageable) -> Result<(), String> {
	let what = what;
	if what.is_empty() {
		return Err(format!("you have no {} files", unstage_type));
	}
	let mut command = Command::new("git");
	command.arg("add");
	for path in what {
		command.arg(path);
	}
	command.output().expect("git add <files> failed to run");
	Ok(())
}

pub fn unstage(what: Vec<String>, unstage_type: UnStageable) -> Result<(), String> {
	if what.is_empty() {
		return Err(format!("you have no {} files", unstage_type));
	}
	let mut command = Command::new("git");
	command.arg("restore").arg("--staged");
	for path in what {
		command.arg(path);
	}
	command
		.output()
		.expect("git restore --staged <files> failed to run");
	Ok(())
}
