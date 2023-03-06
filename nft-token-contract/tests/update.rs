use test_utils::{nft, SandboxEnvironment};

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    let value = nft::nft_total_supply(&sandbox.nft).await?;
    assert_eq!(value, 0);

    Ok(())
}
