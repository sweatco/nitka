use std::mem::{size_of, MaybeUninit};

use anyhow::Result;
use async_trait::async_trait;
use near_sdk::serde::{de::DeserializeOwned, Serialize};
use workspaces::{operations::CallTransaction, Account, Contract};

#[async_trait]
pub trait IntegrationContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self;
    fn with_user(self, account: &Account) -> Self;
    fn user_account(&self) -> Account;
    fn contract(&self) -> &'a Contract;

    async fn call_contract<T: DeserializeOwned, P: Serialize + Send>(&self, method: &str, args: P) -> Result<T> {
        println!("▶️ {method}");
        invoke_transaction(self.contract().call(method), args).await
    }

    async fn call_user<T: DeserializeOwned, P: Serialize + Send>(&self, method: &str, args: P) -> Result<T> {
        println!("▶️ {method}");
        invoke_transaction(self.user_account().call(self.contract().id(), method), args).await
    }
}

async fn invoke_transaction<T: DeserializeOwned, P: Serialize + Send>(tx: CallTransaction<'_>, args: P) -> Result<T> {
    let result = tx.args_json(args).max_gas().transact().await?.into_result()?;

    println!("Result: {:?}", result);

    if size_of::<T>() == 0 {
        // For cases when return type is `()` and we don't need to parse result.
        // This call is safe for zero sized types.
        #[allow(clippy::uninit_assumed_init)]
        Ok(unsafe { MaybeUninit::uninit().assume_init() })
    } else {
        Ok(result.json()?)
    }
}
