use std::{
    future::{Future, IntoFuture},
    marker::PhantomData,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::Result;
use near_sdk::{
    serde::{de::DeserializeOwned, Serialize},
    serde_json::to_vec,
};
use near_workspaces::{
    operations::CallTransaction,
    result::{ExecutionFailure, ExecutionSuccess},
    types::NearToken,
    Account, Contract,
};

static LOGS_ENABLED: AtomicBool = AtomicBool::new(true);

use crate::{measure::outcome_storage::OutcomeStorage, parse_result::ParseResult};

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
    pub fn with_user(mut self, account: &Account) -> Self {
        self.user_account = account.clone().into();
        self
    }

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

impl<T: Send + DeserializeOwned> ContractCall<T> {
    fn prepare_transaction(&self) -> CallTransaction {
        let method = self.method.clone();

        let transaction = if let Some(user_account) = self.user_account.clone() {
            println!("Calling with account: {user_account:?}");
            user_account.call(self.contract.id(), &method)
        } else {
            self.contract.call(&method)
        };

        transaction.args(self.args.clone()).max_gas().deposit(self.deposit)
    }

    async fn call_transaction(&self) -> Result<ExecutionSuccess, ExecutionFailure> {
        println!("▶️ {}", self.method);
        let transaction = self.prepare_transaction();
        let result = transaction.transact().await.unwrap().into_result();
        if let Ok(success) = &result {
            OutcomeStorage::add_result(success);
        }
        log_result(result)
    }

    async fn call(self) -> Result<T> {
        self.call_transaction().await.parse()
    }

    pub async fn result(self) -> Result<ExecutionSuccess, ExecutionFailure> {
        self.call_transaction().await
    }
}

impl<T: Send + Sync + DeserializeOwned + 'static> IntoFuture for ContractCall<T> {
    type Output = Result<T>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move { self.call().await })
    }
}

pub fn set_integration_logs_enabled(enabled: bool) {
    LOGS_ENABLED.store(enabled, Ordering::Relaxed);
}

#[allow(clippy::result_large_err)]
fn log_result(result: Result<ExecutionSuccess, ExecutionFailure>) -> Result<ExecutionSuccess, ExecutionFailure> {
    match result {
        Ok(ref result) => {
            if LOGS_ENABLED.load(Ordering::Relaxed) {
                for log in result.logs() {
                    println!("  📖 {log}");
                }
            }

            println!("  ⛽ {} TGas burned", result.total_gas_burnt.as_tgas());
        }
        Err(ref error) => {
            if LOGS_ENABLED.load(Ordering::Relaxed) {
                for log in error.logs() {
                    println!("  📖 {log}");
                }
            }

            println!("  ⛽ {} TGas burned", error.total_gas_burnt.as_tgas());
        }
    }

    result
}
