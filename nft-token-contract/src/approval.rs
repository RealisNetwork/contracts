use crate::*;
use near_contract_standards::non_fungible_token::approval::{
    ext_nft_approval_receiver, NonFungibleTokenApproval,
};
use near_sdk::{assert_one_yocto, AccountId, Gas, Promise};

const GAS_FOR_NFT_APPROVE: Gas = Gas(10_000_000_000_000);

#[near_bindgen]
impl NonFungibleTokenApproval for Contract {
    /// Add an approved account for a specific token.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of at least 1 yoctoⓃ for security purposes
    /// * Contract MAY require caller to attach larger deposit, to cover cost of storing approver
    ///   data
    /// * Contract MUST panic if called by someone other than token owner
    /// * Contract MUST panic if addition would cause `nft_revoke_all` to exceed single-block gas
    ///   limit
    /// * Contract MUST increment approval ID even if re-approving an account
    /// * If successfully approved or if had already been approved, and if `msg` is present,
    ///   contract MUST call `nft_on_approve` on `account_id`. See `nft_on_approve` description
    ///   below for details.
    ///
    /// Arguments:
    /// * `token_id`: the token for which to add an approval
    /// * `account_id`: the account to add to `approvals`
    /// * `msg`: optional string to be passed to `nft_on_approve`
    ///
    /// Returns void, if no `msg` given. Otherwise, returns promise call to
    /// `nft_on_approve`, which can resolve with whatever it wants.
    #[payable]
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise> {
        assert_one_yocto();

        let mut token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"));

        require!(
            env::predecessor_account_id() == token.owner_id,
            "Predecessor must be token owner"
        );

        let approval_id = token.next_approval_id();
        token.approved_account_ids.insert(&account_id, &approval_id);
        self.token_by_id.insert(&token_id, &token);

        // if given `msg`, schedule call to `nft_on_approve` and return it. Else, return
        // None.
        msg.map(|msg| {
            ext_nft_approval_receiver::ext(account_id)
                .with_static_gas(env::prepaid_gas() - GAS_FOR_NFT_APPROVE)
                .nft_on_approve(token_id, token.owner_id, approval_id, msg)
        })
    }

    /// Revoke an approved account for a specific token.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * If contract requires >1yN deposit on `nft_approve`, contract MUST refund associated
    ///   storage deposit when owner revokes approval
    /// * Contract MUST panic if called by someone other than token owner
    ///
    /// Arguments:
    /// * `token_id`: the token for which to revoke an approval
    /// * `account_id`: the account to remove from `approvals`
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        assert_one_yocto();

        let mut token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"));

        require!(
            env::predecessor_account_id() == token.owner_id,
            "Predecessor must be token owner"
        );

        token.approved_account_ids.remove(&account_id);

        self.token_by_id.insert(&token_id, &token);
    }

    /// Revoke all approved accounts for a specific token.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * If contract requires >1yN deposit on `nft_approve`, contract MUST refund all associated
    ///   storage deposit when owner revokes approvals
    /// * Contract MUST panic if called by someone other than token owner
    ///
    /// Arguments:
    /// * `token_id`: the token with approvals to revoke
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId) {
        assert_one_yocto();

        let mut token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"));

        require!(
            env::predecessor_account_id() == token.owner_id,
            "Predecessor must be token owner"
        );

        token.approved_account_ids.clear();

        self.token_by_id.insert(&token_id, &token);
    }

    /// Check if a token is approved for transfer by a given account, optionally
    /// checking an approval_id
    ///
    /// Arguments:
    /// * `token_id`: the token for which to revoke an approval
    /// * `approved_account_id`: the account to check the existence of in `approvals`
    /// * `approval_id`: an optional approval ID to check against current approval ID for given
    ///   account
    ///
    /// Returns:
    /// if `approval_id` given, `true` if `approved_account_id` is approved with
    /// given `approval_id` otherwise, `true` if `approved_account_id` is in
    /// list of approved accounts
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        self.token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"))
            .is_approved(&approved_account_id, approval_id)
    }
}
