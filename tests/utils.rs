use near_sdk::{env, json_types::U64, PublicKey};
pub use near_sdk::{json_types::U128, serde::Serialize, serde_json, serde_json::Value, Timestamp};
use realis_near::{lockup::LockupInfo, utils::DAY};
use std::{collections::HashMap, str::FromStr};
pub use workspaces::{
    network::{DevAccountDeployer, Testnet},
    result::CallExecutionDetails,
    Account, AccountId, Contract, Worker,
};
use workspaces::{
    operations::Function,
    types::{Gas, SecretKey},
};

pub const WASM_FILE: &str = "./target/wasm32-unknown-unknown/release/realis_near.wasm";
pub const ONE_LIS: u128 = 1_000_000_000_000;
pub const MAX_GAS: Gas = 300_000_000_000_000;

pub type TestWorker = Worker<Testnet>;

pub fn get_alice() -> Account {
    Account::from_file("./tests/res/alice.realis.testnet.json")
}

pub fn get_bob() -> Account {
    Account::from_file("./tests/res/bob.realis.testnet.json")
}

pub fn get_charlie() -> Account {
    Account::from_file("./tests/res/charlie.realis.testnet.json")
}

pub fn get_dave() -> Account {
    Account::from_file("./tests/res/dave.realis.testnet.json")
}

pub struct BackendAccount {}

pub struct CustomBackendAccount {
    pub account: Account,
    pub id_by_pk: AccountId,
}

impl CustomBackendAccount {
    pub fn get_account_from_file(filename: &str) -> CustomBackendAccount {
        let account_content = std::fs::read_to_string(filename);
        let json: HashMap<String, String> =
            serde_json::from_str(account_content.expect("Can't read file").as_str())
                .expect("Can't get JSON");
        Self {
            account: Account::from_file(filename),
            id_by_pk: Self::get_account_id(
                PublicKey::from_str(json.get("public_key").expect("Can't get public_key")).unwrap(),
            ),
        }
    }

    pub fn get_account_id(pk: PublicKey) -> AccountId {
        hex::encode(&pk.as_bytes()[1..])
            .try_into()
            .unwrap_or_else(|_| env::panic_str("Fail to convert PublicKey to AccountId"))
    }
}

impl BackendAccount {
    pub fn get_root() -> CustomBackendAccount {
        CustomBackendAccount::get_account_from_file("./tests/res/backend.realis.testnet.json")
    }

    pub fn get_user1() -> CustomBackendAccount {
        CustomBackendAccount::get_account_from_file(
            "./tests/res/backend_access_keys/user1_backend.realis.testnet.json",
        )
    }

    pub fn get_user2() -> CustomBackendAccount {
        CustomBackendAccount::get_account_from_file(
            "./tests/res/backend_access_keys/user2_backend.realis.testnet.json",
        )
    }

    pub fn get_user3() -> CustomBackendAccount {
        CustomBackendAccount::get_account_from_file(
            "./tests/res/backend_access_keys/user3_backend.realis.testnet.json",
        )
    }

    pub fn get_user4() -> CustomBackendAccount {
        CustomBackendAccount::get_account_from_file(
            "./tests/res/backend_access_keys/user4_backend.realis.testnet.json",
        )
    }
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TestingEnvBuilder {
    total_supply: U128,
    constant_fee: U128,
    percent_fee: u8,
    beneficiary_id: AccountId,
    backend_id: AccountId,
    #[serde(skip_serializing)]
    signer: Account,
}

impl Default for TestingEnvBuilder {
    fn default() -> Self {
        let alice = get_alice();

        Self {
            total_supply: (3_000_000_000 * ONE_LIS).into(),
            constant_fee: ONE_LIS.into(),
            percent_fee: 10,
            beneficiary_id: alice.id().clone(),
            backend_id: alice.id().clone(),
            signer: alice,
        }
    }
}

impl TestingEnvBuilder {
    pub async fn build(self) -> (Contract, TestWorker) {
        let worker = workspaces::testnet()
            .await
            .expect("Fail connect to testnet");
        let wasm = std::fs::read(WASM_FILE).expect("No wasm file found");
        let contract = worker
            .dev_deploy(&wasm)
            .await
            .expect("Fail to deploy contract");
        self.signer
            .call(&worker, contract.id(), "new")
            .args_json(&serde_json::to_value(&self).expect("Fail to serialize input"))
            .expect("Invalid input args")
            .transact()
            .await
            .expect("Fail to init contract");

        (contract, worker)
    }

