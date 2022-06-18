mod utils;

use crate::utils::*;
use near_sdk::{json_types::U128, serde_json};

#[tokio::test]
async fn transfer_from_not_exist_account() {
    // Setup contract
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Transfer from non exist account
    let bob = get_bob();
    let call_result = bob
        .call(&worker, contract.id(), "transfer")
        .args_json(serde_json::json!({
            "recipient_id": get_alice().id(),
            "amount": U128(ONE_LIS)
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .unwrap_err();

    // Assert error
    assert!(
        call_result
            .to_string()
            .contains("Smart contract panicked: User not found"),
        "Transfer should fail"
    );
}

#[tokio::test]
#[ignore]
async fn transfer_more_than_account_balance() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 1_000 LIS
    // Assert Alice have 2_999_999_000 LIS
    // Assert Bob have 1_000 LIS

    // Bob transfer to Charlie 1_001 LIS
    // Assert error

    // Alice transfer to Charlie 3_000_000_000 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn transfer_zero_amount() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 0 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn transfer() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 1_000 LIS
    // Assert Alice have 2_999_999_000 LIS
    // Assert Bob have 1_000 LIS

    // Alice transfer to Charlie 13 LIS
    // Assert Alice have 2_999_998_987 LIS
    // Assert Charlie have 13 LIS

    // Bob transfer to Charlie 100 LIS
    // Assert Bob have 900 LIS
    // Assert Charlie have 113 LIS

    // Charlie transfer to Dave 1 LIS
    // Assert Charlie have 112 LIS
    // Assert Dave have 1 LIS
    todo!()
}

// TODO: tests with expired lockups
