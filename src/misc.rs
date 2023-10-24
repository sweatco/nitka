pub trait ToNear {
    fn to_near(&self) -> near_sdk::AccountId;
}

impl ToNear for workspaces::Account {
    fn to_near(&self) -> near_sdk::AccountId {
        near_sdk::AccountId::new_unchecked(self.id().to_string())
    }
}
