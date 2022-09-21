use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    #[allow(unused_variables)]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        env::panic_str("This contract do not accept deposit");
    }

    #[payable]
    #[allow(unused_variables)]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        unimplemented!()
    }

    #[payable]
    #[allow(unused_variables)]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        unimplemented!()
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        unimplemented!()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.ft.storage_balance_of(account_id)
    }
}
