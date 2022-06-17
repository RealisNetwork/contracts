#[cfg(test)]
mod tests {
    use crate::test::integration_test_utils::integration_tests_utils::*;
    ///Batch transactions with two 'calls',
    /// second should override the state of first.
    ///
    /// # Example
    /// ```
    ///  let (contract, worker) = deploy_contract("".to_string(), "".to_string()).await?;
    ///
    ///  contract.batch(&worker).call(Function::new( "mint")
    ///             .args_json(&serde_json::json!({
    ///                 "recipient_id":"account_id",
    ///                 "nft_metadata":"metadata"
    ///                 ,
    ///             }))?)
    ///             .call(Function::new( "mint")
    ///                     .args_json(&serde_json::json!({
    ///                 "recipient_id":"account_id",
    ///                 "nft_metadata":"metadata"
    ///                 ,
    ///             }))?)
    ///             .transact()
    ///             .await?;
    ///
    ///
    /// ```

    #[tokio::test]
    ///mint nft,start auction then try to sale the same nft.
    async fn auction() -> anyhow::Result<()> {
        Result::Ok(())
    }

    #[tokio::test]
    ///mint nft,start auction then make a bid and check your balance.
    async fn change_balance() -> anyhow::Result<()> {
        Result::Ok(())
    }
}
