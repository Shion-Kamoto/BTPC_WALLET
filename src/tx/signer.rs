use base64::Engine as _;

use pqcrypto_traits::sign::{DetachedSignature as _, SecretKey as _};

/// Sign raw transaction bytes with Dilithium5 and return base64 signature.
/// Accepts secret key as SecretVec<u8> to avoid raw copies in call sites.
use secrecy::{ExposeSecret, SecretBox};

pub fn sign_tx(sk_secret: &SecretBox<Vec<u8>>, tx_bytes: &[u8]) -> anyhow::Result<String> {
    let sk_bytes = sk_secret.expose_secret();
    let sk = pqcrypto_dilithium::dilithium5::SecretKey::from_bytes(sk_bytes)
        .map_err(|_| anyhow::anyhow!("invalid secret key"))?;
    let sig = pqcrypto_dilithium::dilithium5::detached_sign(tx_bytes, &sk);
    Ok(base64::engine::general_purpose::STANDARD.encode(sig.as_bytes()))
}
