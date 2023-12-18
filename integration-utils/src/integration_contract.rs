use async_trait::async_trait;
use near_workspaces::Contract;

use crate::contract_call::ContractCall;

#[async_trait]
pub trait IntegrationContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self;
    fn contract(&self) -> &'a Contract;

    fn make_call<T>(&self, method: &str) -> ContractCall<T> {
        ContractCall::new(method, self.contract().clone())
    }
}
