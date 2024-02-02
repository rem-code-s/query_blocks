// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};

#[derive(CandidType, Deserialize)]
pub struct GetBlocksRequest {
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum VecItem {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(Vec<u8>),
    Text(String),
    Array(Box<Vec<VecItem>>),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Value {
    Int(candid::Int),
    Map(Vec<(String, Box<Value>)>),
    Nat(candid::Nat),
    Nat64(u64),
    Blob(Vec<u8>),
    Text(String),
    Array(Box<Vec<VecItem>>),
}

#[derive(CandidType, Deserialize)]
pub struct ArchivedRangeCallbackRet {
    pub blocks: Vec<Box<Value>>,
}

candid::define_function!(pub ArchivedRangeCallback : (GetBlocksRequest) -> (
    ArchivedRangeCallbackRet,
  ) query);
#[derive(CandidType, Deserialize, Debug)]
pub struct ArchivedRange {
    pub callback: ArchivedRangeCallback,
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct GetBlocksResponse {
    pub certificate: Option<Vec<u8>>,
    pub first_index: candid::Nat,
    pub blocks: Vec<Box<Value>>,
    pub chain_length: u64,
    pub archived_blocks: Vec<ArchivedRange>,
}

#[derive(CandidType, Deserialize)]
pub struct DataCertificate {
    pub certificate: Option<Vec<u8>>,
    pub hash_tree: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
pub struct Burn {
    pub from: Account,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Mint {
    pub to: Account,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct Approve {
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Transfer {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub from: Account,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub spender: Option<Account>,
}

#[derive(CandidType, Deserialize)]
pub struct Transaction {
    pub burn: Option<Burn>,
    pub kind: String,
    pub mint: Option<Mint>,
    pub approve: Option<Approve>,
    pub timestamp: u64,
    pub transfer: Option<Transfer>,
}

#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1CallbackRet {
    pub transactions: Vec<Transaction>,
}

candid::define_function!(pub ArchivedRange1Callback : (GetBlocksRequest) -> (
    ArchivedRange1CallbackRet,
  ) query);
#[derive(CandidType, Deserialize)]
pub struct ArchivedRange1 {
    pub callback: ArchivedRange1Callback,
    pub start: candid::Nat,
    pub length: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionsResponse {
    pub first_index: candid::Nat,
    pub log_length: candid::Nat,
    pub transactions: Vec<Transaction>,
    pub archived_transactions: Vec<ArchivedRange1>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub body: Vec<u8>,
    pub headers: Vec<(String, String)>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub body: Vec<u8>,
    pub headers: Vec<(String, String)>,
    pub status_code: u16,
}

#[derive(CandidType, Deserialize)]
pub enum MetadataValue {
    Int(candid::Int),
    Nat(candid::Nat),
    Blob(Vec<u8>),
    Text(String),
}

#[derive(CandidType, Deserialize)]
pub struct StandardRecord {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result_ {
    Ok(candid::Nat),
    Err(TransferError),
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs {
    pub account: Account,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance {
    pub allowance: candid::Nat,
    pub expires_at: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs {
    pub fee: Option<candid::Nat>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
    pub expected_allowance: Option<candid::Nat>,
    pub expires_at: Option<u64>,
    pub spender: Account,
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    AllowanceChanged {
        current_allowance: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    Expired {
        ledger_time: u64,
    },
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result1 {
    Ok(candid::Nat),
    Err(ApproveError),
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub to: Account,
    pub fee: Option<candid::Nat>,
    pub spender_subaccount: Option<Vec<u8>>,
    pub from: Account,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError {
    GenericError {
        message: String,
        error_code: candid::Nat,
    },
    TemporarilyUnavailable,
    InsufficientAllowance {
        allowance: candid::Nat,
    },
    BadBurn {
        min_burn_amount: candid::Nat,
    },
    Duplicate {
        duplicate_of: candid::Nat,
    },
    BadFee {
        expected_fee: candid::Nat,
    },
    CreatedInFuture {
        ledger_time: u64,
    },
    TooOld,
    InsufficientFunds {
        balance: candid::Nat,
    },
}

#[derive(CandidType, Deserialize)]
pub enum Result2 {
    Ok(candid::Nat),
    Err(TransferFromError),
}
