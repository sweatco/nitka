use async_trait::async_trait;
use near_workspaces::{Account, Contract};
use crate::contract_call::ContractCall;


#[async_trait]
pub trait IntegrationContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self;
    fn with_user(&mut self, account: &Account) -> &mut Self;
    fn user_account(&self) -> Option<Account>;
    fn contract(&self) -> &'a Contract;

    fn make_call<T>(&self, method: &str) -> ContractCall<T> {
        ContractCall::new(method, self.contract().clone())
    }
}
