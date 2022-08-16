# Fungible Token Lockup contract

## Features

- A reusable lockup contract for a select fungible token.
- Lockup schedule can be set as a list of checkpoints with time and balance.
- Supports multiple lockups per account ID.
- Ability to create a lockup that can be terminated
  - A single lockup can be only terminated by a specific account ID.
  - Supports custom vesting schedule that should be ahead of the lockup schedule
  - The vesting schedule can be hidden behind a hash, so it only needs to be revealed in case of termnation.
- Automatic rollbacks if a FT transfer fails.
- Claiming all account's lockups in a single transaction.
- Ability to add new lockups.
- Whitelist for the accounts that can create new lockups.

## Lockup methods

### Claim
Claim lockup with given lockup index.

```rust
  /// Claim function. Claim lockup for account with given `lockup_index`.
  /// 
  /// Requirements
  /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes.
  /// * Caller must have lockup with given `lockup_index`.
  #[payable]
    pub fn claim(&mut self, index: LockupIndex) -> Promise 
```

## View methods

### Token account id
Return token contract `account_id` to receive tokens for lockup.

```rust
  pub fn get_token_account_id(&self) -> AccountId
```

### Deposit whitelist 
Return list of AccoutIds that can create new lockups.

```rust
  pub fn get_deposit_whitelist(&self) -> Vec<AccountId>
```

### Number of lockups
Return number of all lockups on the contract.

```rust 
  pub fn get_num_lockups(&self) -> U64
```

### Number of account lockups
Return number of lockups on a particular account.

```rust 
  pub fn get_account_num_lockups(&self, account_id: AccountId) -> u32
```

### Lockup
Return one particular lockup with given `lockup_index`.

```rust 
  pub fn get_lockup(&self, index: LockupIndex) -> Option<LockupView>
```

### Lockups
Return set of `(lockup_index - lockup)` with given set of 'lockup_index'.

```rust
  pub fn get_lockups(&self, indexes: Vec<LockupIndex>) -> HashMap<LockupIndex, LockupView>
```

### Lockups paged
Return set of `(lockup_index - lockup)` from all lockups with given `from_index` and `limit` arguments.

```rust
  pub fn get_lockups_paged(
        &self,
        from_index: Option<LockupIndex>,
        limit: Option<LockupIndex>,
    ) -> HashMap<LockupIndex, LockupView>
```

### Account lockups
Return set of `(lockup_index - lockup)` from account lockups with given `account_id`, `from_index` and `limit` arguments.

```rust 
   pub fn get_account_lockups(
        &self,
        account_id: AccountId,
        from_index: Option<LockupIndex>,
        limit: Option<LockupIndex>,
    ) -> HashMap<LockupIndex, LockupView> 
```
