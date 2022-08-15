use crate::{receiver::ext_nft_receiver, resolver::ext_nft_resolver, *};
use near_contract_standards::non_fungible_token::{core::NonFungibleTokenCore, TokenId};
use near_sdk::{assert_one_yocto, near_bindgen, AccountId, Gas, PromiseOrValue};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(5_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    /// Simple transfer. Transfer a given `token_id` from current owner to
    /// `receiver_id`.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or, if using Approval
    ///   Management, one of the approved accounts
    /// * `approval_id` is for use with Approval Management,
    ///   see <https://nomicon.io/Standards/NonFungibleToken/ApprovalManagement.html>
    /// * If using Approval Management, contract MUST nullify approved accounts on successful
    ///   transfer.
    /// * TODO: needed? Both accounts must be registered with the contract for transfer to
    ///   succeed. See see <https://nomicon.io/Standards/StorageManagement.html>
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_id`: the token to transfer
    /// * `approval_id`: expected approval ID. A number smaller than 2^53, and therefore
    ///   representable as JSON. See Approval Management standard for full explanation.
    /// * `memo` (optional): for use cases that may benefit from indexing or providing information
    ///   for a transfer
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        #[allow(unused)] memo: Option<String>,
    ) {
        assert_one_yocto();

        let mut token = self.get_token_internal(&token_id);

        require!(
            token.check_approve_and_revoke_all(&env::predecessor_account_id(), approval_id),
            "Not enough permission"
        );

        self.nft_transfer_internal(&token_id, Some(token), receiver_id, true);
    }

    /// Transfer token and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the
    /// NFT contract at the method `nft_resolve_transfer`.
    ///
    /// You can think of this as being similar to attaching native NEAR tokens
    /// to a function call. It allows you to attach any Non-Fungible Token
    /// in a call to a receiver contract.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or, if using Approval
    ///   Management, one of the approved accounts
    /// * The receiving contract must implement `ft_on_transfer` according to the standard. If it
    ///   does not, FT contract's `ft_resolve_transfer` MUST deal with the resulting failed
    ///   cross-contract call and roll back the transfer.
    /// * Contract MUST implement the behavior described in `ft_resolve_transfer`
    /// * `approval_id` is for use with Approval Management extension, see that document for full
    ///   explanation.
    /// * If using Approval Management, contract MUST nullify approved accounts on successful
    ///   transfer.
    ///
    /// Arguments:
    /// * `receiver_id`: the valid NEAR account receiving the token.
    /// * `token_id`: the token to send.
    /// * `approval_id`: expected approval ID. A number smaller than 2^53, and therefore
    ///   representable as JSON. See Approval Management standard for full explanation.
    /// * `memo` (optional): for use cases that may benefit from indexing or providing information
    ///   for a transfer.
    /// * `msg`: specifies information needed by the receiving contract in order to properly handle
    ///   the transfer. Can indicate both a function to call and the parameters to pass to that
    ///   function.
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        #[allow(unused)] memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_one_yocto();

        let mut token = self.get_token_internal(&token_id);

        let old_approvals = token.approved_account_ids.iter().collect();
        let old_owner = token.owner_id.clone();

        require!(
            token.check_approve_and_revoke_all(&env::predecessor_account_id(), approval_id),
            "Not enough permission"
        );
        self.nft_transfer_internal(&token_id, Some(token), receiver_id.clone(), true);

        ext_nft_receiver::ext(receiver_id.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_NFT_TRANSFER_CALL)
            .nft_on_transfer(
                env::predecessor_account_id(),
                old_owner.clone(),
                token_id.clone(),
                msg,
            )
            .then(
                ext_nft_resolver::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .nft_resolve_transfer(old_owner, receiver_id, token_id, Some(old_approvals)),
            )
            .into()
    }

    /// Returns the token with the given `token_id` or `null` if no such token.
    fn nft_token(
        &self,
        token_id: TokenId,
    ) -> Option<near_contract_standards::non_fungible_token::Token> {
        self.token_by_id.get(&token_id).map(|token| token.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use near_contract_standards::non_fungible_token::{
        approval::NonFungibleTokenApproval, core::NonFungibleTokenCore,
        enumeration::NonFungibleTokenEnumeration,
    };
    use near_sdk::{
        json_types::U128,
        test_utils::{accounts, VMContextBuilder},
        testing_env, ONE_YOCTO,
    };

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_transfer_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_transfer(accounts(0), "test".into(), None, None);
    }

    #[test]
    #[should_panic = "Not enough permission"]
    fn nft_transfer_panic_if_called_not_by_owner_or_approved_account() {
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
        contract.nft_transfer(accounts(0), "test".into(), None, None);
    }

    #[test]
    fn nft_transfer_nullify_approved_accounts_after_transfer() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);
        contract.nft_approve("test".into(), accounts(1), None);
        contract.nft_transfer(accounts(2), "test".into(), None, None);

        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert!(token.approved_account_ids.unwrap().is_empty());
    }

    #[test]
    fn nft_transfer() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);
        contract.nft_transfer(accounts(2), "test".into(), None, None);

        assert_eq!(contract.nft_total_supply(), U128(1));
        assert_eq!(contract.nft_supply_for_owner(accounts(2)), U128(1));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert_eq!(token.token_id, "test");
        assert_eq!(token.owner_id, accounts(2));
        assert!(token.metadata.is_none());
        assert!(token.approved_account_ids.unwrap().is_empty())
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_transfer_call_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_transfer_call(accounts(0), "test".into(), None, None, "".into());
    }

    #[test]
    #[should_panic = "Not enough permission"]
    fn nft_transfer_call_panic_if_called_not_by_owner_or_approved_account() {
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
        contract.nft_transfer_call(accounts(0), "test".into(), None, None, "".into());
    }

    #[test]
    fn nft_transfer_call_nullify_approved_accounts_after_transfer() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);
        contract.nft_approve("test".into(), accounts(1), None);
        contract.nft_transfer_call(accounts(2), "test".into(), None, None, "".into());

        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert!(token.approved_account_ids.unwrap().is_empty());
    }

    #[test]
    fn nft_transfer_call() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None);
        contract.nft_transfer_call(accounts(2), "test".into(), None, None, "".into());

        assert_eq!(contract.nft_total_supply(), U128(1));
        assert_eq!(contract.nft_supply_for_owner(accounts(2)), U128(1));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert_eq!(token.token_id, "test");
        assert_eq!(token.owner_id, accounts(2));
        assert!(token.metadata.is_none());
        assert!(token.approved_account_ids.unwrap().is_empty());
    }
}
