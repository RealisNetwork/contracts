#[tokio::test]
#[ignore]
async fn create_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with amount - 3_000 LIS
    // Assert Bob has lockup
    // Assert amount
    // Assert timestamp == default
    // Assert Alice balance

    // Alice create lockup for Charlie with amount - 150 LIS
    // Assert Charlie has lockup
    // Assert amount
    // Assert timestamp == default
    // Assert Alice balance

    // Alice create lockup for Bob with amount - 300 LIS
    // Assert Bob has 2 lockups
    // Assert amounts
    // Assert timestamp == default
    // Assert Alice balance
}

#[tokio::test]
#[ignore]
async fn create_lockup_with_duration() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount = 10 LIS
    // Assert duration = 1 DAY
    // Assert Alice balance
}

#[tokio::test]
#[ignore]
async fn user_claim_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 10 SECONDS, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount = 10 LIS
    // Assert duration = 10 SECONDS
    // Assert Alice balance

    // Wait while lockup time is expired
    // Bob claim lockup

    // Assert Bob's balance = 10 LIS

    // Alice create lockup for Bob with duration = 5 SECONDS, amount = 10 LIS
    // Alice create lockup for Bob with duration = 10 SECONDS, amount = 15 LIS
    // Alice create lockup for Bob with duration = 15 SECONDS, amount = 15 LIS
    // Assert Bob has 3 lockups
    // Assert amounts
    // Assert durations
    // Assert Alice balance

    // Wait while all lockups is expired
    // Bob claim 15 LIS

    // Assert Bob has 2 lockup
    // Assert amounts
    // Assert balance
}

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
async fn user_claimed_not_expired_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount = 10 LIS
    // Assert duration = 1 DAY
    // Assert Alice balance

    // Bob claim lockup
    // Assert error
}

#[tokio::test]
#[ignore]
async fn claim_expired_lockup_when_other_not_expired() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration = 1 SECOND, amount = 10 LIS
    // Alice create lockup for Bob with duration = 1 DAY, amount = 10 LIS
    // Assert Bob has 2 lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait 1 SECOND
    // Bob claim lockup

    // Assert Bob has 1 lockup
    // Assert Bob balance
}

#[tokio::test]
#[ignore]
async fn refund_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Bob refund lockup
    // Assert Bob doesn't have lockup
    // Assert Bob balance
    // Assert Alice balance

    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Alice create lockup for Bob with duration default, amount = 10 LIS
    // Assert Bob has 3 lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Bob refund 1 lockup
    // Assert Bob has 2 lockup
    // Assert amount
    // Assert duration
    // Assert Bob balance
    // Assert Alice balance
}

#[tokio::test]
#[ignore]
async fn refund_not_own_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

}

#[tokio::test]
#[ignore]
async fn refund_non_existed_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Bob refund lockup
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create 10 lockups for Bob with duration 1 min, amount - 10 LIS
    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait while lockups time is expired
    // Bob claim all lockups
    // Assert Bob balance = 100 LIS.

    // Alice create 10 lockups for Bob with duration 1 min, amount for 5 = 10 LIS, the others = 20 LIS
    // Assert Bob has lockups
    // Assert amount
    // Assert duration
    // Assert Alice balance

    // Wait while lockups time is expired
    // Bob claim all lockups with amount = 20 LIS
    // Assert Bob balance = 200 LIS

    // Assert Alice balance = 2_999_999_600 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_without_lockups() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Assert Bob hasn't lockup
    // Bob claim all lockups without error
    // Assert Bob balance = 0 LIS.
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_one_lockup() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockup for Bob with duration 10 seconds, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_990 LIS

    // Bob claim all lockups
    // Assert Bob hasn't lockups
    // Assert Bob balance = 10 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_non_expired_time() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockups for Bob with duration default, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_900 LIS

    // Bob claim all lockups
    // Assert Bob has lockups
    // Assert Bob balance = 0 LIS
}

#[tokio::test]
#[ignore]
async fn claim_all_lockups_with_partially_expired_time() {
    // Setup contract: Alice - owner, total_supply - 3_000_000_000 LIS

    // Alice create lockups for Bob:
    // 5 with duration 10 seconds, amount = 10 LIS
    // 5 with duration default, amount = 10 LIS

    // Assert Bob has lockup
    // Assert amount
    // Assert duration
    // Assert Alice balance = 2_999_999_900 LIS

    // Bob claim all lockups
    // Assert Bob has 5 lockups
    // Assert Bob balance = 50 LIS
}
