use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_sdk::serde_json;

pub mod utils;
use utils::*;

#[tokio::test]
async fn update() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let old_contract = pull_contract(&worker).await?;

    // let old_metadata: FungibleTokenMetadata = old_contract
    //     .view(
    //         "ft_metadata",
    //         serde_json::json!({}).to_string().as_bytes().to_vec(),
    //     )
    //     .await?
    //     .json()?;
    // assert_ne!(
    //     old_metadata.spec, SPEC_METADATA,
    //     "Same version. Forget to change version in contract metadata"
    // );

    // Deploing updated contract
    let wasm = workspaces::compile_project("./").await?;

    let contract = old_contract.as_account().deploy(&wasm).await?.result;

    // Call update function on contract
    contract.call("update").transact().await?.into_result()?;

    // Check new version
    let actual: FungibleTokenMetadata = contract
        .view(
            "ft_metadata",
            serde_json::json!({}).to_string().as_bytes().to_vec(),
        )
        .await?
        .json()?;
    assert_eq!(actual.spec, SPEC_METADATA);

    Ok(())
}
