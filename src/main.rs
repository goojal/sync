use candid::{types::number::Nat, Decode, Encode};
use ic_agent::{
    agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal,
    identity::BasicIdentity, Agent,
};
use rusqlite::{Connection, ToSql};
use serde::Deserialize;
use std::{fs, thread, time::Duration};

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
    let config_list: Vec<CanisterConfig> = serde_json::from_str(config_data.as_str()).unwrap();
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
    let agent = Agent::builder()
        .with_transport(
            ReqwestHttpReplicaV2Transport::create(DEFAULT_IC_GATEWAY)
                .expect("Failed to create Transport for Agent"),
        )
        .with_identity(create_identity())
        .build()
        .expect("Failed to build the Agent");

    let mut begin: Nat = Nat::from(0);
    let mut num = QUERY_NUM_S;
    let mut interval: u64;
    let mut result_len: usize = 0;
    loop {
        if config.canister_type == String::from("token") {
            let history_size: Nat = get_history_size(&agent, config.canister_id.clone()).await;
            let raw_transactions = get_trasnactions(
                &agent,
                config.canister_id.clone(),
                begin.clone(),
                Nat::min(
                    history_size.clone() - begin.clone(),
                    Nat::from(num) - begin.clone(),
                ),
            )
            .await;
            let result = Decode!(raw_transactions.as_slice(), Vec<TxRecord>)
                .expect("Failed to decode the getTransactions response data.");
            if result.len() > 0 {
                println!(
                    "Read from {} to {} from token canister",
                    begin.clone(),
                    begin.clone() + result.len() - 1
                );
            }
            save_to_db(result.clone(), config.db.clone());
            result_len = result.len();
        } else if config.canister_type == String::from("wicp") {
            let history_size: Nat = get_history_size(&agent, config.canister_id.clone()).await;
            let raw_transactions = get_trasnactions(
                &agent,
                config.canister_id.clone(),
                begin.clone(),
                Nat::min(
                    history_size.clone() - begin.clone(),
                    Nat::from(num) - begin.clone(),
                ),
            )
            .await;
            let result = Decode!(raw_transactions.as_slice(), Vec<TxRecordBurn>)
                .expect("Failed to decode the getTransactions response data.");
            if result.len() > 0 {
                println!(
                    "Read from {} to {} from token canister",
                    begin.clone(),
                    begin.clone() + result.len() - 1
                );
            }
            save_to_db(result.clone(), config.db.clone());
            result_len = result.len();
        } else if config.canister_type == String::from("token-registry") {
            let result = get_tokens(
                &agent,
                config.canister_id.clone(),
                begin.clone(),
                Nat::from(num),
            )
            .await;
            if result.len() > 0 {
                println!(
                    "Read from {} to {} from token-registry canister",
                    begin.clone(),
                    begin.clone() + result.len() - 1
                );
            }
            save_to_db(result.clone(), config.db.clone());
            result_len = result.len();
        } else if config.canister_type == String::from("dswap-storage") {
            let history_size: Nat = get_history_size(&agent, config.canister_id.clone()).await;
            let raw_transactions = get_trasnactions(
                &agent,
                config.canister_id.clone(),
                begin.clone(),
                Nat::min(
                    history_size.clone() - begin.clone(),
                    Nat::from(num) - begin.clone(),
                ),
            )
            .await;
            let result = Decode!(raw_transactions.as_slice(), Vec<DSwapOpRecord>)
                .expect("Failed to decode the getTransactions response data.");
            if result.len() > 0 {
                println!(
                    "Read from {} to {} from dswap canister",
                    begin.clone(),
                    begin.clone() + result.len() - 1
                );
            }
            save_to_db(result.clone(), config.db.clone());
            result_len = result.len();
        }

        if result_len < num {
            begin += result_len;
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

async fn get_history_size(agent: &Agent, canister_id: String) -> Nat {
    let empty_arg = 0; // because no arg returns error!
    let response = agent
        .query(
            &Principal::from_text(canister_id.clone()).expect(
                format!(
                    "Failed to convert this canister_id to principal: {}",
                    canister_id
                )
                .as_str(),
            ),
            "historySize",
        )
        .with_arg(&Encode!(&empty_arg).unwrap())
        .call()
        .await
        .expect("Failed to call canister on historySize.");

    let history_size: Nat =
        Decode!(response.as_slice(), Nat).expect("Failed to decode the historySize response data.");

    history_size
}

async fn get_trasnactions(agent: &Agent, canister_id: String, start: Nat, limit: Nat) -> Vec<u8> {
    let response = agent
        .query(
            &Principal::from_text(canister_id.clone()).expect(
                format!(
                    "Failed to convert this canister_id to principal: {}",
                    canister_id
                )
                .as_str(),
            ),
            "getTransactions",
        )
        .with_arg(&Encode!(&start, &limit).unwrap())
        .call()
        .await
        .expect("Failed to call canister on getTransactions");

    response
}

async fn get_tokens(agent: &Agent, canister_id: String, start: Nat, limit: Nat) -> Vec<TokenInfo> {
    let response = agent
        .query(
            &Principal::from_text(canister_id.clone()).expect(
                format!(
                    "Failed to convert this canister_id to principal: {}",
                    canister_id
                )
                .as_str(),
            ),
            "getTokens",
        )
        .with_arg(&Encode!(&start, &limit).unwrap())
        .call()
        .await
        .expect("Failed to call canister on getTokens.");

    let result = Decode!(response.as_slice(), Vec<TokenInfo>)
        .expect("Failed to decode the getTokens response data.");

    result
}

fn save_to_db<T: Database>(data: Vec<T>, db_name: String) {
    println!("Writing {:?} rows to db: {}.", data.len(), db_name);
    if data.len() < 1 {
        return;
    }
    let conn = Connection::open(db_name.as_str())
        .expect(format!("Failed to open db: {}.", db_name).as_str());

    conn.execute(data[0].db_init_command(), [])
        .expect(format!("Failed to create table in db: {}.", db_name).as_str());

    for i in data {
        let insert_values = i.db_insert_values();
        let query_values: Vec<_> = insert_values.iter().map(|x| x as &dyn ToSql).collect();
        conn.execute(i.db_insert_header(), &*query_values)
            .expect(format!("Failed to insert in db: {}.", db_name).as_str());
    }
}

fn create_identity() -> BasicIdentity {
    BasicIdentity::from_pem_file(KEY_PATH).expect("Could not read the key pair.")
}
