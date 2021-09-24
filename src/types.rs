use candid::{
    types::number::{Int, Nat},
    CandidType,
};
use ic_agent::ic_types::Principal;
use serde::Deserialize;

#[allow(non_camel_case_types)]
#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum DSwapOperation {
    deposit,
    withdraw,
    tokenTransfer,
    tokenTransferFrom,
    tokenApprove,
    lpTransfer,
    lpTransferFrom,
    lpApprove,
    createPair,
    swap,
    addLiquidity,
    removeLiquidity,
}

impl DSwapOperation {
    pub fn to_text(&self) -> String {
        match self {
            Self::deposit => String::from("deposit"),
            Self::withdraw => String::from("withdraw"),
            Self::tokenTransfer => String::from("tokenTransfer"),
            Self::tokenTransferFrom => String::from("tokenTransferFrom"),
            Self::tokenApprove => String::from("tokenApprove"),
            Self::lpTransfer => String::from("lpTransfer"),
            Self::lpTransferFrom => String::from("lpTransferFrom"),
            Self::lpApprove => String::from("lpApprove"),
            Self::createPair => String::from("createPair"),
            Self::swap => String::from("swap"),
            Self::addLiquidity => String::from("addLiquidity"),
            Self::removeLiquidity => String::from("removeLiquidity"),
        }
    }
}

#[derive(Debug)]
pub struct DSwapTxRecord {
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
    mint,
    burn(u64),
    transfer,
    transferFrom,
    approve
}

impl TokenOperation {
    pub fn to_text(&self) -> String {
        match self {
            Self::approve => String::from("approve"),
            Self::mint => String::from("mint"),
            Self::burn(c) => format!("burn {}", c),
            Self::transfer => String::from("transfer"),
            Self::transferFrom => String::from("transferFrom"),
        }
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct TokenTxRecord {
    pub index: Nat,
    pub caller: Option<Principal>,
    pub op: TokenOperation,
    pub from: Principal,
    pub to: Principal,
    pub amount: Nat,
    pub fee: Nat,
    pub timestamp: Int,
}

// #[allow(non_camel_case_types)]
// #[derive(CandidType, Deserialize, Clone, Debug)]
// pub enum WICPOperation {
//     approve,
//     burn(u64),
//     mint,
//     transfer,
//     transferFrom,
// }

// impl WICPOperation {
//     pub fn to_text(&self) -> String {
//         match self {
//             Self::approve => String::from("approve"),
//             Self::burn(c) => format!("burn {}", c),
//             Self::mint => String::from("mint"),
//             Self::transfer => String::from("transfer"),
//             Self::transferFrom => String::from("transferFrom"),
//         }
//     }
// }

// #[derive(CandidType, Deserialize, Clone, Debug)]
// pub struct WICPTxRecord {
//     pub index: Nat,
//     pub caller: Option<Principal>,
//     pub op: TxTokenOperationBurn,
//     pub from: Principal,
//     pub to: Principal,
//     pub amount: Nat,
//     pub fee: Nat,
//     pub timestamp: Int,
// }

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

pub trait Database {
    fn db_init_command(&self) -> &str;
    fn db_insert_header(&self) -> &str;
    fn db_insert_values(&self) -> Vec<String>;
}

impl Database for TokenTxRecord {
    fn db_init_command(&self) -> &str {
        return "CREATE TABLE IF NOT EXISTS transactions (
                    id            INTEGER PRIMARY KEY,
                    indexs        INTEGER NOT NULL,
                    caller        TEXT NOT NULL,
                    op            TEXT NOT NULL,
                    froma         TEXT NOT NULL,
                    toa           TEXT NOT NULL,
                    amount        INTEGER NOT NULL,
                    fee           INTEGER NOT NULL,
                    timestamp     INTEGER NOT NULL
                )";
    }

    fn db_insert_header(&self) -> &str {
        return "INSERT INTO transactions (indexs, caller, op, froma, toa, amount, fee, timestamp)
                values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
    }

    fn db_insert_values(&self) -> Vec<String> {
        let caller_or_none: String;
        if self.caller.is_none() {
            caller_or_none = String::from("None");
        } else {
            caller_or_none = self.caller.unwrap().to_string();
        }
        let ret = vec![
            self.index.to_string(),
            caller_or_none,
            self.op.to_text(),
            self.from.to_text().to_string(),
            self.to.to_text().to_string(),
            self.amount.to_string(),
            self.fee.to_string(),
            self.timestamp.to_string(),
        ];
        return ret;
    }
}

// impl Database for WICPTxRecord {
//     fn db_init_command(&self) -> &str {
//         return "CREATE TABLE IF NOT EXISTS transactions (
//                     id            INTEGER PRIMARY KEY,
//                     indexs        INTEGER NOT NULL,
//                     caller        TEXT NOT NULL,
//                     op            TEXT NOT NULL,
//                     froma         TEXT NOT NULL,
//                     toa           TEXT NOT NULL,
//                     amount        INTEGER NOT NULL,
//                     fee           INTEGER NOT NULL,
//                     timestamp     INTEGER NOT NULL
//                 )";
//     }

//     fn db_insert_header(&self) -> &str {
//         return "INSERT INTO transactions (indexs, caller, op, froma, toa, amount, fee, timestamp)
//                 values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";
//     }

//     fn db_insert_values(&self) -> Vec<String> {
//         let caller_or_none: String;
//         if self.caller.is_none() {
//             caller_or_none = String::from("None");
//         } else {
//             caller_or_none = self.caller.unwrap().to_string();
//         }
//         let ret = vec![
//             self.index.to_string(),
//             caller_or_none,
//             self.op.to_text(),
//             self.from.to_text().to_string(),
//             self.to.to_text().to_string(),
//             self.amount.to_string(),
//             self.fee.to_string(),
//             self.timestamp.to_string(),
//         ];
//         return ret;
//     }
// }

impl Database for TokenInfo {
    fn db_init_command(&self) -> &str {
        return "CREATE TABLE IF NOT EXISTS tokens (
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
                )";
    }

    fn db_insert_header(&self) -> &str {
        return "INSERT INTO tokens (canisterId, decimals, fee, indexs, logo, name, owner, symbol, timestamp, totalSupply)
                values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)";
    }

    fn db_insert_values(&self) -> Vec<String> {
        let ret = vec![
            self.canisterId.to_text().to_string(),
            self.decimals.to_string(),
            self.fee.to_string(),
            self.index.to_string(),
            self.logo.to_string(),
            self.name.to_string(),
            self.owner.to_text().to_string(),
            self.symbol.to_string(),
            self.timestamp.to_string(),
            self.totalSupply.to_string(),
        ];
        return ret;
    }
}

impl Database for DSwapTxRecord {
    fn db_init_command(&self) -> &str {
        return "CREATE TABLE IF NOT EXISTS transactions (
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
                )";
    }

    fn db_insert_header(&self) -> &str {
        return "INSERT INTO transactions (caller, op, indexs, token_id, froma, toa, amount, amount0, amount1, timestamp)
                values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)";
    }

    fn db_insert_values(&self) -> Vec<String> {
        let ret = vec![
            self.caller.to_text().to_string(),
            self.op.to_text(),
            self.index.to_string(),
            self.tokenId.to_string(),
            self.from.to_text().to_string(),
            self.to.to_text().to_string(),
            self.amount.to_string(),
            self.amount0.to_string(),
            self.amount1.to_string(),
            self.timestamp.to_string(),
        ];
        return ret;
    }
}
