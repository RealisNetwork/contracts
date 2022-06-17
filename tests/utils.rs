use near_sdk::{json_types::U128, serde::Serialize, AccountId};
pub use workspaces::{network::Testnet, Account, Contract, Worker};

pub fn get_alice() -> Account {
    Account::from_file("./tests/res/realis_alice.testnet.json")
}

pub fn get_bob() -> Account {
    Account::from_file("./tests/res/realis_bob.testnet.json")
}

pub fn get_charlie() -> Account {
    Account::from_file("./tests/res/realis_charlie.testnet.json")
}

pub fn get_dave() -> Account {
    Account::from_file("./tests/res/realis_dave.testnet.json")
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
}
