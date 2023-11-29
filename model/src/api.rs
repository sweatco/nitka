use integration_trait::make_integration_version;

#[make_integration_version]
pub trait ContractApi {
    fn new() -> Self;
    fn test(&mut self) -> u32;
    fn data(&mut self) -> Vec<String>;
}
