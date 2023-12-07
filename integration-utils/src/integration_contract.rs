use std::mem::{size_of, MaybeUninit};

use anyhow::Result;
use async_trait::async_trait;
use near_sdk::serde::{de::DeserializeOwned, Serialize};
use near_workspaces::{operations::CallTransaction, Account, Contract};

#[async_trait]
pub trait IntegrationContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self;
    fn with_user(&mut self, account: &Account) -> &mut Self;
    fn user_account(&self) -> Option<Account>;
    fn contract(&self) -> &'a Contract;

    async fn call<T: DeserializeOwned, P: Serialize + Send>(&self, method: &str, args: P) -> Result<T> {
        println!("▶️ {method}");

        if let Some(user_account) = self.user_account() {
            println!("Calling with account: {user_account:?}");
            return invoke_transaction(method, user_account.call(self.contract().id(), method), args).await;
        }

        invoke_transaction(method, self.contract().call(method), args).await
    }
}

async fn invoke_transaction<T: DeserializeOwned, P: Serialize + Send>(
    method: &str,
    tx: CallTransaction,
    args: P,
) -> Result<T> {
    let result = tx.args_json(args).max_gas().transact().await;

    println!("Result: {result:#?}");

    let result = result?.into_result()?;

    let result = if size_of::<T>() == 0 {
        // For cases when return type is `()` and we don't need to parse result.
        // This call is safe for zero sized types.
        #[allow(clippy::uninit_assumed_init)]
        unsafe {
            MaybeUninit::uninit().assume_init()
        }
    } else {
        result.json()?
    };

    println!("✅ {method}: OK");

    Ok(result)
}
