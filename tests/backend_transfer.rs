use crate::utils::*;
use near_sdk::{json_types::U64, serde_json};
use realis_near::utils::{DAY, ONE_LIS, SECOND};

#[tokio::test]
async fn backend_transfer_from_not_exist_account() {
    // Setup contract: Backend.root - owner/backend
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(BackendAccount::get_root().account.id().clone())
        .build()
        .await;

    // backend_transfer from non exist account
    let user1 = BackendAccount::get_user1();
    let call_result = user1
        .account
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
async fn backend_transfer_more_than_account_balance() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user3 = BackendAccount::get_user3();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    assert_eq!(
        get_balance_info(&root.account, &contract, &worker).await,
        3_000_000_000 * ONE_LIS
    );

    // Owner transfer 3_000_000_000 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(
        &root.account,
        &root.id_by_pk,
        3_000_000_000 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Backend.root backend_transfer to Backend.user2 1_000 LIS
    make_backend_transfer(
        &root.account,
        &user2.id_by_pk,
        1000 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    println!(
        "root_id_by_pk: {:#?}",
        get_balance_info_signed(&root.account, &root.id_by_pk, &contract, &worker).await
    );
    println!(
        "user2_id: {:#?}",
        get_balance_info_signed(&user2.account, &user2.id_by_pk, &contract, &worker).await
    );

    // Assert Backend.root has 2_999_998_900 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &root.id_by_pk, &contract, &worker).await,
        2_999_998_900 * ONE_LIS
    );

    // Assert Beneficiary has 100 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Assert Backend.user2 has 1_000 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        1000 * ONE_LIS
    );

    // Backend.user2 backend_transfer to Backend.user3 1_001 LIS
    let not_enough_balance_result = make_backend_transfer(
        &user2.account,
        &user3.id_by_pk,
        1001 * ONE_LIS,
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(not_enough_balance_result
        .unwrap_err()
        .to_string()
        .contains("Not enough balance"));

    // Backend.root backend_transfer to Backend.user3 3_000_000_000 LIS
    let not_enough_balance_result = make_backend_transfer(
        &root.account,
        &user3.id_by_pk,
        3_000_000_000 * ONE_LIS,
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(not_enough_balance_result
        .unwrap_err()
        .to_string()
        .contains("Not enough balance"));
}

#[tokio::test]
async fn backend_transfer_zero_amount() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Backend.root backend_transfer to Backend.user2 0 LIS
    let transfer_result = make_backend_transfer(
        &root.account,
        &user2.id_by_pk,
        0 * ONE_LIS,
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(transfer_result
        .unwrap_err()
        .to_string()
        .contains("You can't transfer 0 tokens"));
}

#[tokio::test]
async fn backend_transfer() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user3 = BackendAccount::get_user3();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    assert_eq!(
        get_balance_info(&root.account, &contract, &worker).await,
        3_000_000_000 * ONE_LIS
    );

    // Owner transfer 3_000_000_000 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(
        &root.account,
        &root.id_by_pk,
        3_000_000_000 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Backend.root backend_transfer to Backend.user2 1_000 LIS
    make_backend_transfer(
        &root.account,
        &user2.id_by_pk,
        1000 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.root has 2_999_998_900 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &root.id_by_pk, &contract, &worker).await,
        2_999_998_900 * ONE_LIS
    );

    // Assert Beneficiary has 100 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Assert Backend.user2 has 1_000 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        1_000 * ONE_LIS
    );

    // Backend.root backend_transfer to Backend.user3 10 LIS
    make_backend_transfer(
        &root.account,
        &user3.id_by_pk,
        10 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.root has 2_999_998_889 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &root.id_by_pk, &contract, &worker).await,
        2_999_998_889 * ONE_LIS
    );

    // Assert Beneficiary has 101 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        101 * ONE_LIS
    );

    // Assert Backend.user3 has 10 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user3.id_by_pk, &contract, &worker).await,
        10 * ONE_LIS
    );

    // Backend.user2 backend_transfer to Backend.user3 100 LIS
    make_backend_transfer(
        &user2.account,
        &user3.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 890 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        890 * ONE_LIS
    );

    // Assert Beneficiary has 111 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        111 * ONE_LIS
    );

    // Assert Backend.user3 has 110 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user3.id_by_pk, &contract, &worker).await,
        110 * ONE_LIS
    );

    // Backend.user3 backend_transfer to Backend.user4 1 LIS
    make_backend_transfer(&user3.account, &user4.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Assert Backend.user3 has 108.9 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user3.id_by_pk, &contract, &worker).await,
        (108.9 * ONE_LIS as f64) as u128
    );

    // Assert ALice has 111.1 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        (111.1 * ONE_LIS as f64) as u128
    );

    // Assert Backend.user4 has 1 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_with_expired_lockup() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    assert_eq!(
        get_balance_info(&root.account, &contract, &worker).await,
        3_000_000_000 * ONE_LIS
    );

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root backend_transfer to Backend.user2 100 LIS
    make_transfer(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 90 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        90 * ONE_LIS
    );

    // Assert Backend.user2 hasn't lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        0
    );

    // Assert Backend.user4 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_with_not_expired_lockup() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root backend_transfer to Backend.user2 110 LIS
    make_transfer(
        &root.account,
        &user2.id_by_pk,
        110 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 110 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        110 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 0 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        0
    );

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Assert Backend.user4 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_get_balance_from_expired_lockup() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root backend_transfer to Backend.user2 110 LIS
    make_transfer(
        &root.account,
        &user2.id_by_pk,
        110 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 110 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        110 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Backend.user2 backend_transfer to Backend.user4 150 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        150 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 35 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        45 * ONE_LIS
    );

    // Assert Backend.user2 hasn't lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        0
    );

    // Assert Beneficiary has 15 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        15 * ONE_LIS
    );

    // Assert Backend.user4 has 150 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        150 * ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_get_balance_from_two_expired_lockups() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 50 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        50 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has 2 lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        2
    );

    // Backend.user2 backend_transfer to Backend.user4 120 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        120 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 18 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        18 * ONE_LIS
    );

    // Assert Backend.user2 has not lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        0
    );

    // Assert Beneficiary has 12 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        12 * ONE_LIS
    );

    // Assert Backend.user4 has 120 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        120 * ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_get_balance_from_set_of_lockups() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 10 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        10 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 20 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        20 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 25 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        25 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 50 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        50 * ONE_LIS,
        Some(U64(1 * DAY)),
        &contract,
        &worker,
    )
    .await;

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has 5 lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        5
    );

    // Backend.user2 backend_transfer to Backend.user4 50 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        50 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 0 LIS
    assert_eq!(
        get_balance_info_signed(&user2.account, &user2.id_by_pk, &contract, &worker).await,
        0
    );

    // Assert Backend.user2 has 2 lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        2
    );

    // Assert Beneficiary has 5 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        5 * ONE_LIS
    );

    // Assert Backend.user4 has 50 LIS
    assert_eq!(
        get_balance_info_signed(&user2.account, &user4.id_by_pk, &contract, &worker).await,
        50 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 25 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        25 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has 3 lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        3
    );

    // Backend.user2 backend_transfer to Backend.user4 50 LIS
    let transfer_result = make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        50 * ONE_LIS,
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(transfer_result
        .unwrap_err()
        .to_string()
        .contains("Not enough balance"));

    // Assert Backend.user2 has 0 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        0
    );

    // Assert Backend.user2 has 3 lockups
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        3
    );

    // Assert Backend.user4 has 50 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        50 * ONE_LIS
    );
}

