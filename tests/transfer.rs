mod utils;

use crate::utils::*;
use realis_near::utils::{DAY, SECOND};

#[tokio::test]
async fn transfer_from_not_exist_account() {
    // Setup contract
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Transfer from non exist account
    let bob = get_bob();
    let call_result = make_transfer(&bob, &get_alice().id(), ONE_LIS, &contract, &worker).await;

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
        get_balance_info(&alice, &contract, &worker).await
    );

    // Alice transfer to Bob 1_000 LIS
    make_transfer(&alice, &get_bob().id(), 1_000 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Alice has 2_999_999_000 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_000 * ONE_LIS
    );

    // Assert Bob has 1_000 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        1000 * ONE_LIS
    );

    // Bob transfer to Charlie 1_001 LIS
    let response = make_transfer(&bob, &charlie.id(), 1_001 * ONE_LIS, &contract, &worker).await;

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
        3_000_000_000 * ONE_LIS,
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
        get_balance_info(&alice, &contract, &worker).await
    );

    // Alice transfer to Bob 0 LIS
    let response = make_transfer(&alice, &bob.id(), 0 * ONE_LIS, &contract, &worker).await;

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
async fn transfer() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();
    let dave = get_dave();

    // Setup contract
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1_000 LIS
    make_transfer(&alice, &get_bob().id(), 1000 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Alice has 2_999_999_000 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_000 * ONE_LIS
    );

    // Assert Bob has 1_000 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        1_000 * ONE_LIS
    );

    // Alice transfer to Charlie 13 LIS
    make_transfer(&alice, &charlie.id(), 13 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Alice has 2_999_998_987 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_998_987 * ONE_LIS
    );

    // Assert Charlie has 13 LIS
    assert_eq!(
        get_balance_info(&charlie, &contract, &worker).await,
        13 * ONE_LIS
    );

    // Bob transfer to Charlie 100 LIS
    make_transfer(&bob, &get_charlie().id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 900 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        900 * ONE_LIS
    );

    // Assert Charlie has 113 LIS
    assert_eq!(
        get_balance_info(&charlie, &contract, &worker).await,
        113 * ONE_LIS
    );

    // Charlie transfer to Dave 1 LIS
    make_transfer(&charlie, &get_dave().id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Charlie has 112 LIS
    assert_eq!(
        get_balance_info(&charlie, &contract, &worker).await,
        112 * ONE_LIS
    );

    // Assert Dave has 1 LIS
    assert_eq!(get_balance_info(&dave, &contract, &worker).await, ONE_LIS);
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
    make_transfer(&alice, &bob.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 100 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Wait while lockup is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 100 LIS
    make_transfer(&bob, &dave.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob transfer to Dave 100 LIS
    make_transfer(&bob, &dave.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 0 LIS
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Assert Dave has 100 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        200 * ONE_LIS
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
    make_transfer(&alice, &bob.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 100 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Bob transfer to Dave 100 LIS
    make_transfer(&bob, &dave.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 0 LIS
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Assert Dave has 100 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        100 * ONE_LIS
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
    make_transfer(&alice, &bob.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 100 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Wait while lockup is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 150 LIS
    make_transfer(&bob, &dave.id(), 150 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 50 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Assert Bob has not lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Assert Dave has 150 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        150 * ONE_LIS
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
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 50 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        50 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 2 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 2);

    // Wait while lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 150 LIS
    make_transfer(&bob, &dave.id(), 150 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 1 LIS
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, ONE_LIS);

    // Assert Bob has not lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Assert Dave has 150 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        150 * ONE_LIS
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
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 10 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 20 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        20 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        25 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 DAY, amount - 50 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        50 * ONE_LIS,
        Some(DAY),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 5 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 5);

    // Wait while lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Bob transfer to Dave 50 LIS
    make_transfer(&bob, &dave.id(), 50 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 6 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        6 * ONE_LIS
    );

    // Assert Bob has 2 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 2);

    // Assert Dave has 50 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 SECOND, amount - 25 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        25 * ONE_LIS,
        Some(SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert bob has 3 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 3);

    // Bob transfer to Dave 50 LIS
    let response = make_transfer(&bob, &dave.id(), 50 * ONE_LIS, &contract, &worker).await;

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
        get_balance_info(&bob, &contract, &worker).await,
        6 * ONE_LIS
    );

    // Assert Bob has 3 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 3);

    // Assert Dave has 50 LIS
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        50 * ONE_LIS
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
    make_transfer(&alice, &bob.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob has 100 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Alice create lockup for Bob with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Bob transfer to Dave 150 LIS
    let response = make_transfer(&bob, &dave.id(), 150 * ONE_LIS, &contract, &worker).await;

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
        get_balance_info(&bob, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Assert Bob has lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Assert Dave has 0 LIS
    assert_eq!(get_balance_info(&dave, &contract, &worker).await, 0);
}
