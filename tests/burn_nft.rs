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
#[ignore]
async fn burn_nft_non_existed_nft() {
    // Setup contract with owner Alice

    // Alice mint nft for Bob with id = 1,2,3
    // Assert Bob has 3 nft

    // Bob burn nft with id = 5
    // Assert error
    // Assert Bob has 3 nft
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