    pub fn set_total_supply(mut self, amount: u128) -> Self {
        self.total_supply = amount.into();
        self
    }

    pub fn set_constant_fee(mut self, amount: u128) -> Self {
        self.constant_fee = amount.into();
        self
    }

    pub fn set_percent_fee(mut self, amount: u8) -> Self {
        self.percent_fee = amount;
        self
    }

    pub fn set_beneficiary(mut self, account_id: AccountId) -> Self {
        self.beneficiary_id = account_id;
        self
    }

    pub fn set_backend(mut self, account_id: AccountId) -> Self {
        self.backend_id = account_id;
        self
    }

    pub fn set_signer(mut self, signer: Account) -> Self {
        self.signer = signer;
        self
    }
}

pub async fn get_balance_info(account: &Account, contract: &Contract, worker: &TestWorker) -> u128 {
    get_balance_info_signed(account, account.id(), contract, worker).await
}

pub async fn get_balance_info_signed(
    signer: &Account,
    account_id: &AccountId,
    contract: &Contract,
    worker: &TestWorker,
) -> u128 {
    signer
        .call(worker, contract.id(), "get_balance_info")
        .args_json(serde_json::json!({
            "account_id": account_id,
        }))
        .expect("Invalid input args")
        .view()
        .await
        .expect("Cannon get result")
        .json::<U128>()
        .expect("Cannot parse JSON")
        .0
}

pub async fn make_transfer(
    account: &Account,
    recipient_id: &AccountId,
    amount: u128,
    contract: &Contract,
    worker: &TestWorker,
) -> anyhow::Result<CallExecutionDetails> {
    account
        .call(&worker, contract.id(), "transfer")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "amount": U128(amount)
        }))
        .expect("Invalid input args")
        .transact()
        .await
}

pub async fn create_lockup_for_account(
    account: &Account,
    recipient_id: &AccountId,
    amount: u128,
    duration: Option<U64>,
    contract: &Contract,
    worker: &TestWorker,
) -> Timestamp {
    account
        .call(&worker, contract.id(), "create_lockup")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "amount": U128(amount),
            "duration": duration
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Cannon get result")
        .json::<U64>()
        .expect("Cannot parse JSON")
        .0
}

pub async fn get_lockup_info(
    account: &Account,
    contract: &Contract,
    worker: &TestWorker,
) -> Vec<LockupInfo> {
    get_lockup_info_signed(account, account.id(), &contract, &worker).await
}

pub async fn get_lockup_info_signed(
    signer: &Account,
    account_id: &AccountId,
    contract: &Contract,
    worker: &TestWorker,
) -> Vec<LockupInfo> {
    signer
        .call(&worker, contract.id(), "lockups_info")
        .args_json(serde_json::json!({
            "account_id": account_id,
        }))
        .expect("Invalid input args")
        .view()
        .await
        .expect("Cannot get result")
        .json()
        .expect("Cannot parse JSON")
}

pub async fn claim_all_lockup_for_account(
    account: &Account,
    contract: &Contract,
    worker: &Worker<Testnet>,
) -> u128 {
    account
        .call(&worker, contract.id(), "claim_all_lockup")
        .gas(MAX_GAS)
        .args_json(serde_json::json!({}))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Cannon get result")
        .json::<U128>()
        .expect("Cannot parse JSON")
        .0
}

