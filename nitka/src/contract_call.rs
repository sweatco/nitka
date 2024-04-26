use std::{
    future::{Future, IntoFuture},
    marker::PhantomData,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
};

use anyhow::{bail, Result};
use near_workspaces::{
    operations::CallTransaction,
    result::{ExecutionFailure, ExecutionSuccess},
    types::{Gas, NearToken},
    Account, Contract,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_vec;

use crate::measure::utils::pretty_gas_string;

static LOGS_ENABLED: AtomicBool = AtomicBool::new(true);
static FULL_OUTPUT_ENABLED: AtomicBool = AtomicBool::new(false);

use crate::parse_result::ParseResult;

pub struct ContractCall<T> {
    method: String,
    user_account: Option<Account>,
    args: Vec<u8>,
    gas: Option<Gas>,
    deposit: NearToken,
    contract: Contract,
    _p: PhantomData<T>,
}

impl<T> ContractCall<T> {
    pub fn new(method: &str, contract: Contract) -> Self {
        Self {
            method: method.to_string(),
            user_account: None,
            args: vec![],
            gas: None,
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

    pub fn gas(mut self, gas: Gas) -> Self {
        self.gas = gas.into();
        self
    }
}

impl<T: Send + DeserializeOwned> ContractCall<T> {
    fn prepare_transaction(&self) -> CallTransaction {
        let method = self.method.clone();

        let tx = if let Some(user_account) = self.user_account.clone() {
            println!("Calling with account: {user_account:?}");
            user_account.call(self.contract.id(), &method)
        } else {
            self.contract.call(&method)
        };

        let tx = tx.args(self.args.clone());

        let tx = if let Some(gas) = self.gas {
            tx.gas(gas)
        } else {
            tx.max_gas()
        };

        tx.deposit(self.deposit)
    }

    async fn call_transaction(&self) -> Result<ExecutionSuccess, ExecutionFailure> {
        println!("▶️ {}", self.method);
        let transaction = self.prepare_transaction();
        let result = transaction.transact().await.unwrap().into_result();
        log_result(result)
    }

    async fn call(self) -> Result<T> {
        self.call_transaction().await.parse()
    }

    pub async fn result(self) -> Result<ExecutionSuccess, ExecutionFailure> {
        self.call_transaction().await
    }

    pub async fn expect_error(self, error_message: &str) -> Result<()> {
        let Err(err) = self.call().await else {
            bail!("This call should fail");
        };

        if err.to_string().contains(error_message) {
            return Ok(());
        };

        bail!("Expected error: {error_message}.\nGot:\n{err}")
    }

    pub async fn expect_log(self, log: &str) -> Result<()> {
        let result = self.result().await?;

        if result.logs().contains(&log) {
            return Ok(());
        }

        bail!("Expected log: '{log}' not found.\nLogs: {:#?}", result.logs())
    }

    pub async fn dont_expect_log(self, log: &str) -> Result<()> {
        let result = self.result().await?;

        if result.logs().contains(&log) {
            bail!("Log '{log}' is not expected")
        }

        Ok(())
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

pub fn set_integration_full_output(enabled: bool) {
    FULL_OUTPUT_ENABLED.store(enabled, Ordering::Relaxed);
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

            print_gas(result.total_gas_burnt);
        }
        Err(ref error) => {
            if LOGS_ENABLED.load(Ordering::Relaxed) {
                for log in error.logs() {
                    println!("  📖 {log}");
                }
            }

            print_gas(error.total_gas_burnt);
        }
    }

    if FULL_OUTPUT_ENABLED.load(Ordering::Relaxed) {
        println!("  🛠️ result: \n{result:#?}");
    }

    result
}

fn print_gas(gas: Gas) {
    println!("  ⛽ {}", pretty_gas_string(gas));
}
