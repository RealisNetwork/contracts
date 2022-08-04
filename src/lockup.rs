use crate::utils::DEFAULT_LOCK_LIFE_TIME;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    Timestamp,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Lockup {
    GooglePlayBuy(SimpleLockup),
    Staking(SimpleLockup),
    Vesting(SimpleLockup),
}

impl Lockup {
    pub fn is_expired(&self) -> bool {
        match self {
            Self::GooglePlayBuy(lockup) | Self::Staking(lockup) | Self::Vesting(lockup) => {
                lockup.is_expired()
            }
        }
    }

    pub fn get_amount(&self) -> Option<u128> {
        match self {
            Self::GooglePlayBuy(lockup) | Self::Staking(lockup) | Self::Vesting(lockup) => {
                Some(lockup.amount)
            }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct SimpleLockup {
    pub amount: u128,
    pub expire_on: Timestamp,
}

impl SimpleLockup {
    /// `fn get_current_timestamp` returns blocks timestamp in u64.
    ///  # Examples
    /// ```
    /// use realis_near::lockup::Lockup;
    ///
    /// // because of the current timestamp of BLOCK is 0
    /// assert_eq!(Lockup::get_current_timestamp(), 0);
    /// ```
    /// Function for getting timestamp of a block in millis
    pub fn get_current_timestamp() -> u64 {
        near_sdk::env::block_timestamp()
    }

    /// `fn new` creates new lockup instance.
    ///  # Examples
    /// ```
    /// use near_sdk::test_utils::accounts;
    /// use realis_near::account::Account;
    /// use realis_near::lockup::Lockup;
    ///
    /// let mut account = Account::new(accounts(0), 5);
    /// account.lockups.insert(&Lockup::new(55, None));
    ///
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
            expire_on: SimpleLockup::get_current_timestamp()
                + live_time.unwrap_or(DEFAULT_LOCK_LIFE_TIME),
        }
    }

    /// `fn is_expired` check if lockup time is expired.
    ///  # Examples
    /// ```
    /// use realis_near::lockup::Lockup;
    ///
    /// let lockup_not_expired = Lockup::new(55, None);
    /// assert_eq!(lockup_not_expired.is_expired(), false);
    /// let lockup_expired = Lockup::new(55, Some(0));
    /// assert_eq!(lockup_expired.is_expired(), true);
    /// ```
    pub fn is_expired(&self) -> bool {
        Self::get_current_timestamp() >= self.expire_on
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupInfo {
    pub amount: U128,
    pub expire_on: U64,
    #[serde(rename = "type")]
    pub lockup_type: String,
}

impl From<Lockup> for LockupInfo {
    fn from(lockup: Lockup) -> Self {
        match lockup {
            Lockup::GooglePlayBuy(lockup) => LockupInfo {
                amount: U128(lockup.amount),
                expire_on: lockup.expire_on.into(),
                lockup_type: "GooglePlayBuy".to_string(),
            },
            Lockup::Staking(lockup) => LockupInfo {
                amount: U128(lockup.amount),
                expire_on: lockup.expire_on.into(),
                lockup_type: "Staking".to_string(),
            },
            Lockup::Vesting(lockup) => LockupInfo {
                amount: U128(lockup.amount),
                expire_on: lockup.expire_on.into(),
                lockup_type: "Vesting".to_string(),
            },
        }
    }
}
