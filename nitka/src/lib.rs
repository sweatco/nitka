pub use nitka_proc::make_integration_version;

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        pub mod build;
        pub mod context;
        mod contract_call;
        pub mod integration_contract;
        pub mod measure;
        pub mod misc;
        pub mod panic_finder;
        pub mod parse_result;
        mod tests;
        mod unwrap;

        pub use contract_call::{set_integration_full_output, set_integration_logs_enabled, ContractCall};
        pub use near_sdk;
        pub use serde_json::json;
        pub use unwrap::Unwrap;
    }
}
