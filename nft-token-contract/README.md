# Realis nft token contract

## NFT methods

### Mint
Create new NFT token with given id. Can be called only by `contract.owner_account`.

```rust
    /// Simple mint. Create token with a given `token_id` for `owner_id`.
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
    )
```

### Burn

```rust
    /// Simple burn. Remove a given `token_id` from current owner.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or
    ///  one of the approved accounts
    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId, approval_id: Option<u64>)
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

### Approve

```rust
    #[payable]
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise>
```

### Revoke

```rust
    #[payable]
    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId)
```

### Revoke all

```rust
    #[payable]
    fn nft_revoke_all(&mut self, token_id: TokenId)
```

## View methods

### NFT token

```rust
    fn nft_token(
        &self,
        token_id: TokenId,
    ) -> Option<Token>
```

### Is approved

```rust
    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool
```

### Total supply

```rust
    fn nft_total_supply(&self) -> U128
```

### Tokens list

```rust
    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token>
```

### Total supply for owner

```rust
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128
```

### Token list for owner

```rust
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> 
```