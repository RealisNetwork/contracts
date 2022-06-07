use crate::*;

#[near_bindgen]
impl Contract {
    // TODO: add doc @Artem_Levchuk
    pub fn transfer(&mut self, recipient_pk: PublicKey, amount: U128) -> U128 {
        require!(amount.0 != 0, "Can't transfer zero amount");
        let sender_id = env::signer_account_pk();

        require!(sender_id != recipient_pk, "Can't transfer to yourself");
        let fee = amount.0 * self.fee as u128 / 100;
        let sender_balance = self
            .accounts
            .get(&sender_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .decrease_balance(amount.0 + fee);

        self.accounts.insert(&sender_id, &sender_balance);
        self.accounts.insert(
            &self.beneficiary_id,
            &self
                .accounts
                .get(&self.beneficiary_id)
                .unwrap_or_default()
                .increase_balance(fee),
        );
        self.accounts.insert(
            &recipient_pk,
            &self
                .accounts
                .get(&recipient_pk)
                .unwrap_or_default()
                .increase_balance(amount.0),
        );

        sender_balance.free.into()
    }
}
