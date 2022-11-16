pub mod lockup;
pub mod staking;
pub mod token;
//pub mod nft;
pub mod utils;

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use crate::utils::*;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId, env, near_bindgen, PanicOnDefault, PromiseOrValue, serde_json};
use near_sdk::json_types::U128;
use workspaces::{
    network::Sandbox,
    Account, Contract, Worker
};

pub struct SandboxEnvironment {
    pub owner: Account,
    pub token: Contract,
    pub staking: Contract,
    pub lockup: Contract,
    //    pub nft: Contract,
}

impl SandboxEnvironment {
    pub async fn new(worker: &Worker<Sandbox>) -> anyhow::Result<Self> {
        let owner = worker.root_account()?;
        let token = token::pull(
            worker,
            Some(owner.id().clone()),
            None,
            STAKING_CONTRACT_ACCOUNT.parse()?,
        )
        .await?;
        let staking = staking::pull(
            worker,
            Some(owner.id().clone()),
            TOKEN_CONTRACT_ACCOUNT.parse()?,
            LOCKUP_CONTRACT_ACCOUNT.parse()?,
        )
        .await?;
        let lockup = lockup::pull(
            worker,
            Some(owner.id().clone()),
            TOKEN_CONTRACT_ACCOUNT.parse()?,
            vec![STAKING_CONTRACT_ACCOUNT.parse()?],
        )
        .await?;

        owner
            .call(token.id(), "storage_deposit")
            .deposit(NEAR)
            .args_json(serde_json::json!({
                "account_id": staking.id()
            }))
            .transact()
            .await?
            .into_result()?;

        owner
            .call(token.id(), "storage_deposit")
            .deposit(NEAR)
            .args_json(serde_json::json!({
                "account_id": lockup.id()
            }))
            .transact()
            .await?
            .into_result()?;

        Ok(Self {
            owner,
            token,
            staking,
            lockup,
        })
    }
}
