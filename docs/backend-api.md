# Convert PublicKey to AccountId
```js
nearAPI.utils.PublicKey.fromString(pk58).data.hexSlice()
```

# Owner API

### Mint NFT
- type: call
- name: mint
- signer: contract owner
- args:
```json
{
  "recipient_id": "AccountId",
  "nft_metadata": "..."
}
```
- return: NFT id - string number

### GooglePlay buy
- type: call
- name: create_lockup
- signer: contract owner
- args:
duration - optional arg, in nanoseconds, default 3 days
```json
{
  "recipient_id": "AccountId",
  "amount": "12345",
  "duration": 12345
}
```
- return: expire_on - timestamp, in nanoseconds

### GooglePlay refund
- type: call
- name: refund_lockup
- signer: contract owner
- args:
expire_on - lockup expire time in nanoseconds
```json
{
  "recipient_id": "AccountId",
  "expire_on": 12345
}
```
- return: string number, amount that was refunded

# Backend API

### Transfer tokens
- type: call
- name: backend_transfer
- signer: backend account using user `PublicKey`
- args: 
```json
{
  "recipient_id": "AccountId",
  "amount": "12345" 
}
```
- return: string number, LIS left on sender account

### Burn NFT
- type: call
- name: backend_burn
- signer: backend account using user `PublicKey`
- args:
```json
{
  "nft_id": "12345"
} 
```
- return: string number, LIS left on sender account

### Transfer NFT
- type: call
- name: backend_transfer_nft
- signer: backend account using user `PublicKey`
- args:
```json
{
  "recipient_id": "AccountId",
  "nft_id": "12345"
}
```
- return: string number, LIS left on sender account

### Claim lockup
- type: call
- name: backend_claim_lockup
- signer: backend account using user `PublicKey`
- args:
expire_on - lockup expire time in nanoseconds
```json
{
  "expire_on": 12345
}
```
- return: string number, claimed amount

### Claim all lockups
- type: call
- name: backend_claim_all_lockup
- signer: backend account using user `PublicKey`
- args: 
```json
{}
```
- return: string number, claimed amount
