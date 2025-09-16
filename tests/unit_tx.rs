use btpc_wallet::tx::builder::*;

#[test]
fn test_build_basic_tx() {
    let inputs = vec![("txid1".to_string(), 0, 1_000_000_000u64)];
    let tx = build_basic_tx(inputs, "addrX", 900_000_000, 50_000_000, "addrChange").unwrap();
    assert_eq!(tx.vout.len(), 2);
}
