use near_sdk::serde_json;
use test_utils::SandboxEnviroment;

pub const SPEC_METADATA: &str = "1.0.1";

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let contract = SandboxEnviroment::new(&worker).await?.lockup;

    // Check new version
    let actual: String = contract
        .view(
            "get_metadata",
            serde_json::json!({}).to_string().as_bytes().to_vec(),
        )
        .await?
        .json()?;
    assert_eq!(actual, SPEC_METADATA);

    Ok(())
}
