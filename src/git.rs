use crate::args::Stageable;
use crate::args::UnStageable;
use std::error::Error;
use std::process::Command;

fn get_root_dir() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .map_err(|_| "`git` is not in your $PATH")?;
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim())?;
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_owned())
}

pub fn status() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|_| "`git` is not in your $PATH")?;
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim())?;
    }
    let git_status = String::from_utf8(output.stdout)?.trim_end().to_owned();
    if git_status.is_empty() {
        Err("there are no git changes in this directory")?
    } else {
        Ok(git_status)
    }
}

pub fn stage(what: Vec<String>, unstage_type: Stageable) -> Result<(), Box<dyn Error>> {
    if what.is_empty() {
        Err(format!("you have no {} files", unstage_type))?;
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("git add {}", what.join(" ")))
        .current_dir(get_root_dir()?)
        .output()
        .unwrap();
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim())?;
    }
    Ok(())
}

pub fn unstage(what: Vec<String>, unstage_type: UnStageable) -> Result<(), Box<dyn Error>> {
    if what.is_empty() {
        Err(format!("you have no {} files", unstage_type))?;
    }
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("git restore --staged {}", what.join(" ")))
        .current_dir(get_root_dir()?)
        .output()?;
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim().to_owned())?;
    }
    Ok(())
}
