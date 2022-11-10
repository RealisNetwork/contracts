use crate::utils::*;
use near_sdk::{serde_json, json_types::U128};
use near_units::parse_near;
use workspaces::{network::Sandbox, AccountId, Contract, Worker};

pub async fn new(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    backend_ids: Option<Vec<AccountId>>,
    staking_id: AccountId,
) -> anyhow::Result<Contract> {
    let wasm = workspaces::compile_project("../ft-token-contract").await?;
    let contract = worker.dev_deploy(&wasm).await?;
    contract
        .call("new")
        .args_json(serde_json::json!({
        "owner_id": owner_id,
        "backend_ids": backend_ids,
        "staking_id": staking_id,
        }))
        .transact()
        .await?
        .into_result()?;

    Ok(contract)
}

pub async fn pull(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    backend_ids: Option<Vec<AccountId>>,
    staking_id: AccountId,
) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let contract_id: AccountId = TOKEN_CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &mainnet)
        .initial_balance(parse_near!("1000 N"))
        .transact()
        .await?;

    contract
        .call("new")
        .args_json(serde_json::json!({
            "owner_id": owner_id,
            "backend_ids": backend_ids,
            "staking_id": staking_id,
        }))
        .transact()
        .await?
        .into_result()?;

    let wasm = workspaces::compile_project("../ft-token-contract").await?;
    let contract = contract.as_account().deploy(&wasm).await?.result;
    contract.call("update").transact().await?.into_result()?;

    Ok(contract)
}

pub async fn ft_balance_of(contract: &Contract, account_id: &AccountId) -> anyhow::Result<u128> {
    let balance: U128 = contract
        .view(
            "ft_balance_of",
            serde_json::json!({
                "account_id": account_id,
            }).to_string().into_bytes()
        )
        .await?
        .json()?;

    Ok(balance.0)
}
