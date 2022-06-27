#[tokio::test]
#[ignore]
async fn backend_transfer_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1
    // Assert Backend.user1 has nft

    // Backend.user1 transfer nft to Backend.user2
    // Assert Backend.user1 hasn't nft
    // Assert Backend.user2 has nft
}

#[tokio::test]
#[ignore]
async fn backend_transfer_non_existent_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1
    // Assert Backend.user1 has nft

    // Backend.user1 transfer nft with id = 2 to Backend.user2
    // Assert error
    // Assert Backend.user1 has nft
    // Assert Backend.user2 hasn't nft
}

#[tokio::test]
#[ignore]
async fn transfer_nft_not_own_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1
    // Assert Backend.user1 has nft

    // Backend.user2 transfer nft to Backend.user3 with id = 1
    // Assert error
    // Assert Backend.user1 has nft
    // Assert Backend.user2 hasn't nft
    // Assert Backend.user3 hasn't nft
}

#[tokio::test]
#[ignore]
async fn transfer_nft_locked_nft() {
    // Setup contract: Backend.root - owner/ backend

    // Backend.root mint nft for Backend.user1 with id = 1
    // Assert Backend.user1 has nft
    // Change state of nft with id = 1 to Lock

    // Backend.user1 transfer nft to Backend.user2
    // Assert error
    // Assert Backend.user1 has nft
    // Assert Backend.user2 hasn't nft
}
