mod utils;

use crate::utils::*;
use near_sdk::{json_types::U128, serde_json};

#[tokio::test]
async fn backend_transfer_from_not_exist_account() {
    // Setup contract: Backend.root - owner/backend
    let (contract, worker) = TestingEnvBuilder::default()
        .set_owner(BackendAccount::get_root().id().clone())
        .set_backend(BackendAccount::get_root().id().clone())
        .build()
        .await;

    // backend_transfer from non exist account
    let user1 = BackendAccount::get_user1();
    let call_result = user1
        .call(&worker, contract.id(), "backend_transfer")
        .args_json(serde_json::json!({
            "recipient_id": get_alice().id(),
            "amount": U128(ONE_LIS)
        }))
        .expect("Invalid input args")
        .transact()
        .await;

    // Assert error
    assert!(
        call_result
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: User not found"),
        "Transfer should fail"
    );
}

#[tokio::test]
#[ignore]
async fn backend_transfer_more_than_account_balance() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 1_000 LIS
    // Assert Backend.root has 2_999_998_900 LIS
    // Assert Beneficiary has 100 LIS
    // Assert Backend.user2 has 1_000 LIS

    // Backend.user2 backend_transfer to Backend.user3 1_001 LIS
    // Assert error

    // Backend.root backend_transfer to Backend.user3 3_000_000_000 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer_zero_amount() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 0 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 1_000 LIS
    // Assert Backend.root has 2_999_998_900 LIS
    // Assert Beneficiary has 100 LIS
    // Assert Backend.user2 has 1_000 LIS

    // Backend.root backend_transfer to Backend.user3 10 LIS
    // Assert Backend.root has 2_999_998_880 LIS
    // Assert Beneficiary has 101 LIS
    // Assert Backend.user3 has 10 LIS

    // Backend.user2 backend_transfer to Backend.user3 100 LIS
    // Assert Backend.user2 has 890 LIS
    // Assert Beneficiary has 111 LIS
    // Assert Backend.user3 has 110 LIS

    // Backend.user3 backend_transfer to Backend.user4 1 LIS
    // Assert Backend.user3 has 108.9 LIS
    // Assert ALice has 111.1 LIS
    // Assert Backend.user4 has 1 LIS
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer_with_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 100 LIS
    // Assert Backend.user2 has 100 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    // Assert Backend.user2 has lockup

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    // Assert Backend.user2 has 0 LIS
    // Assert Backend.user2 has lockup
    // Assert Backend.user4 has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_with_not_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 100 LIS
    // Assert Backend.user2 has 100 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    // Assert Backend.user2 has lockup

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    // Assert Backend.user2 has 0 LIS
    // Assert Backend.user2 has lockup
    // Assert Backend.user4 has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 100 LIS
    // Assert Backend.user2 has 100 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    // Assert Backend.user2 has lockup

    // Backend.user2 backend_transfer to Backend.user4 150 LIS
    // Assert Backend.user2 has 35 LIS
    // Assert Backend.user2 has not lockups
    // Assert Beneficiary has 15 LIS
    // Assert Backend.user4 has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_two_expired_lockups() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 50 LIS
    // Assert Backend.user2 has 2 lockups

    // Backend.user2 backend_transfer to Backend.user4 120 LIS
    // Assert Backend.user2 has 18 LIS
    // Assert Backend.user2 has not lockups
    // Assert Beneficiary has 12 LIS
    // Assert Backend.user4 has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_set_of_lockups() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 10 LIS
    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 20 LIS
    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 25 LIS
    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 50 LIS
    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    // Assert Backend.user2 has 5 lockups

    // Backend.user2 backend_transfer to Backend.user4 50 LIS
    // Assert Backend.user2 has 0 LIS
    // Assert Backend.user2 has 2 lockups
    // Assert Beneficiary has 5 LIS
    // Assert Backend.user4 has 50 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 25 LIS
    // Assert Backend.user2 has 3 lockups

    // Backend.user2 backend_transfer to Backend.user4 50 LIS
    // Assert error
    // Assert Backend.user2 has 0 LIS
    // Assert Backend.user2 has 3 lockups
    // Assert Backend.user4 has 50 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_not_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 100 LIS
    // Assert Backend.user2 has 100 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    // Assert Backend.user2 has lockup

    // Backend.user2 backend_transfer to Backend.user4 150 LIS
    // Assert error
    // Assert Backend.user2 has 100 LIS
    // Assert Backend.user2 has lockup
    // Assert Backend.user4 has 0 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_lockup_cover_fee() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Backend.user2 100 LIS
    // Assert Backend.user2 has 100 LIS

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    // Assert Backend.user2 has lockup

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    // Assert Backend.user2 has 90 LIS
    // Assert Backend.user2 has not lockup
    // Assert Beneficiary has 10 LIS
    // Assert Backend.user4 has 10 LIS
}
