use integration_trait::make_integration_version;
use near_sdk::Timestamp;

#[cfg(feature = "integration-test")]
pub const HELPER_CONTRACT: &str = "helper_contract";

#[cfg(feature = "integration-test")]
pub struct HelperContract<'a> {
    pub(crate) contract: &'a near_workspaces::Contract,
}

#[cfg(feature = "integration-test")]
impl<'a> integration_utils::integration_contract::IntegrationContract<'a> for HelperContract<'a> {
    fn with_contract(contract: &'a near_workspaces::Contract) -> Self {
        Self { contract }
    }

    fn contract(&self) -> &'a near_workspaces::Contract {
        self.contract
    }
}

#[make_integration_version]
pub trait HelperApi {
    fn new() -> Self;
    fn block_timestamp_ms(&self, some_value: String) -> Timestamp;
}
