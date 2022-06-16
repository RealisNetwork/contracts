#[cfg(test)]
mod tests {
    use crate::test::integration_test_utils::integration_tests_utils::*;

    #[tokio::test]
    ///check if operation correct, assert the gas costs.
    async fn transfer_tokens() -> anyhow::Result<()> {
        let (contract, worker) = deploy_contract("ac".to_string(), "ac".to_string()).await?;

        let result = contract.call(
            &worker, "internal_transfer",
        )
            .args_json(serde_json::json!({
              "recipient_id": "new_acc",
                "sender": contract.as_account().id(),
                "amount":10,
                "is_fee_required":false
            }))?
            .transact()
            .await?;
        assert!(result.total_gas_burnt / 1000000000000 < 10);
        println!("gas burn {}", result.total_gas_burnt);

        let result: U128 = contract.call(
            &worker, "get_balance_info",
        )
            .args_json(serde_json::json!({
                "account_id":"new_acc"

            }))?
            .view()
            .await?
            .json()?;
        println!("account balance  {}", result.0);


        Result::Ok(())
    }

    pub fn transfer_from_not_exist_acc(){
        //TODO:transfer from not existed acc
    }

    pub fn transfer_with_negative_sum(){
        //TODO:transfer_with_negative_sum
    }




}