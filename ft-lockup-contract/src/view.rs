use crate::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupView {
    amount: U128,
    unlock_on: U64,
    is_claimed: bool,
}

impl From<Lockup> for LockupView {
    fn from(lockup: Lockup) -> Self {
        Self {
            amount: lockup.amount.into(),
            unlock_on: lockup.unlock_on.into(),
            is_claimed: lockup.is_claimed,
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_token_account_id(&self) -> AccountId {
        self.token_account_id.clone()
    }

    pub fn get_deposit_whitelist(&self) -> Vec<AccountId> {
        self.deposit_whitelist.to_vec()
    }

    pub fn get_num_lockups(&self) -> U64 {
        self.lockups.len().into()
    }

    pub fn get_account_num_lockups(&self, account_id: AccountId) -> u32 {
        self.account_lockups
            .get(&account_id)
            .unwrap_or_default()
            .len()
            .try_into()
            .unwrap()
    }

    pub fn get_lockup(&self, index: LockupIndex) -> Option<LockupView> {
        self.lockups.get(&index).map(|lockup| lockup.into())
    }

    pub fn get_lockups(&self, indexes: Vec<LockupIndex>) -> HashMap<LockupIndex, LockupView> {
        indexes
            .into_iter()
            .filter_map(|index| self.get_lockup(index).map(|lockup| (index, lockup)))
            .collect()
    }

    pub fn get_lockups_paged(
        &self,
        from_index: Option<LockupIndex>,
        limit: Option<LockupIndex>,
    ) -> HashMap<LockupIndex, LockupView> {
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(self.get_num_lockups().0 as u32);
        (from_index..std::cmp::min(self.get_num_lockups().0 as u32, limit))
            .filter_map(|index| self.get_lockup(index).map(|lockup| (index, lockup)))
            .collect()
    }

    pub fn get_account_lockups(
        &self,
        account_id: AccountId,
        from_index: Option<LockupIndex>,
        limit: Option<LockupIndex>,
    ) -> HashMap<LockupIndex, LockupView> {
        self.account_lockups
            .get(&account_id)
            .unwrap_or_default()
            .into_iter()
            .skip(from_index.unwrap_or_default() as usize)
            .take(limit.unwrap_or(LockupIndex::MAX) as usize)
            .map(|lockup_index| {
                (
                    lockup_index,
                    self.lockups.get(&lockup_index).unwrap().into(),
                )
            })
            .collect()
    }
}
