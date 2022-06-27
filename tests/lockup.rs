use crate::utils::*;
use near_sdk::json_types::{U128, U64};
use realis_near::utils::{DAY, MINUTE, ONE_LIS, SECOND};

#[tokio::test]
async fn create_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob and Charlie 1 LIS to create accounts Bob and Charlie
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");
    make_transfer(&alice, &charlie.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with amount - 3_000 LIS
    let bob_lockup_ts1 =
        create_lockup_for_account(&alice, &bob.id(), 3000 * ONE_LIS, None, &contract, &worker)
            .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount
    assert_eq!(bobs_lockups[0].amount.0, 3000 * ONE_LIS);

    // Assert timestamp == default
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_996_998 * ONE_LIS
    );

    // Alice create lockup for Charlie with amount - 150 LIS
    let charlie_lockup_ts = create_lockup_for_account(
        &alice,
        &charlie.id(),
        150 * ONE_LIS,
        None,
        &contract,
        &worker,
    )
    .await;

    // Assert Charlie has lockup
    let charlies_lockups = get_lockup_info(&charlie, &contract, &worker).await;
    assert_eq!(charlies_lockups.len(), 1);

    // Assert amount
    assert_eq!(charlies_lockups[0].amount.0, 150 * ONE_LIS);

    // Assert timestamp == default
    assert_eq!(charlies_lockups[0].expire_on.0, charlie_lockup_ts);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_996_848 * ONE_LIS
    );

    // Alice create lockup for Bob with amount - 300 LIS
    let bob_lockup_ts2 =
        create_lockup_for_account(&alice, &bob.id(), 300 * ONE_LIS, None, &contract, &worker).await;

    // Assert Bob has 2 lockups
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 2);

    // Assert amounts
    assert_eq!(bobs_lockups[0].amount.0, 3000 * ONE_LIS);
    assert_eq!(bobs_lockups[1].amount.0, 300 * ONE_LIS);

    // Assert timestamp == default
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);
    assert_eq!(bobs_lockups[1].expire_on.0, bob_lockup_ts2);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_996_548 * ONE_LIS
    );
}

#[tokio::test]
async fn create_lockup_with_duration() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount = 10 LIS
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration = 1 DAY
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );
}

#[tokio::test]
async fn user_claim_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 10 SECONDS, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(10 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount = 10 LIS
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration = 10 SECONDS
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Wait while lockup time is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

    // Bob claim lockup
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob's balance = 11 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        11 * ONE_LIS
    );

    let mut bob_timestamps = vec![];

    // Alice create lockup for Bob with duration = 5 SECONDS, amount = 10 LIS
    bob_timestamps.push(
        create_lockup_for_account(
            &alice,
            &bob.id(),
            10 * ONE_LIS,
            Some(U64(5 * SECOND)),
            &contract,
            &worker,
        )
        .await,
    );

    // Alice create lockup for Bob with duration = 10 SECONDS, amount = 15 LIS
    bob_timestamps.push(
        create_lockup_for_account(
            &alice,
            &bob.id(),
            15 * ONE_LIS,
            Some(U64(10 * SECOND)),
            &contract,
            &worker,
        )
        .await,
    );

    // Alice create lockup for Bob with duration = 15 SECONDS, amount = 15 LIS
    bob_timestamps.push(
        create_lockup_for_account(
            &alice,
            &bob.id(),
            15 * ONE_LIS,
            Some(U64(15 * SECOND)),
            &contract,
            &worker,
        )
        .await,
    );

    // Assert Bob has 3 lockups
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 3);

    // Assert amounts
    assert_eq!(
        bobs_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        40 * ONE_LIS
    );

    // Assert durations
    bobs_lockups
        .iter()
        .zip(bob_timestamps.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_949 * ONE_LIS
    );

    // Wait while all lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    // Bob claim 15 LIS
    claim_lockup_for_account(&bob, &contract, &worker, U128(15 * ONE_LIS)).await;

    // Assert Bob has 2 lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 2);

    // Assert amounts
    assert_eq!(
        bobs_lockups
            .into_iter()
            .fold(0u128, |acc, x| acc + x.amount.0),
        25 * ONE_LIS
    );

    // Assert balance
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        26 * ONE_LIS
    );
}

