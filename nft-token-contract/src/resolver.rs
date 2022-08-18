use crate::*;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{ext_contract, near_bindgen, AccountId};
use std::collections::HashMap;

/// Used when an NFT is transferred using `nft_transfer_call`.
/// This is the method that's called after `nft_on_transfer`.
/// This trait is implemented on the NFT contract.
#[ext_contract(ext_nft_resolver)]
pub trait NonFungibleTokenResolver {
    /// Finalize an `nft_transfer_call` chain of cross-contract calls.
    ///
    /// The `nft_transfer_call` process:
    ///
    /// 1. Sender calls `nft_transfer_call` on FT contract
    /// 2. NFT contract transfers token from sender to receiver
    /// 3. NFT contract calls `nft_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. NFT contract resolves promise chain with `nft_resolve_transfer`, and
    /// may    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to `sender_id`
    ///
    /// Arguments:
    /// * `previous_owner_id`: the owner prior to the call to `nft_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `nft_transfer_call`
    /// * `token_id`: the `token_id` argument given to `ft_transfer_call`
    /// * `approvals`: if using Approval Management, contract MUST provide set of original approved
    ///   accounts in this argument, and restore these approved accounts in case of revert.
    ///
    /// Returns true if token was successfully transferred to `receiver_id`.
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approvals: Option<HashMap<AccountId, u64>>,
    ) -> bool;
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approvals: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        let must_revert = match env::promise_result(0) {
            near_sdk::PromiseResult::NotReady => env::abort(),
            near_sdk::PromiseResult::Successful(bytes) => {
                near_sdk::serde_json::from_slice::<bool>(&bytes).unwrap_or(true)
            }
            near_sdk::PromiseResult::Failed => true,
        };

        // if call succeeded, return early
        if !must_revert {
            return true;
        }

        // OTHERWISE, try to set owner back to previous_owner_id and restore
        // approved_account_ids
        let mut token = self.get_token_internal(&token_id);

        if token.owner_id != receiver_id {
            // The token is not owned by the receiver anymore. Can't return it.
            return true;
        }

        token.approved_account_ids.clear();
        token
            .approved_account_ids
            .extend(approvals.unwrap_or_default().into_iter());
        self.nft_transfer_internal(&token_id, Some(token), previous_owner_id, false);

        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{resolver::NonFungibleTokenResolver, *};
    use near_contract_standards::non_fungible_token::{
        approval::NonFungibleTokenApproval, core::NonFungibleTokenCore,
    };
    use near_sdk::{
        test_utils::{accounts, VMContextBuilder},
        testing_env, PromiseResult, RuntimeFeesConfig, VMConfig, ONE_YOCTO,
    };

    #[test]
    fn nft_resolve_transfer_revert_on_promise_failure() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context.clone());
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_approve("test".into(), accounts(1), None);
        let old_token = contract.nft_token("test".into()).unwrap();
        contract.nft_transfer_call(accounts(2), "test".into(), None, None, "".into());

        testing_env!(
            context,
            VMConfig::test(),
            RuntimeFeesConfig::test(),
            HashMap::default(),
            vec![PromiseResult::Failed]
        );
        contract.nft_resolve_transfer(
            accounts(0),
            accounts(2),
            "test".into(),
            old_token.approved_account_ids.clone(),
        );
        let new_token = contract.nft_token("test".into()).unwrap();
        assert_eq!(old_token, new_token);
    }

    #[test]
    fn nft_resolve_transfer_revert_on_promise_return_true() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context.clone());
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_approve("test".into(), accounts(1), None);
        let old_token = contract.nft_token("test".into()).unwrap();
        contract.nft_transfer_call(accounts(2), "test".into(), None, None, "".into());

        testing_env!(
            context,
            VMConfig::test(),
            RuntimeFeesConfig::test(),
            HashMap::default(),
            vec![PromiseResult::Successful(
                near_sdk::serde_json::to_vec(&true).unwrap()
            )]
        );
        contract.nft_resolve_transfer(
            accounts(0),
            accounts(2),
            "test".into(),
            old_token.approved_account_ids.clone(),
        );
        let new_token = contract.nft_token("test".into()).unwrap();
        assert_eq!(old_token, new_token);
    }

    #[test]
    fn nft_resolve() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context.clone());
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_approve("test".into(), accounts(1), None);
        let old_token = contract.nft_token("test".into()).unwrap();
        contract.nft_transfer_call(accounts(2), "test".into(), None, None, "".into());
        let token_after_transfer = contract.nft_token("test".into()).unwrap();

        testing_env!(
            context,
            VMConfig::test(),
            RuntimeFeesConfig::test(),
            HashMap::default(),
            vec![PromiseResult::Successful(
                near_sdk::serde_json::to_vec(&false).unwrap()
            )]
        );
        contract.nft_resolve_transfer(
            accounts(0),
            accounts(2),
            "test".into(),
            old_token.approved_account_ids,
        );
        let new_token = contract.nft_token("test".into()).unwrap();
        assert_eq!(token_after_transfer, new_token);
    }
}
