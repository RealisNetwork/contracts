use near_sdk::serde_json;
use std::collections::HashMap;
use test_utils::{token, utils::*, SandboxEnvironment};

#[tokio::test]
async fn claim() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lock
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": amount.to_string(),
            "msg": serde_json::json!({
                "duration": HOUR,
                "account_id": sandbox.owner.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    worker.fast_forward(10000).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": sandbox.owner.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );
    sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index":lockup_index,
        }))
        .transact()
        .await?
        .into_result()?;
    assert_eq!(
        3_000_000_000_u128 * LIS,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn claim_for_other_account() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;
    sandbox.owner
        .call(sandbox.token.id(), "storage_deposit")
        .deposit(NEAR)
        .args_json(serde_json::json!({
            "account_id": user.id()
        }))
        .transact()
        .await?
        .into_result()?;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lock
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": amount.to_string(),
            "msg": serde_json::json!({
                "duration": SECOND,
                "account_id": user.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    worker.fast_forward(100).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": user.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );
    sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index":lockup_index,
            "account_id": user.id(),
        }))
        .transact()
        .await?
        .into_result()?;
    assert_eq!(
        amount,
        token::ft_balance_of(&sandbox.token, user.id()).await?
    );
    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn claim_error_transfer_tokens() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lock
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": amount.to_string(),
            "msg": serde_json::json!({
                "duration": HOUR,
                "account_id": user.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    worker.fast_forward(10000).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": user.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    // `user` do not have storage deposit on token contract
    // so claim should fail
    let result = user
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index":lockup_index,
        }))
        .transact()
        .await?
        .into_result();
    assert!(result.is_ok());
    let outcome = result.unwrap();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("The account user.test.near is not registered"));
    assert_eq!(0, token::ft_balance_of(&sandbox.token, user.id()).await?);

    Ok(())
}

#[tokio::test]
async fn claim_not_expired_lockup() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
        "receiver_id": sandbox.staking.id(),
        "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lock
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": amount.to_string(),
            "msg": serde_json::json!({
                "duration": HOUR,
                "account_id": sandbox.owner.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    // wait not long enought
    worker.fast_forward(100).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": sandbox.owner.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    let result = sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
        "index":lockup_index,
        }))
        .transact()
        .await?
        .into_result();
    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("Lockup isn't expired"));
    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn claim_not_own_lockup() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;
    let user = sandbox
        .owner
        .create_subaccount("user")
        .initial_balance(10 * NEAR)
        .transact()
        .await?
        .into_result()?;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lockups
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": (amount / 2).to_string(),
            "msg": serde_json::json!({
                "duration": HOUR,
                "account_id": user.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
        "receiver_id": sandbox.lockup.id(),
        "amount": (amount / 2).to_string(),
        "msg": serde_json::json!({
            "duration": HOUR,
            "account_id": sandbox.owner.id(),
        }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    worker.fast_forward(10000).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": user.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    let result = sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index":lockup_index,
        }))
        .transact()
        .await?
        .into_result();
    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("No such lockup for this account"));
    assert_eq!(0, token::ft_balance_of(&sandbox.token, user.id()).await?);
    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );

    Ok(())
}

#[tokio::test]
async fn claim_nothing() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;

    let result = sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index": 0,
        }))
        .transact()
        .await?
        .into_result();
    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("No lockups found"));

    Ok(())
}

#[tokio::test]
async fn claim_not_existed_lockup() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let sandbox = SandboxEnvironment::new(&worker).await?;
    let amount = 100 * LIS;

    // Only staking contract can create lockups
    // so transfer funds to staking contract
    sandbox
        .owner
        .call(sandbox.token.id(), "ft_transfer")
        .deposit(YOCTO)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.staking.id(),
            "amount": amount.to_string(),
        }))
        .transact()
        .await?
        .into_result()?;

    // Create lockups
    sandbox
        .staking
        .as_account()
        .call(sandbox.token.id(), "ft_transfer_call")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "receiver_id": sandbox.lockup.id(),
            "amount": amount.to_string(),
            "msg": serde_json::json!({
                "duration": HOUR,
                "account_id": sandbox.owner.id(),
            }).to_string()
        }))
        .transact()
        .await?
        .into_result()?;

    worker.fast_forward(10000).await?;

    let lockups: HashMap<u32, serde_json::Value> = sandbox
        .owner
        .view(
            sandbox.lockup.id(),
            "get_account_lockups",
            serde_json::json!({
                "account_id": sandbox.owner.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let lockup_index = lockups.keys().next().expect("No lockups created!");

    let result = sandbox
        .owner
        .call(sandbox.lockup.id(), "claim")
        .deposit(YOCTO)
        .gas(300000000000000)
        .args_json(serde_json::json!({
            "index":lockup_index + 1,
        }))
        .transact()
        .await?
        .into_result();
    assert!(result.is_err());
    let outcome = result.unwrap_err();
    let failure = outcome.receipt_failures()[0];
    assert!(failure.is_failure());
    assert!(format!("{:?}", failure).contains("No such lockup for this account"));
    assert_eq!(
        3_000_000_000_u128 * LIS - amount,
        token::ft_balance_of(&sandbox.token, sandbox.owner.id()).await?
    );

    Ok(())
}
