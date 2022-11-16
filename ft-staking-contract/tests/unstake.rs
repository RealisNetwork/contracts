use near_sdk::{env, serde_json};
use test_utils::{token, utils::*, SandboxEnvironment};
use workspaces::AccountId;

#[tokio::test]
async fn unstake_success() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    let old_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check current owner balance
    assert_eq!(old_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    sandbox
        .owner
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({"xtoken_amount": "1000"}))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let unstaked_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(unstaked_balance, (amount - 1) * 1_000);

    Ok(())
}

#[tokio::test]
async fn unstake_zero_amount() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    let old_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check current owner balance
    assert_eq!(old_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    let result = sandbox
        .owner
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({"xtoken_amount": "0"}))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("The xtoken_amount should not be zero"));
    assert_eq!(
        100000000000000000,
        token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn unstake_small_amounts() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    let old_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check current owner balance
    assert_eq!(old_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    let result = sandbox
        .owner
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({"xtoken_amount": "10"}))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert_eq!(
        100000000000000000,
        token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn unstake_partial_success() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let mut sandbox = SandboxEnvironment::new(&worker).await?;
    let contract_id: AccountId = FAKE_LOCKUP_CONTRACT_ACCOUNT.parse()?;
    let amount = 100 * LIS;

    let wasm = workspaces::compile_project("../fake-lockup-contract").await?;
    let contract = sandbox
        .lockup
        .as_account()
        .deploy(&wasm)
        .await?
        .into_result()?;

    let old_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check current owner balance
    assert_eq!(old_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    let result = sandbox
        .owner
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({ "xtoken_amount": (amount * 1000).to_string() }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert_eq!(
        100000000000000000,
        token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn unstake_transfer_error() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let user = worker.dev_create_account().await?;
    let amount = 100 * LIS;

    let old_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check current owner balance
    assert_eq!(old_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make storage deposit for user on staking contract
    user.call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make storage deposit for user on token contract
    user.call(sandbox.token.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    // Make `ft_transfer` to transfer
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer")
        .args_json(serde_json::json!({ "receiver_id": user.id(), "amount": amount.to_string() }))
        .deposit(YOCTO)
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .into_result()?;

    let user_balance = test_utils::token::ft_balance_of(&sandbox.token, user.id()).await?;

    // Check new owner balance
    assert_eq!(user_balance, amount);

    let result = sandbox
        .owner
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({"xtoken_amount": "100_000"}))
        .deposit(YOCTO)
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    // assert!(format!("{:?}", failure).contains("The xtoken_amount should not be zero"));
    assert_eq!(0, token::ft_balance_of(&sandbox.staking, user.id()).await?);

    Ok(())
}

#[tokio::test]
async fn unstake_not_own_tokens() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let user = worker.dev_create_account().await?;
    let amount = 100 * LIS;

    let old_owner_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;
    let old_user_balance = test_utils::token::ft_balance_of(&sandbox.staking, user.id()).await?;

    // Check current owner balance
    assert_eq!(old_owner_balance, 0);
    assert_eq!(old_user_balance, 0);

    // Make storage deposit for owner
    sandbox
        .owner
        .call(sandbox.staking.id(), "storage_deposit")
        .args_json(serde_json::json!({}))
        .deposit(NEAR)
        .transact()
        .await?
        .into_result()?;

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": STAKING_CONTRACT_ACCOUNT,
            "amount": amount.to_string(),
            "msg": "\"Stake\""
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_owner_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;
    let new_user_balance = test_utils::token::ft_balance_of(&sandbox.staking, user.id()).await?;

    // Check new owner balance
    assert_eq!(new_owner_balance, amount * 1_000);
    assert_eq!(new_user_balance, 0);

    let result = user
        .call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({"xtoken_amount": "10_000"}))
        .deposit(YOCTO)
        .gas(300_000_000_000_000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert_eq!(0, token::ft_balance_of(&sandbox.staking, user.id()).await?);

    Ok(())
}
