use crate::error::ErrorKind;
use crate::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn clone(url: impl AsRef<OsStr>, target: impl AsRef<Path>) -> Result<()> {
    if target.as_ref().exists() {
        return Ok(());
    }
    let mut git = Command::new("git");
    git.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    git.arg("clone");
    git.arg("--depth");
    git.arg("1");
    git.arg(url);
    git.arg(target.as_ref());
    let output = git.output()?;
    if !output.status.success() {
        return Err(ErrorKind::CloneError.into());
    }
    Ok(())
}
