#[tokio::test]
#[ignore]
async fn transfer_from_not_exist_account() {
	// Setup contract

	// Transfer from non exist account

	// Assert error
	todo!()
}

#[tokio::test]
#[ignore]
async fn transfer_more_than_account_balance() {
	// Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

	// Alice transfer to Bob 1_000 LIS
	// Assert Alice have 2_999_999_000 LIS
	// Assert Bob have 1_000 LIS

	// Bob transfer to Charlie 1_001 LIS
	// Assert error

	// Alice transfer to Charlie 3_000_000_000 LIS
	// Assert error
	todo!()
}

#[tokio::test]
#[ignore]
async fn transfer_zero_amount() {
	// Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

	// Alice transfer to Bob 0 LIS
	// Assert error
	todo!()
}

#[tokio::test]
#[ignore]
async fn transfer() {
	// Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

	// Alice transfer to Bob 1_000 LIS
	// Assert Alice have 2_999_999_000 LIS
	// Assert Bob have 1_000 LIS

	// Alice transfer to Charlie 13 LIS
	// Assert Alice have 2_999_998_987 LIS
	// Assert Charlie have 13 LIS

	// Bob transfer to Charlie 100 LIS
	// Assert Bob have 900 LIS
	// Assert Charlie have 113 LIS

	// Charlie transfer to Dave 1 LIS
	// Assert Charlie have 112 LIS
	// Assert Dave have 1 LIS
	todo!()
}

// TODO: tests with expired lockups