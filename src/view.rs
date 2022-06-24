use crate::{account::AccountInfo, lockup::LockupInfo, Account, Contract};
use near_sdk::{json_types::U128, AccountId};

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
            .unwrap_or_else(|| Account::new(account_id.clone(), 0).into())
            .into();
        res.into()
    }
}
