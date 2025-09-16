use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxIn {
    pub prevout: OutPoint,
    pub script_sig: Option<String>,
    pub sequence: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxOut {
    pub address: String,
    pub value: u64,
} // value in base units

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub version: u32,
    pub lock_time: u32,
    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub witness: Option<String>, // Dilithium5 sig (base64)
}
