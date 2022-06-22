use crate::utils::*;
use near_sdk::serde_json::json;
use workspaces::{result::CallExecutionDetails, Contract, Worker};

#[tokio::test]
async fn burn_nft() {
    // Setup contract with owner Alice
    let alice = get_alice();
    let bob = get_bob();

    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice mint nft for Bob with id = 1
    let result = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    assert_eq!(result, 0);

    // Assert Bob has 1 nft
    let bobs_nfts = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bobs_nfts.nfts.get(0).is_some());

    // Bob burn nft with id = 0
    let result = test_call_burn_nft(&bob, &contract, 0.into(), &worker).await;
    assert!(result.is_ok());

    // Assert Bob hasn't nft
    let bobs_nfts = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bobs_nfts.nfts.get(0).is_none());

    // Alice mint nft for Bob with id = 1,2,3,4,5
    for _ in 0..6 {
        test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    }
    // Assert Bob has 6 nfts
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 6);

    // Bob burn nfts with id = 3,5
    let _ = test_call_burn_nft(&bob, &contract, 3.into(), &worker).await;
    let _ = test_call_burn_nft(&bob, &contract, 5.into(), &worker).await;

    // Assert Bob has 4 nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 4);

    // Assert Bob has nfts with id = 1,2,4
    let res = bob_info
        .nfts
        .into_iter()
        .filter(|&nft_id| nft_id == 1 || nft_id == 2 || nft_id == 4)
        .count();

    assert_eq!(res, 3);
}

#[tokio::test]
async fn burn_nft_non_existed_nft() {
    let alice = get_alice();
    // Setup contract with owner Alice
    let (contract, worker) = TestingEnvBuilder::default().build().await;
    let bob = get_bob();

    // Alice mint nft for Bob with id = 1,2,3
    for i in 0..3 {
        let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
        assert_eq!(nft_id, i)
    }

    // Assert Bob has 3 nft
    let bobs_nfts = test_call_get_acc_info(&bob, &worker, &contract).await;

    for i in 0..3 {
        assert!(bobs_nfts.nfts.get(i).is_some());
    }

    // Bob burn nft with id = 5
    let result = test_call_burn_nft(&bob, &contract, 5.into(), &worker).await;
    // Assert error
    assert_eq!(
        result.err().unwrap().to_string(),
        "Action #0: ExecutionError(\"Smart contract panicked: Nft not exist\")"
    );
    // Assert Bob has 3 nft
    let bobs_nfts = test_call_get_acc_info(&bob, &worker, &contract).await;
    for i in 0..3 {
        assert!(bobs_nfts.nfts.get(i).is_some());
    }
}

#[tokio::test]
async fn burn_nft_not_own_nft() {
    // Setup contract with owner Alice
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();

    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice mint nft for Bob with id = 0,1,2
    for _ in 0..3 {
        test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    }
    // Alice mint nft for Charlie with id = 3,4,5
    for _ in 0..3 {
        test_call_mint_nft(&contract, &worker, &charlie, &alice).await;
    }
    // Assert Bob has 3 nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 3);
    // Assert Charlie has 3 nft
    let charlie_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(charlie_info.nfts.len(), 3);
    // Bob burn nft with id = 5
    // Assert error
    let result = test_call_burn_nft(&bob, &contract, 5.into(), &worker).await;
    assert!(result.is_err());
    // Bob burn nft with id = 7
    // Assert error
    let result = test_call_burn_nft(&bob, &contract, 7.into(), &worker).await;
    assert!(result.is_err());
    // Charlie burn nft with id = 1
    // Assert error
    let result = test_call_burn_nft(&charlie, &contract, 7.into(), &worker).await;
    assert!(result.is_err());
    // Charlie burn nft with id = 2
    // Assert error
    let result = test_call_burn_nft(&charlie, &contract, 7.into(), &worker).await;
    assert!(result.is_err());
    // Assert Bob has 3 nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 3);
    // Assert Charlie has 3 nft
    let charlie_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(charlie_info.nfts.len(), 3);
}

#[tokio::test]
async fn burn_nft_locked_nft() {
    // Setup contract with owner Alice
    let (contract, worker) = TestingEnvBuilder::default().build().await;
    let alice = get_alice();
    let bob = get_bob();
    // Alice mint nft for Bob
    let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Assert Bob has nft
    let bobs_nfts = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bobs_nfts.nfts.get(nft_id as usize).is_some());
    // Bob change state of nft to locked
    let price: u128 = 100;
    let _ = test_call_sell_nft(&contract, &worker, &bob, nft_id.into(), price.into()).await;

    // Assert state of nft
    let nft_mp_price = test_call_get_nft_marketplace_info(&contract, &worker, nft_id.into()).await;
    assert_eq!(nft_mp_price.0, price);

    // Bob burn nft
    let result = test_call_burn_nft(&bob, &contract, nft_id.into(), &worker).await;
    // Assert error
    assert!(result.is_err());
}
