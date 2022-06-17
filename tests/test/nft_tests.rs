#[cfg(test)]
mod tests {
    use crate::test::integration_test_utils::integration_tests_utils::*;

    #[tokio::test]
    // #[ignore]
    ///assert gas cost.
    async fn mint() -> anyhow::Result<()> {
        let (contract, worker) = deploy_contract("ac".to_string(), "ac".to_string()).await?;

        let result = contract.call(
            &worker, "mint",
        )
            .args_json(serde_json::json!({
              "recipient_id": contract.as_account().id(),
                "nft_metadata": "metadata"
            }))?
            .transact()
            .await?;


        println!("gas burn {}", result.total_gas_burnt);


        Result::Ok(())
    }

    async fn mint_to_not_exist_acc() {
        //TODO:mint to not exist acc,assert exception
    }

    async fn transfer_to_not_exist_acc() {}

    async fn burn_not_your_nft() {}


    async fn burn_not_exist_nft() {}


    async fn sale_not_your_nft() {}


    async fn sale_nft_already_on_sale() {}

    async fn buy_your_own_nft() {}


    async fn start_auction_with_not_your_nft() {}

    async fn start_auction_with_not_exist_nft() {}

    async fn make_bid_to_own_nft() {}

    async fn make_bid_with_out_money() {}

    async fn confirm_not_your_deal() {}



}
