# Realis nft token contract

## NFT methods

### Mint
Create new NFT token with given id. Can be called only by `contract.owner_account`.

```rust
/// Simple mint. Create token with a given `token_id` for `owner_id`
///
/// Requirements 
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than `contract.owner_id`
#[payable]
pub fn nft_mint(
    &mut self,
    token_id: TokenId,
    owner_id: AccountId,
    metadata: Option<TokenMetadata>,
    memo: Option<String>,
)
```

### Burn

```rust
/// Simple burn. Remove a given `token_id` from current owner
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than token owner or
///  one of the approved accounts
#[payable]
pub fn nft_burn(&mut self, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>)
```

### Transfer

```rust
/// Simple transfer. Transfer a given `token_id` from current owner to
/// `receiver_id`.
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than token owner or
///  one of the approved accounts
#[payable]
fn nft_transfer(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    #[allow(unused)] memo: Option<String>,
)
```

### Transfer call

```rust
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
/// * Contract MUST panic if called by someone other than token owner or
///  one of the approved accounts
#[payable]
fn nft_transfer_call(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    #[allow(unused)] memo: Option<String>,
    msg: String,
) -> PromiseOrValue<bool>
```

### Backend transfer
Same as basic transfer, but can be called only by `contract.backend_account` and after transfer save approval rights for `contract.backend_account`.

```rust
/// Transfer a given `token_id` from current owner to `receiver_id`.
/// Same as nft_transfer but can be called only by backend
/// and save approval on this nft for backend account
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than `contract.backend_id` or, 
///  if `contract.backend_id` not one of the approved accounts
#[payable]
pub fn nft_transfer_backend(
    &mut self,
    receiver_id: AccountId,
    token_id: TokenId,
    approval_id: Option<u64>,
    #[allow(unused)] memo: Option<String>,
)
```

### Approve

```rust
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
#[payable]
fn nft_approve(
    &mut self,
    token_id: TokenId,
    account_id: AccountId,
    msg: Option<String>,
) -> Option<Promise>
```

### Revoke
Revoke an approved account for a specific token.

```rust
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
fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId)
```

### Revoke all
Revoke all approved accounts for a specific token.

```rust
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
fn nft_revoke_all(&mut self, token_id: TokenId)
```

### Lock
Using for marketlace to lock nft which was placed on marketlace.

```rust
/// Lock token by a given `token_id`. Remove given `token_id` from
/// list of "free" tokens and add to locked.
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than token owner or
///  one of the approved accounts
#[payable]
pub fn nft_lock(&mut self, token_id: TokenId, approval_id: Option<u64>)
```

### Unlock
Unlock nft, used to remove nft from marketplace

```rust
/// Unlock token by a given `token_id`. Remove given `token_id` from
/// locked and add to "Free"
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than backend account
#[payable]
pub fn nft_unlock(&mut self, token_id: TokenId)
```

### Unlock and transfer
Unlock nft and transfer to new owner, perform buy on marketplace

```rust
/// Unlock token by a given `token_id`. Remove given `token_id` from
/// locked and transfer to `receiver_id`.
///
/// Requirements
/// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
/// * Contract MUST panic if called by someone other than backend account
#[payable]
pub fn nft_unlock_and_transfer_backend(&mut self, token_id: TokenId, receiver_id: AccountId)
```

## View methods

### NFT token
Returns the token with the given `token_id` or `null` if no such token

```rust
/// Returns the token with the given `token_id` or `null` if no such token
fn nft_token(
    &self,
    token_id: TokenId,
) -> Option<Token>
```

### Is approved
Check if a token is approved for transfer by a given account, optionally checking an approval_id

```rust
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
) -> bool
```

### Total supply
Returns the total supply of non-fungible tokens as a string representing

```rust
/// Returns the total supply of non-fungible tokens as a string representing
/// an unsigned 128-bit integer to avoid JSON number limit of 2^53.
fn nft_total_supply(&self) -> U128
```

### Tokens list
Get a list of all tokens

```rust
/// Get a list of all tokens
///
/// Arguments:
/// * `from_index`: a string representing an unsigned 128-bit integer, representing the starting
///   index of tokens to return
/// * `limit`: the maximum number of tokens to return
///
/// Returns an array of Token objects, as described in Core standard
fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token>
```

### Total supply for owner
Get number of tokens owned by a given account

```rust
/// Get number of tokens owned by a given account
///
/// Arguments:
/// * `account_id`: a valid NEAR account
///
/// Returns the number of non-fungible tokens owned by given `account_id` as
/// a string representing the value as an unsigned 128-bit integer to avoid
/// JSON number limit of 2^53.
fn nft_supply_for_owner(&self, account_id: AccountId) -> U128
```

### Token list for owner
Get list of all tokens owned by a given account

```rust
/// Get list of all tokens owned by a given account
///
/// Arguments:
/// * `account_id`: a valid NEAR account
/// * `from_index`: a string representing an unsigned 128-bit integer, representing the starting
///   index of tokens to return
/// * `limit`: the maximum number of tokens to return
///
/// Returns a paginated list of all tokens owned by this account
fn nft_tokens_for_owner(
    &self,
    account_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<Token> 
```

### Locked tokens supply for owner
Get number of locked tokens owned by a given account

```rust
/// Get number of locked tokens owned by a given account
///
/// Arguments:
/// * `account_id`: a valid NEAR account
///
/// Returns the number of locked non-fungible tokens owned by given `account_id` as
/// a string representing the value as an unsigned 128-bit integer to avoid
/// JSON number limit of 2^53.
pub fn nft_locked_supply_per_owner(&self, account_id: AccountId) -> U128
```

### Locked tokens list for owner
Get list of all locked tokens owned by a given account

```rust
/// Get list of all locked tokens owned by a given account
///
/// Arguments:
/// * `account_id`: a valid NEAR account
/// * `from_index`: a string representing an unsigned 128-bit integer, representing the starting
///   index of tokens to return
/// * `limit`: the maximum number of tokens to return
///
/// Returns a paginated list of all locked tokens owned by this account
pub fn nft_locked_tokens_for_owner(
    &self,
    account_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>,
) -> Vec<Token>
```
