use integration_trait::make_integration_version;
use integration_utils::contract_call::ContractCall;
use near_sdk::{json_types::U128, PromiseOrValue};

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

impl ContractNameInterface for () {
    fn init() -> Self {}

    fn initialize_with_name(_name: String) -> Self {}

    fn receive_name(&self) -> String {
        String::default()
    }

    fn set_name(&mut self, _name: String) {}

    fn burn(&mut self) -> PromiseOrValue<U128> {
        todo!()
    }

    fn update_contract(&mut self) {
        todo!()
    }
}

impl ContractNameInterfaceIntegration for () {
    fn init(&self) -> ContractCall<()> {
        todo!()
    }

    fn initialize_with_name(&self, _name: String) -> ContractCall<()> {
        todo!()
    }

    fn receive_name(&self) -> ContractCall<String> {
        todo!()
    }

    fn set_name(&mut self, _name: String) -> ContractCall<()> {
        todo!()
    }

    fn burn(&mut self) -> ContractCall<U128> {
        todo!()
    }

    fn update_contract(&mut self, _code: Vec<u8>) -> ContractCall<()> {
        todo!()
    }
}
