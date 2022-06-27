use crate::*;
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        let sender_id = env::signer_account_id();
        self.internal_transfer(sender_id, recipient_id, amount.0, false)
            .into()
    }

    pub fn burn(&mut self, nft_id: U128) {
        self.assert_running();
        let target_account_id = env::signer_account_id();
        self.internal_burn_nft(target_account_id, nft_id.0);
    }

    pub fn transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        self.internal_transfer_nft(env::signer_account_id(), recipient_id, nft_id.0);
    }

    pub fn sell_nft(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.internal_sell_nft(nft_id.0, price.0, env::signer_account_id());
    }

    pub fn buy_nft(&mut self, nft_id: U128) -> U128 {
        self.assert_running();
        let result = self.internal_buy_nft(nft_id.0, env::signer_account_id());

        U128::from(result)
    }

    pub fn change_price(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.nfts
            .change_price_nft(&nft_id.0, price.0, env::signer_account_id());
    }

    pub fn auction(&mut self, nft_id: U128, price: U128, deadline: U128) {
        self.start_auction(
            nft_id.0,
            price.0,
            deadline.0 as u64,
            env::signer_account_id(),
        );
    }

    pub fn bid(&mut self, nft_id: U128, price: U128) {
        self.make_bid(nft_id.0, price.0, env::signer_account_id());
    }

    pub fn confirm(&mut self, nft_id: U128) {
        self.confirm_deal(nft_id.0, env::signer_account_id());
    }

    pub fn claim_lockup(&mut self, amount: U128) -> U128 {
        self.assert_running();
        let target_id = env::signer_account_id();
        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let total_claimed = target_account.claim_lockup(amount.0, target_id.clone());
        self.accounts.insert(&target_id, &target_account.into());
        U128(total_claimed)
    }

    pub fn claim_all_lockup(&mut self) -> U128 {
        self.assert_running();
        let target_id = env::signer_account_id();
        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let total_claimed = target_account.claim_all_lockups(target_id.clone());
        self.accounts.insert(&target_id, &target_account.into());
        U128(total_claimed)
    }

    pub fn stake(&mut self, amount: U128) -> U128 {
        self.assert_running();
        let staker_id = env::signer_account_id();
        self.internal_stake(staker_id, amount.0).into()
    }

    pub fn unstake(&mut self, x_amount: U128) -> U128 {
        self.assert_running();
        let staker_id = env::signer_account_id();
        self.internal_unstake(staker_id, x_amount.0).into()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}

#[cfg(test)]
mod tests {
    use crate::{lockup::Lockup, nft::Nft, utils::tests_utils::*};

