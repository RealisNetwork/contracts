use crate::{Account, Contract, *};
use near_sdk::{env, near_bindgen, require, AccountId};
use primitive_types::U256;

#[near_bindgen]
impl Contract {
    /// `fn internal_transfer` transfers tokens from one user to another,
    /// returns sender balance left
    /// # Examples
    /// ```
    /// use near_sdk::json_types::U128;
    /// use realis_near::Contract;
    /// use near_sdk::test_utils::accounts;
    /// use realis_near::account::Account;
    ///
    /// let sender_id = accounts(0);
    /// let sender_account = Account::new(sender_id.clone(), 30);
    /// let receiver_id = accounts(1);
    /// let mut contract = Contract::new(Some(U128(3000000000)), Some(U128(50)), Some(10), None, None);
    /// contract.accounts.insert(&sender_id, &sender_account.into());
    /// let sender_balance_left = contract.internal_transfer(sender_id, receiver_id, 20 , false);
    /// let reciever_account: Account = contract.accounts.get(&accounts(1)).unwrap().into();
    /// assert_eq!(reciever_account.free, 20);
    /// assert_eq!(sender_balance_left, 10);
    /// ```
    /// # Arguments
    ///  * `sender` - `AccountId` of transferring user
    ///  * `recipient_id`- `AccountId` of user to be transferred.
    ///  * `amount` - The amount of tokens to be transferred
    ///  * `is_fee_required` - Is the user pays for transaction
    /// This function checks if amount != 0 (if no it would panic), takes fee
    /// and amount from sender (via take_fee() function), increases
    /// beneficiary balance for fee and increases recipient balance
    /// by amount, if beneficiary exists, balance will be increased, in case
    /// beneficiary account doesn't exist, account will be created with
    /// balance of amount.
    pub fn internal_transfer(
        &mut self,
        sender: AccountId,
        recipient_id: AccountId,
        amount: u128,
        is_fee_required: bool,
    ) -> u128 {
        require!(amount > 0, "You can't transfer 0 tokens");
        require!(
            sender != recipient_id,
            "You can't transfer tokens to yourself"
        );

        // Charge fee and amount
        let sender_balance_left = self.take_fee(sender, Some(amount), is_fee_required);
        // Try to get recipient
        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id)
            .unwrap_or_else(|| Account::new(recipient_id.clone(), 0).into())
            .into();

        // Increase recipient balance
        recipient_account.free += amount;
        self.accounts
            .insert(&recipient_id, &recipient_account.into());

