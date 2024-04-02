use std::{env, fs};

pub trait ToNear {
    fn to_near(&self) -> near_sdk::AccountId;
}

impl ToNear for near_workspaces::Account {
    fn to_near(&self) -> crate::AccountId {
        self.id().to_string().try_into().unwrap()
    }
}

pub fn load_wasm(wasm_path: &str) -> Vec<u8> {
    let current_dir = env::current_dir().expect("Failed to get current dir");
    let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path))
        .unwrap_or_else(|_| panic!("Failed to get wasm file path for: {wasm_path} current_dir: {current_dir:?}"));

    fs::read(&wasm_filepath).unwrap_or_else(|_| panic!("Failed to load wasm from: {wasm_filepath:?}"))
}
