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
            "Not enought permission"
        );

        self.nft_transfer_internal(&token_id, Some(token), receiver_id);
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
            "Not enought permission"
        );
        self.nft_transfer_internal(&token_id, Some(token), receiver_id.clone());

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
    #[test]
    fn nft_transfer_assert_one_yocto() {
        todo!()
    }

    #[test]
    #[should_panic = "Not enought permission"]
    fn nft_transfer_panic_if_called_not_by_owner_or_approved_account() {
        todo!()
    }

    #[test]
    fn nft_transfer_nullify_approved_accounts_after_transfer() {
        todo!()
    }

    #[test]
    fn nft_transfer() {
        todo!()
    }

     #[test]
    fn nft_transfer_call_assert_one_yocto() {
        todo!()
    }

    #[test]
    #[should_panic = "Not enought permission"]
    fn nft_transfer_call_panic_if_called_not_by_owner_or_approved_account() {
        todo!()
    }

    #[test]
    fn nft_transfer_call_nullify_approved_accounts_after_transfer() {
        todo!()
    }

    #[test]
    fn nft_transfer_call() {
        todo!()
    }
}