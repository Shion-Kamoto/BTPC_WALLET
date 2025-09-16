use anyhow::Result;
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdfParams {
    pub m: u32,
    pub t: u32,
    pub p: u32,
    #[serde(default = "kdf_name")]
    pub name: String,
}
fn kdf_name() -> String {
    "argon2id".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletFile {
    pub version: u32,
    pub network: String,
    pub address: String,
    pub public_key: String,     // base64
    pub secret_key_enc: String, // base64
    pub cipher: String,         // chacha20poly1305
    pub kdf: KdfParams,
    pub nonce: String, // base64
    pub balance_cached: u64,
    pub last_scanned_height: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

pub struct EncBundle {
    pub salt: Vec<u8>,
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
}

pub fn default_kdf_params() -> KdfParams {
    KdfParams {
        m: 65536,
        t: 3,
        p: 1,
        name: "argon2id".into(),
    }
}

pub fn derive_key(passphrase: &str, salt: &[u8], m: u32, t: u32, p: u32) -> Result<[u8; 32]> {
    // Use Argon2id with std feature enabled, convert errors to anyhow
    let params = argon2::Params::new(m, t, p, None)
        .map_err(|e| anyhow::anyhow!(format!("argon2 params: {e}")))?;
    let argon2 = Argon2::new_with_secret(
        &[],
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    )
    .map_err(|e| anyhow::anyhow!(format!("argon2 ctx: {e}")))?;
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow::anyhow!(format!("argon2 derive: {e}")))?;
    Ok(key)
}

pub fn encrypt_sk_with_params(passphrase: &str, sk: &[u8], kdf: &KdfParams) -> Result<EncBundle> {
    let mut salt = [0u8; 16];
    getrandom::getrandom(&mut salt)?;
    let key = derive_key(passphrase, &salt, kdf.m, kdf.t, kdf.p)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce = [0u8; 12];
    getrandom::getrandom(&mut nonce)?;
    let ct = cipher
        .encrypt(Nonce::from_slice(&nonce), sk)
        .map_err(|e| anyhow::anyhow!(format!("aead encrypt: {e}")))?;
    Ok(EncBundle {
        salt: salt.to_vec(),
        nonce,
        ciphertext: ct,
    })
}

pub fn encrypt_sk(passphrase: &str, sk: &[u8]) -> Result<EncBundle> {
    encrypt_sk_with_params(passphrase, sk, &default_kdf_params())
}

/// Decrypt using base64-encoded salt string and raw nonce/ciphertext
pub fn decrypt_sk(
    password: &str,
    salt_str: &str,
    nonce_bytes: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>> {
    let salt = general_purpose::STANDARD.decode(salt_str)?;
    // We don't know the KDF params here; caller must supply correct params for derive.
    // For wallet file decryption, caller should read KDF (m,t,p) from JSON and pass via derive.
    // For backward compatibility with our ops helper, try default params first; if it fails, try a small fallback set.
    let try_params = [
        default_kdf_params(),
        KdfParams {
            m: 65536,
            t: 2,
            p: 1,
            name: "argon2id".into(),
        },
        KdfParams {
            m: 262144,
            t: 2,
            p: 1,
            name: "argon2id".into(),
        },
    ];
    for params in try_params {
        if let Ok(key) = derive_key(password, &salt, params.m, params.t, params.p) {
            let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
            if let Ok(pt) = cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
                return Ok(pt);
            }
        }
    }
    Err(anyhow::anyhow!("decryption failed with known KDF params"))
}
