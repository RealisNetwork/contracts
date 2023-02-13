use near_sdk::serde_json;
use near_units::parse_near;
use workspaces::{network::Sandbox, AccountId, Contract, Worker};

pub const CONTRACT_ACCOUNT: &str = "token.v1.realisnetwork.near";
pub const STAKING_ACCOUNT: &str = "staking.v1.realisnetwork.near";
pub const LOCKUP_ACCOUNT: &str = "lockup.v1.realisnetwork.near";
pub const SPEC_METADATA: &str = "ft-1.0.1";

pub async fn pull_contract(worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    let mainnet = workspaces::mainnet_archival().await?;
    let contract_id: AccountId = CONTRACT_ACCOUNT.parse()?;
    let contract = worker
        .import_contract(&contract_id, &mainnet)
        .initial_balance(parse_near!("5000 N"))
        .transact()
        .await?;

    contract
        .call("new")
        .args_json(
            serde_json::json!({ "staking_id": STAKING_ACCOUNT, "lockup_id": LOCKUP_ACCOUNT }),
        )
        .transact()
        .await?
        .into_result()?;

    Ok(contract)
}

pub async fn pull_and_update_contract(worker: &Worker<Sandbox>) -> anyhow::Result<Contract> {
    let wasm = workspaces::compile_project("./").await?;

    let contract = pull_contract(&worker)
        .await?
        .as_account()
        .deploy(&wasm)
        .await?
        .result;

    contract.call("update").transact().await?.into_result()?;

    Ok(contract)
}