        sender_balance_left
    }

    /// `fn take_fee` used to take users money and fee, returns sender balance left
    /// # Examples
    /// ```
    /// use near_sdk::json_types::U128;
    /// use near_sdk::test_utils::accounts;
    /// use realis_near::account::Account;
    /// use realis_near::Contract;
    /// let sender_id = accounts(0);
    ///
    /// let sender_account = Account::new(sender_id.clone(), 30);
    /// let mut contract = Contract::new(Some(U128(3000000000)), Some(U128(50)), Some(10), None, None);
    /// contract.accounts.insert(&sender_id, &sender_account.into());
    /// let sender_balance_left = contract.take_fee(sender_id, Some(15), false);
    /// assert_eq!(sender_balance_left, 15);
    /// ```
    /// # Arguments
    ///  * `sender` - `AccountId` of transferring user
    ///  * `amount` - The amount of tokens to be taken while transaction
    ///  * `is_fee_required` - Is the user pays for transaction
    /// This function decreases sender balance in (100 + percent_fee) * amount
    /// and increases beneficiary balance by percent_fee * amount where amount
    /// is Some(u128) and percent_fee is > 0
    /// In case amount in None, function decreases sender balance by
    /// constant_fee and increases beneficiary balance by constant_fee,
    /// where constant_fee >= 0; Function will take and redirect fee to beneficiary
    /// if and only if flag is_fee_required is true
    pub fn take_fee(
        &mut self,
        sender: AccountId,
        amount: Option<u128>,
        is_fee_required: bool,
    ) -> u128 {
        // Calculate total charged amount
        let (charge, fee) = match (amount, is_fee_required) {
            (Some(amount), true) => (
                ((U256::from(amount) * (U256::from(self.percent_fee) + U256::from(100)))
                    / U256::from(100))
                .as_u128(),
                ((U256::from(amount) * U256::from(self.percent_fee as u128)) / U256::from(100))
                    .as_u128(),
            ),
            (Some(amount), false) => (amount, 0),
            (None, true) => (self.constant_fee, self.constant_fee),
            (None, false) => (0, 0),
        };

        // Check if user exists and get account, if user don't exist, rollback transfer
        let mut sender_account: Account = self
            .accounts
            .get(&sender)
            .unwrap_or_else(|| env::panic_str("User not found"))
            .into();

        // Check if user have enough tokens to pay for transaction and to send
        if sender_account.free < charge {
            sender_account.claim_all_lockups(sender.clone());
        }

        // Check if user have enough tokens to send
        require!(
            sender_account.free >= amount.unwrap_or_default(),
            "Not enough balance"
        );

        // Check if user has enough tokens to pay fee, if no, rollback transaction
        require!(sender_account.free >= charge, "Can't pay some fees");

        sender_account.free -= charge;
        let free = sender_account.free;
        self.accounts.insert(&sender, &sender_account.into());

        // Try get beneficiary account
        let mut beneficiary_account: Account = self
            .accounts
            .get(&self.beneficiary_id)
            .unwrap_or_else(|| Account::new(self.beneficiary_id.clone(), 0).into())
            .into();
        // Increase beneficiary balance
        beneficiary_account.free += fee;
        self.accounts
            .insert(&self.beneficiary_id, &beneficiary_account.into());

        free
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils::tests_utils::*;

    #[test]
    fn transfer() {
        let (mut contract, _context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 228

        // receiver
        let receiver_id = accounts(1);
        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(1), 9 * ONE_LIS).into()); // Will be 29

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 20 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();
        assert_eq!(account.free, 3000000002 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 228 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 29 * ONE_LIS);
    }

    #[test]
    fn transfer_overflow() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), u128::MAX).into()); // Will be 228

        contract.internal_transfer(
            sender_id.clone(),
            accounts(1),
            u128::MAX - u128::MAX / 100 * 11,
            true,
        );
    }

    #[test]
    fn transfer_without_fee() {
        let (mut contract, _context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 228

        // receiver
        let receiver_id = accounts(1);
        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(1), 9 * ONE_LIS).into()); // Will be 29

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 20 * ONE_LIS, false);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();
        assert_eq!(account.free, 3000000000 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 230 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 29 * ONE_LIS);
    }

    #[test]
    fn take_fee_without_fee() {
        let (mut contract, _context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 228

        contract.take_fee(sender_id.clone(), None, false);

        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);
    }

    #[test]
    #[should_panic = "You can't transfer tokens to yourself"]
    fn transfer_tokens_to_itself() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(1), 250 * ONE_LIS).into());

        contract.internal_transfer(sender_id.clone(), sender_id, 20 * ONE_LIS, true);
    }

    #[test]
    #[should_panic = "Not enough balance"]
    fn transfer_not_enough_balance() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 250

        // receiver
        let receiver_id = accounts(1);
        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(1), 9 * ONE_LIS).into()); // Will be 9

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 251 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();
        assert_eq!(account.free, 0 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 9 * ONE_LIS);
    }

    #[test]
    #[should_panic = "User not found"]
    fn transfer_sender_not_valid() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = AccountId::from_str("someone.testnet").unwrap(); // Sender is not registered

        // receiver
        let receiver_id = accounts(1);
        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(0), 9 * ONE_LIS).into()); // Will be 9

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 250 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();

        assert_eq!(account.free, 0 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 9 * ONE_LIS);
    }

    #[test]
    #[should_panic = "You can't transfer 0 tokens"]
    fn transfer_zero() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 250

        // receiver
        let receiver_id = accounts(1);
        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(0), 9 * ONE_LIS).into()); // Will be 9

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 0, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into(); // TRY SEND INVALID BALANCE

        assert_eq!(account.free, 0 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 9 * ONE_LIS);
    }

    #[test]
    fn transfer_to_no_account() {
        let (mut contract, _context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);
        contract
            .accounts
            .insert(&sender_id, &Account::new(accounts(0), 250 * ONE_LIS).into()); // Will be 228

        // receiver
        let receiver_id = accounts(1);

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 20 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();

        assert_eq!(account.free, 3000000002 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 228 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 20 * ONE_LIS);
    }

    #[test]
    fn transfer_whith_lockups() {
        let (mut contract, mut context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);

        let mut account_sender: Account = Account::new(accounts(0), 250 * ONE_LIS).into();

        account_sender.lockups.insert(&Lockup {
            amount: 36 * ONE_LIS,
            expire_on: 1654762489,
        });

        contract.accounts.insert(&sender_id, &account_sender.into());

        // receiver
        let receiver_id = accounts(1);

        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(0), 9 * ONE_LIS).into()); // Will be 9

        testing_env!(context
            .block_timestamp(1655102539992)
            .predecessor_account_id(accounts(0))
            .build());

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 260 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();

        assert_eq!(account.free, 3000000026 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 0);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 269 * ONE_LIS);
    }

    #[test]
    fn transfer_with_many_lockups() {
        let (mut contract, mut context) = init_test_env(Some(accounts(2)), None, None);

        // Sender
        let sender_id = accounts(0);

        let mut account_sender: Account = Account::new(accounts(0), 250 * ONE_LIS).into();

        account_sender.lockups.insert(&Lockup {
            amount: 10 * ONE_LIS,
            expire_on: 1654867011023,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 17 * ONE_LIS,
            expire_on: 1654867011023,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 14 * ONE_LIS,
            expire_on: 1654867011023,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 4 * ONE_LIS,
            expire_on: u64::MAX,
        });

        contract.accounts.insert(&sender_id, &account_sender.into());

        // receiver
        let receiver_id = accounts(1);

        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(0), 9 * ONE_LIS).into()); // Will be 9

        println!("TS before: {}", context.context.block_timestamp);

        testing_env!(context
            .block_timestamp(1655102539992)
            .predecessor_account_id(accounts(0))
            .build());

        println!("TS after: {}", context.context.block_timestamp);

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 260 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();

        assert_eq!(account.free, 3000000026 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 5 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 269 * ONE_LIS);
    }

    #[test]
    #[should_panic]
    fn transfer_with_many_but_not_enough_lockups() {
        let (mut contract, _context) = init_test_env(None, None, None);

        // Sender
        let sender_id = accounts(0);

        let mut account_sender: Account = Account::new(accounts(0), 250 * ONE_LIS).into();

        account_sender.lockups.insert(&Lockup {
            amount: 10 * ONE_LIS,
            expire_on: 1654762489,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 12 * ONE_LIS,
            expire_on: u64::MAX,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 4 * ONE_LIS,
            expire_on: 1654762489,
        });

        account_sender.lockups.insert(&Lockup {
            amount: 4 * ONE_LIS,
            expire_on: u64::MAX,
        });

        contract.accounts.insert(&sender_id, &account_sender.into());

        // receiver
        let receiver_id = accounts(1);

        contract
            .accounts
            .insert(&receiver_id, &Account::new(accounts(0), 9 * ONE_LIS).into()); // Will be 9

        contract.internal_transfer(sender_id.clone(), receiver_id.clone(), 251 * ONE_LIS, true);

        let account: Account = contract
            .accounts
            .get(&contract.beneficiary_id.clone())
            .unwrap()
            .into();

        assert_eq!(account.free, 30000000025 * ONE_LIS);
        let account: Account = contract.accounts.get(&sender_id).unwrap().into();
        assert_eq!(account.free, 0 * ONE_LIS);
        let account: Account = contract.accounts.get(&receiver_id).unwrap().into();
        assert_eq!(account.free, 260 * ONE_LIS);
    }
}
