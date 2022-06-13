use near_sdk::{AccountId, Balance, near_bindgen, require};
use near_sdk::env::panic_str;

use crate::*;

#[near_bindgen]
impl Contract {
    ///Put NFT on sale, block it an make available to make new bit.
    pub fn start_auction(&mut self, nft_id: NftId, price: Balance, deadline: near_sdk::Timestamp) {
        self.nfts.sell_nft(nft_id, price, deadline);
    }

    ///Check if new bit is:
    ///  - User have enough tokens
    ///  - Deadline not expired.
    ///  - New bit more then last one.
    ///
    /// Block tokens of new bit owner and unlock money from previous bit.
    pub fn make_bit(&mut self, account_id: AccountId, nft_id: NftId, new_price: Balance) {
        let acc = self.accounts.get(&account_id);
        require!(acc.is_some(),"User not found");

        let mut account = Account::from(acc.unwrap());
        require!(account.free >= new_price,"Not enough tokens");

        let prev_bit = self.nfts.get_bit(nft_id);
        self.nfts.change_price_nft(nft_id, new_price, Some(account_id.clone()));

        account.free -= new_price;
        account.lockups.insert(&Lockup::new(new_price, Some(prev_bit.deadline)));
        self.accounts.insert(&account_id, &VAccount::V1(account));

        if prev_bit.account_id.is_some() {
            let mut account: Account = Account::from(self.accounts.get(&prev_bit.account_id.unwrap())
                .unwrap_or_else(|| panic_str("Account not exist")));

            account.free += prev_bit.price;
            account.lockups.remove(&Lockup::new(prev_bit.price, Some(prev_bit.deadline)));
            self.accounts.insert(&account_id, &VAccount::V1(account));
        }
    }

     ///Finish  transfer NFT from one user to new owner,could be possible if:
     ///    -Auction expired.
     ///    -accountId belongs  previous NFT owner or owner of max bit.
     /// Unblock NFT.
     /// Transfer tokens from NFT owner to buyer and NFT in the opposite direction.
    pub fn confirm_deal(&mut self, nft_id: NftId, account_id: AccountId) {
        let last_bit = self.nfts.get_bit(nft_id);
        let nft = self.nfts.get_nft(nft_id);

        require!(last_bit.get_deadline() > &env::block_timestamp(),"Auction in progress");

        if last_bit.account_id.is_none() {
            require!(nft.owner_id==account_id,"Only for nft owner");
            self.nfts.unlock_nft(nft_id);
            return;
        }

        let new_nft_owner = last_bit.account_id.unwrap();
        require!((&nft.owner_id==&account_id)
            ||(&new_nft_owner==&account_id),"Only for nft owner or owner of max bit");


        let mut account = Account::from(self.accounts.get(&new_nft_owner)
            .unwrap_or_else(|| panic_str("Account not exist")));

        account.lockups.remove(&Lockup::new(last_bit.price, Some(last_bit.deadline)));

        self.accounts.insert(&new_nft_owner, &VAccount::V1(account));

        let mut account = Account::from(self.accounts.get(&nft.owner_id)
            .unwrap_or_else(|| panic_str("Account not exist")));

        account.free += last_bit.price;

        self.nfts.unlock_nft(nft_id);
        self.accounts.insert(&nft.owner_id, &VAccount::V1(account));
        self.nfts.update_nft(nft_id, nft.set_owner_id(new_nft_owner.clone()).unlock_nft());


        self.nfts.transfer_nft(new_nft_owner, nft_id);
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use near_sdk::{AccountId, env, testing_env, VMContext};
    use near_sdk::json_types::U128;
    use near_sdk::RuntimeFeesConfig;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::VMConfig;

    use crate::{Account, Contract, VAccount};
    use crate::StorageKey::Accounts;

    pub fn get_context(caller_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }

    pub fn get_contract() -> Contract {
        let mut contract = Contract::new(U128::from(123), U128::from(1), 10, None, None);
        let i_will_sell = Account::new(1000);
        let i_will_buy = Account::new(1000);
        let i_will_try_to_buy_but_i_just_a_student = Account::new(10);
        contract.accounts.insert(&AccountId::new_unchecked("i_will_sell".to_string()), &VAccount::V1(i_will_sell));
        contract.accounts.insert(&AccountId::new_unchecked("i_will_buy".to_string()), &VAccount::V1(i_will_buy));
        contract.accounts.insert(&AccountId::new_unchecked("i_will_try_to_buy_but_i_just_a_student".to_string()), &VAccount::V1(i_will_try_to_buy_but_i_just_a_student));
        let nft_id =
            contract.nfts.mint_nft(
                AccountId::new_unchecked(
                    "i_will_sell".to_string()),
                "metadata".to_string());
        contract.start_auction(0, 100, env::block_timestamp() + 10000);
        contract
    }

    #[test]
    #[should_panic(expected = "Not enough tokens")]
    fn test_bit_with_out_money() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        contract.make_bit(AccountId::new_unchecked("i_will_try_to_buy_but_i_just_a_student".to_string()), 0, 101);
    }

    #[test]
    #[should_panic(expected = "Bit less then last one")]
    fn test_bit_less_then_it_cost() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context.clone(), VMConfig::free(), RuntimeFeesConfig::free());


        contract.make_bit(AccountId::new_unchecked("i_will_buy".to_string()), 0, 99);
    }

    #[test]
    #[should_panic(expected = "Nft isn't exist or isn't on sale")]
    fn test_bit_on_wrong_nft_id() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context.clone(), VMConfig::free(), RuntimeFeesConfig::free());


        contract.make_bit(AccountId::new_unchecked("i_will_buy".to_string()), 2, 101);
    }
    #[test]
    fn money_locked() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context.clone(), VMConfig::free(), RuntimeFeesConfig::free());


        contract.make_bit(AccountId::new_unchecked("i_will_buy".to_string()), 0, 1000);

      let ac= Account::from(contract.accounts.get(&AccountId::new_unchecked("i_will_buy".to_string())).unwrap());
        assert_eq!(ac.free,0);

    }



    #[test]
    fn test_bit_correct_nft_deal() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context.clone(), VMConfig::free(), RuntimeFeesConfig::free());

        contract.nfts.mint_nft(AccountId::new_unchecked("i_will_sell".to_string()), "metadata".to_string());
        contract.start_auction(1, 10, context.block_timestamp + 5);
        contract.make_bit(AccountId::new_unchecked("i_will_buy".to_string()), 1, 11);
        std::thread::sleep(Duration::from_millis(1000));
        contract.confirm_deal(1, AccountId::new_unchecked("i_will_buy".to_string()));

        assert_eq!(contract.nfts.get_nft(1).owner_id, AccountId::new_unchecked("i_will_buy".to_string()));

        assert_eq!(Account::from(contract.accounts.get(&AccountId::new_unchecked("i_will_buy".to_string())).unwrap()).free, 1000 - 11);
        assert_eq!(Account::from(contract.accounts.get(&AccountId::new_unchecked("i_will_sell".to_string())).unwrap()).free, 1000 + 11);
    }
}
