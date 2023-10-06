use core::sync::atomic::Ordering;
use std::{
    process::{Command, Stdio},
    sync::atomic::AtomicBool,
};

use anyhow::bail;

static CONTRACT_READY: AtomicBool = AtomicBool::new(false);

/// Compile contract in release mode and prepare it for integration tests usage
pub fn build_contract() -> anyhow::Result<()> {
    if CONTRACT_READY.load(Ordering::Relaxed) {
        return Ok(());
    }

    // Assuming that the Makefile is in root repository directory
    let output = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?;
    assert!(output.status.success(), "Failed to get Git repository root path");
    let git_root = String::from_utf8_lossy(&output.stdout)
        .trim_end_matches('\n')
        .to_string();

    let output = Command::new("make")
        .arg("build")
        .current_dir(git_root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("Failed to build contract");
    }

    CONTRACT_READY.store(true, Ordering::Relaxed);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::build::build_contract;

    #[test]
    fn test_build_contract() -> anyhow::Result<()> {
        build_contract()?;
        build_contract()?;
        build_contract()
    }
}
