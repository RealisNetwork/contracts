use crate::utils::*;
use near_sdk::{json_types::U128, serde_json};
use near_units::parse_near;
use workspaces::{network::Sandbox, AccountId, Contract, Worker};

pub async fn pull(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    token_account_id: AccountId,
    lockup_account_id: AccountId,
) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let contract_id: AccountId = STAKING_CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &mainnet)
        .initial_balance(parse_near!("1000 N"))
        .transact()
        .await?;

    contract
    .call("new")
    .args_json(serde_json::json!({"owner_id": owner_id, "token_account_id": token_account_id, "lockup_account_id": lockup_account_id}))
    .transact()
    .await?
    .into_result()?;

    let wasm = workspaces::compile_project("../ft-staking-contract").await?;
    let contract = contract.as_account().deploy(&wasm).await?.result;
    contract.call("update").transact().await?.into_result()?;

    Ok(contract)
}

pub async fn ft_total_supply(contract: &Contract) -> anyhow::Result<u128> {
    let amount: U128 = contract
        .view(
            "ft_total_supply",
            serde_json::json!({}).to_string().into_bytes(),
        )
        .await?
        .json()?;

    Ok(amount.0)
}