#[tokio::test]
#[should_panic(expected = "Cannon get result: Action #0: \
    ExecutionError(\"Smart contract panicked: Only owner can do this\")")]
async fn not_contract_owner_create_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob create lockup
    create_lockup_for_account(
        &bob,
        &bob.id(),
        15 * ONE_LIS,
        Some(U64(15 * SECOND)),
        &contract,
        &worker,
    )
    .await;
}

#[tokio::test]
#[should_panic(expected = "Cannon get result: Action #0: \
    ExecutionError(\"Smart contract panicked: Not enough balance\")")]
async fn not_enough_balance_to_create_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with amount - 3_000_000_001 LIS
    create_lockup_for_account(
        &alice,
        &bob.id(),
        3_000_000_001 * ONE_LIS,
        Some(U64(15 * SECOND)),
        &contract,
        &worker,
    )
    .await;
}

#[tokio::test]
async fn user_claimed_not_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    let bob_lockup_ts = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount = 10 LIS
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration = 1 DAY
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Bob claim lockup
    let claimed = claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert that we claimed nothing
    assert_eq!(claimed, 0);
}

#[tokio::test]
async fn claim_expired_lockup_when_other_not_expired() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 SECOND, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    let bob_lockup_ts2 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has 2 lockups
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 2);

    // Assert amount
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);
    assert_eq!(bobs_lockups[1].amount.0, 10 * ONE_LIS);

    // Assert duration
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);
    assert_eq!(bobs_lockups[1].expire_on.0, bob_lockup_ts2);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_979 * ONE_LIS
    );

    // Wait 1 SECOND
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Bob claim lockup
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob has 1 lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Assert Bob balance
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        11 * ONE_LIS
    );
}

#[tokio::test]
async fn refund_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    let bob_lockup_ts1 =
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Bob refund lockup
    refund_lockup_for_account(&alice, &contract, &worker, &bob.id(), bob_lockup_ts1).await;

    // Assert Bob doesn't have lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Assert Bob balance
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, ONE_LIS);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_999 * ONE_LIS
    );

    let mut bob_lockup_ts = vec![];

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    bob_lockup_ts.push(
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await,
    );

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    bob_lockup_ts.push(
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await,
    );

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    bob_lockup_ts.push(
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await,
    );

    // Assert Bob has 3 lockups
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 3);

    // Assert amounts
    assert_eq!(
        bobs_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        30 * ONE_LIS
    );

    // Assert durations
    bobs_lockups
        .iter()
        .zip(bob_lockup_ts.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_969 * ONE_LIS
    );

    // Bob refund 1 lockup
    refund_lockup_for_account(
        &alice,
        &contract,
        &worker,
        &bob.id(),
        bob_lockup_ts.pop().unwrap(),
    )
    .await;

    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;

    // Assert Bob has 2 lockup
    assert_eq!(bobs_lockups.len(), 2);

    // Assert amount
    assert_eq!(
        bobs_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        20 * ONE_LIS
    );

    // Assert duration
    bobs_lockups
        .iter()
        .zip(bob_lockup_ts.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Bob balance
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, ONE_LIS);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_979 * ONE_LIS
    );
}

#[tokio::test]
#[should_panic(expected = "Cannon get result: Action #0: \
ExecutionError(\"Smart contract panicked: Only owner can do this\")")]
async fn refund_not_own_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    let bob_lockup_ts1 =
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await;

    // Alice tries to refund Bob's lockup
    refund_lockup_for_account(&bob, &contract, &worker, &bob.id(), bob_lockup_ts1).await;
}

#[tokio::test]
#[should_panic(expected = "Cannon get result: Action #0: \
    ExecutionError(\"Smart contract panicked: No such lockup\")")]
async fn refund_non_existed_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob refund lockup
    refund_lockup_for_account(&alice, &contract, &worker, &bob.id(), 25 * SECOND).await;
}

