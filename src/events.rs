use std::fmt;

use crate::{NftId, State};
use near_sdk::{
    env,
    json_types::U128,
    serde::{Deserialize, Serialize},
    serde_json, AccountId, Timestamp,
};

/// Rules of logging events on `Near`.
/// Should include:
///  *`NFT_METADATA_NAME` -name of current standard.
/// *`NFT_METADATA_SPEC` -version of current standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
pub const NFT_STANDARD_NAME: &str = "nep171";

/// `EventLogVariant` help to use several variants of logging events.
/// `NftMint` using for log event about creating new nft.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    NftMint(NftMintLog),
    NftBurnLog(NftBurnLog),
    LockupClaimedLog(LockupClaimedLog),
    ChangeState(ChangeStateLog),
    ChangeBeneficiary(ChangeBeneficiaryLog),
    LockupRefundLog(LockupRefundLog),
    LockupCreatedLog(LockupCreatedLog),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl EventLog {
    pub fn emit(&self) {
        env::log_str(&self.to_string());
    }
}

impl From<EventLogVariant> for EventLog {
    fn from(event: EventLogVariant) -> Self {
        Self {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event,
        }
    }
}

impl fmt::Display for EventLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "EVENT_JSON:{}",
            &serde_json::to_string(self).map_err(|_| fmt::Error)?
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintLog {
    pub owner_id: String,
    pub meta_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupClaimedLog {
    pub amount: U128,
    pub account_id: AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeStateLog {
    pub from: State,
    pub to: State,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ChangeBeneficiaryLog {
    pub from: AccountId,
    pub to: AccountId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupRefundLog {
    pub amount: U128,
    pub account_id: AccountId,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupCreatedLog {
    pub amount: U128,
    pub recipient_id: AccountId,
    pub expire_on: Timestamp,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftBurnLog {
    pub account_id: AccountId,
    pub nft_id: NftId,
}
