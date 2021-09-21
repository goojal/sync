use ic_agent::{Agent, identity::BasicIdentity, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use candid::{Encode, Decode, CandidType, types::number::{Nat, Int}};
use serde::Deserialize;
use std::{fs, env, thread, time::Duration};
use rusqlite::Connection;

mod types;

use types::*;

const CONFIG_PATH: &str = "./config.json";
const DEFAULT_IC_GATEWAY: &str = "https://ic0.app";
const KEY_PATH: &str = "/Users/cyan/.config/dfx/identity/default/identity.pem";
const INTERVAL: u64 = 1 * 60 * 1000; // 1 min
const INTERVAL_S: u64 = 0; // 0 s
const QUERY_NUM: usize = 100;
const QUERY_NUM_S: usize = 13000;

#[derive(Debug, Deserialize)]
struct CanisterConfig {
    canister_id: String,
    canister_type: String,
    db: String,
}

#[tokio::main]
async fn main() {
    let config_data = fs::read_to_string(CONFIG_PATH).expect("Unable to read config file");
    let config_list: Vec<CanisterConfig> = serde_json::from_str(&config_data[..]).unwrap();
    let mut join_handle_list = vec![];
    for config in config_list {
        println!("Starting following canister: {:?}", config);
        let handle = tokio::spawn(async move {
            sync_canister(config).await;
        });
        join_handle_list.push(handle);
    }
    for handle in join_handle_list {
        handle.await.unwrap();
    }
}

async fn sync_canister(config: CanisterConfig) {
    let mut begin: Nat = Nat::from(0);

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
        if config.canister_type == String::from("token") {
            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "historySize")
                .with_arg(&Encode!(&begin, &Nat::from(num)).unwrap())
                .call()
                .await
                .expect("Failed to call canister");
            let history_size = Decode!(response.as_slice(), Nat).expect("Failed to decode the historySize response data.");

            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "getTransactions")
                .with_arg(&Encode!(&begin, &Nat::min(history_size.clone(), Nat::from(num) - begin.clone())).unwrap())
                .call()
                .await
                .expect("Failed to call canister");

            let result = Decode!(response.as_slice(), Vec<TxRecord>).expect("Failed to decode the getTransactions response data.");
            println!("Read from {} to {} from token canister", begin.clone(), begin.clone() + result.len() - 1);
            save_transactions_to_db(result.clone(), config.db.clone());

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
        } else if config.canister_type == String::from("token-burn") {
            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "historySize")
                .with_arg(&Encode!(&begin, &Nat::from(num)).unwrap())
                .call()
                .await
                .expect("Failed to call canister");
            let history_size = Decode!(response.as_slice(), Nat).expect("Failed to decode the historySize response data.");

            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "getTransactions")
                .with_arg(&Encode!(&begin, &Nat::min(history_size.clone(), Nat::from(num) - begin.clone())).unwrap())
                .call()
                .await
                .expect("Failed to call canister");

            let result = Decode!(response.as_slice(), Vec<TxRecordBurn>).expect("Failed to decode the getTransactions response data.");
            println!("Read from {} to {} from token canister", begin.clone(), begin.clone() + result.len() - 1);
            save_burn_transactions_to_db(result.clone(), config.db.clone());

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
        } else if config.canister_type == String::from("token-registry") {
            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "getTokens")
                .with_arg(&Encode!(&begin, &Nat::from(num)).unwrap())
                .call()
                .await
                .expect("Failed to call canister");

            let result = Decode!(response.as_slice(), Vec<TokenInfo>).expect("Failed to decode the getTokens response data.");
            println!("Read from {} to {} from token-registry canister", begin.clone(), begin.clone() + result.len() - 1);
            save_tokens_to_db(result.clone(), config.db.clone());

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
        } else if config.canister_type == String::from("dswap-storage") {
            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "historySize")
                .with_arg(&Encode!(&begin, &Nat::from(num)).unwrap())
                .call()
                .await
                .expect("Failed to call canister");
            let history_size = Decode!(response.as_slice(), Nat).expect("Failed to decode the historySize response data.");

            let response = agent.query(&Principal::from_text(config.canister_id.clone()).expect("Failed to convert canister_id to principal"), "getTransactions")
                .with_arg(&Encode!(&begin, &Nat::min(history_size.clone(), Nat::from(num) - begin.clone())).unwrap())
                .call()
                .await
                .expect("Failed to call canister");

            let result = Decode!(response.as_slice(), Vec<DSwapOpRecord>).expect("Failed to decode the response data.");
            println!("Read from {} to {} from dswap canister", begin.clone(), begin.clone() + result.len() - 1);
            save_dswaps_to_db(result.clone(), config.db.clone());

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
}

