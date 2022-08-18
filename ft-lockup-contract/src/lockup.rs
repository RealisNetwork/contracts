use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, near_bindgen, require, AccountId, Balance, Promise, PromiseResult,
    Timestamp, ONE_YOCTO,
};

use crate::{Contract, ContractExt, LockupIndex, GAS_FOR_CLAIM_CALLBACK, GAS_FOR_FT_TRANSFER};

#[derive(BorshDeserialize, BorshSerialize, Eq, PartialEq, PartialOrd, Hash)]
pub struct Lockup {
    pub amount: Balance,
    pub unlock_on: Timestamp,
    pub is_claimed: bool,
}

impl Lockup {
    pub fn claim(
        &mut self,
        token_id: AccountId,
        account_id: AccountId,
        index: LockupIndex,
    ) -> Promise {
        require!(!self.is_claimed, "Lockup already claimed");
        require!(
            self.unlock_on <= near_sdk::env::block_timestamp(),
            "Lockup isn't expired"
        );
        self.is_claimed = true;

        ext_ft_core::ext(token_id)
            .with_static_gas(env::prepaid_gas() - GAS_FOR_FT_TRANSFER)
            .with_attached_deposit(ONE_YOCTO)
            .ft_transfer(account_id.clone(), self.amount.into(), None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_CLAIM_CALLBACK)
                    .lockup_claim_callback(index, account_id),
            )
    }
}

#[ext_contract(ext_self)]
trait SelfCallback {
    fn lockup_claim_callback(&mut self, index: LockupIndex, account_id: AccountId);
}

#[near_bindgen]
impl SelfCallback for Contract {
    #[private]
    fn lockup_claim_callback(&mut self, index: LockupIndex, account_id: AccountId) {
        match env::promise_result(0) {
            PromiseResult::NotReady => env::abort(),
            PromiseResult::Successful(_) => {
                self.lockups.remove(&index);
                let mut account_lockups = self
                    .account_lockups
                    .get(&account_id)
                    .unwrap_or_else(|| env::panic_str("No lockups found"));
                account_lockups.remove(&index);
                self.account_lockups.insert(&account_id, &account_lockups);
            }
            PromiseResult::Failed => {
                let mut lockup = self
                    .lockups
                    .get(&index)
                    .unwrap_or_else(|| env::panic_str("No such lockup for this account"));
                lockup.is_claimed = false;
                self.lockups.insert(&index, &lockup);
            }
        }
    }
}