#[tokio::test]
async fn backend_transfer_get_balance_from_not_expired_lockup() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root backend_transfer to Backend.user2 100 LIS
    make_transfer(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 DAY, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * DAY)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Backend.user2 backend_transfer to Backend.user4 150 LIS
    let transfer_result = make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        150 * ONE_LIS,
        &contract,
        &worker,
    )
    .await;

    // Assert error
    assert!(transfer_result
        .unwrap_err()
        .to_string()
        .contains("Not enough balance"));

    // Assert Backend.user2 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Assert Backend.user4 has 0 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        0
    );
}

#[tokio::test]
async fn backend_transfer_lockup_cover_fee() {
    let root = BackendAccount::get_root();
    let alice = get_alice();
    let user2 = BackendAccount::get_user2();
    let user4 = BackendAccount::get_user4();

    // Setup contract: Backend.root - owner/backend, Alice - beneficiary,
    // total_supply - 3_000_000_000 LIS
    let (contract, worker) = TestingEnvBuilder::default()
        .set_signer(BackendAccount::get_root().account)
        .set_backend(root.account.id().clone())
        .set_beneficiary(alice.id().clone())
        .build()
        .await;

    // Owner transfer 1 LIS to account_id = hash(backend_root.pk) = root_id_by_pk
    make_transfer(&root.account, &root.id_by_pk, ONE_LIS, &contract, &worker)
        .await
        .unwrap();

    // Backend.root backend_transfer to Backend.user2 100 LIS
    make_transfer(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );

    // Backend.root create lockup for Backend.user2 with duration = 1 SECOND, amount - 100 LIS
    create_lockup_for_account(
        &root.account,
        &user2.id_by_pk,
        100 * ONE_LIS,
        Some(U64(1 * SECOND)),
        &contract,
        &worker,
    )
    .await;

    // Assert Backend.user2 has lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        1
    );

    // Backend.user2 backend_transfer to Backend.user4 100 LIS
    make_backend_transfer(
        &user2.account,
        &user4.id_by_pk,
        100 * ONE_LIS,
        &contract,
        &worker,
    )
    .await
    .unwrap();

    // Assert Backend.user2 has 90 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user2.id_by_pk, &contract, &worker).await,
        90 * ONE_LIS
    );

    // Assert Backend.user2 has not lockup
    assert_eq!(
        get_lockup_info_signed(&root.account, &user2.id_by_pk, &contract, &worker)
            .await
            .len(),
        0
    );

    // Assert Beneficiary has 10 LIS
    assert_eq!(
        get_balance_info(&alice, &contract, &worker).await,
        10 * ONE_LIS
    );

    // Assert Backend.user4 has 100 LIS
    assert_eq!(
        get_balance_info_signed(&root.account, &user4.id_by_pk, &contract, &worker).await,
        100 * ONE_LIS
    );
}
