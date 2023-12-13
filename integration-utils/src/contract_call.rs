use std::{
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
};

use anyhow::Result;
use near_sdk::{
    serde::{de::DeserializeOwned, Serialize},
    serde_json::to_vec,
};
use near_workspaces::{
    operations::CallTransaction,
    types::{NearToken},
    Account, Contract,
};

pub struct ContractCall<T> {
    method: String,
    user_account: Option<Account>,
    args: Vec<u8>,
    deposit: NearToken,
    contract: Contract,
    _p: PhantomData<T>,
}

impl<T> ContractCall<T> {
    pub(crate) fn new(method: &str, contract: Contract) -> Self {
        Self {
            method: method.to_string(),
            user_account: None,
            args: vec![],
            deposit: NearToken::default(),
            contract,
            _p: PhantomData,
        }
    }
}

impl<T> ContractCall<T> {
    pub fn args(mut self, args: Vec<u8>) -> Self {
        self.args = args;
        self
    }

    /// Similar to `args`, specify an argument that is JSON serializable and can be
    /// accepted by the equivalent contract. Recommend to use something like
    /// `serde_json::json!` macro to easily serialize the arguments.
    pub fn args_json<U: Serialize>(mut self, args: U) -> Result<Self> {
        self.args = to_vec(&args)?;
        Ok(self)
    }

    /// Specify the amount of tokens to be deposited where `deposit` is the amount of
    /// tokens in yocto near.
    pub fn deposit(mut self, deposit: NearToken) -> Self {
        self.deposit = deposit;
        self
    }
}

impl<T: DeserializeOwned> ContractCall<T> {
    pub async fn call(self) -> Result<T> {
        let method = self.method.clone();

        let transaction = if let Some(user_account) = self.user_account.clone() {
            println!("Calling with account: {user_account:?}");
            user_account.call(self.contract.id(), &method)
        } else {
            self.contract.call(&method)
        };

        let transaction = transaction.args(self.args.clone()).max_gas().deposit(self.deposit);

        invoke_transaction(&self.method, transaction).await
    }
}

async fn invoke_transaction<T: DeserializeOwned>(method: &str, tx: CallTransaction) -> Result<T> {
    let result = tx.transact().await;

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
