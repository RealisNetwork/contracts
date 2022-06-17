#[tokio::test]
#[ignore]
async fn not_contract_owner_create_lockup() {
	// Setup contract: Alice - owner

	// Bob create lockup
	// Assert error
	todo!()
}

#[tokio::test]
#[ignore]
async fn not_enough_balance_to_create_lockup() {
	// Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

	// Alice create lockup for Bob with amount - 3_000_000_001 LIS
	// Assert error
	todo!()
}

#[tokio::test]
#[ignore]
async fn create_lockup() {
	// Setup contract: Alice - owner, total-supply - 3_000_000_000 LIS

	// Alice create lockup for Bob with amount - 3_000 LIS
	// Assert Bob has lockup
	// Assert amount
	// Assert timestamp == default

	// Alice create lockup for Charlie with amount - 150 LIS
	// Assert Charlie has lockup
	// Assert amount
	// Assert timestamp == default

	// Alice create lockup for Bob with amount - 300 LIS
	// Assert Bob has 2 lockups
	// Assert amounts
	// Assert timestamp == default
}

#[tokio::test]
#[ignore]
async fn create_lockup_with_duration() {
	todo!()
}