//! All the logic described here applies to the NFT auction.
use crate::{lockup::Lockup, Account, Contract, ContractExt, NftId, StorageKey, VAccount};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, Vector},
    env,
    env::panic_str,
    near_bindgen, require, AccountId, Balance, Timestamp,
};

/// Auction structure implement logic of NFT auction.
/// Manage bids and auctions deals.
/// # Fields
/// * `nft_map` - key - NFT ID, value - Auction NFT data,
/// stores information about each auction lot.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Auction {
    nft_map: UnorderedMap<NftId, DealData>,
}

impl Default for Auction {
    fn default() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsAuction),
        }
    }
}

impl Auction {
    pub fn new() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsAuction),
        }
    }

    /// Count of all auction lots
    pub fn auction_nft_count(&self) -> u64 {
        self.nft_map.len()
    }

    /// HashMap of all auction lots.
    pub fn get_map(&self) -> &UnorderedMap<NftId, DealData> {
        &self.nft_map
    }

    /// Checks if the NFT is an auction lot.
    pub fn is_in_auction(&self, nft_id: &NftId) -> bool {
        self.nft_map.get(nft_id).is_some()
    }

    /// Return last bid for auction lot.
    fn get_last_bid(&self, nft_id: &NftId) -> Option<Bid> {
        self.nft_map
            .get(nft_id)
            .unwrap_or_else(|| panic_str("Not in auction."))
            .get_last_bid()
    }

    /// Return deal information for auction lot.
    pub fn get_deal_info(&self, nft_id: &NftId) -> DealData {
        self.nft_map
            .get(nft_id)
            .unwrap_or_else(|| panic_str("Deal not found."))
    }

    /// Create new auction lot.
    pub fn start_auction(
        &mut self,
        nft_id: &NftId,
        price: Balance,
        deadline: Timestamp,
        account_id: AccountId,
    ) {
        self.nft_map
            .insert(nft_id, &DealData::new(deadline, account_id, price));
    }

    /// Create a new bid.
    /// Tokens of latest bid will be locked.
    /// Tokens of previous bid will be returned.
    pub fn make_bid(
        &mut self,
        account_id: AccountId,
        nft_id: &NftId,
        price: Balance,
    ) -> Option<Bid> {
        let mut deal_data = self
            .nft_map
            .get(nft_id)
            .unwrap_or_else(|| panic_str("Not in auction."));
        require!(
            deal_data.deadline > env::block_timestamp(),
            "Auction expired."
        );
        let last_bid = self.get_last_bid(nft_id);
        let highest = match &last_bid {
            Some(last_bid) => last_bid.get_price(),
            None => self.nft_map.get(nft_id).unwrap().get_start_price(),
        };
        require!(highest >= price, "Less or equal to the last bid.");
        deal_data.add_bid(&Bid::new(price, account_id));
        self.nft_map.insert(nft_id, &deal_data);
        last_bid
    }

    /// End auction. This action available for winner of auction or owner of
    /// lot.
    pub fn confirm_deal(&mut self, nft_id: &NftId, account_id: AccountId) -> DealData {
        let deal_data = self
            .nft_map
            .get(nft_id)
            .unwrap_or_else(|| panic_str("Not in auction."));
        require!(
            deal_data.get_deadline() < env::block_timestamp(),
            "Auction in progress."
        );
        match deal_data.get_last_bid() {
            None => require!(
                account_id == *deal_data.get_owner_id(),
                "Only for NFT owner or for owner of highest bid."
            ),
            Some(bid) => require!(
                (account_id == *deal_data.get_owner_id()) || bid.is_owner(&account_id),
                "Only for NFT owner or for owner of highest bid."
            ),
        };
        self.nft_map.remove(nft_id).unwrap()
    }
}

/// DealData stores information about auction lot.
/// # Fields
/// * deadline - date of ending a auction.
/// * deal_owner - owner of auction lot.
/// * start_price - basic price for lot.
/// * bids - list of all bids.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct DealData {
    deadline: Timestamp,
    deal_owner: AccountId,
    start_price: Balance,
    bids: Vector<Bid>,
}

