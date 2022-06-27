use crate::{account::AccountInfo, lockup::LockupInfo, Account, Contract, State};
use near_sdk::{env, json_types::U128, AccountId};
use crate::nft::Nft;
use near_sdk::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Setting {
    pub constant_fee: u128,
    pub percent_fee: u8,
    // Commission in percents over transferring amount. for example, 10
    // (like 10%)
    // Owner of the contract. Example, `Realis.near` or `Volvo.near`
    pub owner_id: AccountId,
    // Allowed user from backend, with admin permission.
    pub backend_ids: Vec<AccountId>,
    // Fee collector.
    pub beneficiary_id: AccountId,
    // State of contract.
    pub state: State,
}

impl From<&Contract> for Setting {
    fn from(contract: &Contract) -> Self {
        Self {
            constant_fee: contract.constant_fee,
            percent_fee: contract.percent_fee,
            owner_id: contract.owner_id.clone(),
            backend_ids: contract.backend_ids.to_vec(),
            beneficiary_id: contract.beneficiary_id.clone(),
            state: contract.state.clone(),
        }
    }
}

impl Contract {

    pub fn lockups_info(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> Vec<LockupInfo> {
        match self.accounts.get(&account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                user_account.get_lockups(from_index, limit)
            }
            None => {
                vec![]
            }
        }
    }

    pub fn get_balance_info(&self, account_id: AccountId) -> U128 {
        match self.accounts.get(&account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                U128(user_account.free)
            }
            None => U128(0u128),
        }
    }

    pub fn get_account_info(&self, account_id: &AccountId) -> AccountInfo {
        let res: Account = self
            .accounts
            .get(account_id)
            .unwrap_or_else(|| env::panic_str("Account not found."))
            .into();

        res.into()
    }

    // Return NFT price
    pub fn get_nft_marketplace_info(&self, nft_id: U128) -> U128 {
        self.internal_get_nft_marketplace_info(nft_id.0).into()
    }

    pub fn get_nft_info(&self, nft_id: U128) -> Nft {
        self.nfts.get_nft(&nft_id.0).into()
    }

    pub fn get_nft_price(&self, nft_id: U128) -> U128 {
        self.internal_get_nft_marketplace_info(nft_id.0).into()
    }

    pub fn get_contract_settings(&self) -> Setting {
        self.into()
    }
}
