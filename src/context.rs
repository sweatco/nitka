use std::{collections::HashMap, env, fs};

use near_workspaces::{
    network::{Sandbox, Testnet},
    prelude::TopLevelAccountCreator,
    types::NearToken,
    Account, Contract, DevNetwork, Worker,
};

use crate::build::build_contract;

const ONE_MINUTE_BLOCKS_HEIGHT: u64 = 240;

pub struct Context<T> {
    worker: Worker<T>,
    root_account: Account,
    pub accounts: HashMap<String, Account>,
    pub contracts: HashMap<&'static str, Contract>,
}

impl Context<Sandbox> {
    pub async fn new(contracts: &[&'static str]) -> anyhow::Result<Self> {
        Self::with_worker(contracts, near_workspaces::sandbox().await?).await
    }
}

impl Context<Testnet> {
    pub async fn new(contracts: &[&'static str]) -> anyhow::Result<Self> {
        Self::with_worker(contracts, near_workspaces::testnet().await?).await
    }
}

impl<T: DevNetwork + TopLevelAccountCreator + 'static> Context<T> {
    async fn with_worker(contract_names: &[&'static str], worker: Worker<T>) -> anyhow::Result<Self> {
        println!("🏭 Initializing context");

        build_contract()?;

        let root_account = worker.dev_create_account().await?;

        let mut contracts = HashMap::<&'static str, Contract>::new();

        for name in contract_names {
            let contract = worker
                .dev_deploy(&Self::load_wasm(&format!("../res/{name}.wasm")))
                .await?;

            println!("@@ contract {} deployed to {}", name, contract.id());

            contracts.insert(name, contract);
        }

        Ok(Context {
            worker,
            root_account,
            accounts: HashMap::new(),
            contracts,
        })
    }

    pub async fn account(&mut self, name: &str) -> anyhow::Result<Account> {
        if !self.accounts.contains_key(name) {
            let account = self
                .root_account
                .create_subaccount(name)
                .initial_balance(NearToken::from_near(3))
                .transact()
                .await?
                .into_result()?;

            self.accounts.insert(name.to_string(), account);
        }

        Ok(self.accounts.get(name).unwrap().clone())
    }

    fn load_wasm(wasm_path: &str) -> Vec<u8> {
        let current_dir = env::current_dir().expect("Failed to get current dir");
        let wasm_filepath = fs::canonicalize(current_dir.join(wasm_path)).expect("Failed to get wasm file path");
        fs::read(wasm_filepath).expect("Failed to load wasm")
    }
}

impl Context<Sandbox> {
    pub async fn fast_forward_hours(&self, hours: u64) -> anyhow::Result<()> {
        self.fast_forward_minutes(hours * 60).await
    }

    pub async fn fast_forward_minutes(&self, minutes: u64) -> anyhow::Result<()> {
        let blocks_to_advance = ONE_MINUTE_BLOCKS_HEIGHT * minutes;
        println!("⏳ Fast forward to {minutes} minutes ({blocks_to_advance} blocks)...");
        self.worker.fast_forward(blocks_to_advance).await?;
        Ok(())
    }
}
