use crate::*;
use near_sdk::{env, require, AccountId, PublicKey};

/// Converts `PublicKey` to `AccountId`
pub fn convert_pk_to_account_id(pk: PublicKey) -> AccountId {
    hex::encode(&pk.as_bytes()[1..])
        .try_into()
        .unwrap_or_else(|_| env::panic_str("Fail to convert PublicKey to AccountId"))
}

impl Contract {
    pub fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id.clone(),
            "Only owner can mint nft"
        );
    }
}

pub fn assert_backend() {
    todo!()
}
