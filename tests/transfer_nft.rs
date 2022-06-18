#[tokio::test]
#[ignore]
async fn transfer_nft() {
    // Setup contract: Alice - owner

    // Alice mint nft for Bob with id = 1
    // Assert Bob has nft

    // Bob transfer nft to Dave
    // Assert Bob hasn't nft
    // Assert Dave has nft
}

#[tokio::test]
#[ignore]
async fn transfer_non_existent_nft() {
    // Setup contract: Alice - owner

    // Alice mint nft for Bob with id = 1
    // Assert Bob has nft

    // Bob transfer nft with id = 2 to Dave
    // Assert error
    // Assert Bob has nft
    // Assert Dave hasn't nft
}

#[tokio::test]
#[ignore]
async fn transfer_nft_not_own_nft() {
    // Setup contract: Alice - owner

    // Alice mint nft for Bob with id = 1
    // Assert Bob has nft

    // Dave transfer nft to Charlie with id = 1
    // Assert error
    // Assert Bob has nft
    // Assert Dave hasn't nft
    // Assert Charlie hasn't nft
}

#[tokio::test]
#[ignore]
async fn transfer_nft_locked_nft() {
    // Setup contract: Alice - owner

    // Alice mint nft for Bob with id = 1
    // Assert Bob has nft
    // Change state of nft with id = 1 to Lock

    // Bob transfer nft to Dave
    // Assert error
    // Assert Bob has nft
    // Assert Dave hasn't nft
}
