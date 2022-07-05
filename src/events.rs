use std::fmt;

use crate::State;
use near_sdk::{
    env,
    json_types::{U128, U64},
    serde::{Serialize, Deserialize},
    serde_json, AccountId,
};

/// Rules of logging events on `Near`.
/// Should include:
///  *`NFT_METADATA_NAME` -name of current standard.
/// *`NFT_METADATA_SPEC` -version of current standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
pub const NFT_STANDARD_NAME: &str = "nep171";

/// `EventLogVariant` help to use several variants of logging events.
/// `NftMint` using for log event about creating new nft.
#[derive(Serialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant<'a> {
    LockupClaimed(Vec<LockupClaimed<'a>>),
    LockupRefund(LockupRefund<'a>),
    LockupCreated(LockupCreated<'a>),
    NftMint(NftMint<'a>),
    NftBurn(NftBurn<'a>),
    ChangeState(ChangeState),
    ChangeBeneficiary(ChangeBeneficiary<'a>),
    ChangeConstantFee(ChangeConstantFee<'a>),
    ChangePercentFee(ChangePercentFee),
    ChangeOwnerId(ChangeOwnerId<'a>),
    ChangeDefaultLockupTime(ChangeDefaultLockupTime<'a>),
    AddBackendId(BackendId<'a>),
    RemoveBackendId(BackendId<'a>),
    IncreaseBalance(IncreaseBalance<'a>),
    Stake(StakingStake<'a>),
    Unstake(StakingUnstake<'a>),
    AddToStakingPool(AddToStakingPool<'a>),
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog<'a> {
    pub standard: String,
    pub version: String,

    #[serde(flatten)]
    pub event: EventLogVariant<'a>,
}

impl<'a> EventLog<'a> {
    pub fn emit(&'a self) {
        env::log_str(&self.to_string());
    }
}

impl<'a> From<EventLogVariant<'a>> for EventLog<'a> {
    fn from(event: EventLogVariant<'a>) -> Self {
        Self {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event,
        }
    }
}

impl<'a> fmt::Display for EventLog<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupClaimed<'a> {
    pub amount: U128,
    pub account_id: &'a AccountId,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupRefund<'a> {
    pub amount: U128,
    pub account_id: &'a AccountId,
    pub timestamp: U64,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupCreated<'a> {
    pub amount: U128,
    pub recipient_id: &'a AccountId,
    pub expire_on: U64,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMint<'a> {
    pub owner_id: &'a AccountId,
    pub meta_data: &'a str,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftBurn<'a> {
    pub account_id: &'a AccountId,
    pub nft_id: U128,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeState {
    pub from: State,
    pub to: State,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeBeneficiary<'a> {
    pub from: &'a AccountId,
    pub to: &'a AccountId,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeOwnerId<'a> {
    pub from: &'a AccountId,
    pub to: &'a AccountId,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangePercentFee {
    pub from: u8,
    pub to: u8,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeConstantFee<'a> {
    pub from: &'a U128,
    pub to: &'a U128,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeDefaultLockupTime<'a> {
    pub from: &'a U64,
    pub to: &'a U64,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct BackendId<'a> {
    pub accounts: &'a Vec<AccountId>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct IncreaseBalance<'a> {
    pub account_id: &'a AccountId,
    pub amount: &'a U128,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingStake<'a> {
    pub staker_id: &'a AccountId,
    pub amount: U128,
    pub x_amount: U128,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingUnstake<'a> {
    pub staker_id: &'a AccountId,
    pub amount: U128,
    pub x_amount: U128,
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AddToStakingPool<'a> {
    pub account_id: &'a AccountId,
    pub amount: U128,
    pub pool_total_supply: U128,
}
