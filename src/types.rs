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

impl DSwapOperation {
    pub fn to_text(&self) -> String {
        match self {
            Self::deposit => String::from("deposit"),
            Self::withdraw => String::from("withdraw"),
            Self::tokenTransfer => String::from("tokenTransfer"),
            Self::tokenApprove => String::from("tokenApprove"),
            Self::lpTransfer => String::from("lpTransfer"),
            Self::lpApprove => String::from("lpApprove"),
            Self::createPair => String::from("createPair"),
            Self::swap => String::from("swap"),
            Self::addLiquidity => String::from("addLiquidity"),
            Self::removeLiquidity => String::from("removeLiquidity"),
        }
    }
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

#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum TxTokenOperation {
    approve, mint, transfer, transferFrom
}

impl TxTokenOperation {
    pub fn to_text(&self) -> String {
        match self {
            Self::approve => String::from("approve"),
            Self::mint => String::from("mint"),
            Self::transfer => String::from("transfer"),
            Self::transferFrom => String::from("transferFrom"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TxRecord {
    pub index: Nat,
    pub caller: Option<Principal>,
    pub op: TxTokenOperation,
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub fee: Nat,
    pub timestamp: Int,
}

#[allow(non_snake_case)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    pub canisterId: Principal,
    pub decimals: u8, // Nat8
    pub fee: Nat,
    pub index: Nat,
    pub logo: String,
    pub name: String,
    pub owner: Principal,
    pub symbol: String,
    pub timestamp: Int,
    pub totalSupply: Nat,
}
