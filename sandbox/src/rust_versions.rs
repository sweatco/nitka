#![cfg(test)]
#![allow(dead_code)]
#![allow(unused_imports)]

use anyhow::Result;
use integration_utils::{build::git_root, misc::load_wasm};
use near_workspaces::Account;

use crate::misc::build_with_version;

async fn acc_from_file(path: &str) -> Result<Account> {
    let worker = near_workspaces::testnet().await?;
    let account = Account::from_file(path, &worker)?;
    Ok(account)
}

async fn acc_1_69() -> Result<Account> {
    acc_from_file("/Users/sweatcoin/.near-credentials/testnet/test_1_69.testnet.json").await
}

async fn acc_1_75() -> Result<Account> {
    acc_from_file("/Users/sweatcoin/.near-credentials/testnet/test_1_75.testnet.json").await
}

#[ignore]
#[tokio::test]
async fn rust_versions() -> Result<()> {
    let _1_69 = build_with_version("1.69")?;
    let _1_75 = build_with_version("1.75")?;

    // let _1_69 = load_wasm(&format!("{}/temp/helper_contract_1.69.wasm", git_root()?));
    // let _1_75 = load_wasm(&format!("{}/temp/helper_contract_1.75.wasm", git_root()?));

    dbg!(_1_69.len());
    dbg!(_1_75.len());

    let worker = near_workspaces::testnet().await?;

    let acc_1_69 = Account::from_file(
        "/Users/sweatcoin/.near-credentials/testnet/test_1_69.testnet.json",
        &worker,
    )?;

    let acc_1_75 = Account::from_file(
        "/Users/sweatcoin/.near-credentials/testnet/test_1_75.testnet.json",
        &worker,
    )?;

    let _contract = acc_1_69.deploy(&_1_69).await?.into_result()?;
    let _contract = acc_1_75.deploy(&_1_75).await?.into_result()?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn test() -> Result<()> {
    let wasm = load_wasm("/Users/sweatcoin/near-sand/sand/target/wasm32-unknown-unknown/contract/sand.wasm");
    let _contract = acc_1_75().await?.deploy(&wasm).await?;
    Ok(())
}
