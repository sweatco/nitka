use std::{
    fs::copy,
    process::{Command, Stdio},
};

use anyhow::{bail, Result};
use nitka::{build::git_root, misc::load_wasm};

fn run_command<const N: usize, const I: usize>(args: [&str; N], env: [(&str, &str); I]) -> Result<()> {
    let mut command = Command::new(args.first().unwrap());

    for env in env {
        command.env(env.0, env.1);
    }

    for arg in args.iter().skip(1) {
        command.arg(arg);
    }

    let output = command
        .current_dir(git_root()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("Failed to run command: {args:?}. Output: {output:?}");
    }

    Ok(())
}

pub fn build_with_version(version: &str) -> Result<Vec<u8>> {
    run_command(
        [
            "rustup",
            "toolchain",
            "install",
            &format!("{version}-aarch64-apple-darwin"),
        ],
        [],
    )?;

    run_command(["rustup", "target", "add", "wasm32-unknown-unknown"], [])?;

    run_command(
        [
            "cargo",
            &format!("+{version}-aarch64-apple-darwin"),
            "build",
            "-p",
            "helper-contract",
            "--target",
            "wasm32-unknown-unknown",
            "--profile=contract",
        ],
        [("CARGO_TARGET_DIR", &format!("./temp/{version}"))],
    )?;

    let contract_path = format!(
        "{}/temp/{version}/wasm32-unknown-unknown/contract/helper_contract.wasm",
        git_root()?
    );

    let final_contract_path = format!("{}/temp/helper_contract_{version}.wasm", git_root()?);

    copy(contract_path, &final_contract_path)?;

    Ok(load_wasm(&final_contract_path))
}
