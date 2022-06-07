use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use crate::*;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::AccountId;
use near_sdk::serde::{Deserialize, Serialize};

// TODO: wrap by VNft
#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Nft {
    pub owner_pk: PublicKey,
    pub metadata: NftRealisMetadata,
    pub status: NftStatus,
}

impl Nft {
    pub fn new(owner_pk: PublicKey, metadata: NftRealisMetadata) -> Self {
        Self { owner_pk, metadata, status: NftStatus::Free }
    }

    pub fn set_status(mut self, status: NftStatus) -> Self {
        self.status = status;
        self
    }
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq)]
pub enum NftStatus {
    Free,
    Delegated,
    Marketplace,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct NftRealisMetadata {
    // pub mint_id: u32,
    // pub name: String,
    // pub image: String,
    // pub rarity: Rarity,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Relic,
}

#[near_bindgen]
impl Contract {
    pub fn mint_nft(
        &mut self,
        token_id: &TokenId,
        token_owner_pk: PublicKey,
        nft: NftRealisMetadata,
    ) {
        require!(
            self.owner_id == env::signer_account_pk(),
            "Permission denied"
        );
        let nft = Nft::new(token_owner_pk.clone(), nft);
        self.internal_add_nft(&token_owner_pk, token_id, nft);
    }

    pub fn burn_nft(&mut self, token_id: TokenId) {
        let sender_pk = env::signer_account_pk();
        require!(self.owner_id == sender_pk, "Permission denied");
        self.internal_remove_nft(&sender_pk, &token_id);
    }

    pub fn transfer_nft(
        &mut self,
        token_owner_pk: &PublicKey,
        recipient_pk: &PublicKey,
        token_id: &TokenId,
    ) {
        require!(
            self.owner_id == env::signer_account_pk(),
            "Permission denied"
        );
        let nft = self.internal_remove_nft(token_owner_pk, token_id);
        self.internal_add_nft(recipient_pk, token_id, nft);
    }
}

#[near_bindgen]
impl Contract {
    fn nft_total_supply(&self) -> U128 {
        U128(self.nfts.len().into())
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Nft> {
        self.nfts
            .values()
            .skip(from_index.unwrap_or(U128(0)).0 as usize)
            .take(limit.unwrap_or_else(|| self.nfts.len()) as usize)
            .collect()
    }

    fn nft_supply_for_owner(&self, account_pk: PublicKey) -> U128 {

        // Shit, тут account_id а не PublicKey
        // self.accounts.get()
        todo!()
    }

    fn nft_tokens_for_owner(&self, account_pk: PublicKey, from_index: Option<U128>, limit: Option<u64>) -> Vec<Nft> {
        todo!()
    }
}
// TODO: impl near_contract_standards::NonFungibleTokenEnumeration for Contract
