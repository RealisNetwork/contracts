use near_sdk::{ext_contract, serde_json};
use near_units::{parse_gas, parse_near};
use serde::{Deserialize, Serialize};
use workspaces::{network::Sandbox, Account, AccountId, BlockHeight, Contract, Worker};

const OWNER_ID: &str = "testnetacc.testnet";
const STAKING_CONTRACT_ID: &str = "staking.v1.testnetacc.testnet";
const CONTRACT_ACCOUNT: &str = "token.v1.testnetacc.testnet";
const EXPECTED_NFT_METADATA: &str = r#"{
  "spec": "ft-1.0.1",
  "name": "Realis Network LIS token",
  "symbol": "LIS",
  "icon": null,
  "reference": null,
  "reference_hash": null,
  "decimals": 12
}"#;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct TokenMetadata {
    spec: String,
    name: String,
    symbol: String,
    icon: Option<String>,
    reference: Option<String>,
    reference_hash: Option<String>,
    decimals: u64,
}

fn expected() -> TokenMetadata {
    serde_json::from_str(EXPECTED_NFT_METADATA).unwrap()
}

async fn pull_contract(_owner: &Account, worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    let testnet = workspaces::testnet_archival().await?;
    let contract_id: AccountId = CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &testnet)
        .initial_balance(parse_near!("1000 N"))
        .transact()
        .await?;

    // owner
    //     .call(contract.id(), "init_method_name")
    //     .args_json(serde_json::json!({
    //         "arg1": value1,
    //         "arg2": value2,
    //     }))
    //     .transact()
    //     .await?;

    Ok(contract)
}

#[tokio::test]
async fn test_migration() -> anyhow::Result<()> {
    let wasm = workspaces::compile_project("./").await?;

    let worker = workspaces::sandbox().await?;
    let contract = worker.dev_deploy(&wasm).await?;

    contract
        .call("new")
        .args_json(serde_json::json!({
            "staking_id": STAKING_CONTRACT_ID
        }))
        .transact()
        .await?
        .into_result()?;

    let actual: TokenMetadata = contract
        .view(
            "ft_metadata",
            serde_json::json!({}).to_string().as_bytes().to_vec(),
        )
        .await?
        .json()?;

    assert_eq!(actual, expected());

    Ok(())
}
