mod account;
mod common;
mod metadata;
mod nft;
mod token;
mod view;

use crate::common::convert_pk_to_account_id;
use crate::nft::Nft;
use account::Account;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, require, PanicOnDefault, Promise, PublicKey, AccountId};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub accounts: UnorderedMap<PublicKey, Account>,
    pub nfts: UnorderedMap<TokenId, Nft>,
    pub owner_id: PublicKey,
    pub beneficiary_id: PublicKey,
    pub fee: u8,
}

#[near_bindgen]
impl Contract {
    // TODO: add doc @Artem_Levchuk
    #[init]
    pub fn new(total_supply: U128, fee: u8, beneficiary_pk: PublicKey) -> Self {
        let owner_id = env::signer_account_pk();

        let mut accounts = UnorderedMap::new(b"a");
        accounts.insert(&owner_id, &Account::new(total_supply.into()));
        let nft = UnorderedMap::new(b'n');
        Self {
            accounts,
            nfts: nft,
            owner_id,
            beneficiary_id: beneficiary_pk,
            fee,
        }
    }

    // TODO: add doc @Artem_Levchuk
    pub fn upgrade_account(_account_id: PublicKey) {
        todo!()
    }

    // For test purpose
    // testnetacc.testnet - 0a6ed56fac9449f4a2ddce63f892532ff643f345b80d95462e0d3dfa24372f90
    // realis.testnet - c9758d8856edc41e16ba907df1b27fc4a128d8b8b7599823e521ce612cfd7584
    // arrot - ef9a3ddf12005bc3a751b7960170b87ef4efdc5bda32bec1102cfc62680f58a9
    pub fn create_account(public_key: PublicKey, pk: PublicKey, amount: U128) -> Promise {
        let account_id = convert_pk_to_account_id(public_key);
        Promise::new(account_id)
            .create_account()
            .add_full_access_key(pk)
            .transfer(amount.0)
    }

    pub fn add_access_key(&mut self, pk: PublicKey) {
        require!(self.accounts.keys().find(|pk| pk == pk).is_some(), "Public key is already exist");
        // require No such account_id registered


        self.accounts.insert(&pk, &Account::new(0));
        // TODO: Save pk -> AccountId env::predecessor_account_id
        // TODO: emit event
    }
}

impl Contract {
    // TODO: add doc @Artem_Levchuk
    pub fn internal_add_nft(&mut self, owner_pk: &PublicKey, token_id: &TokenId, nft: Nft) {
        require!(self.nfts.values().find(|token_id| token_id == token_id).is_some(), "NFT already exist");

        let mut account = self
            .accounts
            .get(owner_pk)
            .unwrap_or_else(|| env::panic_str("No such account"));
        account.nft.insert(token_id);
        self.accounts.insert(owner_pk, &account);

        self.nfts.insert(token_id, &nft);
    }

    // TODO: add doc @Artem_Levchuk
    pub fn internal_remove_nft(&mut self, owner_pk: &PublicKey, token_id: &TokenId) -> Nft {
        let mut account = self
            .accounts
            .get(owner_pk)
            .unwrap_or_else(|| env::panic_str("No such account"));
        account.nft.remove(token_id);
        self.accounts.insert(owner_pk, &account);

        self.nfts
            .remove(token_id)
            .unwrap_or_else(|| env::panic_str("No such NFT"))
    }
}

// /// The accounts to which `owner` have full access
// whitelist: LookupSet<AccountId>,
// ...
// pub fn add_to_whitelist() // only AccountId can do this
// pub fn remove_from_whitelist()  // only AccountId|Owner can do this
// ...
// owner_api


// Contract_id = contract.realis.near
// all accounts is like [...].contract.realis.near
// ...
// register_account()
// ...
// claim_account




// Кароче как я понимаю задачу по аккаунтам
// 1. У каждого юзера должен быть какой-нибудь аккаунт
// 2. Создание этого аккаунта должно быть бесплатным
// 3. Мы должны иметь возможность покрыть комисию транзакций для этих аккаунтов

// Что у нас есть сейчас?
// 1. По сути мы не генерим аккаунт а генерим только ключ доступа(access key)
//    но по идеи этот ключ может быть сразу implicit аккаунтом
// 2. Создание ключа бесплатное, но его добавление стоит достаточно мало
// 3. Мы имеем возможность покрывать комисию для этого пользователя

// Какие есть проблемы с этим?
// В таком случае контракт должен работать с PublicKey, а не AcccountId
// поскольку в near аккаунт может иметь сколько угодно ключей
// и они могут быть одинаковыми для разных аккаунтов
// Возникает проблема с тем, что контракту будет непонятно
// к какому именно аккаунту мы хотим обратится.

// user_realis.near: PublicKey: "Aksadincqo3piw33"
// user_soul.near: PublicKey: "Aksadincqo3piw33"

// contractKeys: {
//  "Aksadincqo3piw33",
// }
