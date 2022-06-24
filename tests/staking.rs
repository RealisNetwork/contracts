mod utils;

use crate::utils::*;
use near_sdk::json_types::U64;
use realis_near::utils::{DAY, SECOND};

#[tokio::test]
async fn regular_staking_test() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 250 LiS
    make_transfer(&alice, &bob.id(), 250 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // register Dave with 150 LiS
    make_transfer(&alice, &dave.id(), 150 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // stake as Bob 100 LiS
    let mut bob_staked_x = make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    // stake as Bob 100 LiS
    bob_staked_x += make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    println!("{}", bob_staked_x);

    // Assert Bob tokens was taken, 50 Lis reminds
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Airdrop 100 LIS
    make_add_to_pool(&alice, 200 * ONE_LIS, &contract, &worker).await;

    // stake as Dave  100 LiS
    let dave_staked_x = make_stake(&dave, 100 * ONE_LIS, &contract, &worker).await;

    // Assert Dave's tokens was taken
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Set default staking lockup time as 10 seconds
    set_def_staking_lockup_time(&alice, 10 * SECOND, &contract, &worker).await;

    // Bob unstake
    make_unstake(&bob, bob_staked_x, &contract, &worker).await;

    // Dave unstake
    make_unstake(&dave, dave_staked_x, &contract, &worker).await;

    // Wait till lockups are expired
    tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;

    // claim loockup for staiking for Dave
    claim_all_lockup_for_account(&dave, &contract, &worker).await;

    // Claim lockups for Bob
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob's balance == 450
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        450 * ONE_LIS
    );

    // Assert Dave`s balance == 150
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        150 * ONE_LIS
    );
}

//TODO fix me
#[tokio::test]
async fn regular_staking_test_decimal_course() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();
    let dave = get_dave();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 250 LiS
    make_transfer(&alice, &bob.id(), 250 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // register Dave with 150 LiS
    make_transfer(&alice, &dave.id(), 150 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // stake as Bob 100 LiS
    let mut bob_staked_x = make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    // stake as Bob 100 LiS
    bob_staked_x += make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    println!("{}", bob_staked_x);

    // Assert Bob tokens was taken, 50 Lis reminds
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Airdrop 101 LIS
    make_add_to_pool(&alice, 101 * ONE_LIS, &contract, &worker).await;

    // stake as Dave  100 LiS
    let dave_staked_x = make_stake(&dave, 101 * ONE_LIS, &contract, &worker).await;

    // Assert Dave's tokens was taken
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        49 * ONE_LIS
    );

    // Set default staking lockup time as 10 seconds
    set_def_staking_lockup_time(&alice, 10 * SECOND, &contract, &worker).await;

    // Bob unstake
    make_unstake(&bob, bob_staked_x, &contract, &worker).await;

    // Dave unstake
    make_unstake(&dave, dave_staked_x, &contract, &worker).await;

    // Wait till lockups are expired
    tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;

    // claim loockup for staiking for Dave
    claim_all_lockup_for_account(&dave, &contract, &worker).await;

    // Claim lockups for Bob
    claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert Bob's balance == 450
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        351 * ONE_LIS
    );

    // Assert Dave`s balance == 150
    assert_eq!(
        get_balance_info(&dave, &contract, &worker).await,
        150 * ONE_LIS
    );
}

#[tokio::test]
async fn staking_with_expired_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 250 LiS
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // create lockup 7 for Bob on 50 LIS on 10 seconds
    create_n_lockups_for_account(
        &alice,
        &bob.id(),
        50 * ONE_LIS,
        Some(U64(10 * SECOND)),
        7,
        &contract,
        &worker,
    )
    .await;

    // Wait till lockup expired
    tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;

    // Bob stake 300LIS
    let mut bob_staked_x = make_stake(&bob, 300 * ONE_LIS, &contract, &worker).await;

    // Assert Bob's balance == 51
    assert_eq!(
        get_balance_info(&bob, &contract, &worker).await,
        51 * ONE_LIS
    );

    // Assert Bob got 300xLis
    assert_eq!(bob_staked_x, 300 * ONE_LIS * 1000);
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Not enough balance\")")]
async fn staking_not_enough_balance() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Transfer to Bob 1 LIS
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 100 LIS
    make_stake(&bob, 50 * ONE_LIS, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Not enough balance\")")]
async fn staking_not_expired_lockup_test() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 100 LiS
    make_transfer(&alice, &bob.id(), ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Create lockup 4 for Bob for 10 days and 100 LIS and 2 lockup for 10 second and 50 Lis
    create_n_lockups_for_account(
        &alice,
        &bob.id(),
        50 * ONE_LIS,
        Some(U64(10 * SECOND)),
        2,
        &contract,
        &worker,
    )
    .await;
    create_n_lockups_for_account(
        &alice,
        &bob.id(),
        100 * ONE_LIS,
        Some(U64(10 * DAY)),
        4,
        &contract,
        &worker,
    )
    .await;

    // Bob stakes 200 LIS
    make_stake(&bob, 200 * ONE_LIS, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: User not found\")")]
async fn staking_account_not_exist_test() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // Some unknown Bob stakes 100 LIS
    make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Contract is paused\")")]
