use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};

pub enum VAccount {
    V1(Account),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    // pub free: Balance
    // pub lockups: Vec<Lockup>
    // pub nfts: Vec<NftId>
}

impl From<Account> for VAccount {
    fn from(account: Account) -> Self {
        VAccount::V1(account)
    }
}

impl Default for Account {
    fn default() -> Self {
        todo!()
    }
}
