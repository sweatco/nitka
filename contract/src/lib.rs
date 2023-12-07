use std::{collections::HashSet, str::FromStr};

use model::api::ContractApi;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, UnorderedSet, Vector},
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
};

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Lockups,
    AccountLockups,
    DepositWhitelist,
    DraftOperatorsWhitelist,
    Drafts,
    DraftGroups,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub token_account_id: AccountId,
    pub lockups: Vector<AccountId>,
    pub account_lockups: LookupMap<AccountId, HashSet<AccountId>>,
    pub deposit_whitelist: UnorderedSet<AccountId>,
    pub draft_operators_whitelist: UnorderedSet<AccountId>,

    pub next_draft_id: AccountId,
    pub drafts: LookupMap<AccountId, AccountId>,
    pub next_draft_group_id: AccountId,
    pub draft_groups: UnorderedMap<AccountId, AccountId>,

    pub miltisig: Option<AccountId>,

    some_data: Vec<String>,
}

#[near_bindgen]
impl ContractApi for Contract {
    #[init]
    #[private]
    fn new() -> Self {
        Self {
            lockups: Vector::new(StorageKey::Lockups),
            account_lockups: LookupMap::new(StorageKey::AccountLockups),
            token_account_id: AccountId::from_str("sokol").unwrap(),
            deposit_whitelist: UnorderedSet::new(StorageKey::DepositWhitelist),
            draft_operators_whitelist: UnorderedSet::new(StorageKey::DraftOperatorsWhitelist),
            next_draft_id: AccountId::from_str("sokol").unwrap(),
            drafts: LookupMap::new(StorageKey::Drafts),
            next_draft_group_id: AccountId::from_str("sokol").unwrap(),
            draft_groups: UnorderedMap::new(StorageKey::DraftGroups),
            miltisig: None,
            some_data: vec!["a".to_string()],
        }
    }

    fn test(&mut self) -> u32 {
        555
    }

    fn data(&mut self) -> Vec<String> {
        self.some_data.clone()
    }

    fn log_and_panic(&mut self) {
        env::log_str("Hello!");
        env::log_str("Panic!");
        panic!("A")
    }
}
