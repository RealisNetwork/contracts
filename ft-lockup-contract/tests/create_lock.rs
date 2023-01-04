use test_utils::{token, utils::*, SandboxEnvironment};

#[tokio::test]
async fn create_lock_with_other_token() -> anyhow::Result<()> {
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
        .args_json(serde_json::json!({
            "account_id": sandbox.lockup.id()
        }))
        .transact()
        .await?
        .into_result()?;

    let result = user
        .call(fake_token_contract.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": LIS.to_string(),
            "msg": "",
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
async fn create_lock_not_whitelisted_creator() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    let result = sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": LIS.to_string(),
            "msg": "",
        }))
        .transact()
        .await?
        .into_result();

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("Not in deposit whitelist"));

    Ok(())
}

#[tokio::test]
async fn create_lock_with_0_amount() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    let result = sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": "0",
            "msg": "",
        }))
        .transact()
        .await?
        .into_result();

    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("The amount should be a positive number"));

    Ok(())
}
