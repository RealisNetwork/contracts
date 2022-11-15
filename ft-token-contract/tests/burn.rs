use near_sdk::serde_json;
use test_utils::{token, utils::*, SandboxEnviroment};

#[tokio::test]
async fn burn_success() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnviroment::new(&worker).await?;

    sandbox
        .owner
        .call(sandbox.token.id(), "ft_burn")
        .args_json(serde_json::json!({
            "amount": (3_000 * LIS).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    let balance = token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?;
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(balance, (3_000_000_000 - 3_000) * LIS);
    assert_eq!(total_supply, (3_000_000_000 - 3_000) * LIS);

    Ok(())
}

#[tokio::test]
async fn burn_more_than_have() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnviroment::new(&worker).await?;

    let result = sandbox
    .owner
    .call(sandbox.token.id(), "ft_burn")
    .args_json(serde_json::json!({
    "amount": (3_000_000_001 * LIS).to_string()
    }))
    .transact()
    .await?
    .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("The account doesn't have enough balance"));

    let balance = token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?;
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(balance, 3_000_000_000 * LIS);
    assert_eq!(total_supply, 3_000_000_000 * LIS);

    Ok(())
}