use std::time::SystemTime;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

const LOCK_LIFE_TIME: u64 = 60 * 60 * 24 * 3; // secs * mins  * hours * days

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Lock {
    pub amount: u128,
    pub time_stamp: u64,
}

impl Default for Lock {
    fn default() -> Self {
        Self {
            amount: 0,
            time_stamp: Lock::get_current_timestamp(),
        }
    }
}

impl Lock {

    pub fn get_current_timestamp() -> u64 {
        SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn new(amount: u128) -> Self {
        Self {
            amount,
            time_stamp: Lock::get_current_timestamp(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Self::get_current_timestamp() - self.time_stamp >= LOCK_LIFE_TIME
    }
}