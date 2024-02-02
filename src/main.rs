use candid::{CandidType, Deserialize, Nat};
use ic_agent::{identity::Secp256k1Identity, Agent, AgentError};
use ic_utils::Canister;
use rust_call_example::types::GetBlocksResponse;

#[derive(CandidType, Deserialize)]
struct Args {
    pub start: Nat,
    pub length: Nat,
}

async fn get_data() -> () {
    let identity =
        Secp256k1Identity::from_pem_file("identity.pem").expect("failed to read pem file");

    let agent = Agent::builder()
        .with_url("https://icp0.io")
        .with_identity(identity)
        .build()
        .expect("failed to create agent");

    let canister = Canister::<'_>::builder()
        .with_agent(&agent)
        .with_canister_id("zfcdd-tqaaa-aaaaq-aaaga-cai")
        .build()
        .expect("failed to create canister");

    let call: Result<(GetBlocksResponse,), AgentError> = canister
        .update("get_blocks")
        .with_arg(Args {
            start: Nat::from(316000), // <------------------
            length: Nat::from(10),    // <------------------
        })
        .build::<(GetBlocksResponse,)>() // expected type as icc format (trailing comma)
        .call_and_wait()
        .await;

    println!("Response: {:?}", call);
}

#[tokio::main]
async fn main() {
    get_data().await;
}