pub async fn claim_lockup_for_account(
    account: &Account,
    contract: &Contract,
    worker: &Worker<Testnet>,
    amount: U128,
) -> u128 {
    account
        .call(&worker, contract.id(), "claim_lockup")
        .args_json(serde_json::json!({ "amount": amount }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Cannon get result")
        .json::<U128>()
        .expect("Cannot parse JSON")
        .0
}

pub async fn refund_lockup_for_account(
    account: &Account,
    contract: &Contract,
    worker: &Worker<Testnet>,
    recipient_id: &AccountId,
    expire_on: u64,
) -> u128 {
    account
        .call(&worker, contract.id(), "refund_lockup")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "expire_on": expire_on
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Cannon get result")
        .json::<U128>()
        .expect("Cannot parse JSON")
        .0
}

pub async fn create_n_lockups_for_account(
    signer: &Account,
    recipient_id: &AccountId,
    amount: u128,
    duration: Option<U64>,
    n: u64, // The n lockups will be created
    contract: &Contract,
    worker: &TestWorker,
) -> Vec<u64> {
    let mut transaction = signer.batch(&worker, contract.id());
    let mut timestamps = vec![];
    let duration = duration.unwrap_or_else(|| U64(3 * DAY)).0;

    for index in 1..=n {
        // We have to use index here to identify unique calls
        // (in other case it will create only one lockup)
        transaction = transaction.call(
            Function::new("create_lockup")
                .args_json(serde_json::json!({
                      "recipient_id": recipient_id,
                      "amount": U128(amount),
                      "duration": U64(duration + index)
                }))
                .expect("Cannot make JSON"),
        );
        timestamps.push(index);
    }
    let transaction_result = transaction
        .transact()
        .await
        .expect("Can't transact")
        .json::<U64>()
        .expect("Can`t parse JSON")
        .0;

    // Return obtained timestamps
    timestamps
        .iter()
        .map(|elem| elem + transaction_result - n)
        .collect::<Vec<u64>>()
}

pub async fn make_backend_transfer(
    signer: &Account,
    recipient_id: &AccountId,
    amount: u128,
    contract: &Contract,
    worker: &TestWorker,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(&worker, contract.id(), "backend_transfer")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "amount": U128(amount)
        }))
        .expect("Invalid input args")
        .transact()
        .await
}

pub async fn add_to_backends(
    signer: &Account,
    account_ids: Vec<&AccountId>,
    contract: &Contract,
    worker: &TestWorker,
) -> anyhow::Result<CallExecutionDetails> {
    signer
        .call(&worker, contract.id(), "owner_add_backends")
        .args_json(serde_json::json!({
            "account_ids": account_ids,
        }))
        .expect("Invalid input args")
        .transact()
        .await
}

// Returns staked xLis, takes as amount LIS
pub async fn make_stake(
    signer: &Account,
    amount: u128,
    contract: &Contract,
    worker: &TestWorker,
) -> u128 {
    signer
        .call(&worker, contract.id(), "stake")
        .args_json(serde_json::json!({
            "amount": U128(amount),
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Can't get result")
        .json::<U128>()
        .expect("Can't parse JSON")
        .0
}

// Returns unstaked LIS, takes as amount xLIS
pub async fn make_unstake(
    signer: &Account,
    x_amount: u128,
    contract: &Contract,
    worker: &TestWorker,
) -> u128 {
    signer
        .call(&worker, contract.id(), "unstake")
        .args_json(serde_json::json!({
            "x_amount": U128(x_amount),
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Can't get result")
        .json::<U128>()
        .expect("Can't parse JSON")
        .0
}

// Returns added LIS, takes as amount LIS
pub async fn make_add_to_pool(
    signer: &Account,
    amount: u128,
    contract: &Contract,
    worker: &TestWorker,
) -> u128 {
    signer
        .call(&worker, contract.id(), "owner_add_to_staking_pool")
        .args_json(serde_json::json!({
            "amount": U128(amount),
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Can't get result")
        .json::<U128>()
        .expect("Can't parse JSON")
        .0
}

// Takes as time timestamp
pub async fn set_def_staking_lockup_time(
    signer: &Account,
    time: u64,
    contract: &Contract,
    worker: &TestWorker,
) -> CallExecutionDetails {
    signer
        .call(&worker, contract.id(), "owner_set_default_lockup_time")
        .args_json(serde_json::json!({
            "time": U64(time),
        }))
        .expect("Invalid input args")
        .transact()
        .await
        .expect("Can't get result")
}
