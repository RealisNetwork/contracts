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
async fn transfer_more_than_account_balance() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 1_000 LIS
    // Assert Alice has 2_999_999_000 LIS
    // Assert Bob has 1_000 LIS

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
    // Assert Alice has 2_999_999_000 LIS
    // Assert Bob has 1_000 LIS

    // Alice transfer to Charlie 13 LIS
    // Assert Alice has 2_999_998_987 LIS
    // Assert Charlie has 13 LIS

    // Bob transfer to Charlie 100 LIS
    // Assert Bob has 900 LIS
    // Assert Charlie has 113 LIS

    // Charlie transfer to Dave 1 LIS
    // Assert Charlie have 112 LIS
    // Assert Dave has 1 LIS
    todo!()
}

#[tokio::test]
#[ignore]
async fn transfer_with_expired_lockup() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Assert Bob has lockup

    // Bob transfer to Dave 100 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has lockup
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn transfer_with_not_expired_lockup() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has lockup

    // Bob transfer to Dave 100 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has lockup
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn transfer_get_balance_from_expired_lockup() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Assert Bob has lockup

    // Wait while lockup is expired
    // Bob transfer to Dave 150 LIS
    // Assert Bob has 50 LIS
    // Assert Bob has not lockups
    // Assert Dave has 100 LIS
}

#[tokio::test]
#[ignore]
async fn transfer_get_balance_from_two_expired_lockups() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    // Alice create lockup for Bob with duration = 1 SECOND, amount - 50 LIS
    // Assert Bob has 2 lockups

    // Wait while lockups is expired
    // Bob transfer to Dave 150 LIS
    // Assert Bob has 0 LIS
    // Assert Bob has not lockups
    // Assert Dave has 150 LIS
}

#[tokio::test]
#[ignore]
async fn transfer_get_balance_from_set_of_lockups() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 10 LIS
    // Alice create lockup for Bob with duration = 1 SECOND, amount - 20 LIS
    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    // Alice create lockup for Bob with duration = 1 DAY, amount - 50 LIS
    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has 5 lockups

    // Wait while lockups is expired
    // Bob transfer to Dave 50 LIS
    // Assert Bob has 5 LIS
    // Assert Bob has 2 lockups
    // Assert Bob has 5 LIS
    // Assert Dave has 50 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    // Assert bob has 3 lockups

    // Bob transfer to Dave 50 LIS
    // Assert error
    // Assert Bob has 5 LIS
    // Assert Bob has 3 lockups
    // Assert Dave has 50 LIS
}

#[tokio::test]
#[ignore]
async fn transfer_get_balance_from_not_expired_lockup() {
    // Setup contract: ALice - owner, total_supply - 3_000_000_000 LIS

    // Alice transfer to Bob 100 LIS
    // Assert Bob has 100 LIS

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    // Assert Bob has lockup

    // Bob transfer to Dave 150 LIS
    // Assert error
    // Assert Bob has 100 LIS
    // Assert Bob has lockup
    // Assert Dave has 0 LIS
}