#[tokio::test]
async fn claim_all_lockups() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create 10 lockups for Bob with duration 1 min, amount - 10 LIS

    let mut timestamps = create_n_lockups_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(MINUTE)),
        10,
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 10);

    // Assert amount
    assert_eq!(
        bobs_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        100 * ONE_LIS
    );

    // Assert duration
    bobs_lockups
        .iter()
        .zip(timestamps.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_899 * ONE_LIS
    );

    // Wait while lockups time is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    // Bob claim all lockups
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob balance = 100 LIS.
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        101 * ONE_LIS
    );

    timestamps.clear();

    // Alice create 10 lockups for Bob with duration 1 min,
    // amount for 5 = 10 LIS, the others = 20 LIS
    timestamps.append(
        create_n_lockups_for_account(
            &alice,
            &bob.id(),
            10 * ONE_LIS,
            Some(U64(MINUTE)),
            5,
            &contract,
            &worker,
        )
        .await
        .as_mut(),
    );

    timestamps.append(
        create_n_lockups_for_account(
            &alice,
            &bob.id(),
            20 * ONE_LIS,
            Some(U64(MINUTE)),
            5,
            &contract,
            &worker,
        )
        .await
        .as_mut(),
    );

    // Assert Bob has lockups
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 10);

    // Assert amount
    assert_eq!(
        bobs_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        150 * ONE_LIS
    );

    // Assert duration
    bobs_lockups
        .iter()
        .zip(timestamps.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_749 * ONE_LIS
    );

    // Wait while lockups time is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    // Bob claim all lockups with amount = 20 LIS
    for lockup in bobs_lockups {
        if lockup.amount.0 == 20 * ONE_LIS {
            claim_lockup_for_account(&bob, &contract, &worker, lockup.amount).await;
        }
    }

    // Assert Bob balance = 200 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        201 * ONE_LIS
    );

    // Assert Alice balance = 2_999_999_749 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_749 * ONE_LIS
    );
}

#[tokio::test]
async fn claim_all_lockups_without_lockups() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Assert Bob hasn't lockup
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Bob claim all lockups without error
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob balance = 1 LIS.
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, ONE_LIS);
}

#[tokio::test]
async fn claim_all_lockups_with_one_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration 10 seconds, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(U64(10 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration
    assert_eq!(bobs_lockups[0].expire_on.0, bob_lockup_ts1);

    // Assert Alice balance = 2_999_999_989 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Wait till lockup expired
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Bob claim all lockups
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob hasn't lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 0);

    // Assert Bob balance = 10 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        11 * ONE_LIS
    );
}

#[tokio::test]
async fn claim_all_lockups_with_non_expired_time() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockups for Bob with duration default, amount = 10 LIS
    let timestamp =
        create_lockup_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, &contract, &worker).await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount
    assert_eq!(bobs_lockups[0].amount.0, 10 * ONE_LIS);

    // Assert duration
    assert_eq!(bobs_lockups[0].expire_on.0, timestamp);

    // Assert Alice balance = 2_999_999_989 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Bob claim all lockups
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob has lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 1);

    // Assert Bob balance = 1 LIS
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, ONE_LIS);
}

#[tokio::test]
async fn claim_all_lockups_with_partially_expired_time() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob 1 LIS to create accounts Bob
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    let mut timestamps = vec![];

    // Alice create lockups for Bob:
    // 6 with duration 10 seconds, amount = 10 LIS
    timestamps.append(
        create_n_lockups_for_account(
            &alice,
            &bob.id(),
            10 * ONE_LIS,
            Some(U64(10 * SECOND)),
            6,
            &contract,
            &worker,
        )
        .await
        .as_mut(),
    );

    // 5 with duration default, amount = 10 LIS
    timestamps.append(
        create_n_lockups_for_account(&alice, &bob.id(), 10 * ONE_LIS, None, 5, &contract, &worker)
            .await
            .as_mut(),
    );

    // Assert Bob has lockup
    let bob_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bob_lockups.len(), 11);

    // Assert amount
    assert_eq!(
        bob_lockups.iter().fold(0u128, |acc, x| acc + x.amount.0),
        110 * ONE_LIS
    );

    // Assert duration
    bob_lockups
        .iter()
        .zip(timestamps.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on.0, timestamp);
        });

    // Assert Alice balance = 2_999_999_900 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_889 * ONE_LIS
    );

    // Wait till lockups expired
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Bob claim all lockups
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob has 5 lockups
    assert_eq!(get_lockup_info(&bob, &contract, &worker).await.len(), 5);

    // Assert Bob balance = 50 LIS
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        61 * ONE_LIS
    );
}