impl DealData {
    pub fn new(deadline: Timestamp, deal_owner: AccountId, start_price: Balance) -> Self {
        Self {
            deadline,
            deal_owner,
            start_price,
            bids: Vector::new(StorageKey::NftsAuctionBids),
        }
    }

    /// Return last bid.
    pub fn get_last_bid(&self) -> Option<Bid> {
        self.bids.get(self.bids.len() - 1)
    }

    /// Return date of ending a auction.
    pub fn get_deadline(&self) -> Timestamp {
        self.deadline
    }

    /// All bids for lot.
    pub fn get_all_bids(&self) -> &Vector<Bid> {
        &self.bids
    }

    /// Add bid for lot.
    pub fn add_bid(&mut self, bid: &Bid) {
        self.bids.push(bid);
    }

    /// Return owner id.
    pub fn get_owner_id(&self) -> &AccountId {
        &self.deal_owner
    }

    /// Return basic price.
    pub fn get_start_price(&self) -> Balance {
        self.start_price
    }
}

/// Bid - offer for lot.
/// # Fields
/// * account_id - offer owner.
/// * price - offered price for lot.
#[derive(Debug, BorshSerialize, BorshDeserialize, Eq, PartialEq)]
pub struct Bid {
    account_id: AccountId,
    price: Balance,
}

impl Bid {
    pub fn new(price: Balance, account_id: AccountId) -> Self {
        Self { account_id, price }
    }

    /// Return owner of bid.
    pub fn get_owner(&self) -> &AccountId {
        &self.account_id
    }

    /// Check if `account_id` owner of bid.
    pub fn is_owner(&self, account_id: &AccountId) -> bool {
        self.account_id == *account_id
    }

    /// Return offered price for lot.
    pub fn get_price(&self) -> Balance {
        self.price
    }
}

#[near_bindgen]
impl Contract {
    // TODO: this is need to be here?
    pub fn start_auction(&mut self, nft_id: NftId, price: Balance, deadline: Timestamp) {
        self.nfts
            .start_auction(&nft_id, price, deadline, env::signer_account_id());
    }

    pub fn make_bid(&mut self, nft_id: NftId, price: Balance) {
        let mut buyer_account = Account::from(
            self.accounts
                .get(&env::signer_account_id())
                .unwrap_or_else(|| panic_str("Account not found")),
        );

        require!(buyer_account.free >= price, "Not enough money.");

        let data = self.nfts.get_deal_data(&nft_id);
        if let Some(last_bid) = self.nfts.make_bid(env::signer_account_id(), &nft_id, price) {
            let mut last_buyer_account = Account::from(
                self.accounts
                    .get(last_bid.get_owner())
                    .unwrap_or_else(|| panic_str("Account not found")),
            );
            last_buyer_account.free += last_bid.get_price();
            // TODO: Lockup::new() will have same hash?
            last_buyer_account
                .lockups
                .remove(&Lockup::new(last_bid.price, Some(data.deadline)));
            self.accounts
                .insert(&last_bid.account_id, &VAccount::V1(last_buyer_account));
        }
        buyer_account.free -= price;
        // TODO: need to be locked?
        buyer_account
            .lockups
            .insert(&Lockup::new(price, Some(data.get_deadline())));
    }

    pub fn confirm_deal(&mut self, nft_id: NftId) {
        let deal_data = self.nfts.confirm_deal(&nft_id, env::signer_account_id());
        match deal_data.get_last_bid() {
            None => {}
            Some(the_winner) => {
                let mut win_buyer_account = Account::from(
                    self.accounts
                        .get(&the_winner.account_id)
                        .unwrap_or_else(|| panic_str("Account not found")),
                );
                // TODO: Lockup::new will have same hash?
                win_buyer_account.lockups.remove(&Lockup::new(
                    the_winner.price,
                    Some(deal_data.get_deadline()),
                ));
                self.accounts
                    .insert(&the_winner.account_id, &VAccount::V1(win_buyer_account));
                let mut nft_owner = Account::from(
                    self.accounts
                        .get(deal_data.get_owner_id())
                        .unwrap_or_else(|| panic_str("Account not found")),
                );
                nft_owner.free += the_winner.price;

                self.accounts
                    .insert(deal_data.get_owner_id(), &VAccount::V1(nft_owner));
            }
        }
    }
}
