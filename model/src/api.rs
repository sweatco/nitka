#[cfg(feature = "integration-test")]
pub const MY_CONTRACT: &str = "my_contract";

#[cfg(feature = "integration-test")]
pub struct MyContract<'a> {
    pub(crate) contract: &'a near_workspaces::Contract,
}

#[cfg(feature = "integration-test")]

impl<'a> integration_utils::integration_contract::IntegrationContract<'a> for MyContract<'a> {
    fn with_contract(contract: &'a near_workspaces::Contract) -> Self {
        Self { contract }
    }

    fn contract(&self) -> &'a near_workspaces::Contract {
        self.contract
    }
}

#[integration_trait::make_integration_version]
pub trait ContractApi {
    fn new() -> Self;
    fn test(&mut self) -> u32;
    fn data(&mut self) -> Vec<String>;
    fn set_data(&mut self, data: Vec<String>);
    fn log_and_panic(&mut self);
}