async fn staking_contract_is_paused() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 100 LiS
    make_transfer(&alice, &bob.id(), 100 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // set contract state = paused
    alice
        .call(&worker, contract.id(), "change_state")
        .args_json(serde_json::json!({
           "state": "Paused",
        }))
        .expect("Can't serialize")
        .transact()
        .await
        .expect("Can't get result");

    // Bob stakes 100LIS
    make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: You can't stake zero tokens\")")]
async fn staking_zero_amount() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 250 LiS
    make_transfer(&alice, &bob.id(), 250 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 0LIS
    make_stake(&bob, 0, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Zero pool balance\")")]
async fn add_to_empty_pool() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // add_to_pool 200LIS
    make_add_to_pool(&alice, 200 * ONE_LIS, &contract, &worker).await;

    // Assert error
}

#[tokio::test]
async fn staking_from_unstaked_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 200 LiS
    make_transfer(&alice, &bob.id(), 200 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 200 LIS
    let bob_staked_x = make_stake(&bob, 200 * ONE_LIS, &contract, &worker).await;

    // Set default staking lockup time = 10 seconds
    set_def_staking_lockup_time(&alice, 10 * SECOND, &contract, &worker).await;

    // Bob unsakes 200 xLIS
    make_unstake(&bob, bob_staked_x, &contract, &worker).await;

    // Wait till lockup expire
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Bob stakes 200LIS
    make_stake(&bob, 200 * ONE_LIS, &contract, &worker).await;

    // Assert Bob's balance == 0
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);
}

#[tokio::test]
async fn claim_not_expired_staking_lockup() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 200 LiS
    make_transfer(&alice, &bob.id(), 200 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 200 LIS
    let bob_staked_x = make_stake(&bob, 200 * ONE_LIS, &contract, &worker).await;

    // Assert Bob has no tokens
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);

    // Bob unstakes 200 xLIS
    make_unstake(&bob, bob_staked_x, &contract, &worker).await;

    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);

    // Bob claims all lockups
    let claimed = claim_all_lockup_for_account(&bob, &contract, &worker).await;

    // Assert claimed == 0
    assert_eq!(get_balance_info(&bob, &contract, &worker).await, 0);
    assert_eq!(claimed, 0);
}

#[tokio::test]
async fn stake_several_times_and_unstake_total_staked() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 500 LiS
    make_transfer(&alice, &bob.id(), 500 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 50 LIS
    let mut bob_staked_x = make_stake(&bob, 50 * ONE_LIS, &contract, &worker).await;

    // Bob stakes 100 LIS
    bob_staked_x += make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    // Bob stakes 150 LIS
    bob_staked_x += make_stake(&bob, 150 * ONE_LIS, &contract, &worker).await;

    // Bob stakes 100 LIS
    bob_staked_x += make_stake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    // Bob stakes 50 LIS
    bob_staked_x += make_stake(&bob, 50 * ONE_LIS, &contract, &worker).await;

    // Bob stakes 50 LIS
    bob_staked_x += make_stake(&bob, 50 * ONE_LIS, &contract, &worker).await;

    // Set default staking lockup time as 10 seconds
    set_def_staking_lockup_time(&alice, 10 * SECOND, &contract, &worker).await;

    // Bob unstakes 500 xLIS
    let unstaked = make_unstake(&bob, bob_staked_x, &contract, &worker).await;

    // Assert Bob got 500Lis
    assert_eq!(unstaked, 500 * ONE_LIS);
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Only owner can do this\")")]
async fn add_to_not_owner() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 200 LiS
    make_transfer(&alice, &bob.id(), 200 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob adds to pool 200 LIS
    make_add_to_pool(&bob, 200 * ONE_LIS, &contract, &worker).await;
}

#[tokio::test]
#[should_panic(expected = "Can't get result: Action #0: \
ExecutionError(\"Smart contract panicked: Not enough x balance\"")]
async fn unstake_more_than_staked() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 200 LiS
    make_transfer(&alice, &bob.id(), 200 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 200 LIS
    let mut bob_staked_x = make_stake(&bob, 200 * ONE_LIS, &contract, &worker).await;

    // Assert that Bob got 200xLIS
    assert_eq!(bob_staked_x, 200 * ONE_LIS * 1000);

    // Bob unstakes 201xLIS
    make_unstake(&bob, 201 * ONE_LIS * 1000, &contract, &worker).await;
}

#[tokio::test]
async fn unstake_by_parts() {
    // User Initialization
    let alice = get_alice();
    let bob = get_bob();

    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default().build().await;

    // register Bob with 500 LiS
    make_transfer(&alice, &bob.id(), 500 * ONE_LIS, &contract, &worker)
        .await
        .expect("Failed to transfer");

    // Bob stakes 500 LIS
    let mut bob_staked_x = make_stake(&bob, 500 * ONE_LIS, &contract, &worker).await;

    // Assert bob has 500xLIS
    assert_eq!(bob_staked_x, 500 * ONE_LIS * 1000);

    // Set default staking lockup time as 10 seconds
    set_def_staking_lockup_time(&alice, 10 * SECOND, &contract, &worker).await;

    // Bob unstakes 100 LIS
    let mut bob_unstake_x = make_unstake(&bob, 100 * ONE_LIS, &contract, &worker).await;

    // Bob unstakes 50 LIS
    bob_unstake_x += make_unstake(&bob, 50 * ONE_LIS, &contract, &worker).await;

    // Bob unstakes 50 LIS
    bob_unstake_x += make_unstake(&bob, 50 * ONE_LIS, &contract, &worker).await;

    // Bob unstakes 150 LIS
    bob_unstake_x += make_unstake(&bob, 150 * ONE_LIS, &contract, &worker).await;

    // Bob unstakes 150 LIS
    bob_unstake_x += make_unstake(&bob, 150 * ONE_LIS, &contract, &worker).await;

    // Assert Bob got 500xLis
    assert_eq!(bob_unstake_x, 500 * ONE_LIS / 1000);
}
