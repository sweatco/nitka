use near_workspaces::Contract;

use crate::ContractCall;

pub fn make_call<T>(contract: &Contract, method: &str) -> ContractCall<T> {
    ContractCall::new(method, contract.clone())
}
