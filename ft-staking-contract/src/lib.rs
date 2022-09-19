use near_contract_standards::fungible_token::receiver::ext_ft_receiver;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, is_promise_success,
    json_types::U128,
    near_bindgen, require,
    serde_json::json,
    AccountId, Balance, Gas, PanicOnDefault, Promise, PromiseOrValue,
};
use xtoken::XTokenCost;

pub mod ft_token_core;
pub mod ft_token_receiver;
pub mod metadata;
pub mod update;
pub mod xtoken;

pub const MILLISECOND: u64 = 1_000_000;
pub const SECOND: u64 = 1000 * MILLISECOND;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;
pub const WEEK: u64 = 7 * DAY;

pub const GAS_FOR_UNSTAKE: Gas = Gas(20_000_000_000_000);
pub const GAS_FOR_UNSTAKE_CALLBACK: Gas = Gas(20_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    /// Token contract account id to receive tokens for staking
    pub token_account_id: AccountId,
    /// Lockup contract account id
    pub lockup_account_id: AccountId,
    /// AccountID -> Account xtokens balance.
    accounts: LookupMap<AccountId, Balance>,
    /// Total supply of the all tokens in staking
    total_supply: Balance,
    /// Total supply of the all xtokens
    total_xtoken_supply: Balance,
    /// Determine the price ratio between tokens and xtokens
    xtoken_cost: XTokenCost,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: Option<AccountId>,
        token_account_id: AccountId,
        lockup_account_id: AccountId,
    ) -> Self {
        Self {
            owner_id: owner_id.unwrap_or_else(env::predecessor_account_id),
            token_account_id,
            lockup_account_id,
            accounts: LookupMap::new(b"a"),
            total_supply: 0,
            total_xtoken_supply: 0,
            xtoken_cost: XTokenCost::default(),
        }
    }

    pub fn unstake(&mut self, xtoken_amount: Option<U128>) -> Promise {
        let xtoken_amount = xtoken_amount.map(|a| a.0).unwrap_or_else(|| {
            self.accounts
                .get(&env::predecessor_account_id())
                .unwrap_or_default()
        });
        require!(
            xtoken_amount > 0,
            "The xtoken_amount should be a positive number"
        );
        self.unstake_internal(&env::predecessor_account_id(), xtoken_amount)
    }

    #[private]
    pub fn transfer_on_unstake_callback(&mut self, account_id: AccountId, amount: U128) {
        if is_promise_success() {
            return;
        }
        // Rollback account stake
        self.stake_internal(&account_id, amount.into());
    }
}

impl Contract {
    pub fn stake_internal(&mut self, account_id: &AccountId, amount: Balance) {
        let xtokens_amount = self.xtoken_cost.convert_to_xtokens(amount);
        self.total_xtoken_supply += amount;
        self.total_xtoken_supply += xtokens_amount;
        let account_xtokens_amount = self.accounts.get(account_id).unwrap_or_default();
        self.accounts
            .insert(account_id, &(account_xtokens_amount + xtokens_amount));

        near_contract_standards::fungible_token::events::FtMint {
            owner_id: account_id,
            amount: &xtokens_amount.into(),
            memo: None,
        }
        .emit();
    }

    pub fn unstake_internal(&mut self, account_id: &AccountId, xtoken_amount: Balance) -> Promise {
        let amount = self.xtoken_cost.convert_to_amount(xtoken_amount);
        self.total_supply -= amount;
        self.total_xtoken_supply -= xtoken_amount;
        let account_xtokens_amount = self.accounts.get(account_id).unwrap_or_default();
        self.accounts
            .insert(account_id, &(account_xtokens_amount - xtoken_amount));

        near_contract_standards::fungible_token::events::FtBurn {
            owner_id: account_id,
            amount: &xtoken_amount.into(),
            memo: None,
        }
        .emit();

        ext_ft_receiver::ext(self.lockup_account_id.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_UNSTAKE)
            .ft_on_transfer(
                env::current_account_id(),
                amount.into(),
                json!({
                    "duration": format!("{}", WEEK),
                    "account_id": account_id,
                })
                .to_string(),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_UNSTAKE_CALLBACK)
                    .transfer_on_unstake_callback(account_id.clone(), amount.into()),
            )
    }

    pub fn add_to_pool_internal(&mut self, amount: Balance) {
        self.total_supply += amount;
        if self.total_xtoken_supply != 0 {
            self.xtoken_cost = XTokenCost::new(self.total_supply, self.total_xtoken_supply);
        }
    }
}
