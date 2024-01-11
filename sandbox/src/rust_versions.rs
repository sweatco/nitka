#![cfg(test)]

use anyhow::Result;
use near_workspaces::Account;

use crate::misc::build_with_version;

#[tokio::test]
async fn rust_versions() -> Result<()> {
    let _1_69 = build_with_version("1.69")?;
    let _1_75 = build_with_version("1.75")?;

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
