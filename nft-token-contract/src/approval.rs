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

#[cfg(test)]
mod tests {
    use crate::*;
    use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;
    use near_sdk::{
        test_utils::{accounts, VMContextBuilder},
        testing_env, ONE_YOCTO,
    };

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_approve_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_approve("test".into(), accounts(0), None);
    }

    #[test]
    #[should_panic = "Predecessor must be token owner"]
    fn nft_approve_should_panic_if_called_not_by_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_approve("test".into(), accounts(2), None);
    }

    #[test]
    fn nft_approve_call_on_approve_if_message_provided() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        let option_promise = contract.nft_approve("test".into(), accounts(2), Some("test".into()));
        assert!(option_promise.is_some());
    }

    #[test]
    fn nft_approve() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let token_id: String = "test".into();

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint(token_id.clone(), accounts(1), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_approve(token_id.clone(), accounts(2), None);

        assert!(contract.nft_is_approved(token_id, accounts(2), None));
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_revoke_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_revoke("test".into(), accounts(0));
    }

    #[test]
    #[should_panic = "Predecessor must be token owner"]
    fn nft_revoke_should_panic_if_called_not_by_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_revoke("test".into(), accounts(2));
    }

    #[test]
    fn nft_revoke() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let token_id: String = "test".into();

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint(token_id.clone(), accounts(0), None);

        contract.nft_approve(token_id.clone(), accounts(1), None);
        contract.nft_approve(token_id.clone(), accounts(2), None);
        contract.nft_approve(token_id.clone(), accounts(3), None);

        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(3), None));

        contract.nft_revoke(token_id.clone(), accounts(2));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(3), None));

        contract.nft_revoke(token_id.clone(), accounts(3));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(3), None));

        contract.nft_revoke(token_id.clone(), accounts(1));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(3), None));
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_revoke_all_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_revoke_all("test".into());
    }

    #[test]
    #[should_panic = "Predecessor must be token owner"]
    fn nft_revoke_all_should_panic_if_called_not_by_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_revoke_all("test".into());
    }

    #[test]
    fn nft_revoke_all() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let token_id: String = "test".into();

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint(token_id.clone(), accounts(0), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_approve(token_id.clone(), accounts(1), None);
        contract.nft_approve(token_id.clone(), accounts(2), None);
        contract.nft_approve(token_id.clone(), accounts(3), None);

        assert!(contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(contract.nft_is_approved(token_id.clone(), accounts(3), None));

        contract.nft_revoke_all(token_id.clone());
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(1), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(2), None));
        assert!(!contract.nft_is_approved(token_id.clone(), accounts(3), None));
    }
}
