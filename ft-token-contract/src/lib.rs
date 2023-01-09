use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider},
    FungibleToken,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue, Timestamp,
};

mod ft_core;
mod lis_token;
mod owner;
mod storage_impl;
mod update;

pub const FT_METADATA_SPEC: &str = "ft-1.0.1";
pub const DEFAULT_MINT_AMOUNT: u128 = 3_000_000_000 * 10_u128.pow(12);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub staking_contract: AccountId,
    pub ft: FungibleToken,
    pub last_mint: Timestamp,
    pub backend: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: Option<AccountId>,
        backend_ids: Option<Vec<AccountId>>,
        staking_id: AccountId,
    ) -> Self {
        let owner_id = owner_id.unwrap_or_else(env::predecessor_account_id);
        let mut this = Self {
            owner_id: owner_id.clone(),
            staking_contract: staking_id.clone(),
            ft: FungibleToken::new(b"a".to_vec()),
            last_mint: env::block_timestamp(),
            backend: UnorderedSet::new(b"b".to_vec()),
        };

        this.backend
            .extend(backend_ids.unwrap_or_default().into_iter());

        this.ft.internal_register_account(&owner_id);
        this.ft.internal_register_account(&staking_id);

        this.ft.internal_deposit(&owner_id, DEFAULT_MINT_AMOUNT);
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &DEFAULT_MINT_AMOUNT.into(),
            memo: None,
        }
        .emit();

        this
    }
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Realis Network LIS token"),
            symbol: String::from("LIS"),
            icon: Some(String::from("\
            data:image/svg+xml;base64,\
            PHN2ZyB3aWR0aD0iNTEyIiBoZWlnaHQ9IjUxMiIgdmlld0JveD0iMCAwIDUxMiA1MTIiIGZpbGw9Im5vbmU\
            iIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxyZWN0IHdpZHRoPSI1MTIiIGhlaWdodD\
            0iNTEyIiByeD0iMjU2IiBmaWxsPSJibGFjayIvPgo8cGF0aCBkPSJNMTQwLjY2NiAyMzcuMTIxQzE1MC4yM\
            DEgMzM0LjczNyAxNzEuMTM1IDM4Mi42MTIgMjg2LjM4MSAzNTguMTU2QzM0MC44OTQgMzQ2Ljc1NyAzODYu\
            OTEgMzMzLjQ5MyA0NDQuNTMyIDM0My4wMjdDNDQ0LjUzMiAzNDMuMDI3IDQ2OC4zNjkgMzQ4LjYyMyA0NTI\
            uMjAyIDM0OC4yMDhDNDMwLjg1MiAzNDcuNzk0IDM2Ni4zODkgMzc4LjA1MiAzMTYuMjI5IDQwNy42OUMxOT\
            QuOTcyIDQ3OS42MDYgODkuODgzMiA0MDMuNzUyIDE0MC42NjYgMjM3LjEyMVoiIGZpbGw9InVybCgjcGFpb\
            nQwX2xpbmVhcl80MTZfNTQpIi8+CjxwYXRoIGQ9Ik0xNTAuMDI1IDIzMC45NDhDMTExLjg2NCAyODQuODky\
            IDEwMS41MTcgMzE4LjM3MSAxNzAuODQ4IDM1Ni44ODZDMjAzLjc1OCAzNzUuMDQzIDIzMi44MjcgMzg4LjY\
            yMSAyNTguMDExIDQxOC43NjlDMjU4LjAxMSA0MTguNzY5IDI2Ny43MDkgNDMyLjAwNSAyNTkuNzM5IDQyNC\
            44NTlDMjQ5LjA0MiA0MTUuMjY3IDIwMy4wNDMgNDAyLjIxMiAxNjQuMzYzIDM5NS4zNDFDNzAuODYzMiAzN\
            zguMjg5IDUwLjY4MDEgMjkzLjQxIDE1MC4wMjUgMjMwLjk0OFoiIGZpbGw9InVybCgjcGFpbnQxX3JhZGlh\
            bF80MTZfNTQpIi8+CjxwYXRoIGQ9Ik0xNjMuNDE2IDIzOC4yMDRDMTIzLjQ3MSAyMzcuMTg4IDEwMyAyNDM\
            uMzMxIDEwNy4yODUgMjkxLjE2OUMxMDkuMTYyIDMxMy44MzEgMTEyLjM5OSAzMzMuMDg3IDEwNS41MjkgMz\
            U1LjkwNEMxMDUuNTI5IDM1NS45MDQgMTAyLjA3MyAzNjUuMjIyIDEwMy4wNDggMzU4LjgwMkMxMDQuMzI2I\
            DM1MC4xMTQgOTUuMTcxNCAzMjIuNTk5IDg1LjczNzIgMzAwLjk4M0M2Mi42MDk0IDI0OC4xNjYgOTguNDUy\
            NyAyMDkuMzczIDE2My40MTYgMjM4LjIwNFoiIGZpbGw9InVybCgjcGFpbnQyX3JhZGlhbF80MTZfNTQpIi8\
            +CjxwYXRoIGQ9Ik0yNjMuODU5IDMyLjk3OTJDMjYxLjM3MiA0Ny45MDE0IDI3MS4xMTQgNzkuNDAzOCAyOD\
            QuNzk0IDEwOC4wMDVDMjg1LjIwOSAxMDguODM0IDI5Ni40MDIgNDUuNjIxNyAyOTYuODE2IDQ2LjQ1MDdDM\
            zA2Ljc2NiA2Ni41NTQyIDMyOC43MzcgMTMyLjA0NiAzMTguMzczIDE1OC4xNkMzMTcuOTU4IDE1OS40MDMg\
            MzE5LjYxNyAxNjAuMjMyIDMyMC40NDYgMTYwLjY0N0MzNTAuNzA4IDE4NS4xMDMgMzM3LjIzNSAxNjcuMDc\
            yIDMzNy4yMzUgMTk2LjI5NEMzMzcuMjM1IDIwMS42ODMgMzU0LjY0NiAyMDcuOTAxIDM1NC40MzkgMjA4Lj\
            EwOEMzNTMuNDAzIDIxNC41MzMgMzUwLjkxNSAyMTguMDU2IDM0OC4yMjEgMjIxLjE2NUMzMzkuOTMgMjMxL\
            jExMyAzMzIuNjc1IDIyMy4yMzcgMzA4LjgzOCAyMzIuNzcxQzMwMi40MTMgMjM1LjI1OCAzMDcuMTggMjM0\
            LjAxNCAzMDIuMjA1IDI2My44NTlDMjk1Ljk4NyAzMDEuNTc5IDI1NC45NDcgMzA1LjEwMiAyMjUuNzIxIDI\
            4Ni40NDlDOTYuNzk1IDIwMy41NDggMTQ3LjU3OCAyOTcuNjQxIDIxMC43OTcgMzE0LjAxNEMyNDguNTIxID\
            MyMy43NTUgMjgyLjEgMzEzLjgwNyAzMzguNjg2IDMzMi4yNTJDMzM5LjkzIDMzMi42NjcgMzI1LjAwNiAzM\
            jguOTM2IDMwNS4zMTUgMzI3LjlDMjk1LjE1OCAzMjcuMjc4IDI4Mi43MjIgMzI5LjU1OCAyNjguNjI3IDMz\
            MS44MzhDMjQxLjY4MSAzMzYuMzk3IDIxNi42MDEgMzQ2LjEzOCAxODcuMTY3IDMzNy4wMTlDMTU4LjE0OSA\
            zMjcuOSAxNDIuMzk2IDI4Ni40NDkgMTQwLjMyMyAyNzYuOTE2QzEyMC4yMTcgMTkwLjY5OSAyMjEuNTc1ID\
            IzNi4wODcgMjE5LjcxIDI0MS42ODNDMjE1Ljc3MSAyNTMuNzAzIDI2NC40ODEgMjQwLjQzOSAyNjIuNDA4I\
            DIxOC44ODVDMjU4LjQ3IDIxNi42MDUgMjM2LjI5MiAyMDkuNTU5IDI0MC42NDUgMjAyLjMwNUMyNTEuMjE2\
            IDE4NC42ODggMjU5LjkyMSAxNzcuMjI3IDI1OS45MjEgMTc1LjE1NUMyNTkuOTIxIDE3My4yODkgMjQ1LjI\
            wNSAxNjIuNzE5IDIzOS40MDEgMTQ3LjU5QzIyOS4yNDQgMTIxLjA2MiAyMzQuNjM0IDc2LjUwMjMgMjYzLj\
            g1OSAzMi45NzkyWiIgZmlsbD0idXJsKCNwYWludDNfcmFkaWFsXzQxNl81NCkiLz4KPHBhdGggZmlsbC1yd\
            WxlPSJldmVub2RkIiBjbGlwLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0yOTYuMjA3IDIyOS4yMzlDMjg2LjY3MiAy\
            MjUuNTA5IDI3Ny41NTIgMjE2LjgwNCAyNzYuMTAxIDIxMy42OTVDMjcyLjM3IDIwNS4xOTggMjg3LjA4NyA\
            yMDEuNDY4IDMxMC41MDkgMjEwLjc5NEMzMzMuMzEgMjE5LjcwNiAzMzQuNzYgMjE2LjE4MiAzNTQuNDUyID\
            IwOC4zMDdDMzU0LjQ1MiAyMDguMzA3IDM1My42MjMgMjEzLjkwMyAzNTAuOTI4IDIxNy44NEMzNDguNDQxI\
            DIyMS41NzEgMzQ2LjE2MSAyMjMuNjQ0IDM0NS45NTMgMjIzLjg1MUMzNDEuODA4IDIyNy4xNjcgMzM4LjA3\
            NyAyMjcuMTY3IDMzMS44NTkgMjI3Ljc4OUMzMjkuNzg2IDIyNy45OTYgMzI3LjUwNiAyMjguMjAzIDMyNC4\
            2MDQgMjI4LjYxOEMzMTcuMzQ5IDIyOS44NjEgMzExLjMzOCAyMzIuMzQ4IDMwOS4wNTggMjMzLjE3N0MzMD\
            kuMDU4IDIzMy4xNzcgMzA4LjQzNiAyMzMuMzg0IDMwNy42MDcgMjMzLjc5OUMzMDcuMTkzIDIzNC4wMDYgM\
            zA2LjE1NiAyMzQuNDIxIDMwNS45NDkgMjM1LjI1QzMwNS4xMiAyMzcuNTI5IDMwNS4zMjcgMjQwLjg0NiAz\
            MDUuMTIgMjQzLjEyNUMzMDQuNzA1IDI0OS43NTcgMzA0LjI5MSAyNTUuOTc1IDMwMy44NzYgMjU2LjM4OUM\
            yOTkuNzMxIDI1OC42NjkgMzA1LjMyNyAyMzIuOTcgMjk2LjIwNyAyMjkuMjM5WiIgZmlsbD0iI0ZGOTUxQy\
            IvPgo8cGF0aCBkPSJNMTQ2LjM4MSAyNDQuMjcxQzE0MC42NDUgMjIyLjg2MiAxMzQuMzE0IDIxMi44MTggM\
            TA5LjQyIDIyMi40MDZDOTcuNjUwNSAyMjcuMDQzIDg3LjgzOTggMjMxLjUxIDc0LjYxODEgMjMxLjUzNkM3\
            NC42MTgxIDIzMS41MzYgNjkuMTM1OCAyMzEuMjg3IDcyLjY5OTEgMjMwLjY0OUM3Ny41Njg1IDIyOS44OTk\
            gOTAuODA0OCAyMjAuOTI1IDEwMC45MjkgMjEyLjQ4NEMxMjUuNDE4IDE5Mi4xNDIgMTUxLjY4NyAyMDUuMj\
            YyIDE0Ni4zODEgMjQ0LjI3MVoiIGZpbGw9InVybCgjcGFpbnQ0X2xpbmVhcl80MTZfNTQpIi8+CjxkZWZzP\
            go8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDE2XzU0IiB4MT0iMTI3LjYxNCIgeTE9IjMz\
            NS45NDUiIHgyPSI0NTguMDYyIiB5Mj0iMzM1Ljk0NSIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2U\
            iPgo8c3RvcCBzdG9wLWNvbG9yPSIjRkYzRjIxIi8+CjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iI0\
            ZFODcxNiIvPgo8L2xpbmVhckdyYWRpZW50Pgo8cmFkaWFsR3JhZGllbnQgaWQ9InBhaW50MV9yYWRpYWxfN\
            DE2XzU0IiBjeD0iMCIgY3k9IjAiIHI9IjEiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBncmFk\
            aWVudFRyYW5zZm9ybT0idHJhbnNsYXRlKDI2MC4zNDcgNDIwLjI3NCkgcm90YXRlKC0xMzkuMjU4KSBzY2F\
            sZSgyMTguOTcyIDIxOC45NDcpIj4KPHN0b3Agc3RvcC1jb2xvcj0iI0ZFODExNiIvPgo8c3RvcCBvZmZzZX\
            Q9IjEiIHN0b3AtY29sb3I9IiNGRjQ1MjAiLz4KPC9yYWRpYWxHcmFkaWVudD4KPHJhZGlhbEdyYWRpZW50I\
            GlkPSJwYWludDJfcmFkaWFsXzQxNl81NCIgY3g9IjAiIGN5PSIwIiByPSIxIiBncmFkaWVudFVuaXRzPSJ1\
            c2VyU3BhY2VPblVzZSIgZ3JhZGllbnRUcmFuc2Zvcm09InRyYW5zbGF0ZSg5Ni40MzEyIDMyOS4xNTcpIHJ\
            vdGF0ZSgtNzcuMTEzOCkgc2NhbGUoODkuMjgyOCA4OS4yNzI3KSI+CjxzdG9wIG9mZnNldD0iMC4wMjk4Nz\
            U5IiBzdG9wLWNvbG9yPSIjRkU4NTE2Ii8+CjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb2xvcj0iI0ZGNDUyM\
            CIvPgo8L3JhZGlhbEdyYWRpZW50Pgo8cmFkaWFsR3JhZGllbnQgaWQ9InBhaW50M19yYWRpYWxfNDE2XzU0\
            IiBjeD0iMCIgY3k9IjAiIHI9IjEiIGdyYWRpZW50VW5pdHM9InVzZXJTcGFjZU9uVXNlIiBncmFkaWVudFR\
            yYW5zZm9ybT0idHJhbnNsYXRlKDI0Ni4wNzYgMTg3LjA3Mykgc2NhbGUoMTMzLjE5MiAxMzMuMTc3KSI+Cj\
            xzdG9wIHN0b3AtY29sb3I9IiNGRjhDMTYiLz4KPHN0b3Agb2Zmc2V0PSIxIiBzdG9wLWNvbG9yPSIjRkYzR\
            jIxIi8+CjwvcmFkaWFsR3JhZGllbnQ+CjxsaW5lYXJHcmFkaWVudCBpZD0icGFpbnQ0X2xpbmVhcl80MTZf\
            NTQiIHgxPSI3Ni4xMDU5IiB5MT0iMjExLjYzIiB4Mj0iMTQ5Ljg4MSIgeTI9IjIyOS4zMjQiIGdyYWRpZW5\
            0VW5pdHM9InVzZXJTcGFjZU9uVXNlIj4KPHN0b3Agc3RvcC1jb2xvcj0iI0Y0NkYyMyIvPgo8c3RvcCBvZm\
            ZzZXQ9IjAuNTE1NjI1IiBzdG9wLWNvbG9yPSIjRjk1RDIxIi8+CjxzdG9wIG9mZnNldD0iMSIgc3RvcC1jb\
            2xvcj0iI0ZGNDUyMCIvPgo8L2xpbmVhckdyYWRpZW50Pgo8L2RlZnM+Cjwvc3ZnPgo=")),
            reference: None,
            reference_hash: None,
            decimals: 12,
        }
    }
}

impl Contract {
    pub fn backend_register_account(&mut self, account_id: &AccountId) {
        if !self.ft.accounts.contains_key(account_id)
            && self.backend.contains(&env::predecessor_account_id())
        {
            self.ft.internal_register_account(account_id);
        }
    }
}
