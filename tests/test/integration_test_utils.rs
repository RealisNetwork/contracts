

pub mod integration_tests_utils {
    pub use std::str::FromStr;

    pub use near_sdk::json_types::U128;
    pub use near_sdk::serde_json;
    pub use workspaces::{AccountId, Contract, Worker};
    pub use workspaces::network::DevAccountDeployer;
    pub use workspaces::network::Testnet;
    use workspaces::operations::Function;

    pub const WASM_PATH: &str = "/Users/glebprotasov/Desktop/Development/WorkSpaceRust/Realis_projects/nearContract/contracts/target/wasm32-unknown-unknown/release/realis_near.wasm";

    /// RUNNING OF SEVERAL TESTS IN THE SAME TIME COULD LEAD TO 'dispatch dropped without returning error'.
    /// Run all integration tests with command  `cargo test --test mod` .
    /// Func return initialized and ready to use contract.
    ///
    ///
    /// # Return Fields
    ///  * `Worker` -define where environment is running,
    ///            testnet,mainnet or whatever.
    ///  * `Contract` -struct for interacting with contract,
    ///             such as using call and view func or delete contract.
    /// # Examples
    ///
    ///  Call func:
    /// ```
    ///  let(contract,worker) = deploy_contract().await?;
    ///
    /// contract.call(&worker,"func_name")
    /// .args_json(
    ///           &serde_json({
    ///                        func args
    ///  })
    ///           )?
    /// .transact()?;
    /// ```
    ///  View func:
    /// ```
    /// let(contract,worker) = deploy_contract().await?;
    ///
    ///let result: serde_json::Value = contract
    ///         .call(&worker, "get_nft_info")
    ///  .args_json(
    ///           &serde_json({
    ///                        func args
    ///  })
    /// .
    ///         .view()
    ///         .await?
    ///         .json()?;
    ///
    /// ```
    ///
    ///
    ///
    pub async fn deploy_contract(beneficiary_id: String, backend_id: String) -> anyhow::Result<(Contract, Worker<Testnet>)> {
        let worker = workspaces::testnet().await?;
        let wasm = std::fs::read(WASM_PATH)?;
        let contract = worker.dev_deploy(&wasm).await?;
        println!("Contract id {}", contract.id());
        init_contract(beneficiary_id, backend_id, &worker, &contract).await?;


        Result::Ok((contract, worker))
    }
    /// Initialization of contract.
  async fn init_contract(beneficiary_id: String, backend_id: String, worker: &Worker<Testnet>, contract: &Contract) -> anyhow::Result<()> {
        let result = contract
            .call(
                worker,
                "new")
            .args_json(
                &serde_json::json!({
                    "total_supply":"30000000",
                    "constant_fee":"5",
                    "percent_fee":10,
                    "beneficiary_id":beneficiary_id,
                    "backend_id":backend_id,

                })
            )?
            .transact()
            .await?;

        println!("Deploy contact gas burnt {:?}", result.total_gas_burnt / 1000000000000);

        Result::Ok(())
    }
}