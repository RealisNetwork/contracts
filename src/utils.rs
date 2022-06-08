use near_sdk::{env, AccountId, PublicKey};

/// Converts `PublicKey` to `AccountId`
pub fn _convert_pk_to_account_id(pk: PublicKey) -> AccountId {
    hex::encode(&pk.as_bytes()[1..])
        .try_into()
        .unwrap_or_else(|_| env::panic_str("Fail to convert PublicKey to AccountId"))
}

pub fn _assert_owner() {
    todo!()
}

pub fn _assert_backend() {
    todo!()
}
