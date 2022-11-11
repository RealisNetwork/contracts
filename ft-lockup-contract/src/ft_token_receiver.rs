use crate::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FtMessage {
    duration: U64,
    account_id: AccountId,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_eq!(
            env::predecessor_account_id(),
            self.token_account_id,
            "Invalid token ID"
        );
        // let amount = amount.into();
        self.assert_deposit_whitelist(&sender_id);

        let ft_message: FtMessage = serde_json::from_str(&msg).unwrap();

        let index = self.next_index();
        let lockup = Lockup {
            amount: amount.0,
            unlock_on: env::block_timestamp() + ft_message.duration.0,
            is_claimed: false,
        };

        self.lockups.insert(&index, &lockup);
        let mut account_lockups = self
            .account_lockups
            .get(&ft_message.account_id)
            .unwrap_or_default();
        account_lockups.insert(index);
        self.account_lockups
            .insert(&ft_message.account_id, &account_lockups);

        PromiseOrValue::Value(U128(0))
    }
}
