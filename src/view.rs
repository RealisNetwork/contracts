use crate::{account::AccountInfo, lockup::LockupInfo, nft::Nft, *};
use near_sdk::{env, json_types::U128, serde::Serialize, AccountId};

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Setting {
    pub constant_fee: U128,
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
            constant_fee: U128(contract.constant_fee),
            percent_fee: contract.percent_fee,
            owner_id: contract.owner_id.clone(),
            backend_ids: contract.backend_ids.to_vec(),
            beneficiary_id: contract.beneficiary_id.clone(),
            state: contract.state.clone(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn lockups_info(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> Vec<LockupInfo> {
        self.accounts
            .get(&account_id)
            .map(|user| {
                let user_account: Account = user.into();
                user_account.get_lockups(from_index, limit)
            })
            .unwrap_or_default()
    }

    pub fn get_balance_info(&self, account_id: AccountId) -> U128 {
        self.accounts
            .get(&account_id)
            .map(|user| {
                let user_account: Account = user.into();
                U128(user_account.free)
            })
            .unwrap_or(U128(0u128))
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

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    fn info_get_balance_test() {
        // Indexes are default
        let (mut contract, _context) = init_test_env(None, None, None);
        let account: Account = Account::new(accounts(0), 250 * ONE_LIS);
        let account_id = accounts(0);

        contract.accounts.insert(&account_id, &account.into());
        assert_eq!(contract.get_balance_info(account_id).0, 250 * ONE_LIS);
    }
}
