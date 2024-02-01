use integration_trait::make_integration_version;
use near_sdk::{json_types::U128, PromiseOrValue};

use crate::integration_contract::IntegrationTraitTestContract;

pub mod integration_contract {
    use integration_utils::integration_contract::IntegrationContract;
    use near_workspaces::Contract;

    pub const MY_CONTRACT: &str = "my_contract";

    pub struct IntegrationTraitTestContract<'a> {
        pub(crate) contract: &'a Contract,
    }

    impl<'a> IntegrationContract<'a> for IntegrationTraitTestContract<'a> {
        fn with_contract(contract: &'a Contract) -> Self {
            Self { contract }
        }

        fn contract(&self) -> &'a Contract {
            self.contract
        }
    }
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
