use std::time::SystemTime;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::Timestamp;

const DEFAULT_LOCK_LIFE_TIME: u64 = 60 * 60 * 24 * 3; // secs * mins  * hours * days

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Lock {
    pub amount: u128,
    pub expire_on: Timestamp,
}

impl Lock {

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
            expire_on: Lock::get_current_timestamp_dev() + live_time.unwrap_or(DEFAULT_LOCK_LIFE_TIME),
        }
    }

    pub fn is_expired(&self) -> bool {
        Self::get_current_timestamp_dev() >= self.expire_on
    }
}