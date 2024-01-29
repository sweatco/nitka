use std::collections::HashMap;

use near_workspaces::{
    network::{Sandbox, Testnet},
    prelude::TopLevelAccountCreator,
    types::NearToken,
    Account, Contract, DevNetwork, Worker,
};

use crate::{build::build_contract, misc::load_wasm};

const ONE_MINUTE_BLOCKS_HEIGHT: u64 = 240;

pub struct Context<T> {
    root_account: Account,
    pub worker: Worker<T>,
    pub accounts: HashMap<String, Account>,
    pub contracts: HashMap<&'static str, Contract>,
}

impl Context<Sandbox> {
    pub async fn new(
        contracts: &[&'static str],
        rebuild_contract: bool,
        make_command: Option<&str>,
    ) -> anyhow::Result<Self> {
        Self::with_worker(
            contracts,
            near_workspaces::sandbox().await?,
            rebuild_contract,
            make_command,
        )
        .await
    }
}

impl Context<Testnet> {
    pub async fn new(
        contracts: &[&'static str],
        rebuild_contract: bool,
        make_command: Option<&str>,
    ) -> anyhow::Result<Self> {
        Self::with_worker(
            contracts,
            near_workspaces::testnet().await?,
            rebuild_contract,
            make_command,
        )
        .await
    }
}

impl<T: DevNetwork + TopLevelAccountCreator + 'static> Context<T> {
    async fn with_worker(
        contract_names: &[&'static str],
        worker: Worker<T>,
        rebuild_contract: bool,
        make_command: Option<&str>,
    ) -> anyhow::Result<Self> {
        println!("🏭 Initializing context");

        if rebuild_contract {
            build_contract(make_command)?;
        }

        let root_account = worker.dev_create_account().await?;

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

            println!("@@ contract {} deployed to {}", name, contract.id());

            contracts.insert(name, contract);
        }

        context.contracts = contracts;

        Ok(context)
    }

    pub async fn account(&mut self, name: &str) -> anyhow::Result<Account> {
        if !self.accounts.contains_key(name) {
            let account = self
                .root_account
                .create_subaccount(name)
                .initial_balance(NearToken::from_near(10))
                .transact()
                .await?
                .into_result()?;

            self.accounts.insert(name.to_string(), account);
        }

        Ok(self.accounts.get(name).unwrap().clone())
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
