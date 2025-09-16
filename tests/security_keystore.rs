use secrecy::ExposeSecret;

#[test]
fn test_rekey_and_decrypt_with_new_pass() {
    // Create a temp wallet file
    let dir = tempfile::tempdir().unwrap();
    let wallet = dir.path().join("wallet.json");

    // Minimal flow: create mnemonic-derived wallet via ops::create_new_wallet
    let addr = btpc_wallet::wallet::ops(&wallet, "old-pass", "testnet").unwrap();
    assert!(!addr.is_empty());

    // Change passphrase + KDF params
    btpc_wallet::wallet::ops::change_passphrase(
        &wallet,
        "old-pass",
        "new-pass",
        "testnet",
        "backup",
        Some(131072),
        Some(2),
        Some(1),
    )
    .unwrap();

    // Now try to unlock with new-pass
    let (_addr, sk) =
        btpc_wallet::wallet::ops::load_secret_key_and_address(&wallet, "new-pass").unwrap();
    assert!(!sk.expose_secret().is_empty());

    // Backup export then verify backup
    let backup = dir.path().join("backup.json");
    btpc_wallet::wallet::ops::export_encrypted_backup(&wallet, &backup).unwrap();
    btpc_wallet::wallet::ops::export_encrypted_backup(&backup, "new-pass").unwrap();
}
