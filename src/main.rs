use ic_agent::{Agent, identity::BasicIdentity, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use candid::{Encode, Decode, CandidType, types::number::{Nat, Int}};
use serde::Deserialize;
use std::{thread, time::Duration};
use rusqlite::Connection;
use std::env;

mod types;

use types::*;

const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";
const KEY_PATH: &str = "/Users/cyan/.config/dfx/identity/default/identity.pem";
const DSWAP_STORAGE: &str = "gsf2f-kaaaa-aaaah-qaj4q-cai";
const INTERVAL: u64 = 1 * 60 * 1000; // 1 min
const INTERVAL_S: u64 = 0; // 0 s
const QUERY_NUM: usize = 100;
const QUERY_NUM_S: usize = 13000;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut begin: Nat = if args.len() < 2 {
        Nat::from(0)
    } else {
        Nat::from(args[1].parse::<u64>().expect("input not number"))
    };

    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(create_identity())
        .build()
        .expect("Failed to build the Agent");
    
    let mut num = QUERY_NUM_S;
    let mut interval = 0;
    loop {        
        let response = agent.query(&Principal::from_text(DSWAP_STORAGE).expect("Failed to convert DSWAP_STORAGE to principal"), "getHistory")
            .with_arg(&Encode!(&begin, &Nat::from(num)).unwrap())
            .call()
            .await
            .expect("Failed to call canister");

        let result = Decode!(response.as_slice(), Vec<DSwapOpRecord>).expect("Failed to decode the response data.");
        save_to_db(result.clone());
        println!("from {} to {}", begin.clone(), begin.clone() + result.len() - 1);
        if result.len() < num {
            begin += result.len();
            num = QUERY_NUM;
            interval = INTERVAL;
            thread::sleep(Duration::from_millis(interval));
        } else {
            begin += num;
            num = QUERY_NUM_S;
            interval = INTERVAL_S;
            thread::sleep(Duration::from_millis(interval));
        }
    }
}

fn save_to_db(data: Vec<DSwapOpRecord>) {
    println!("{:?}", data.len());
    let conn = Connection::open("history.db").expect("Failed to open history.db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id            INTEGER PRIMARY KEY,
            caller        TEXT NOT NULL,
            op            TEXT NOT NULL,
            indexs        INTEGER NOT NULL,
            token_id      TEXT NOT NULL,
            froma          TEXT NOT NULL,
            toa            TEXT NOT NULL,
            amount        INTEGER NOT NULL,
            amount0       INTEGER NOT NULL,
            amount1       INTEGER NOT NULL,
            timestamp     INTEGER NOT NULL
        )",
        [],
    ).expect("Failed to new history table.");

    for i in data {
        conn.execute(
            "INSERT INTO history (caller, op, indexs, token_id, froma, toa, amount, amount0, amount1, timestamp)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            &[&i.caller.to_text().to_string(), &op_to_text(i.op), &i.index.to_string(), &i.tokenId, &i.from.to_text().to_string(),
            &i.to.to_text().to_string(), &i.amount.to_string(), &i.amount0.to_string(), &i.amount1.to_string(), &i.timestamp.to_string()],
        ).expect("Failed to insert ops to historay table.");
    } 
}

fn op_to_text(o: DSwapOperation) -> String {
    match o {
        DSwapOperation::deposit => String::from("deposit"),
        DSwapOperation::withdraw => String::from("withdraw"),
        DSwapOperation::tokenTransfer => String::from("tokenTransfer"),
        DSwapOperation::tokenApprove => String::from("tokenApprove"),
        DSwapOperation::lpTransfer => String::from("lpTransfer"),
        DSwapOperation::lpApprove => String::from("lpApprove"),
        DSwapOperation::createPair => String::from("createPair"),
        DSwapOperation::swap => String::from("swap"),
        DSwapOperation::addLiquidity => String::from("addLiquidity"),
        DSwapOperation::removeLiquidity => String::from("removeLiquidity"),
    }
}

fn create_identity() -> BasicIdentity {
    BasicIdentity::from_pem_file(KEY_PATH).expect("Could not read the key pair.")
}
