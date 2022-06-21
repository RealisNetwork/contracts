#[tokio::test]
#[ignore]
async fn burn_nft() {
    // Setup contract with owner Alice

    // Alice mint nft for Bob with id = 1
    // Assert Bob has 1 nft

    // Bob burn nft with id = 1
    // Assert Bob hasn't nft

    // Alice mint nft for Bob with id = 1,2,3,4,5
    // Assert Bob has 5 nfts

    // Bob burn nfts with id = 3,5
    // Assert Bob has 3 nft
    // Assert Bob has nfts with id = 1,2,4
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
