use std::process::Command;

pub fn status() -> Option<String> {
	let output = Command::new("git")
		.arg("status")
		.arg("--porcelain")
		.output()
		.expect("git status --porcelain failed to run");
	let output = String::from_utf8(output.stdout)
		.expect("git status --porcelain failed to convert to a string")
		.trim_end()
		.to_owned();
	if output.is_empty() {
		None
	} else {
		Some(output)
	}
}
