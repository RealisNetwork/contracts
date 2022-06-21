mod utils;

use crate::utils::*;
use realis_near::{
    lockup::Lockup,
    utils::{DAY, SECOND},
};

#[tokio::test]
async fn create_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let charlie = get_charlie();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Alice transfer to Bob and Charlie 1 LIS to create accounts Bob and Charlie
    make_transfer(&alice, &bob.id(), 1 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");
    make_transfer(&alice, &charlie.id(), 1 * ONE_LIS, &contract, &worker)
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
    assert_eq!(bobs_lockups.first().unwrap().amount, U128(3000 * ONE_LIS));

    // Assert timestamp == default
    assert_eq!(bobs_lockups.first().unwrap().expire_on, bob_lockup_ts1);

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
    assert_eq!(
        charlies_lockups.first().unwrap().amount,
        U128(150 * ONE_LIS)
    );

    // Assert timestamp == default
    assert_eq!(
        charlies_lockups.first().unwrap().expire_on,
        charlie_lockup_ts
    );

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
    assert_eq!(bobs_lockups.first().unwrap().amount, U128(3000 * ONE_LIS));
    assert_eq!(bobs_lockups.last().unwrap().amount, U128(300 * ONE_LIS));

    // Assert timestamp == default
    assert_eq!(bobs_lockups.first().unwrap().expire_on, bob_lockup_ts1);
    assert_eq!(bobs_lockups.last().unwrap().expire_on, bob_lockup_ts2);

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
    make_transfer(&alice, &bob.id(), 1 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(1 * DAY),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount = 10 LIS
    assert_eq!(bobs_lockups.first().unwrap().amount, U128(10 * ONE_LIS));

    // Assert duration = 1 DAY
    assert_eq!(bobs_lockups.first().unwrap().expire_on, bob_lockup_ts1);

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
    make_transfer(&alice, &bob.id(), 1 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Alice create lockup for Bob with duration = 10 SECONDS, amount = 10 LIS
    let bob_lockup_ts1 = create_lockup_for_account(
        &alice,
        &bob.id(),
        10 * ONE_LIS,
        Some(10 * SECOND),
        &contract,
        &worker,
    )
    .await;

    // Assert Bob has lockup
    let bobs_lockups = get_lockup_info(&bob, &contract, &worker).await;
    assert_eq!(bobs_lockups.len(), 1);

    // Assert amount = 10 LIS
    assert_eq!(bobs_lockups.first().unwrap().amount, U128(10 * ONE_LIS));

    // Assert duration = 10 SECONDS
    assert_eq!(bobs_lockups.first().unwrap().expire_on, bob_lockup_ts1);

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_989 * ONE_LIS
    );

    // Wait while lockup time is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(11)).await;

    // Bob claim lockup
    claim_lockup_for_account(&bob, &contract, &worker, bob_lockup_ts1).await;

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
            Some(5 * SECOND),
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
            Some(10 * SECOND),
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
            Some(15 * SECOND),
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
        bobs_lockups
            .clone()
            .into_iter()
            .fold(0u128, |acc, x| acc + x.amount.0),
        40 * ONE_LIS
    );

    // Assert durations
    bobs_lockups
        .iter()
        .zip(bob_timestamps.clone().iter())
        .for_each(|(lockup, timestamp)| {
            assert_eq!(&lockup.expire_on, timestamp);
        });

    // Assert Alice balance
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        2_999_999_949 * ONE_LIS
    );

    // Wait while all lockups is expired
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

    // Bob claim 15 LIS
    claim_lockup_for_account(&bob, &contract, &worker, *bob_timestamps.get(2).unwrap()).await;

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
#[ignore]
async fn not_contract_owner_create_lockup() {
    // Setup contract: Alice - owner

    // Bob create lockup
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn not_enough_balance_to_create_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with amount - 3_000_000_001 LIS
    // Assert error
    todo!()
}

#[tokio::test]
#[ignore]
async fn user_claimed_not_expired_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount = 10 LIS
    // Assert duration = 1 DAY
    // Assert Alice balance

    // Bob claim lockup
    // Assert error
}

#[tokio::test]
#[ignore]
async fn claim_expired_lockup_when_other_not_expired() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount = 10 LIS
    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    // Assert Bob has 2 lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait 1 SECOND
    // Bob claim lockup

    // Assert Bob has 1 lockup
    // Assert Bob balance
}

#[tokio::test]
#[ignore]
async fn refund_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Bob refund lockup
    // Assert Bob doesn't have lockup
    // Assert Bob balance
    // Assert Alice balance

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Assert Bob has 3 lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Bob refund 1 lockup
    // Assert Bob has 2 lockup
    // Assert amount
    // Assert duration
    // Assert Bob balance
    // Assert Alice balance
}

#[tokio::test]
#[ignore]
async fn refund_not_own_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
}

#[tokio::test]
#[ignore]
async fn refund_non_existed_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Bob refund lockup
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create 10 lockups for Bob with duration 1 min, amount - 10 LIS
    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait while lockups time is expired
    // Bob claim all lockups
    // Assert Bob balance = 100 LIS.

    // Alice create 10 lockups for Bob with duration 1 min,
    // amount for 5 = 10 LIS, the others = 20 LIS
    // Assert Bob has lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait while lockups time is expired
    // Bob claim all lockups with amount = 20 LIS
    // Assert Bob balance = 200 LIS

    // Assert Alice balance = 2_999_999_600 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_without_lockups() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Assert Bob hasn't lockup
    // Bob claim all lockups without error
    // Assert Bob balance = 0 LIS.
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_one_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration 10 seconds, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_990 LIS

    // Bob claim all lockups
    // Assert Bob hasn't lockups
    // Assert Bob balance = 10 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_non_expired_time() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockups for Bob with duration default, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_900 LIS

    // Bob claim all lockups
    // Assert Bob has lockups
    // Assert Bob balance = 0 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_partially_expired_time() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockups for Bob:
    // 5 with duration 10 seconds, amount = 10 LIS
    // 5 with duration default, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_900 LIS

    // Bob claim all lockups
    // Assert Bob has 5 lockups
    // Assert Bob balance = 50 LIS
}
