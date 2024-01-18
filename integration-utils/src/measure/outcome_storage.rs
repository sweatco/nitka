use std::{
    collections::BTreeMap,
    future::Future,
    mem::transmute,
    sync::{Mutex, MutexGuard},
};

use near_workspaces::{
    result::{ExecutionOutcome, ExecutionSuccess},
    types::Gas,
    Account,
};

type Map = BTreeMap<String, ExecutionSuccess>;

static STORAGE: OutcomeStorage = OutcomeStorage {
    measuring: Mutex::new(vec![]),
    data: Mutex::new(Map::new()),
};

pub struct OutcomeStorage {
    measuring: Mutex<Vec<String>>,
    data: Mutex<Map>,
}

impl OutcomeStorage {
    fn get_measuring() -> MutexGuard<'static, Vec<String>> {
        STORAGE.measuring.lock().expect("STORAGE.measuring.lock()")
    }

    fn get_data() -> MutexGuard<'static, Map> {
        STORAGE.data.lock().expect("STORAGE.data.lock()")
    }

    pub fn start_measuring(account: &Account) {
        let mut measuring = Self::get_measuring();
        assert!(!measuring.iter().any(|a| a == account.id().as_str()));
        measuring.push(account.id().to_string());
    }

    fn stop_measuring(account: &Account) {
        let mut measuring = Self::get_measuring();
        measuring.retain_mut(|a| a != account.id().as_str());
    }

    /// Execute command and measure total gas price
    pub async fn measure_total<Output>(
        account: &Account,
        future: impl Future<Output = anyhow::Result<Output>>,
    ) -> anyhow::Result<(Gas, Output)> {
        Self::start_measuring(account);
        let output = future.await?;
        Self::stop_measuring(account);

        Ok((OutcomeStorage::get_total_gas(account), output))
    }

    /// Execute command and measure one of its operations gas price
    pub async fn measure_operation<Output>(
        label: &str,
        account: &Account,
        future: impl Future<Output = anyhow::Result<Output>>,
    ) -> anyhow::Result<(Gas, Output)> {
        Self::start_measuring(account);
        let output = future.await?;
        Self::stop_measuring(account);

        let result = OutcomeStorage::get_labeled_result(account, label);

        Ok((result.gas_burnt, output))
    }
}

impl OutcomeStorage {
    /// Store successful execution result
    pub fn add_result(result: &ExecutionSuccess) {
        let execution = result.outcome().executor_id.clone();

        if !Self::get_measuring().contains(&execution.to_string()) {
            return;
        }

        let existing = Self::get_data().insert(execution.to_string(), clone_execution_success(result));
        assert!(existing.is_none());
    }

    fn get_total_gas(account: &Account) -> Gas {
        let data = Self::get_data();

        let Some(success) = data.get(account.id().as_str()) else {
            let accs: Vec<_> = data.keys().into_iter().collect();

            panic!("Failed to get account data for: {account:?}. Existing accounts: {accs:?}");
        };

        success.total_gas_burnt
    }

    /// Get execution result for given manager account
    fn get_labeled_result(account: &Account, label: &str) -> ExecutionOutcome {
        Self::get_data()
            .get(account.id().as_str())
            .expect("get(account.id().as_str())")
            .outcomes()
            .into_iter()
            .find(|outcome| outcome.logs.iter().any(|log| log.contains(label)))
            .expect("find(|outcome| outcome.logs.iter().any(|log| log.contains(label)))")
            .clone()
    }
}

/// A crutch to clone unclonable `Value`
/// Remove this after: https://github.com/near/near-workspaces-rs/pull/345 landed
fn clone_execution_success(success: &ExecutionSuccess) -> ExecutionSuccess {
    #[derive(Clone)]
    struct ExecutionDetails {
        _transaction: ExecutionOutcome,
        _receipts: Vec<ExecutionOutcome>,
    }

    #[derive(Clone)]
    struct ClonableExecutionSuccess {
        _total_gas_burnt: Gas,
        _value: String,
        _details: ExecutionDetails,
    }

    let clonable: &ClonableExecutionSuccess = unsafe { transmute(success) };

    let clone = clonable.clone();

    unsafe { transmute(clone) }
}
