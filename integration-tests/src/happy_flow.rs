#![cfg(test)]

use helper_model::api::HelperApiIntegration;
use integration_utils::{panic_finder::PanicFinder, parse_result::ParseResult};
use integration_utils::contract_call::set_integration_full_output;
use my_model::api::ContractApiIntegration;

use crate::context::{prepare_contract, IntegrationContext};

#[tokio::test]
#[mutants::skip]
async fn happy_flow() -> anyhow::Result<()> {
    set_integration_full_output(true);

    println!("👷🏽 Run happy flow test");

    let context = prepare_contract().await?;

    assert_eq!(555, context.my_contract().test().await?);

    let result = context.my_contract().test().result().await;

    assert!(matches!(result, Result::Ok(_)));

    let value: u32 = result.parse()?;

    assert_eq!(555, value);

    let data = context.my_contract().data().await?;

    assert_eq!(vec!["a".to_string()], data);

    dbg!(context.helper().block_timestamp_ms("Hello".to_string()).await?);

    Ok(())
}

#[tokio::test]
#[mutants::skip]
async fn log_after_panic() -> anyhow::Result<()> {
    println!("👷🏽 Run log_after_panic test");

    let context = prepare_contract().await?;

    let result = context.my_contract().log_and_panic().result().await;

    assert!(result.has_panic("Smart contract panicked"));

    let Err(err) = result else { unreachable!() };

    assert!(err.to_string().contains("Smart contract panicked"));

    Ok(())
}
