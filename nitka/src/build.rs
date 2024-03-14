use std::{
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::{bail, Result};

static CONTRACT_READY: Mutex<bool> = Mutex::new(false);

pub fn git_root() -> Result<String> {
    let output = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?;
    assert!(output.status.success(), "Failed to get Git repository root path");
    let git_root = String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .to_string();

    Ok(git_root)
}

/// Compile contract in release mode and prepare it for integration tests usage
pub fn build_contract(make_command: Option<&str>) -> Result<()> {
    let mut ready = CONTRACT_READY.lock().unwrap();

    if *ready {
        return Ok(());
    }

    let output = Command::new("make")
        .arg(make_command.unwrap_or("build"))
        // Assuming that the Makefile is in root repository directory
        .current_dir(git_root()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("Failed to build contract. Output: {output:?}");
    }

    *ready = true;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::thread::spawn;

    use crate::build::build_contract;

    #[tokio::test]
    async fn test_build_contract() -> anyhow::Result<()> {
        let handles: Vec<_> = (0..10).map(|_| spawn(|| build_contract(None).unwrap())).collect();

        handles.into_iter().for_each(|handle| handle.join().unwrap());

        Ok(())
    }
}
