//! All the logic described here applies to the NFT auction.
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, UnorderedMap},
    env,
    env::panic_str,
    require, AccountId, Balance, Timestamp,
};

use crate::{Account, Contract, NftId, StorageKey};

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
    /// Count of all auction lots
    pub fn auction_lots_count(&self) -> u64 {
        self.nft_map.len()
    }

    /// HashMap of all auction lots.
    pub fn get_auction_lots(&self) -> &UnorderedMap<NftId, DealData> {
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
            .get_bid()
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
        account_id: &AccountId,
    ) {
        self.nft_map
            .insert(nft_id, &DealData::new(deadline, account_id, price));
    }

    /// Create a new bid.
    /// Tokens of latest bid will be locked.
    /// Tokens of previous bid will be returned.
    pub fn make_bid(
        &mut self,
        account_id: &AccountId,
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
        require!(
            &deal_data.deal_owner != account_id,
            "NFT owner can't make a bid."
        );
        let last_bid = self.get_last_bid(nft_id);
        let highest = match &last_bid {
            Some(last_bid) => last_bid.get_price(),
            None => self.nft_map.get(nft_id).unwrap().get_start_price(),
        };
        require!(highest <= price, "Less or equal to the last bid.");
        deal_data.set_bid(price, account_id);
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
        match deal_data.get_bid() {
            None => require!(
                &account_id == deal_data.get_owner_id(),
                "Only for NFT owner or for owner of highest bid."
            ),
            Some(bid) => require!(
                (&account_id == deal_data.get_owner_id()) || bid.is_owner(&account_id),
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
    highest_bid: LazyOption<Bid>,
}

impl DealData {
    pub fn new(deadline: Timestamp, deal_owner: &AccountId, start_price: Balance) -> Self {
        Self {
            deadline,
            deal_owner: deal_owner.clone(),
            start_price,
            highest_bid: LazyOption::new(StorageKey::NftsAuctionBids, None),
        }
    }

    /// Return bid.
    pub fn get_bid(&self) -> Option<Bid> {
        self.highest_bid.get()
    }

    /// Return date of ending a auction.
    pub fn get_deadline(&self) -> Timestamp {
        self.deadline
    }

    /// Add bid for lot.
    pub fn set_bid(&mut self, price: Balance, account_id: &AccountId) {
        self.highest_bid.set(&Bid::new(price, account_id));
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
    pub fn new(price: Balance, account_id: &AccountId) -> Self {
        Self {
            account_id: account_id.clone(),
            price,
        }
    }

    /// Return owner of bid.
    pub fn get_owner(&self) -> &AccountId {
        &self.account_id
    }

    /// Check if `account_id` owner of bid.
    pub fn is_owner(&self, account_id: &AccountId) -> bool {
        &self.account_id == account_id
    }

    /// Return offered price for lot.
    pub fn get_price(&self) -> Balance {
        self.price
    }
}

impl Contract {
    // TODO: left it here?
    pub fn start_auction(
        &mut self,
        nft_id: NftId,
        price: Balance,
        deadline: Timestamp,
        account_id: AccountId,
    ) {
        self.nfts
            .start_auction(&nft_id, price, deadline, &account_id);
    }

    pub fn make_bid(&mut self, nft_id: NftId, price: Balance, account_id: AccountId) {
        let mut buyer_account = Account::from(
            self.accounts
                .get(&account_id)
                .unwrap_or_else(|| panic_str("Account not found")),
        );

        require!(buyer_account.free >= price, "Not enough money.");
        if let Some(last_bid) = self.nfts.make_bid(&account_id, &nft_id, price) {
            let mut last_buyer_account: Account = self
                .accounts
                .get(last_bid.get_owner())
                .unwrap_or_else(|| panic_str("Account not found"))
                .into();

            last_buyer_account.free += last_bid.get_price();
            self.accounts
                .insert(&last_bid.account_id, &last_buyer_account.into());
        }
        buyer_account.free -= price;
        self.accounts.insert(&account_id, &buyer_account.into());
    }

    pub fn confirm_deal(&mut self, nft_id: NftId) {
        let deal_data = self.nfts.confirm_deal(&nft_id, env::signer_account_id());

        if let Some(the_winner) = deal_data.get_bid() {
            let win_account: Account = self
                .accounts
                .get(&the_winner.account_id)
                .unwrap_or_else(|| panic_str("Account not found"))
                .into();
            self.accounts
                .insert(&the_winner.account_id, &win_account.into());
            let mut nft_owner: Account = self
                .accounts
                .get(deal_data.get_owner_id())
                .unwrap_or_else(|| panic_str("Account not found"))
                .into();
            nft_owner.free += the_winner.price;

            self.accounts
                .insert(deal_data.get_owner_id(), &nft_owner.into());
        }
    }
}

#[cfg(test)]
mod tests {}
