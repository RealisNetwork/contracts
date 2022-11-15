use near_contract_standards::storage_management::StorageBalance;
use near_sdk::serde_json;
use test_utils::{token, utils::*, SandboxEnviroment};

#[tokio::test]
async fn backend_can_register_account_throw_transfer() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnviroment::new(&worker).await?;

    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    let storage_balance: Option<StorageBalance> = sandbox
        .token
        .view(
            "storage_balance_of",
            serde_json::json!({
                "account_id": user.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    assert!(storage_balance.is_none());

    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.backend.id(),
            "amount": "1"
        }))
        .transact()
        .await?
        .into_result()?;

    sandbox
        .backend
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": user.id(),
            "amount": "1"
        }))
        .transact()
        .await?
        .into_result()?;
    let storage_balance: Option<StorageBalance> = sandbox
        .token
        .view(
            "storage_balance_of",
            serde_json::json!({
                "account_id": user.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    assert!(storage_balance.is_some());
    let balance = token::ft_balance_of(&sandbox.token, user.id()).await?;
    assert_eq!(balance, 1);
    let total_supply = token::ft_total_supply(&sandbox.token).await?;
    assert_eq!(total_supply, 3_000_000_000 * LIS);

    Ok(())
}
