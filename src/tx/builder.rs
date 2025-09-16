use super::model::{OutPoint, Transaction, TxIn, TxOut};
use anyhow::Result;

pub fn build_basic_tx(
    inputs: Vec<(String, u32, u64)>,
    dest: &str,
    amount: u64,
    fee: u64,
    change_addr: &str,
) -> Result<Transaction> {
    let mut vin = Vec::new();
    let mut total_in: u64 = 0;
    for (txid, vout, value) in inputs {
        vin.push(TxIn {
            prevout: OutPoint { txid, vout },
            script_sig: None,
            sequence: 0xFFFFFFFF,
        });
        total_in = total_in
            .checked_add(value)
            .ok_or_else(|| anyhow::anyhow!("overflow"))?;
    }
    let mut vout = Vec::new();
    vout.push(TxOut {
        address: dest.to_string(),
        value: amount,
    });
    let needed = amount
        .checked_add(fee)
        .ok_or_else(|| anyhow::anyhow!("overflow"))?;
    if total_in < needed {
        return Err(anyhow::anyhow!("insufficient funds"));
    }
    let change = total_in - needed;
    if change > 0 {
        vout.push(TxOut {
            address: change_addr.to_string(),
            value: change,
        });
    }
    Ok(Transaction {
        version: 1,
        lock_time: 0,
        vin,
        vout,
        witness: None,
    })
}
