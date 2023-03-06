use near_sdk::json_types::U64;

pub const TOKEN_CONTRACT_ACCOUNT: &str = "token.v1.realisnetwork.near";
pub const NFT_CONTRACT_ACCOUNT: &str = "nft.v1.realisnetwork.near";
pub const STAKING_CONTRACT_ACCOUNT: &str = "staking.v1.realisnetwork.near";
pub const LOCKUP_CONTRACT_ACCOUNT: &str = "lockup.v1.realisnetwork.near";
pub const FAKE_LOCKUP_CONTRACT_ACCOUNT: &str = "fakelockup.v1.realisnetwork.near";

pub const NEAR: u128 = 10_u128.pow(24);
pub const LIS: u128 = 10_u128.pow(12);
pub const YOCTO: u128 = 1;

pub const MILLISECOND: U64 = U64(1_000_000);
pub const SECOND: U64 = U64(1000 * MILLISECOND.0);
pub const MINUTE: U64 = U64(60 * SECOND.0);
pub const HOUR: U64 = U64(60 * MINUTE.0);
pub const DAY: U64 = U64(24 * HOUR.0);
pub const WEEK: U64 = U64(7 * DAY.0);
