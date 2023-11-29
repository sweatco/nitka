#![cfg(test)]

use model::api::ContractApiIntegration;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
#[mutants::skip]
async fn happy_flow() -> anyhow::Result<()> {
    println!("👷🏽 Run happy flow test");

    let context = prepare_contract().await?;

    assert_eq!(555, context.my_contract().test().await?);

    dbg!(context.my_contract().data().await?);

    Ok(())
}
