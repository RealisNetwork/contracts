# Staking

## Staking flow

| State | <div style="width:200px">Action</div> | Pool Change  | Pool LIS supply | Pool xLIS supply | LIS/xLIS |
|-------|:--------------------------------------|:------------:|:---------------:|:----------------:|:--------:|
| 1     | User_1 stake 100 LIS                  |     100      |       100       |       100        |    1     |
| 2     | Airdrop 100 LIS                       |     100      |       200       |       100        |    2     |
| 3     | User_2 stake 100 LIS                  |     100      |       300       |       150        |    2     |
| 4     | Airdrop 300 LIS                       |     300      |       600       |       150        |    4     |
| 5     | User_3 stake 200 LIS                  |     200      |       800       |       200        |    4     |
| 6     | User_2 unstake 50 xLIS                |     200      |       600       |       150        |    4     |
| 7     | User_1 unstake 100 xLIS               |     400      |       200       |        50        |    4     |
| 8     | User_3 unstake 50 xLIS                |     200      |        0        |        0         |    4     |
| 9     | User_4 stake 100 LIS                  |     100      |       100       |        25        |    4     |
| 10    | Airdrop 100 LIS                       |     100      |       200       |        25        |    8     |
| 11    | User_5 stake 500 LIS                  |     500      |       700       |        40        |    8     |

## API

### Stake
- type: call
- name: stake
- signer: anyone
- args:
amount - amount of LIS to stake
```json
{
  "amount": "12345"
}
```
- return: x_amount - string number

### Unstake
- type: call
- name: unstake
- signer: anyone
- args:
x_amount - amount of xLIS to unstake
```json
{
  "x_amount": "12345"
}
```
- return: amount - string number

### Add Liquidity to Pool
- type: call
- name: owner_add_to_staking_pool
- signer: contract owner
- args:
amount - amount of tokens added to pool
```json
{
  "amount": "12345"
}
```
- return: amount - string number

### Account Information
- type: view
- name: get_account_info
- signer: anyone
- args:
```json
{
  "account_id": "some.account.near"
}
```
- return:

`free` - balance of the user in LIS tokens  
`x_staked` - balance of the user in xLIS    
`lockups` - array of locked LIS tokens, 
should be filtered by the field `type` only `Staking`   
`expire_on` - time in nanoseconds lockup when will be unlocked(can be claimed)  
`nfts` - do not relate to staking, skip     
`lockups_free` - balance of the LIS that can be claimed by user
(lockups that expired)

```json
{
  "free": "12345",
  "x_staked": "12345",
  "lockups": [{
    "amount": "12345",
    "expire_on": "12345",
    "type": "Staking"
  }],
  "nfts": ["0", "1"],
  "lockups_free": "12345"
}
```