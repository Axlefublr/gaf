use crate::args::Stageable;
use crate::args::UnStageable;
use std::process::Command;

fn get_root_dir() -> Result<String, String> {
    let output = match Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
    {
        Err(_) => return Err("`git` is not in your $PATH".to_owned()),
        Ok(v) => v,
    };
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)
            .expect("git rev-parse stderr should convert to a string")
            .trim()
            .to_owned());
    }
    let root_dir = String::from_utf8(output.stdout)
        .expect("git rev-parse failed to convert to a string")
        .trim()
        .to_owned();
    Ok(root_dir)
}

pub fn status() -> Result<String, String> {
    let output = match Command::new("git").arg("status").arg("--porcelain").output() {
        Err(_) => return Err("`git` is not in your $PATH".to_owned()),
        Ok(v) => v,
    };
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr)
            .expect("git status stderr should convert to a string")
            .trim()
            .to_owned());
    }
    let git_status = String::from_utf8(output.stdout)
        .expect("git status --porcelain failed to convert to a string")
        .trim_end()
        .to_owned();
    if git_status.is_empty() {
        Err("there are no git changes in this directory".to_owned())
    } else {
        Ok(git_status)
    }
}

pub fn stage(what: Vec<String>, unstage_type: Stageable) -> Result<(), String> {
    if what.is_empty() {
        return Err(format!("you have no {} files", unstage_type));
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("git add {}", what.join(" ")))
        .current_dir(get_root_dir()?)
        .output()
        .unwrap();
    match output.status.success() {
        true => Ok(()),
        false => Err(String::from_utf8(output.stderr)
            .expect("git add stderr should convert to a string")
            .trim()
            .to_owned()),
    }
}

pub fn unstage(what: Vec<String>, unstage_type: UnStageable) -> Result<(), String> {
    if what.is_empty() {
        return Err(format!("you have no {} files", unstage_type));
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("git restore --staged {}", what.join(" ")))
        .current_dir(get_root_dir()?)
        .output()
        .unwrap();
    match output.status.success() {
        true => Ok(()),
        false => Err(String::from_utf8(output.stderr)
            .expect("git restore stderr should convert to a string")
            .trim()
            .to_owned()),
    }
}
