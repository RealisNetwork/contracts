use crate::*;
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    pub free: U128,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        Self {
            free: account.free.into(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_account_info(&self, public_key: PublicKey) -> AccountInfo {
        self.accounts.get(&public_key).unwrap_or_default().into()
    }
}
