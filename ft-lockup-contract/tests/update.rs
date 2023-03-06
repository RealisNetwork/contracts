use test_utils::{lockup, SandboxEnvironment};

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let contract = SandboxEnvironment::new(&worker).await?.lockup;

    let amount = lockup::get_num_lockups(&contract).await?;
    assert_eq!(amount, 0_u64);

    Ok(())
}