    #[test]
    #[should_panic = "Contract is paused"]
    fn transfer_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.transfer(accounts(1), U128(100));
    }

    #[test]
    fn transfer() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        let account_1 = Account::new(accounts(0), 50);
        let account_2 = Account::new(accounts(1), 10);

        contract.accounts.insert(&accounts(1), &account_1.into());
        contract.accounts.insert(&accounts(2), &account_2.into());

        testing_env!(context.signer_account_id(accounts(1)).build());

        contract.transfer(accounts(2), U128(25));

        let account_1: Account = contract.accounts.get(&accounts(1)).unwrap().into();
        let account_2: Account = contract.accounts.get(&accounts(2)).unwrap().into();
        assert_eq!(account_1.free, 25);
        assert_eq!(account_2.free, 35);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn burn_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.burn(U128(1));
    }

    #[test]
    fn burn_nft_test() {
        let owner = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.mint(owner.clone(), "Duck".to_string());
        let owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(contract.nfts.nft_count(), 1);
        assert_eq!(owner_account.nfts.len(), 1);
        contract.burn(U128(nft_id.0));
        let owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(contract.nfts.nft_count(), 0);
        assert_eq!(owner_account.nfts.len(), 0);
    }

    #[test]
    #[should_panic = "Nft not exist"]
    fn burn_nft_test_not_exists() {
        let owner = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        contract.burn(U128(1));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn sell_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.sell_nft(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn change_price_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.change_price(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn transfer_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.transfer_nft(accounts(1), U128(100));
    }

    #[test]
    fn transfer_nft_test() {
        let owner = accounts(0);
        let recipient = accounts(1);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        contract
            .accounts
            .insert(&recipient, &Account::new(recipient.clone(), 0).into());
        let nft_id = contract.nfts.mint_nft(&owner, "Duck".to_string());
        contract.transfer_nft(recipient.clone(), U128(nft_id));
        let nft: Nft = contract.nfts.get_nft(&nft_id).into();
        assert_eq!(nft.owner_id, recipient);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn buy_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.buy_nft(U128(1));
    }

    #[test]
    fn claim_all_lockups() {
        let owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(accounts(0), 5);
        owner_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup::new(5, None)));
        owner_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup::new(6, None)));
        contract.accounts.insert(&owner, &owner_account.into());

        testing_env!(context
            .signer_account_id(accounts(0))
            .block_timestamp(999999999999999999)
            .build());

        contract.claim_all_lockup();
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 16);
    }

    #[test]
    fn claim_lockup() {
        let owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(accounts(0), 50);
        owner_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup {
                amount: 5,
                expire_on: 0,
            }));
        owner_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup {
                amount: 5,
                expire_on: 0,
            }));
        owner_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup {
                amount: 5,
                expire_on: 3,
            }));
        contract.accounts.insert(&owner, &owner_account.into());
        testing_env!(context
            .signer_account_id(accounts(0))
            .block_timestamp(2)
            .build());
        contract.claim_lockup(U128(5));
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 55);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn claim_lockup_panic() {
        let owner = accounts(0);
        let (mut contract, _context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));
        contract.state = State::Paused;

        contract.claim_lockup(U128(1));
    }

    #[test]
    fn stake_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        let account: Account = contract.accounts.get(&user1).unwrap().into();

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        contract.stake(U128(250));

        // Assert total supply is amount of staked tokens
        assert_eq!(contract.staking.get_total_supply(), 250);
    }

    #[test]
    #[should_panic = "Not enough balance"]
    fn stake_over_balance() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        let account: Account = contract.accounts.get(&user1).unwrap().into();

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        contract.stake(U128(251 * ONE_LIS));
    }

    #[test]
    #[should_panic = "User not found"]
    fn stake_no_user_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // set signer as User 1
        testing_env!(context.signer_account_id(accounts(0)).build());

        contract.stake(U128(251 * ONE_LIS));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn stake_paused_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        contract.state = State::Paused;

        // set signer as User 1
        testing_env!(context.signer_account_id(accounts(0)).build());

        contract.stake(U128(251 * ONE_LIS));
    }

    #[test]
    fn unstake_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        let user1_staked = contract.stake(U128(100 * ONE_LIS));

        // Unstake tokens
        contract.unstake(user1_staked);

        // Wait till lockups are expired
        testing_env!(context.block_timestamp(9999999999999999).build());

        // claim loockup for staiking for User 1
        contract.claim_all_lockup();

        // Assert user1 balance == 250
        let account: Account = contract.accounts.get(&user1).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn unstake_contract_paused() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        let user1_staked = contract.stake(U128(100 * ONE_LIS));

        contract.state = State::Paused;

        // Unstake tokens
        contract.unstake(user1_staked);
    }

    #[test]
    #[should_panic = "Not enough x balance"]
    fn unstake_over_staked_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        let user1_staked = contract.stake(U128(100 * ONE_LIS));

        // Unstake tokens
        contract.unstake(U128(user1_staked.0 + 10));
    }

    #[test]
    #[should_panic = "No such account"]
    fn unstake_no_user_test() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // set signer as User 1
        testing_env!(context.signer_account_id(accounts(0)).build());

        // Unstake tokens
        contract.unstake(U128(9));
    }
}
