use near_sdk::{json_types::U128, serde::Serialize, serde_json, serde_json::Value, Timestamp};
use realis_near::lockup::LockupInfo;
use workspaces::{network::DevAccountDeployer, result::CallExecutionDetails};
pub use workspaces::{network::Testnet, Account, AccountId, Contract, Worker};

pub const WASM_FILE: &str = "./target/wasm32-unknown-unknown/release/realis_near.wasm";
pub const ONE_LIS: u128 = 1_000_000_000_000;

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

pub struct BackendAccount;

impl BackendAccount {
    pub fn get_root() -> Account {
        Account::from_file("./tests/res/backend.realis.testnet.json")
    }

    pub fn get_user1() -> Account {
        Account::from_file("./tests/res/backend_access_keys/user1_backend.realis.testnet.json")
    }

    pub fn get_user2() -> Account {
        Account::from_file("./tests/res/backend_access_keys/user2_backend.realis.testnet.json")
    }

    pub fn get_user3() -> Account {
        Account::from_file("./tests/res/backend_access_keys/user3_backend.realis.testnet.json")
    }

    pub fn get_user4() -> Account {
        Account::from_file("./tests/res/backend_access_keys/user4_backend.realis.testnet.json")
    }

    pub fn get_account_id(account: &Account) -> AccountId {
        todo!()
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
    pub async fn build(self) -> (Contract, Worker<Testnet>) {
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

pub async fn balance_info(
    account: &Account,
    contract: &Contract,
    worker: &Worker<Testnet>,
) -> u128 {
    let view_result = account
        .call(worker, contract.id(), "get_balance_info")
        .args_json(serde_json::json!({
            "account_id": account.id(),
        }))
        .expect("Invalid input args")
        .view()
        .await;

    view_result
        .unwrap()
        .json::<Value>()
        .unwrap()
        .as_str()
        .unwrap()
        .parse::<u128>()
        .unwrap()
}

pub async fn make_transfer(
    account: &Account,
    recipient_id: &AccountId,
    amount: U128,
    contract: &Contract,
    worker: &Worker<Testnet>,
) -> anyhow::Result<CallExecutionDetails> {
    account
        .call(&worker, contract.id(), "transfer")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "amount": amount
        }))
        .expect("Invalid input args")
        .transact()
        .await
}

pub async fn make_lockup(
    account: &Account,
    recipient_id: &AccountId,
    amount: U128,
    duration: Option<Timestamp>,
    contract: &Contract,
    worker: &Worker<Testnet>,
) -> anyhow::Result<CallExecutionDetails> {
    account
        .call(&worker, contract.id(), "create_lockup")
        .args_json(serde_json::json!({
            "recipient_id": recipient_id,
            "amount": amount,
            "duration": duration
        }))
        .expect("Invalid input args")
        .transact()
        .await
}

pub async fn lockup_info(
    account: &Account,
    from_index: &Option<usize>,
    limit: &Option<usize>,
    contract: &Contract,
    worker: &Worker<Testnet>,
) -> Vec<LockupInfo> {
    let view_result = account
        .call(&worker, contract.id(), "lockups_info")
        .args_json(serde_json::json!({
            "account_id": account.id(),
            "from_index": from_index,
            "limit": limit
        }))
        .expect("Invalid input args")
        .view()
        .await;

    view_result.unwrap().json::<Vec<LockupInfo>>().unwrap()
}
