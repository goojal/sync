use ic_agent::{Agent, identity::BasicIdentity, ic_types::Principal, agent::http_transport::ReqwestHttpReplicaV2Transport};
use candid::{Encode, Decode, CandidType, types::number::{Nat, Int}};
use serde::Deserialize;
use std::{thread, time::Duration};
use rusqlite::Connection;
use std::env;

#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum DSwapOperation {
    deposit, withdraw, tokenTransfer, tokenApprove,
    lpTransfer, lpApprove, 
    createPair, swap, addLiquidity, removeLiquidity
}

#[allow(non_snake_case)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DSwapOpRecord {
    pub caller: Principal,
    pub op: DSwapOperation,
    pub index: Nat,
    pub tokenId: String,
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub amount0: Nat,
    pub amount1: Nat,
    pub timestamp: Int,
}

#[derive(Debug)]
pub struct DSwapHistory {
    index: u64,
    caller: String,
    op: DSwapOperation,
    token_id: String,
    from: String,
    to: String,
    amount: u64,
    amount0: u64,
    amount1: u64,
    timestamp: u64,
}


#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TokenOperation {
    mint, burn, transfer, approve, init
}

#[allow(non_snake_case)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TokenOpRecord {
    caller: Principal,
    op: TokenOperation,
    index: Nat,
    from: Principal,
    to: Principal,
    amount: Nat,
    fee: Nat,
    timestamp: Int,
}

#[derive(Debug)]
pub struct TokenHistory {
    index: u64,
    caller: String,
    op: TokenOperation,
    from: String,
    to: String,
    amount: u64,
    fee: u64,
    timestamp: u64,
}