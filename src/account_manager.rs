use crate::utils::convert_pk_to_account_id;
use crate::*;
use near_sdk::{ext_contract, near_bindgen, require};
use near_sdk::{Balance, Gas, Promise, PromiseOrValue, PublicKey};

impl Contract {
    pub fn resolve_account(&self, public_key: PublicKey) -> AccountId {
        self.registered_accounts
            .get(&public_key)
            .unwrap_or_else(|| convert_pk_to_account_id(public_key))
    }
}

pub const CREATE_AND_REGISTER_ACCOUNT_CALLBACK_GAS: Gas = Gas(20_000_000_000_000);
pub const MIN_ACCOUNT_STORAGE_AMOUNT: Balance = 0; // TODO change this

#[near_bindgen]
impl Contract {
    /// User provide access for back-end to its assets
    /// by sign key
    pub fn register_account(&mut self) {
        let account_id = env::predecessor_account_id();
        let public_key = env::signer_account_pk();

        require!(
            self.registered_accounts
                .insert(&public_key, &account_id)
                .is_none(),
            "This public key already registered"
        )
    }

    /// User deny access for back-end to its assets
    /// by specific key
    pub fn unregister_account(&mut self, public_key: PublicKey) {
        let account_id = env::predecessor_account_id();

        let removed_account_id = self
            .registered_accounts
            .remove(&public_key)
            .unwrap_or_else(|| env::panic_str("Account not registered for this key"));

        require!(removed_account_id == account_id, "Not allow");
    }

    /// Create new account for implicit account
    /// Add new access key
    /// Move all assets to new account
    pub fn create_and_register_account(
        &mut self,
        account_id: AccountId,
        new_public_key: PublicKey,
    ) -> PromiseOrValue<u8> {
        self.assert_owner();

        let old_public_key = env::signer_account_pk();
        require!(
            !self.registered_accounts.contains_key(&old_public_key),
            "This public key already registered"
        );

        Promise::new(account_id.clone())
            .create_account()
            .transfer(MIN_ACCOUNT_STORAGE_AMOUNT)
            .add_full_access_key(new_public_key)
            .then(
                ext_self_create_account::ext(env::current_account_id())
                    .with_static_gas(CREATE_AND_REGISTER_ACCOUNT_CALLBACK_GAS)
                    .create_and_register_account_callback(old_public_key, account_id),
            )
            .into()
    }

    // not sure we need this
    // Add new access key for implicit account
    // pub fn add_access_key_for_implicit_account(new_public_key: PublicKey) -> PromiseOrValue<u8> {
    // 	assert_owner();
    //
    // 	Promise::new(account_id)
    // 		.transfer(MIN_IMPLICIT_ACCOUNT_STORAGE_AMOUNT)
    // 		.add_full_access_key(new_public_key)
    // 		.than
    // }
}

#[ext_contract(ext_self_create_account)]
pub trait CreateAccount {
    fn create_and_register_account_callback(
        &mut self,
        old_public_key: PublicKey,
        new_account_id: AccountId,
    ) -> u8;
}

#[near_bindgen]
impl Contract {
    /// Move all assets to new account
    #[private]
    pub fn create_and_register_account_callback(
        &mut self,
        old_public_key: PublicKey,
        new_account_id: AccountId,
    ) -> u8 {
        self.registered_accounts
            .insert(&old_public_key, &new_account_id);

        let _old_account_id = convert_pk_to_account_id(old_public_key);
        // TODO: move all assets from old_account_id to  new_account_id
        todo!()
    }
}
