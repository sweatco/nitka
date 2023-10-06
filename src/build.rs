use core::sync::atomic::Ordering;
use std::{
    process::{Command, Stdio},
    sync::atomic::AtomicBool,
};

static CONTRACT_READY: AtomicBool = AtomicBool::new(false);

/// Compile contract in release mode and prepare it for integration tests usage
pub fn build_contract() -> anyhow::Result<()> {
    if CONTRACT_READY.load(Ordering::Relaxed) {
        return Ok(());
    }

    Command::new("make")
        .arg("build")
        // .current_dir("..")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

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
