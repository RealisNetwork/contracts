mod utils;

use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json;
use near_sdk::serde_json::{json, Value};
use workspaces::{Contract, Worker};
use realis_near::types::NftId;
use crate::utils::*;
use near_sdk::serde::{Deserialize, self};

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    pub free: U128,
    pub lockups: Vec<LockupInfo>,
    pub nfts: Vec<NftId>,
    pub lockups_free: U128,
}

#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LockupInfo {
    pub amount: U128,
    pub expire_on: U64,
}

async fn mint_nft(contract: &Contract, worker: &Worker<Testnet>, acc_to_mint: &Account, signer_acc: &Account) -> u128 {
    let nft_id = signer_acc.call(&worker, contract.id(), "mint")
        .args_json(
            &json!({
                "recipient_id": &acc_to_mint.id(),
                "nft_metadata": "metadata",
            })
        )
        .unwrap()
        .transact()
        .await
        .unwrap()
        .json::<U128>();

    assert!(nft_id.is_ok());

    nft_id.unwrap().0
}

async fn get_acc_info(account: &Account, worker: &Worker<Testnet>, contract: &Contract) -> AccountInfo {
  let res =  contract
        .call(&worker, "get_account_info")
        .args_json(
            &json!({
                "account_id":account.id()
            }))
        .unwrap()
        .transact()
        .await
        .unwrap()
        .json::<AccountInfo>();
    assert!(res.is_ok());

    res.unwrap()
}

async fn burn_nft_fn(caller_acc: &Account, contract: &Contract, nft_id: usize, worker: &Worker<Testnet>) {
    let res = caller_acc.call(&worker, &contract.id(), &"internal_burn_nft")
        .args_json(&json!({
            "target_id": caller_acc.id(),
            "nft_id": nft_id,
        }))
        .unwrap()
        .transact()
        .await;
    assert!(res.is_ok())
}

#[tokio::test]
// #[ignore]
async fn burn_nft() {
    // Setup contract with owner Alice
    let alice = get_alice();
    let bob = get_bob();

    let (contract, worker) = TestingEnvBuilder::default()
        .build()
        .await;

    // Alice mint nft for Bob with id = 1
    let result = mint_nft(&contract, &worker, &bob, &alice).await;
    assert_eq!(result, 0);

    // Assert Bob has 1 nft
    let bobs_nfts = get_acc_info(&bob, &worker, &contract).await;
    assert!(bobs_nfts.nfts.get(0).is_some());


    // Bob burn nft with id = 0
    burn_nft_fn(&bob, &contract, 0, &worker).await;


    // Assert Bob hasn't nft
    let bobs_nfts = get_acc_info(&bob, &worker, &contract).await;
    assert!(bobs_nfts.nfts.get(0).is_none());

    // Alice mint nft for Bob with id = 1,2,3,4,5
    for _  in 0..6 {
    mint_nft(&contract, &worker, &bob, &alice).await;
    }
    // Assert Bob has 6 nfts
    let bob_info = get_acc_info(&bob,&worker,&contract).await;
    assert_eq!(bob_info.nfts.len(),6);


    // Bob burn nfts with id = 3,5
    burn_nft_fn(&bob,&contract,3,&worker).await;
    burn_nft_fn(&bob,&contract,5,&worker).await;

    // Assert Bob has 4 nft
    let bob_info = get_acc_info(&bob,&worker,&contract).await;
    assert_eq!(bob_info.nfts.len(),4);

    // Assert Bob has nfts with id = 1,2,4
   let res = bob_info
       .nfts
       .iter()
       .filter(|nft_id| **nft_id == 1 || **nft_id == 2 || **nft_id == 4 )
       .count();

    assert_eq!(res,3);

}

#[tokio::test]
async fn burn_nft_non_existed_nft() {
    let alice = get_alice();
    // Setup contract with owner Alice
    let (contract, worker) = TestingEnvBuilder::default().build().await;
    let bob = get_bob();

    // Alice mint nft for Bob with id = 1,2,3
    for i in 0..3 {
        let nft_id = alice.call(&worker, contract.id(), "mint")
            .args_json(&json!({
                "recipient_id": bob.id(),
                "nft_metadata": "metadata"
            }))
            .expect("Invalid arguments")
            .transact()
            .await
            .expect("Cant create NFT")
            .json::<U128>()
            .unwrap();
        assert_eq!(nft_id.0, i)
    }

    // Assert Bob has 3 nft
    let bobs_nfts = contract
        .call(&worker, "get_account_info")
        .args_json(
            &json!({
                "account_id": bob.id()
            }))
        .expect("Invalid arguments")
        .transact()
        .await
        .expect("Cant get info")
        .json::<AccountInfo>()
        .unwrap();
    for i in 0..3 {
        assert!(bobs_nfts.nfts.get(i).is_some());
    }

    // Bob burn nft with id = 5
    let result = bob.call(&worker, &contract.id(), &"internal_burn_nft")
        .args_json(&json!({
            "target_id": bob.id(),
            "nft_id": U128::from(5),
        }))
        .expect("Invalid arguments")
        .transact()
        .await;
    // Assert error
    assert!(result.is_err());
    // Assert Bob has 3 nft
    let bobs_nfts = contract
        .call(&worker, "get_account_info")
        .args_json(
            &json!({
                "account_id": bob.id()
            }))
        .expect("Invalid arguments")
        .transact()
        .await
        .expect("Cant get info")
        .json::<AccountInfo>()
        .unwrap();
    for i in 0..3 {
        assert!(bobs_nfts.nfts.get(i).is_some());
    }
}

#[tokio::test]
#[ignore]
async fn burn_nft_not_own_nft() {
    // Setup contract with owner Alice

    // Alice mint nft for Bob with id = 1,2,3
    // Alice mint nft for Charlie with id = 5,6,7
    // Assert Bob has 3 nft
    // Assert Charlie has 3 nft

    // Bob burn nft with id = 5
    // Assert error
    // Bob burn nft with id = 7
    // Assert error
    // Charlie burn nft with id = 1
    // Assert error
    // Charlie burn nft with id = 2
    // Assert error
    // Assert Bob has 3 nft
    // Assert Charlie has 3 nft
}

#[tokio::test]
#[ignore]
async fn burn_nft_locked_nft() {
    // Setup contract with owner Alice

    // Alice mint nft for Bob
    // Assert Bob has nft
    // Bob change state of nft to locked
    // Assert state of nft

    // Bob burn nft
    // Assert error
}
