use near_sdk::serde_json;
use test_utils::{token, utils::*, SandboxEnvironment};

#[tokio::test]
async fn add_to_pool_change_price() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 1000 * LIS;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    // Storage deposit for new user
    user.call(sandbox.token.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result()?;
    user.call(sandbox.staking.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result()?;

    // Transfer some LIS to user
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": user.id(),
            "amount": amount.to_string(),
        }))
        .deposit(YOCTO)
        .transact()
        .await?
        .into_result()?;

    // User stake LIS
    user.call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
            "msg": "\"Stake\"",
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    // Owner add tokens to pool
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
            "msg": "\"AddToPool\"",
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    user.call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({}))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    // For test simplification
    // look for lockup contract balance == user balance
    // because of only one lockup existes
    let balance = token::ft_balance_of(&sandbox.token, sandbox.lockup.id()).await?;

    assert_eq!(balance, amount * 2);

    Ok(())
}

#[tokio::test]
async fn add_to_empty_pool_not_change_price() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 1000 * LIS;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    // Storage deposit for new user
    user.call(sandbox.token.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result()?;
    user.call(sandbox.staking.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result()?;

    // Transfer some LIS to user
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "receiver_id": user.id(),
            "amount": amount.to_string(),
        }))
        .deposit(YOCTO)
        .transact()
        .await?
        .into_result()?;

    // Owner add tokens to pool
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
            "msg": "\"AddToPool\"",
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    // User stake LIS
    user.call(sandbox.token.id(), "ft_transfer_call")
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
            "msg": "\"Stake\"",
        }))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    user.call(sandbox.staking.id(), "unstake")
        .args_json(serde_json::json!({}))
        .deposit(YOCTO)
        .gas(300000000000000)
        .transact()
        .await?
        .into_result()?;

    // For test simplification
    // look for lockup contract balance == user balance
    // because of only one lockup existes
    let balance = token::ft_balance_of(&sandbox.token, sandbox.lockup.id()).await?;

    assert_eq!(balance, amount);

    Ok(())
}
