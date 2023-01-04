pub mod utils;

use near_sdk::serde_json;
use test_utils::{token, utils::*, SandboxEnvironment};
use utils::*;

#[tokio::test]
async fn stake() -> anyhow::Result<()> {
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
        .args_json(
            serde_json::json!({ "receiver_id": STAKING_ACCOUNT, "amount": amount.to_string(), "msg": "\"Stake\"" })
        )
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance =
        test_utils::token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    Ok(())
}

#[tokio::test]
async fn stake_zero_amount() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 0 * LIS;

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
    let result = sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
                "receiver_id": STAKING_ACCOUNT,
                "amount": amount.to_string(),
                "msg": "\"Stake\"",
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("The amount should be a positive number"));
    assert_eq!(
        0,
        token::ft_balance_of(&sandbox.staking, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn stake_other_token() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let user = worker.root_account()?;
    let fake_token_contract = token::new(
        &worker,
        Some(user.id().clone()),
        None,
        STAKING_CONTRACT_ACCOUNT.parse()?,
    )
    .await?;

    user.call(fake_token_contract.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({ "account_id": sandbox.staking.id() }))
        .transact()
        .await?
        .into_result()?;

    let result = user
        .call(fake_token_contract.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": STAKING_ACCOUNT,
            "amount": LIS.to_string(),
            "msg": "\"Stake\"",
        }))
        .transact()
        .await?
        .into_result();

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("Invalid token ID"));

    Ok(())
}

#[tokio::test]
async fn stake_for_other() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;
    let user = worker.root_account()?;

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

    let stake_for = serde_json::json!({"StakeFor": { "account_id": user.id() } });

    // Make `ft_transfer_call` to stake
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(
            serde_json::json!({ "receiver_id": STAKING_ACCOUNT, "amount": amount.to_string(), "msg": stake_for.to_string() })
        )
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    let new_balance = test_utils::token::ft_balance_of(&sandbox.staking, user.id()).await?;

    // Check new owner balance
    assert_eq!(new_balance, amount * 1_000);

    Ok(())
}
