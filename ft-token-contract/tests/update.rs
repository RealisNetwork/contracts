use test_utils::{token, SandboxEnvironment};

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let contract = SandboxEnvironment::new(&worker).await?.token;

    let amount = token::ft_total_supply(&contract).await?;

    assert_eq!(amount, 3000000000000000000000);

    Ok(())
}
