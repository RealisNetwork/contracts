use crate::utils::*;

#[tokio::test]
async fn sell_nft() {
    // Setup contract: Alice - owner
    let alice = get_alice();
    let bob = get_bob();

    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice mint nft for Bob with id = 0
    let nft_id = test_call_mint_nft(&contract, &worker, &bob, &alice).await;
    // Assert Bob has nft
    let bob_info = test_call_get_acc_info(&bob, &worker, &contract).await;
    assert_eq!(bob_info.nfts.len(), 1);
    // Bob sell nft with id = 0
    let price: u128 = 100;
    test_call_sell_nft(&contract, &worker, &bob, nft_id.into(), price.into()).await;
    // Assert nft on sell
    let nft_mp_price = test_call_get_nft_marketplace_info(&contract, &worker, nft_id.into()).await;
    assert_eq!(nft_mp_price.0, price);
}
