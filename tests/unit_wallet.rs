use base64::{engine::general_purpose, Engine as _};

use btpc_wallet::wallet::keystore::{decrypt_sk, encrypt_sk};
use btpc_wallet::wallet::{key::*, mnemonic::*};

#[test]
fn test_address_derivation_len() {
    let kp = generate_keypair();
    let addr = derive_address_from_pk(&kp.pk);
    assert_eq!(addr.len(), 128); // 64-byte digest in hex
}

#[test]
fn test_encrypt_decrypt_sk_roundtrip() {
    let kp = generate_keypair();
    let enc = encrypt_sk("test-pass", &kp.sk).unwrap();

    // decrypt_sk expects the salt as a base64 string (&str), not raw bytes
    let salt_b64 = general_purpose::STANDARD.encode(&enc.salt);
    let dec = decrypt_sk("test-pass", &salt_b64, &enc.nonce, &enc.ciphertext).unwrap();

    assert_eq!(dec, kp.sk);
}

#[test]
fn test_mnemonic_roundtrip_seed_len() {
    let m = generate_mnemonic_24();
    let seed = mnemonic_to_seed(&m, None);
    assert_eq!(seed.len(), 64);
}

#[test]
fn test_mnemonic_deterministic_keygen_reproducible() {
    // Valid BIP-39 24-word mnemonic
    let phrase = "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length";
    let m = bip39::Mnemonic::parse_normalized(phrase).unwrap();

    let (pk1, sk1, addr1) = derive_dilithium5_keypair_from_mnemonic(&m, Some("pass"));
    let (pk2, sk2, addr2) = derive_dilithium5_keypair_from_mnemonic(&m, Some("pass"));

    assert_eq!(pk1, pk2);
    assert_eq!(sk1, sk2);
    assert_eq!(addr1, addr2);
    assert_eq!(addr1.len(), 128);
}
