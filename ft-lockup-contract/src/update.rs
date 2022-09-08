use crate::*;
use near_contract_standards::upgrade::Ownable;

impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

#[near_bindgen]
impl Contract {
    #[init(ignore_state)]
    pub fn update(owner_id: Option<AccountId>) -> Self {
        #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
        pub struct OldContract {
            pub owner_id: AccountId,
            /// Token contract account id to receive tokens for lockup
            pub token_account_id: AccountId,
            /// Account IDs that can create new lockups.
            pub deposit_whitelist: UnorderedSet<AccountId>,
            /// All lockups
            pub lockups: UnorderedMap<LockupIndex, Lockup>,
            /// Lockups indexes by AccountId
            pub account_lockups: LookupMap<AccountId, HashSet<LockupIndex>>,

            pub index: LockupIndex,
        }

        let contract: OldContract =
            env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"));

        Self {
            owner_id: owner_id.unwrap_or_else(env::predecessor_account_id),
            token_account_id: contract.token_account_id,
            deposit_whitelist: contract.deposit_whitelist,
            lockups: contract.lockups,
            account_lockups: contract.account_lockups,
            index: contract.index,
        }
    }
}
