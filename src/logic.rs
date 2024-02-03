use std::collections::HashMap;

use candid::{CandidType, Deserialize, Nat, Principal};
use ic_agent::{Agent, AgentError};
use ic_utils::{call::SyncCall, Canister};

use crate::types::{Account, GetTransactionsResponse, Transaction};

#[derive(CandidType, Deserialize)]
struct Args {
    pub start: Nat,
    pub length: Nat,
}

pub async fn get_transactions() -> Vec<Transaction> {
    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .build()
        .expect("failed to create agent");

    let canister = Canister::<'_>::builder()
        .with_agent(&agent)
        .with_canister_id("zfcdd-tqaaa-aaaaq-aaaga-cai")
        .build()
        .expect("failed to create canister");

    let call: Result<(GetTransactionsResponse,), AgentError> = canister
        .query("get_transactions")
        .with_arg(Args {
            start: Nat::from(316000),  // <------------------
            length: Nat::from(317065), // <------------------
        })
        .build::<(GetTransactionsResponse,)>()
        .call()
        .await;

    call.unwrap().0.transactions.into_iter().collect()
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
