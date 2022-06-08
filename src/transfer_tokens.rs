use crate::*;
use crate::{Account, Contract};
use near_sdk::env;
use near_sdk::near_bindgen;
use near_sdk::{require, AccountId};

#[near_bindgen]
impl Contract {
    /// This function checks if amount != 0 (if no it would panic), taskes fee and amount from sender
    /// (via take_fee() function), increases beneficiary balance for fee and increases recipient balance
    /// by amount, if beneficiary exists, balance will be increased, in case beneficiary account
    /// doesn't exist, account will be created with balance of amount
    pub fn internal_transfer(
        &mut self,
        sender: AccountId,
        recipient_id: AccountId,
        amount: u128,
    ) -> u128 {
        require!(amount > 0, "You can't transfer 0 tokens");

        // Charge fee and amount
        let sender_balance_left = self.take_fee(sender, Some(amount));
        // Try to get recipient
        let mut recipient_account = self.accounts.get(&recipient_id).unwrap_or_default();

        // Increase recipient balance
        recipient_account.free += amount;
        self.accounts.insert(&recipient_id, &recipient_account);

        sender_balance_left
    }

    /// This function decreases sender balance in (100 + percent_fee) * amount
    /// and increases beneficiary balance by percent_fee * amount where amount
    /// is Some(u128) and percent_fee is > 0
    /// In case amount in None, function decreases sender balance by constant_fee
    /// and increases beneficiary balance by constant_fee, where constant_fee >= 0
    pub fn take_fee(&mut self, sender: AccountId, amount: Option<u128>) -> u128 {
        // Calculate total charged amount
        let (charge, fee) = if amount.is_some() {
            // TODO: use U256
            (
                (amount.unwrap() * (self.percent_fee as u128 + 100)) / 100,
                (amount.unwrap() * self.percent_fee as u128) / 100,
            )
        } else {
            (self.constant_fee, self.constant_fee)
        };

        // Check if user exists and get account, if user don't exist, rollback transfer
        let mut sender_account = self
            .accounts
            .get(&sender)
            .unwrap_or_else(|| env::panic_str("User not found"));

        // Check if user have enough tokens to send
        require!(
            sender_account.free >= amount.unwrap_or_default(),
            "Not enough balance"
        );

        // Check if user has enough tokens to pay fee, if no, rollback transaction
        require!(sender_account.free >= charge, "Can't pay some fees");

        sender_account.free -= charge;
        self.accounts.insert(&sender, &sender_account);

        // Try get beneficials account
        let mut beneficials_account = self.accounts.get(&self.beneficiary_id).unwrap_or_default();
        // Increase beneficials ballance
        beneficials_account.free += fee;
        self.accounts
            .insert(&self.beneficiary_id, &beneficiary_account.into());

        free
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::collections::{LookupMap, LookupSet};
    use std::str::FromStr;

    fn get_contract() -> Contract {
        Contract {
            beneficiary_id: AccountId::from_str("alice.testnet").unwrap(), // Will be 2
            constant_fee: 5,
            percent_fee: 10,
            accounts: LookupMap::new(b"m"),
            nfts: LookupMap::new(StorageKey::Nfts),
            owner_id: env::predecessor_account_id(),
            backend_id: env::predecessor_account_id(),
            state: State::Running,
        }
    }

    #[test]
    fn transfer() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        contract.accounts.insert(
            &AccountId::from_str(sender_id).unwrap(),
            &Account::new(250).into(),
        ); // Will be 228

        // reciever
        let reciever_id = "mike.testnet";
        contract.accounts.insert(
            &AccountId::from_str(reciever_id).unwrap(),
            &Account {
                free: 9,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 29

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            20,
        );

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            2
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            228
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            29
        );
    }

    #[test]
    #[should_panic = "Not enough balance"]
    fn transfer_not_enough_balance() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        contract.accounts.insert(
            &AccountId::from_str(sender_id).unwrap(),
            &Account {
                free: 250,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 228

        // reciever
        let reciever_id = "mike.testnet";
        contract.accounts.insert(
            &AccountId::from_str(reciever_id).unwrap(),
            &Account {
                free: 9,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 29

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            251,
        );

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            0
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            250
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            9
        );
    }

    #[test]
    #[should_panic = "Can't pay some fees"]
    fn transfer_cant_pay_fees() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        contract.accounts.insert(
            &AccountId::from_str(sender_id).unwrap(),
            &Account {
                free: 250,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 228

        // reciever
        let reciever_id = "mike.testnet";
        contract.accounts.insert(
            &AccountId::from_str(reciever_id).unwrap(),
            &Account {
                free: 9,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 29

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            250,
        );

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            0
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            250
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            9
        );
    }

    #[test]
    #[should_panic]
    fn transfer_sender_not_valid() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        // THERE NO SENDER ACCOUNT

        // reciever
        let reciever_id = "mike.testnet";
        contract.accounts.insert(
            &AccountId::from_str(reciever_id).unwrap(),
            &Account {
                free: 9,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 29

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            250,
        );

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            0
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            250
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            9
        );
    }

    #[test]
    #[should_panic]
    fn transfer_zero() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        contract.accounts.insert(
            &AccountId::from_str(sender_id).unwrap(),
            &Account {
                free: 250,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        );

        // reciever
        let reciever_id = "mike.testnet";
        contract.accounts.insert(
            &AccountId::from_str(reciever_id).unwrap(),
            &Account {
                free: 9,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 29

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            0,
        ); // TRY SEND INVALID BALANCE

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            0
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            250
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            9
        );
    }

    #[test]
    fn transfer_to_no_account() {
        let mut contract = get_contract();

        // Sender
        let sender_id = "gregory.testnet";
        contract.accounts.insert(
            &AccountId::from_str(sender_id).unwrap(),
            &Account {
                free: 250,
                nfts: LookupSet::new(StorageKey::NftId),
            },
        ); // Will be 228

        // reciever
        let reciever_id = "mike.testnet";
        // THERE IS NO RECEIVER ACCOUNT

        contract.internal_transfer(
            AccountId::from_str(sender_id).unwrap(),
            AccountId::from_str(reciever_id.clone()).unwrap(),
            20,
        );

        assert_eq!(
            contract
                .accounts
                .get(&contract.beneficiary_id.clone())
                .unwrap()
                .free,
            2
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(sender_id).unwrap())
                .unwrap()
                .free,
            228
        );
        assert_eq!(
            contract
                .accounts
                .get(&AccountId::from_str(reciever_id).unwrap())
                .unwrap()
                .free,
            20
        );
    }
}
