use crate::utils::*;
use near_sdk::serde_json;
use near_units::parse_near;
use workspaces::{network::Sandbox, AccountId, Contract, Worker};

pub async fn pull(
    worker: &Worker<Sandbox>,
    owner_id: Option<AccountId>,
    token_account_id: AccountId,
    deposit_whitelist: Vec<AccountId>,
) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let contract_id: AccountId = LOCKUP_CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &mainnet)
        .initial_balance(parse_near!("1000 N"))
        .transact()
        .await?;

    contract
        .call("new")
        .args_json(serde_json::json!({ "owner_od": owner_id, "token_account_id": token_account_id, "deposit_whitelist": deposit_whitelist }))
        .transact()
        .await?
        .into_result()?;

    let wasm = workspaces::compile_project("../ft-lockup-contract").await?;
    let contract = contract.as_account().deploy(&wasm).await?.result;
    contract.call("update").transact().await?.into_result()?;

    Ok(contract)
}
