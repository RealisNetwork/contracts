use crate::utils::*;
use near_sdk::{json_types::U128, serde_json};
use near_units::parse_near;
use workspaces::{network::Sandbox, AccountId, Contract, Worker};

pub async fn new(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    backend_id: Option<AccountId>,
) -> anyhow::Result<Contract> {
    let wasm = workspaces::compile_project("../nft-token-contract").await?;
    let contract = worker.dev_deploy(&wasm).await?;
    contract
        .call("new")
        .args_json(serde_json::json!({
        "owner_id": owner_id,
        "backend_id": backend_id,
        }))
        .transact()
        .await?
        .into_result()?;

    Ok(contract)
}

pub async fn pull(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    backend_id: Option<AccountId>,
) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let contract_id: AccountId = NFT_CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &mainnet)
        .initial_balance(parse_near!("5000 N"))
        .transact()
        .await?;

    contract
        .call("new")
        .args_json(serde_json::json!({
            "owner_id": owner_id,
            "backend_id": backend_id,
        }))
        .transact()
        .await?
        .into_result()?;

    let wasm = workspaces::compile_project("../nft-token-contract").await?;
    let contract = contract.as_account().deploy(&wasm).await?.result;
    contract.call("update").transact().await?.into_result()?;

    Ok(contract)
}

pub async fn nft_total_supply(contract: &Contract) -> anyhow::Result<u128> {
    let total_supply: U128 = contract
        .view(
            "nft_total_supply",
            serde_json::json!({}).to_string().into_bytes(),
        )
        .await?
        .json()?;
    Ok(total_supply.0)
}
