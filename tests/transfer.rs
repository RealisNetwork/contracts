mod utils;

use crate::utils::*;
use near_sdk::json_types::U128;
use realis_near::utils::{DAY, SECOND};

#[tokio::test]
async fn transfer_from_not_exist_account() {
    // Setup contract
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Transfer from non exist account
    let bob = get_bob();
    let call_result =
        make_transfer(&bob, &get_alice().id(), U128(ONE_LIS), &contract, &worker).await;

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
async fn transfer_more_than_account_balance() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice's balance before the transaction
    println!(
        "Result Alice before the transaction: {:#?}",
        balance_info(&alice, &contract, &worker).await
    );

    // Alice transfer to Bob 1_000 LIS
    make_transfer(
        &alice,
        &get_bob().id(),
        U128(1_000 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert Alice has 2_999_999_000 LIS
    assert_eq!(
        balance_info(&alice, &contract, &worker).await,
        2_999_999_000u128 * ONE_LIS
    );

    // Assert Bob has 1_000 LIS11
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        1000u128 * ONE_LIS
    );

    // Bob transfer to Charlie 1_001 LIS
    let response = make_transfer(
        &bob,
        &charlie.id(),
        U128(1_001 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(
        response
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: Not enough balance"),
        "Transfer should fail"
    );

    // Alice transfer to Charlie 3_000_000_000 LIS
    let response = make_transfer(
        &alice,
        &charlie.id(),
        U128(3_000_000_000 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(
        response
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: Not enough balance"),
        "Transfer should fail"
    );
}

#[tokio::test]
async fn transfer_zero_amount() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;
    // Alice's balance before the transaction
    println!(
        "Result Alice before the transaction: {:#?}",
        balance_info(&alice, &contract, &worker).await
    );

    // Alice transfer to Bob 0 LIS
    let response = make_transfer(&alice, &bob.id(), U128(0 * ONE_LIS), &contract, &worker).await;

    // Assert error
    assert!(
        response
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: You can't transfer 0 tokens"),
        "Transfer should fail"
    );
}

#[tokio::test]
async fn transfer_integration_test() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();
    let dave = get_dave();

    // Setup contract
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice's balance before the transaction
    println!(
        "Result Alice before the transaction: {:#?}",
        balance_info(&alice, &contract, &worker).await
    );

    // Alice transfer to Bob 1_000 LIS
    make_transfer(
        &alice,
        &get_bob().id(),
        U128(1000 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert Alice has 2_999_999_000 LIS
    assert_eq!(
        balance_info(&alice, &contract, &worker).await,
        2_999_999_000u128 * ONE_LIS
    );

    // Assert Bob has 1_000 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        1_000u128 * ONE_LIS
    );

    // Alice transfer to Charlie 13 LIS
    make_transfer(
        &alice,
        &charlie.id(),
        U128(13 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert Alice has 2_999_998_987 LIS
    assert_eq!(
        balance_info(&alice, &contract, &worker).await,
        2_999_998_987u128 * ONE_LIS
    );

    // Assert Charlie has 13 LIS
    assert_eq!(
        balance_info(&charlie, &contract, &worker).await,
        13u128 * ONE_LIS
    );

    // Bob transfer to Charlie 100 LIS
    make_transfer(
        &bob,
        &get_charlie().id(),
        U128(100 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 900 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        900u128 * ONE_LIS
    );

    // Assert Charlie has 113 LIS
    assert_eq!(
        balance_info(&charlie, &contract, &worker).await,
        113u128 * ONE_LIS
    );

    // Charlie transfer to Dave 1 LIS
    make_transfer(
        &charlie,
        &get_dave().id(),
        U128(1 * ONE_LIS),
        &contract,
        &worker,
    )
    .await;

    // Assert Charlie has 112 LIS
    assert_eq!(
        balance_info(&charlie, &contract, &worker).await,
        112u128 * ONE_LIS
    );

    // Assert Dave has 1 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        1u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_with_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 100 LIS
    make_transfer(&alice, &bob.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 100 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        100u128 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Bob transfer to Dave 100 LIS
    make_transfer(&bob, &dave.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 0 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        0u128 * ONE_LIS
    );

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 100 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        100u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_with_not_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 100 LIS
    make_transfer(&alice, &bob.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 100 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        100u128 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Bob transfer to Dave 100 LIS
    make_transfer(&bob, &dave.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 0 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        0u128 * ONE_LIS
    );

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 100 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        100u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_get_balance_from_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 100 LIS
    make_transfer(&alice, &bob.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 100 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        100u128 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Wait while lockup is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 150 LIS
    make_transfer(&bob, &dave.id(), U128(150 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 50 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        50u128 * ONE_LIS
    );

    // Assert Bob has not lockups
    assert_eq!(
        0,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 150 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        150u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_get_balance_from_two_expired_lockups() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer 1 LIS to Bob
    make_transfer(&alice, &bob.id(), U128(1 * ONE_LIS), &contract, &worker).await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 50 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(50 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 2 lockups
    assert_eq!(
        2,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Wait while lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 150 LIS
    make_transfer(&bob, &dave.id(), U128(150 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 1 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        1u128 * ONE_LIS
    );

    // Assert Bob has not lockups
    assert_eq!(
        0,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 150 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        150u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_get_balance_from_set_of_lockups() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer 1 LIS to Bob
    make_transfer(&alice, &bob.id(), U128(1 * ONE_LIS), &contract, &worker).await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 10 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(10 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 20 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(20 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(25 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 DAY, amount - 50 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(50 * ONE_LIS),
        Some(1 * DAY),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 5 lockups
    assert_eq!(
        5,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Wait while lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 50 LIS
    make_transfer(&bob, &dave.id(), U128(50 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 6 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        6u128 * ONE_LIS
    );

    // Assert Bob has 2 lockups
    assert_eq!(
        2,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 50 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        50u128 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(25 * ONE_LIS),
        Some(1 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert bob has 3 lockups
    assert_eq!(
        3,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Bob transfer to Dave 50 LIS
    let response = make_transfer(&bob, &dave.id(), U128(50 * ONE_LIS), &contract, &worker).await;

    // Assert error
    assert!(
        response
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: Not enough balance"),
        "Transfer should fail"
    );

    // Assert Bob has 6 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        6u128 * ONE_LIS
    );

    // Assert Bob has 3 lockups
    assert_eq!(
        3,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 50 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        50u128 * ONE_LIS
    );
}

#[tokio::test]
async fn transfer_get_balance_from_not_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 100 LIS
    make_transfer(&alice, &bob.id(), U128(100 * ONE_LIS), &contract, &worker).await;

    // Assert Bob has 100 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        100u128 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    make_lockup(
        &alice,
        &bob.id(),
        U128(100 * ONE_LIS),
        Some(1 * DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Bob transfer to Dave 150 LIS
    let response = make_transfer(&bob, &dave.id(), U128(150 * ONE_LIS), &contract, &worker).await;

    // Assert error
    assert!(
        response
            .unwrap_err()
            .to_string()
            .contains("Smart contract panicked: Not enough balance"),
        "Transfer should fail"
    );

    // Assert Bob has 100 LIS
    assert_eq!(
        balance_info(&bob, &contract, &worker).await,
        100u128 * ONE_LIS
    );

    // Assert Bob has lockup
    assert_eq!(
        1,
        lockup_info(&bob, &None, &None, &contract, &worker)
            .await
            .len()
    );

    // Assert Dave has 0 LIS
    assert_eq!(
        balance_info(&dave, &contract, &worker).await,
        0u128 * ONE_LIS
    );
}
