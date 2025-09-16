use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone)]
pub struct RpcClient {
    pub base: Url,
    http: reqwest::blocking::Client,
}

impl RpcClient {
    pub fn new(base: &str) -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?; // build here, because builder returns ClientBuilder

        Ok(Self {
            base: base.parse()?,
            http: client,
        })
    }

    pub fn get_balance(&self, addr: &str) -> anyhow::Result<BalanceResp> {
        let url = self.base.join(&format!("address/{}/balance", addr))?;
        let resp = self.http.get(url).send()?;
        if !resp.status().is_success() {
            anyhow::bail!("balance http {}", resp.status());
        }
        Ok(resp.json::<BalanceResp>()?)
    }

    pub fn get_utxos(&self, addr: &str) -> anyhow::Result<Vec<Utxo>> {
        let url = self.base.join(&format!("address/{}/utxos", addr))?;
        let resp = self.http.get(url).send()?;
        if !resp.status().is_success() {
            anyhow::bail!("utxos http {}", resp.status());
        }
        Ok(resp.json::<Vec<Utxo>>()?)
    }

    pub fn get_history(&self, addr: &str, limit: usize) -> anyhow::Result<Vec<TxHistoryItem>> {
        let url = self
            .base
            .join(&format!("address/{}/history?limit={}", addr, limit))?;
        let resp = self.http.get(url).send()?;
        if !resp.status().is_success() {
            anyhow::bail!("history http {}", resp.status());
        }
        Ok(resp.json::<Vec<TxHistoryItem>>()?)
    }

    pub fn broadcast(&self, tx: &serde_json::Value) -> anyhow::Result<BroadcastResp> {
        let url = self.base.join("tx/broadcast")?;
        let resp = self.http.post(url).json(tx).send()?;
        if !resp.status().is_success() {
            anyhow::bail!("broadcast http {}", resp.status());
        }
        Ok(resp.json::<BroadcastResp>()?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceResp {
    pub confirmed: u64,
    pub pending: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastResp {
    pub txid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxHistoryItem {
    pub txid: String,
    pub height: Option<u64>,
    pub timestamp: Option<u64>,
    pub delta: i64, // positive for received, negative for sent (incl. fee)
    pub fee: Option<u64>,
}
