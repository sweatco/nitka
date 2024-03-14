use nitka_proc::make_integration_version;
use near_sdk::Timestamp;

#[cfg(feature = "integration-test")]
pub struct HelperContract<'a> {
    pub contract: &'a near_workspaces::Contract,
}

#[make_integration_version]
pub trait HelperApi {
    fn new() -> Self;
    fn block_timestamp_ms(&self, some_value: String) -> Timestamp;
}
