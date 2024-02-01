use integration_trait::make_integration_version;
use near_sdk::{json_types::U128, PromiseOrValue};
use near_workspaces::Contract;

pub struct IntegrationTraitTestContract<'a> {
    pub contract: &'a Contract,
}

#[make_integration_version]
pub trait ContractNameInterface {
    fn init() -> Self;
    fn initialize_with_name(name: String) -> Self;

    fn receive_name(&self) -> String;
    fn set_name(&mut self, name: String);

    /// Initialize multisig contract.
    /// @params num_confirmations: k of n signatures required to perform operations.
    fn burn(&mut self) -> PromiseOrValue<U128>;

    #[update]
    fn update_contract(&mut self);
}
