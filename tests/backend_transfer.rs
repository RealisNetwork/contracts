mod utils;

use crate::utils::*;
use near_sdk::{json_types::U128, serde_json};

#[tokio::test]
async fn backend_transfer_from_not_exist_account() {
    // Setup contract: Backend.root - owner/backend
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root())
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

    // Backend.root backend_transfer to Bob 1_000 LIS
    // Assert Backend.root has 2_999_998_900 LIS
    // Assert Beneficiary has 100 LIS
    // Assert Bob has 1_000 LIS

    // Bob backend_transfer to Charlie 1_001 LIS
    // Assert error

    // Backend.root backend_transfer to Charlie 3_000_000_000 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer_zero_amount() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 0 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 1_000 LIS
    // Assert Backend.root has 2_999_998_900 LIS
    // Assert Beneficiary has 100 LIS
    // Assert Bob has 1_000 LIS

    // Backend.root backend_transfer to Charlie 10 LIS
    // Assert Backend.root has 2_999_998_880 LIS
    // Assert Beneficiary has 101 LIS
    // Assert Charlie has 10 LIS

    // Bob backend_transfer to Charlie 100 LIS
    // Assert Bob has 890 LIS
    // Assert Beneficiary has 111 LIS
    // Assert Charlie has 110 LIS

    // Charlie backend_transfer to Dave 1 LIS
    // Assert Charlie has 108.9 LIS
    // Assert ALice has 111.1 LIS
    // Assert Dave has 1 LIS
    todo!()
}

#[tokio::test]
#[ignore]
async fn backend_transfer_with_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Assert Bob has lockup

    // Bob backend_transfer to Dave 100 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has lockup
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_with_not_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Backend.root create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has lockup

    // Bob backend_transfer to Dave 100 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has lockup
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Assert Bob has lockup

    // Bob backend_transfer to Dave 150 LIS
    // Assert Bob has 35 LIS
    // Assert Bob has not lockups
    // Assert Beneficiary has 15 LIS
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_two_expired_lockups() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 50 LIS
    // Assert Bob has 2 lockups

    // Bob backend_transfer to Dave 120 LIS
    // Assert Bob has 18 LIS
    // Assert Bob has not lockups
    // Assert Beneficiary has 12 LIS
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_set_of_lockups() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 10 LIS
    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 20 LIS
    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    // Backend.root create lockup for Bob with duration = 1 DAY, amount - 50 LIS
    // Backend.root create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has 5 lockups

    // Bob backend_transfer to Dave 50 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has 2 lockups
    // Assert Beneficiary has 5 LIS
    // Assert Dave has 50 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    // Assert bob has 3 lockups

    // Bob backend_transfer to Dave 50 LIS
    // Assert error
    // Assert Bob has 0 LIS
    // Assert Bob has 3 lockups
    // Assert Dave has 50 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_get_balance_from_not_expired_lockup() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Backend.root create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has lockup

    // Bob backend_transfer to Dave 150 LIS
    // Assert error
    // Assert Bob has 100 LIS
    // Assert Bob has lockup
    // Assert Dave has 0 LIS
}

#[tokio::test]
#[ignore]
async fn backend_transfer_lockup_cover_fee() {
    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS

    // Backend.root backend_transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Backend.root create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Assert Bob has lockup

    // Bob backend_transfer to Dave 100 LIS
    // Assert Bob has 90 LIS
    // Assert Bob has not lockup
    // Assert Beneficiary has 10 LIS
    // Assert Dave has 10 LIS
}
