use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen, AccountId, PanicOnDefault, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct FakeLockupContract {}

#[near_bindgen]
impl FakeLockupContract {
    #[init]
    pub fn new() -> Self {
        FakeLockupContract {}
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for FakeLockupContract {
    fn ft_on_transfer(
        &mut self,
        _sender_id: AccountId,
        amount: U128,
        _msg: String,
    ) -> PromiseOrValue<U128> {
        env::log_str("Successfully transfered half of your amonut");
        PromiseOrValue::Value(U128(amount.0 / 2))
    }
}
