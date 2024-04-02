use std::collections::HashMap;

use anyhow::Result;
use near_workspaces::{
    network::{Sandbox, Testnet},
    prelude::TopLevelAccountCreator,
    types::NearToken,
    Account, Contract, DevNetwork, Worker,
};

use crate::{build::build_contract, misc::load_wasm};

const ONE_MINUTE_BLOCKS_HEIGHT: u64 = 240;

#[derive(Debug)]
pub struct Context<T> {
    root_account: Account,
    pub worker: Worker<T>,
    pub accounts: HashMap<String, Account>,
    pub contracts: HashMap<&'static str, Contract>,
}

impl Context<Sandbox> {
    pub async fn new(contracts: &[&'static str], rebuild_contract: bool, make_command: Option<&str>) -> Result<Self> {
        let worker = near_workspaces::sandbox().await?;
        let root_account = worker.root_account()?;
        Self::with_worker(contracts, root_account, worker, rebuild_contract, make_command).await
    }
}

impl Context<Testnet> {
    pub async fn new(contracts: &[&'static str], rebuild_contract: bool, make_command: Option<&str>) -> Result<Self> {
        let worker = near_workspaces::testnet().await?;
        let root_account = worker.dev_create_account().await?;
        Self::with_worker(contracts, root_account, worker, rebuild_contract, make_command).await
    }
}

impl<Network: DevNetwork + TopLevelAccountCreator + 'static> Context<Network> {
    async fn with_worker(
        contract_names: &[&'static str],
        root_account: Account,
        worker: Worker<Network>,
        rebuild_contract: bool,
        make_command: Option<&str>,
    ) -> Result<Self> {
        println!("🏭 Initializing context");

        if rebuild_contract {
            build_contract(make_command)?;
        }

        let mut context = Context {
            root_account,
            worker: worker.clone(),
            accounts: HashMap::new(),
            contracts: HashMap::new(),
        };

        let mut contracts = HashMap::<&'static str, Contract>::new();

        for name in contract_names {
            let account = context.account(name).await?;

            let contract = account
                .deploy(&load_wasm(&format!("../res/{name}.wasm")))
                .await?
                .into_result()?;

            println!("📃 сontract {name} deployed to {}", contract.id());

            contracts.insert(name, contract);
        }

        context.contracts = contracts;

        Ok(context)
    }

    pub async fn account(&mut self, name: &str) -> Result<Account> {
        self.account_with_balance(name, NearToken::from_near(5_000)).await
    }

    pub async fn account_with_balance(&mut self, name: &str, balance: NearToken) -> Result<Account> {
        if !self.accounts.contains_key(name) {
            let account = self
                .root_account
                .create_subaccount(name)
                .initial_balance(balance)
                .transact()
                .await?
                .into_result()?;

            self.accounts.insert(name.to_string(), account);
        }

        Ok(self.accounts.get(name).unwrap().clone())
    }
}

impl Context<Sandbox> {
    pub async fn fast_forward_hours(&self, hours: u64) -> Result<()> {
        self.fast_forward_minutes(hours * 60).await
    }

    pub async fn fast_forward_minutes(&self, minutes: u64) -> Result<()> {
        let blocks_to_advance = ONE_MINUTE_BLOCKS_HEIGHT * minutes;
        println!("⏳ Fast forward to {minutes} minutes ({blocks_to_advance} blocks)...");
        self.worker.fast_forward(blocks_to_advance).await?;
        Ok(())
    }

    pub async fn fast_forward_one_block(&self) -> Result<()> {
        self.worker.fast_forward(1).await?;
        Ok(())
    }
}
