use near_sdk::{env, AccountId, PublicKey};

pub fn convert_pk_to_account_id(pk: PublicKey) -> AccountId {
    hex::encode(&pk.as_bytes()[1..])
        .try_into()
        .unwrap_or_else(|_| env::panic_str("Fail to convert PublicKey to AccountId"))
}

#[cfg(test)]
mod tests {
    use crate::common::convert_pk_to_account_id;
    use near_sdk::{AccountId, PublicKey};
    use std::str::FromStr;

    #[test]
    fn convert_from_pk_to_account_id() {
        let pk =
            PublicKey::from_str("ed25519:9wsn2EQSRt24MHanpA158PNhqdf7HAgvKU9mQqXsKJTf").unwrap();
        let account_id = convert_pk_to_account_id(pk);

        assert_eq!(
            account_id.to_string(),
            "84ec459f7e9cb01fac422034c5eedffcda1d0546e1220ff76cb19dfceb05979e"
        )
    }
}
