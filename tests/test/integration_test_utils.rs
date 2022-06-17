pub mod integration_tests_utils {
    pub use std::str::FromStr;

    pub use near_sdk::{json_types::U128, serde_json};
    use workspaces::operations::Function;
    pub use workspaces::{
        network::{DevAccountDeployer, Testnet},
        AccountId, Contract, Worker,
    };

    #[allow()]
    pub const WASM_PATH: &str = "/Users/glebprotasov/Desktop\
    /Development/WorkSpaceRust/Realis_projects\
    /nearContract/contracts/target/wasm32-unknown-unknown/release/realis_near.wasm";

    /// Run this tests in a single thread to avoid 'dispatch dropped without returning error'
    /// So, use for running integration tests command
    /// ` cargo test --test mod -- --nocapture --test-threads 1` .
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
    pub async fn deploy_contract(
        beneficiary_id: String,
        backend_id: String,
    ) -> anyhow::Result<(Contract, Worker<Testnet>)> {
        let worker = workspaces::testnet().await?;
        let wasm = std::fs::read(WASM_PATH)?;
        let contract = worker.dev_deploy(&wasm).await?;
        println!("Contract id {}", contract.id());
        init_contract(beneficiary_id, backend_id, &worker, &contract).await?;

        Result::Ok((contract, worker))
    }
    /// Initialization of contract.
    async fn init_contract(
        beneficiary_id: String,
        backend_id: String,
        worker: &Worker<Testnet>,
        contract: &Contract,
    ) -> anyhow::Result<()> {
        let result = contract
            .call(worker, "new")
            .args_json(&serde_json::json!({
                "total_supply":"30000000",
                "constant_fee":"5",
                "percent_fee":10,
                "beneficiary_id":beneficiary_id,
                "backend_id":backend_id,

            }))?
            .transact()
            .await?;

        println!(
            "Deploy contact gas burnt {:?}",
            result.total_gas_burnt / 1000000000000
        );

        Result::Ok(())
    }
}
