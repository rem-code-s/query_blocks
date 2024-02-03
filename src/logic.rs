use std::collections::HashMap;

use async_recursion::async_recursion;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_agent::{Agent, AgentError};
use ic_utils::{call::SyncCall, Canister};

use crate::ledger_types::{Account, GetTransactionsResponse, Transaction};

#[derive(CandidType, Deserialize)]
struct Args {
    pub start: Nat,
    pub length: Nat,
}

#[derive(CandidType, Deserialize)]
struct TransactionResult {
    transactions: Vec<Transaction>,
}

pub static LEDGER_CANISTER_ID: &str = "zfcdd-tqaaa-aaaaq-aaaga-cai";

#[async_recursion]
pub async fn get_transactions() -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = vec![];

    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .build()
        .expect("failed to create agent");

    let canister = Canister::<'_>::builder()
        .with_agent(&agent)
        .with_canister_id("zfcdd-tqaaa-aaaaq-aaaga-cai") // ledger
        .build()
        .expect("failed to create canister");

    // ledger
    let call: Result<(GetTransactionsResponse,), AgentError> = canister
        .query("get_transactions")
        .with_arg(Args {
            start: Nat::from(0),
            length: Nat::from(317311),
        })
        .build::<(GetTransactionsResponse,)>()
        .call()
        .await;

    let data = call.unwrap().0;
    let archived = data.archived_transactions;
    let not_archived_transactions = data.transactions;

    for t in archived {
        let callback_transactions = get_callback_transactions(
            t.callback.0.principal.to_string().as_str(),
            t.callback.0.method.as_str(),
            Args {
                start: t.start,
                length: t.length,
            },
        )
        .await;

        for t in callback_transactions {
            transactions.push(t);
        }
    }

    for t in not_archived_transactions {
        transactions.push(t);
    }

    transactions
}

async fn get_callback_transactions(principal: &str, method: &str, args: Args) -> Vec<Transaction> {
    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .build()
        .expect("failed to create agent");

    let canister = Canister::<'_>::builder()
        .with_agent(&agent)
        .with_canister_id(principal) // ledger
        .build()
        .expect("failed to create canister");

    // ledger
    let call: Result<(TransactionResult,), AgentError> = canister
        .query(method)
        .with_arg(args)
        .build::<(TransactionResult,)>()
        .call()
        .await;

    let data = call.unwrap().0;

    data.transactions

    // archive
    // let call: Result<(TransactionResult,), AgentError> = canister
    //     .query("get_transactions")
    //     .with_arg(Args {
    //         start: Nat::from(0),
    //         length: Nat::from(1000000),
    //     })
    //     .build::<(TransactionResult,)>()
    //     .call()
    //     .await;

    // call.unwrap().0.transactions
}

pub async fn get_accounts() -> Vec<Account> {
    let transactions = get_transactions().await;

    // Use a HashMap to deduplicate accounts by owner
    let mut accounts: HashMap<Principal, Account> = HashMap::new();

    // I assume that only mint and transfer transactions can create balances
    for transaction in transactions {
        if let Some(data) = transaction.mint {
            accounts.insert(data.to.owner, data.to);
        }
        if let Some(data) = transaction.transfer {
            accounts.insert(data.to.owner, data.to);
            accounts.insert(data.from.owner, data.from);
        }
        // dont think this is needed
        if let Some(data) = transaction.burn {
            accounts.insert(data.from.owner, data.from);
            data.spender
                .map(|spender| accounts.insert(spender.owner, spender));
        }
    }

    accounts
        .into_iter()
        .map(|(_, account)| account)
        .collect::<Vec<Account>>()
}

pub async fn get_principals() -> Vec<String> {
    let accounts = get_accounts().await;
    accounts
        .into_iter()
        .map(|account| account.owner.to_string())
        .collect()
}
