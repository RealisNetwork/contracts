use near_sdk::{
    json_types::U128,
    serde::Serialize,
    serde_json,
    serde_json::{json, Value},
    Timestamp,
};
use realis_near::{account::AccountInfo, lockup::LockupInfo, utils::DAY};
pub use workspaces::{
    network::{DevAccountDeployer, Testnet},
    result::CallExecutionDetails,
    Account, AccountId, Contract, Worker,
};
use workspaces::{operations::Function, types::Gas};

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

// TEST FUNCTIONS
pub async fn test_call_mint_nft(
    contract: &Contract,
    worker: &Worker<Testnet>,
    acc_to_mint: &Account,
    signer_acc: &Account,
) -> u128 {
    signer_acc
        .call(&worker, contract.id(), "mint")
        .args_json(&json!({
            "recipient_id": &acc_to_mint.id(),
            "nft_metadata": "metadata",
        }))
        .expect("Mint nft, wrong args.")
        .transact()
        .await
        .expect("Mint nft, Fail to make transaction.")
        .json::<U128>()
        .expect("Mint nft, result is not OK")
        .0
}

pub async fn test_call_get_acc_info(
    account: &Account,
    worker: &Worker<Testnet>,
    contract: &Contract,
) -> AccountInfo {
    contract
        .call(&worker, "get_account_info")
        .args_json(&json!({
            "account_id": account.id()
        }))
        .expect("Get account info, wrong args.")
        .transact()
        .await
        .expect("Get account info, Fail to make transaction.")
        .json::<AccountInfo>()
        .expect("Get account info, result is not OK")
}

pub async fn test_call_insert_acc(
    account: &Account,
    worker: &Worker<Testnet>,
    contract: &Contract,
    signer: &Account,
) -> AccountInfo {
    signer
        .call(&worker, contract.id(), "insert_account")
        .args_json(&json!({
            "account_id": account.id()
        }))
        .expect("Insert account, wrong args.")
        .transact()
        .await
        .expect("Insert account , Fail to make transaction.")
        .json::<AccountInfo>()
        .expect("Insert account, result is not OK")
}

pub async fn test_call_burn_nft(
    caller_acc: &Account,
    contract: &Contract,
    nft_id: U128,
    worker: &Worker<Testnet>,
) -> anyhow::Result<CallExecutionDetails> {
    caller_acc
        .call(&worker, &contract.id(), "burn")
        .args_json(&json!({
            "nft_id": nft_id,
        }))
        .expect("Burn nft, wrong arg")
        .transact()
        .await
}

pub async fn test_call_sell_nft(
    contract: &Contract,
    worker: &Worker<Testnet>,
    seller: &Account,
    nft_id: U128,
    price: U128,
) {
    seller
        .call(&worker, contract.id(), "sell_nft")
        .args_json(&json!({"nft_id": nft_id, "price": price}))
        .expect("Invalid arguments")
        .transact()
        .await
        .expect("Cant sell NFT");
}

pub async fn test_call_transfer_nft(
    contract: &Contract,
    worker: &Worker<Testnet>,
    acc_recipient: &Account,
    acc_sender: &Account,
    nft_id: U128,
) -> anyhow::Result<CallExecutionDetails> {
    acc_sender
        .call(&worker, contract.id(), "transfer_nft")
        .args_json(&json!({"nft_id": nft_id, "recipient_id": acc_recipient.id()}))
        .expect("Transfer_nft, Invalid args")
        .transact()
        .await
}

/// Return NFT price
pub async fn test_call_get_nft_marketplace_info(
    contract: &Contract,
    worker: &Worker<Testnet>,
    nft_id: U128,
) -> U128 {
    contract
        .call(&worker, "get_nft_marketplace_info")
        .args_json(&json!({ "nft_id": nft_id }))
        .expect("Invalid arguments.")
        .transact()
        .await
        .expect("Result in not OK")
        .json::<U128>()
        .expect("Fail to parse nft")
}

pub async fn get_balance_info(account: &Account, contract: &Contract, worker: &TestWorker) -> u128 {
    account
        .call(worker, contract.id(), "get_balance_info")
        .args_json(serde_json::json!({
            "account_id": account.id(),
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
    duration: Option<Timestamp>,
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
        .json::<Timestamp>()
        .expect("Cannot parse JSON")
}

pub async fn get_lockup_info(
    account: &Account,
    contract: &Contract,
    worker: &TestWorker,
) -> Vec<LockupInfo> {
    account
        .call(&worker, contract.id(), "lockups_info")
        .args_json(serde_json::json!({
            "account_id": account.id(),
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
    expire_on: u64,
) -> u128 {
    account
        .call(&worker, contract.id(), "claim_lockup")
        .args_json(serde_json::json!({ "expire_on": expire_on }))
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
    duration: Option<Timestamp>,
    n: u64, // The n lockups will be created
    contract: &Contract,
    worker: &TestWorker,
) -> Vec<u64> {
    let mut transaction = signer.batch(&worker, contract.id());
    let mut timestamps = vec![];

    for index in 1..=n {
        // We have to use index here to identify unique calls
        // (in other case it will create only one lockup)
        transaction = transaction.call(
            Function::new("create_lockup")
                .args_json(serde_json::json!({
                      "recipient_id": recipient_id,
                      "amount": U128(amount),
                      "duration": duration.unwrap_or(3 * DAY) + index
                }))
                .expect("Cannot make JSON"),
        );
        timestamps.push(index);
    }
    let transaction_result = transaction
        .transact()
        .await
        .expect("Can't transact")
        .json::<u64>()
        .expect("Can`t parse JSON");

    // Return obtained timestamps
    timestamps
        .iter()
        .map(|elem| elem + transaction_result - n)
        .collect::<Vec<u64>>()
}
