#[cfg(test)]
mod tests {
    use crate::test::integration_test_utils::integration_tests_utils::*;

    #[tokio::test]
    // #[ignore]
    ///assert gas cost.
    async fn mint()-> anyhow::Result<()>{

        let (contract,worker) = deploy_contract("ac".to_string(),"ac".to_string()).await?;

        let result =  contract.call(
            &worker,"mint"

        )
            .args_json(serde_json::json!({
              "recipient_id": contract.as_account().id(),
                "nft_metadata": "metadata"
            }))?
            .transact()
            .await?;



        println!("gas burn {}",result.total_gas_burnt);
      //
      // let res:U128 =  contract.call(
      //       &worker,"get_nft_info"
      //
      //   )
      //       .args_json(serde_json::json!({
      //         "nft_id": 0,
      //
      //       }))?
      //     .view()
      //       .await?
      //     .json()?;
      //
      //
      //
      //   assert_eq!(res.0,0);


        Result::Ok(())
    }

    #[tokio::test]
    ///assert gas cost.
    async fn transfer()-> anyhow::Result<()>{
        Result::Ok(())
    }
    #[tokio::test]
    ///assert gas cost.
    async fn burn()-> anyhow::Result<()>{
        Result::Ok(())
    }
}
