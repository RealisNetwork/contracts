#[tokio::test]
#[ignore]
async fn backend_burn_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1
    // Assert Backend.user1 has 1 nft

    // Backend.user1 burn nft with id = 1
    // Assert Backend.user1 hasn't nft

    // Backend.root mint nft for Backend.user1 with id = 1,2,3,4,5
    // Assert Backend.user1 has 5 nfts

    // Backend.user1 burn nfts with id = 3,5
    // Assert Backend.user1 has 3 nft
    // Assert Backend.user1 has nfts with id = 1,2,4
}

#[tokio::test]
#[ignore]
async fn burn_nft_non_existed_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1,2,3
    // Assert Backend.user1 has 3 nft

    // Backend.user1 burn nft with id = 5
    // Assert error
    // Assert Backend.user1 has 3 nft
}

#[tokio::test]
#[ignore]
async fn burn_nft_not_own_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1 with id = 1,2,3
    // Backend.root mint nft for Backend.user2 with id = 5,6,7
    // Assert Backend.user1 has 3 nft
    // Assert Backend.user2 has 3 nft

    // Backend.user1 burn nft with id = 5
    // Assert error
    // Backend.user1 burn nft with id = 7
    // Assert error
    // Backend.user2 burn nft with id = 1
    // Assert error
    // Backend.user2 burn nft with id = 2
    // Assert error
    // Assert Backend.user1 has 3 nft
    // Assert Backend.user2 has 3 nft
}

#[tokio::test]
#[ignore]
async fn burn_nft_locked_nft() {
    // Setup contract: Backend.root - owner/backend

    // Backend.root mint nft for Backend.user1
    // Assert Backend.user1 has nft
    // Backend.user1 change state of nft to locked
    // Assert state of nft

    // Backend.user1 burn nft
    // Assert error
}
