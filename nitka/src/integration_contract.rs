use near_workspaces::Contract;

use crate::contract_call::ContractCall;

pub fn make_call<T>(contract: &Contract, method: &str) -> ContractCall<T> {
    ContractCall::new(method, contract.clone())
}