fn save_transactions_to_db(data: Vec<TxRecord>, db_name: String) {
    println!("Writing {:?} token transaction rows to db.", data.len());
    let conn = Connection::open(&db_name[..]).expect("Failed to open db.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id            INTEGER PRIMARY KEY,
            indexs        INTEGER NOT NULL,
            caller        TEXT NOT NULL,
            op            TEXT NOT NULL,
            froma         TEXT NOT NULL,
            toa           TEXT NOT NULL,
            amount        INTEGER NOT NULL,
            fee           INTEGER NOT NULL,
            timestamp     INTEGER NOT NULL
        )",
        [],
    ).expect("Failed to create new transactions table.");

    for i in data {
        let caller_or_none: String;
        if i.caller.is_none() {
            caller_or_none = String::from("None");
        } else {
            caller_or_none = i.caller.unwrap().to_string();
        }
        conn.execute(
            "INSERT INTO transactions (indexs, caller, op, froma, toa, amount, fee, timestamp)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&i.index.to_string(), &caller_or_none, &i.op.to_text(), &i.from.to_text().to_string(),
            &i.to.to_text().to_string(), &i.amount.to_string(), &i.fee.to_string(), &i.timestamp.to_string()],
        ).expect("Failed to insert transaction to transactions table.");
    } 
}

fn save_burn_transactions_to_db(data: Vec<TxRecordBurn>, db_name: String) {
    println!("Writing {:?} token-burn transaction rows to db.", data.len());
    let conn = Connection::open(&db_name[..]).expect("Failed to open db.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id            INTEGER PRIMARY KEY,
            indexs        INTEGER NOT NULL,
            caller        TEXT NOT NULL,
            op            TEXT NOT NULL,
            froma         TEXT NOT NULL,
            toa           TEXT NOT NULL,
            amount        INTEGER NOT NULL,
            fee           INTEGER NOT NULL,
            timestamp     INTEGER NOT NULL
        )",
        [],
    ).expect("Failed to create new transactions table.");

    for i in data {
        let caller_or_none: String;
        if i.caller.is_none() {
            caller_or_none = String::from("None");
        } else {
            caller_or_none = i.caller.unwrap().to_string();
        }
        conn.execute(
            "INSERT INTO transactions (indexs, caller, op, froma, toa, amount, fee, timestamp)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&i.index.to_string(), &caller_or_none, &i.op.to_text(), &i.from.to_text().to_string(),
            &i.to.to_text().to_string(), &i.amount.to_string(), &i.fee.to_string(), &i.timestamp.to_string()],
        ).expect("Failed to insert transaction to transactions table.");
    } 
}

fn save_tokens_to_db(data: Vec<TokenInfo>, db_name: String) {
    println!("Writing {:?} token info rows to db.", data.len());
    let conn = Connection::open(&db_name[..]).expect("Failed to open db.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            id            INTEGER PRIMARY KEY,
            canisterId    TEXT NOT NULL,
            decimals      INTEGER NOT NULL,
            fee           INTEGER NOT NULL,
            indexs        INTEGER NOT NULL,
            logo          TEXT NOT NULL,
            name          TEXT NOT NULL,
            owner         TEXT NOT NULL,
            symbol        TEXT NOT NULL,
            timestamp     INTEGER NOT NULL,
            totalSupply   INTEGER NOT NULL
        )",
        [],
    ).expect("Failed to create new tokens table.");

    for i in data {
        conn.execute(
            "INSERT INTO tokens (canisterId, decimals, fee, indexs, logo, name, owner, symbol, timestamp, totalSupply)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            &[&i.canisterId.to_text().to_string(), &i.decimals.to_string(), &i.fee.to_string(), &i.index.to_string(), 
            &i.logo.to_string(), &i.name.to_string(), &i.owner.to_text().to_string(), &i.symbol.to_string(),
            &i.timestamp.to_string(), &i.totalSupply.to_string()],
        ).expect("Failed to insert token info to tokens table.");
    } 
}

fn save_dswaps_to_db(data: Vec<DSwapOpRecord>, db_name: String) {
    println!("Writing {:?} dswap transaction rows to db.", data.len());
    let conn = Connection::open(&db_name[..]).expect("Failed to open db.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
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
    ).expect("Failed to new transactions table.");

    for i in data {
        conn.execute(
            "INSERT INTO transactions (caller, op, indexs, token_id, froma, toa, amount, amount0, amount1, timestamp)
            values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            &[&i.caller.to_text().to_string(), &i.op.to_text(), &i.index.to_string(), &i.tokenId, &i.from.to_text().to_string(),
            &i.to.to_text().to_string(), &i.amount.to_string(), &i.amount0.to_string(), &i.amount1.to_string(), &i.timestamp.to_string()],
        ).expect("Failed to insert ops to transactions table.");
    } 
}

fn create_identity() -> BasicIdentity {
    BasicIdentity::from_pem_file(KEY_PATH).expect("Could not read the key pair.")
}
