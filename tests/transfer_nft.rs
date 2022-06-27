pub use crate::utils::*;

#[tokio::test]
async fn transfer_nft() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice mint nft for Bob with id = 0
    let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;

    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bob_info.nfts.get(0).is_some());

    // Bob transfer nft to Dave
    let result = test_call_transfer_nft(&contract, &worker, &dave, &bob, nft_id.into()).await;
    assert!(result.is_ok());
    // Assert Bob hasn't nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 0);
    // Assert Dave has nft
    let dave_info = test_call_get_acc_info(&dave, &worker, &contract).await;
    assert_eq!(dave_info.nfts.len(), 1);
}

#[tokio::test]
async fn transfer_non_existent_nft() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    let (contract, worker) = TestingEnvBuilder::default().build().await;
    // Alice mint nft for Bob with id = 0
    let _ = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Add Dave to contract and mint NFT
    let _ = test_call_mint_nft(&contract, &worker, &dave, &alice).await;
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 1);
    // Bob transfer nft with id = 2 to Dave
    let result = test_call_transfer_nft(&contract, &worker, &dave, &bob, 2.into()).await;
    // Assert error
    assert_eq!(
        result.err().unwrap().to_string(),
        "Action #0: ExecutionError(\"Smart contract panicked: Nft not exist\")"
    );
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 1);
    // Assert Dave hasn't extra nft
    let dave_info = test_call_get_acc_info(&dave, &worker, &contract).await;
    assert_eq!(dave_info.nfts.len(), 1);
}

#[tokio::test]
async fn transfer_nft_not_own_nft() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();
    let charlie = get_charlie();

    let (contract, worker) = TestingEnvBuilder::default().build().await;
    // Alice mint nft for Bob with id = 0;
    let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Add Charlie to contract and mint NFT
    let _ = test_call_mint_nft(&contract, &worker, &charlie, &alice).await;
    // Add Dave to contract and mint NFT
    let _ = test_call_mint_nft(&contract, &worker, &dave, &alice).await;
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bob_info.nfts.get(0).is_some());

    // Dave transfer nft to Charlie with id = 0
    let result = test_call_transfer_nft(&contract, &worker, &dave, &charlie, nft_id.into()).await;
    // Assert error
    assert_eq!(
        result.err().unwrap().to_string(),
        "Action #0: ExecutionError(\"Smart contract panicked: Only for NFT owner.\")"
    );

    // Assert Dave hasn't extra nft
    let dave_info = test_call_get_acc_info(&dave, &worker, &contract).await;
    assert_eq!(dave_info.nfts.len(), 1);
    // Assert Charlie hasn't extra nft
    let charlie_info = test_call_get_acc_info(&charlie, &worker, &contract).await;
    assert_eq!(charlie_info.nfts.len(), 1);
}

#[tokio::test]
async fn transfer_nft_locked_nft() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    let (contract, worker) = TestingEnvBuilder::default().build().await;
    // Alice mint nft for Bob with id = 1
    let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Add Dave to contract and mint NFT
    let _ = test_call_mint_nft(&contract, &worker, &dave, &alice).await;
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 1);
    // Change state of nft with id = 1 to Lock
    let price: u128 = 100;
    test_call_sell_nft(&contract, &worker, &bob, nft_id.into(), price.into()).await;
    // Bob transfer nft to Dave
    let result = test_call_transfer_nft(&contract, &worker, &dave, &bob, nft_id.into()).await;
    // Assert error
    assert_eq!(
        result.err().unwrap().to_string(),
        "Action #0: ExecutionError(\"Smart contract panicked: Nft locked up\")"
    );
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 1);
    // Assert Dave hasn't extra nft
    let dave_info = test_call_get_acc_info(&dave, &worker, &contract).await;
    assert_eq!(dave_info.nfts.len(), 1);
}
