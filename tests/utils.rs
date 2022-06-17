use near_sdk::{json_types::U128, serde::Serialize, AccountId};
pub use workspaces::{network::Testnet, Account, Contract, Worker};

pub fn get_alice() -> Account {
    todo!()
}

pub fn get_bob() -> Account {
    todo!()
}

pub fn get_charlie() -> Account {
    todo!()
}

pub fn get_dave() -> Account {
    todo!()
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TestingEnvBuilder {
    total_supply: U128,
    constant_fee: U128,
    percent_fee: u8,
    beneficiary_id: AccountId,
    backend_id: AccountId,
}

impl Default for TestingEnvBuilder {
    fn default() -> TestingEnvBuilder {
        todo!()
    }
}

impl TestingEnvBuilder {
    pub async fn build(self) -> (Contract, Worker<Testnet>) {
        todo!()
    }

    pub fn set_constant_fee(&mut self, amount: u128) -> Self {
        todo!()
    }

    // TODO: other setters
}
