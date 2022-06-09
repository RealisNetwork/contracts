use std::time::SystemTime;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::Timestamp;
use crate::lockup;

const DEFAULT_LOCK_LIFE_TIME: u64 = 60 * 60 * 24 * 3; // secs * mins  * hours * days




#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Lockup {
    pub amount: u128,
    pub expire_on: Timestamp,
}

impl Lockup {

    pub fn get_current_timestamp() -> u64 {
        println!("Near TS: {}, Sys TS: {}", near_sdk::env::block_timestamp(), SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
        near_sdk::env::block_timestamp()
    }

    pub fn get_current_timestamp_dev() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn new(amount: u128, live_time: Option<u64>) -> Self {
        Self {
            amount,
            expire_on: Lockup::get_current_timestamp_dev() + live_time.unwrap_or(DEFAULT_LOCK_LIFE_TIME),
        }
    }

    pub fn is_expired(&self) -> bool {
        Self::get_current_timestamp_dev() >= self.expire_on
    }
}

#[derive(BorshSerialize, Debug)]
pub struct LockupInfo {
    pub amount: U128,
    pub expire_on: Timestamp,
}

impl From<Lockup> for LockupInfo {
    fn from(lockup: Lockup) -> Self {
        LockupInfo {
          amount: U128(lockup.amount),
          expire_on: lockup.expire_on,
        }
    }
}
