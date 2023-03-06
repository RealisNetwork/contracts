use test_utils::{staking, SandboxEnvironment};

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let contract = SandboxEnvironment::new(&worker).await?.staking;

    let amount = staking::ft_total_supply(&contract).await?;
    assert_eq!(amount, 0_u128);

    Ok(())
}
