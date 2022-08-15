# Realis nft token contract

## NFT methods

### Mint
Create new NFT token with given id. Can be called only by `contract.owner_account`.

```rust
    /// Simple burn. Create token with a given `token_id` for `owner_id`.
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
```

### Transfer call

```rust
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
```

## View methods