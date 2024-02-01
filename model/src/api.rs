#[cfg(feature = "integration-test")]
pub struct MyContract<'a> {
    pub contract: &'a near_workspaces::Contract,
}

#[integration_trait::make_integration_version]
pub trait ContractApi {
    fn new() -> Self;
    fn test(&mut self) -> u32;
    fn data(&mut self) -> Vec<String>;
    fn set_data(&mut self, data: Vec<String>);
    fn log_and_panic(&mut self);
}
