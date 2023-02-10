use crate::*;
use near_contract_standards::{
    fungible_token::{receiver::ext_ft_receiver, resolver::ext_ft_resolver},
    upgrade::Ownable,
};
use near_sdk::{assert_one_yocto, env, json_types::U128, Gas, Promise};

#[near_bindgen]
impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn owner_add_backend(&mut self, backend_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();

        self.backend.extend(backend_ids.into_iter());
    }

    #[payable]
    pub fn owner_remove_backend(&mut self, backend_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();

        backend_ids.iter().for_each(|v| {
            self.backend.remove(v);
        });
    }

    pub fn get_backend_accounts(&self) -> Vec<AccountId> {
        self.backend.iter().collect()
    }

    #[payable]
    pub fn ft_freeze_call(
        &mut self,
        account_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> Promise {
        assert_one_yocto();
        self.assert_owner();

        require!(
            self.suspensioned_accounts.contains(&account_id),
            "Not suspensioned account"
        );
        require!(
            env::prepaid_gas() > GAS_FOR_FT_TRANSFER_CALL,
            "More gas is required"
        );

        const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
        const GAS_FOR_FT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

        let receiver_id = self.lockup_contract.clone();
        let sender_id = account_id;
        let amount: Balance = amount.into();
        self.ft
            .internal_transfer(&sender_id, &receiver_id, amount, memo);
        // Initiating receiver's call and the callback
        ext_ft_receiver::ext(receiver_id.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL)
            .ft_on_transfer(sender_id.clone(), amount.into(), msg)
            .then(
                ext_ft_resolver::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .ft_resolve_transfer(sender_id, receiver_id, amount.into()),
            )
            .into()
    }
}
