use anyhow::Result;
use async_trait::async_trait;
use integration_utils::integration_contract::IntegrationContract;
use model::api::ContractApiIntegration;
use near_workspaces::{Account, Contract};

pub const MY_CONTRACT: &str = "my_contract";

pub struct MyContract<'a> {
    contract: &'a Contract,
    account: Option<Account>,
}

#[async_trait]
impl ContractApiIntegration for MyContract<'_> {
    async fn new(&self) -> Result<()> {
        self.call("new", ()).await
    }

    async fn test(&mut self) -> Result<u32> {
        self.call("test", ()).await
    }

    async fn data(&mut self) -> Result<Vec<String>> {
        self.call("data", ()).await
    }

    async fn log_and_panic(&mut self) -> Result<()> {
        self.call("log_and_panic", ()).await
    }
}

impl<'a> IntegrationContract<'a> for MyContract<'a> {
    fn with_contract(contract: &'a Contract) -> Self {
        Self {
            contract,
            account: None,
        }
    }

    fn with_user(&mut self, account: &Account) -> &mut Self {
        self.account = account.clone().into();
        self
    }

    fn user_account(&self) -> Option<Account> {
        self.account.clone()
    }

    fn contract(&self) -> &'a Contract {
        self.contract
    }
}
