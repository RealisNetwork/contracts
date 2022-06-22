use crate::utils::*;

mod utils;

#[tokio::test]
#[should_panic(expected = "Mint nft, Fail to make transaction.: \
Action #0: ExecutionError(\"Smart contract panicked: Only owner can do this\")")]
async fn mint_not_by_owner() {
    // Setup contract: Alice - owner
    let bob = get_bob();
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Bob call mint
    let result = test_call_mint_nft(&contract, &worker, &bob, &bob).await;

    // Assert error
}

#[tokio::test]
async fn mint() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let charlie = get_charlie();
    let bob = get_bob();

    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice mint for Bob
    test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Assert Bob has NFT
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert!(bob_info.nfts.get(0).is_some());
    // Alice mint for Charlie
    test_call_mint_nft(&contract, &worker, &charlie, &alice).await;
    // Alice mint for Charlie
    test_call_mint_nft(&contract, &worker, &charlie, &alice).await;
    // Assert Charlie has 2 NFTs
    let charlie_info = test_call_get_acc_info(&charlie, &worker, &contract).await;
    assert_eq!(charlie_info.nfts.len(), 2);
}
