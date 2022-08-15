# Realis ft token contract

## Interface structure

```rust
pub struct FungibleTokenMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
    pub decimals: u8,
}
```

## Fungible token methods

### Transfer
Transfer tokens from one user to other user. 

```rust
    /// Simple token transfer. Transfer `amount` from caller of the method to `receiver_id`.
    ///
    /// Requirements: 
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes.
    /// * Caller of the method must be different with `receiver_id`.
    /// * Amount must be positive number.
    #[payable]
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>
    )
```

### Transfer call

Same as `Transfer` with gas requirements and initiating receiver's call and the callback

```rust
    /// Token transfer with initiating receiver's call and the callback. Transfer `amount` from caller of the method to `receiver_id` with prepaired gas limitation.
    ///
    /// Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes.
    /// * Caller of the method must be different with `receiver_id`.
    /// * Amount must be positive number.
    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>
```

### Burn

Burn tokens from account & total supply contract.

```rust
    /// Simple token burn. Remove a given `amount` from fn `caller`.
    ///
    /// Requirements: 
    /// * Amount must be positive number.
    pub fn ft_burn(&mut self, amount: U128)
```

## View methods

### Token metadata

```rust
    fn ft_metadata(&self) -> FungibleTokenMetadata
```

### Total supply

Returns total supply tokens on contract.

```rust
    /// Shows `contract.total_supply`. 
    fn ft_total_supply(&self) -> U128
```

### Account balance

Returns balance of account.

```rust
    fn ft_balance_of(&self, account_id: AccountId) -> U128
```
