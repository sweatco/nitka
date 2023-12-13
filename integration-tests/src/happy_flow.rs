#![cfg(test)]

use model::api::ContractApiIntegration;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
#[mutants::skip]
async fn happy_flow() -> anyhow::Result<()> {
    println!("👷🏽 Run happy flow test");

    let context = prepare_contract().await?;

    assert_eq!(555, context.my_contract().test().call().await?);

    dbg!(context.my_contract().data().call().await?);

    Ok(())
}

#[tokio::test]
#[mutants::skip]
async fn log_after_panic() -> anyhow::Result<()> {
    println!("👷🏽 Run log_after_panic test");

    let context = prepare_contract().await?;

    let Err(err) = context.my_contract().log_and_panic().call().await else {
        unreachable!()
    };

    assert!(err.to_string().contains("Smart contract panicked"));

    Ok(())
}
