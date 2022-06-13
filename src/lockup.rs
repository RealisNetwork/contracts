use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    serde::Serialize,
    Timestamp,
};
use std::time::SystemTime;

const DEFAULT_LOCK_LIFE_TIME: u64 = 1000 * 60 * 60 * 24 * 3; // millis * secs * mins  * hours * days

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Lockup {
    pub amount: u128,
    pub expire_on: Timestamp,
}

impl Lockup {
    /// `fn get_current_timestamp` returns blocks timestamp in u64.
    ///  # Examples
    /// ```
    /// get_current_timestamp();
    /// ```
    /// Function for getting timestamp of a block in millis
    pub fn get_current_timestamp() -> u64 {
        near_sdk::env::block_timestamp()
    }

    /// `fn new` creates new lockup instance.
    ///  # Examples
    /// ```
    /// let mut account = Account::new(5);
    /// account.lockups.insert(&Lockup::new(55, None));
    /// ```
    /// # Arguments
    ///  * `live_time` - time in millis lockup will be lock
    ///  * `amount` - The amount of tokens lockup containing
    /// When the new lockup is created and live_time isn't set
    /// new expire_on value is generated (in millis) using
    /// default lifetime, in case there are set lifetime,
    /// expire time is generated using custom live_time
    pub fn new(amount: u128, live_time: Option<u64>) -> Self {
        Self {
            amount,
            expire_on: Lockup::get_current_timestamp()
                + live_time.unwrap_or(DEFAULT_LOCK_LIFE_TIME),
        }
    }

    /// `fn is_expired` returns true if lockup time is expired.
    ///  # Examples
    /// ```
    /// let collection = self.lockups.to_vec();
    ///
    ///    let fold = collection
    ///          .iter()
    ///          .filter(|lock| lock.is_expired())
    ///          .map(|lock| {
    ///              self.lockups.remove(lock);
    ///             lock
    ///      })
    ///     .fold(0, |acc, lock| acc + lock.amount);
    ///     self.free += fold;
    ///     fold
    /// ```
    pub fn is_expired(&self) -> bool {
        Self::get_current_timestamp() >= self.expire_on
    }
}

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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
