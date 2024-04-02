use near_sdk::{json_types::U128, PromiseOrValue};
use near_workspaces::Contract;
use nitka_proc::make_integration_version;

pub struct IntegrationTraitTestContract<'a> {
    pub contract: &'a Contract,
}

#[make_integration_version]
pub trait ContractNameInterface {
    fn init() -> Self;
    fn initialize_with_name(name: String) -> Self;

    #[deposit_one_yocto]
    fn receive_name(&self) -> String;

    #[deposit_yocto = method_for_deposit()]
    fn set_name(&mut self, name: String);

    /// Initialize multisig contract.
    /// @params num_confirmations: k of n signatures required to perform operations.
    fn burn(&mut self) -> PromiseOrValue<U128>;
}
