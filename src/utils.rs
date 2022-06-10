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
            "Only owner can do this"
        );
    }

    pub fn assert_backend(&self) {
        require!(env::signer_account_id() == self.backend_id, "Not allowed");
    }

    pub fn assert_running(&self) {
        require!(self.state == State::Running, "Contract is paused");
    }
}

#[allow(dead_code)]
pub fn assert_backend() {
    todo!()
}
