use near_sdk::serde_json;
use test_utils::{token, utils::*, SandboxEnvironment};

#[tokio::test]
async fn mint_success() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    worker.fast_forward(1_500_000).await?;

    sandbox
        .owner
        .call(sandbox.token.id(), "ft_mint")
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result()?;

    let balance = token::ft_balance_of(&sandbox.token, sandbox.staking.id()).await?;
    assert_ne!(balance, 0);
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(total_supply, 3_000_410_000 * LIS);

    Ok(())
}

#[tokio::test]
async fn mint_too_early() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    let result = sandbox
        .owner
        .call(sandbox.token.id(), "ft_mint")
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("Too early"));

    let balance = token::ft_balance_of(&sandbox.token, sandbox.staking.id()).await?;
    assert_eq!(balance, 0);
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(total_supply, 3_000_000_000 * LIS);

    Ok(())
}

#[tokio::test]
async fn mint_not_owner() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    let result = user
        .call(sandbox.token.id(), "ft_mint")
        .args_json(serde_json::json!({}))
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("Owner must be predecessor"));

    let balance = token::ft_balance_of(&sandbox.token, sandbox.staking.id()).await?;
    assert_eq!(balance, 0);
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(total_supply, 3_000_000_000 * LIS);

    Ok(())
}
