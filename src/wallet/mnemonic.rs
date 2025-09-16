use bip39::{Language, Mnemonic};
use hkdf::Hkdf;
use sha2::{Digest, Sha512};

/// Create a 24-word mnemonic using standard BIP-39 English wordlist.
pub fn generate_mnemonic_24() -> Mnemonic {
    Mnemonic::generate_in(Language::English, 24).expect("entropy gen")
}

/// Convert mnemonic (and optional passphrase) to a 64-byte seed.
pub fn mnemonic_to_seed(mnemonic: &Mnemonic, passphrase: Option<&str>) -> Vec<u8> {
    let pass = passphrase.unwrap_or("");
    let seed = mnemonic.to_seed(pass);
    seed.to_vec()
}

/// Derive a Dilithium5-like keypair bytes and address from a mnemonic.
///
/// This is **deterministic**: the same mnemonic + passphrase will always yield
/// the same pk/sk/address. Actual Dilithium5 signing uses pqcrypto with a
/// proper SecretKey, but here we produce stable byte buffers for wallet restore.
pub fn derive_dilithium5_keypair_from_mnemonic(
    mnemonic: &Mnemonic,
    passphrase: Option<&str>,
) -> (Vec<u8>, Vec<u8>, String) {
    // Step 1: derive BIP39 seed
    let seed = mnemonic.to_seed(passphrase.unwrap_or(""));

    // Step 2: HKDF-SHA512 derive 64 bytes
    let hk = Hkdf::<Sha512>::new(None, &seed);
    let mut okm = [0u8; 64];
    hk.expand(b"BTPC-DILITHIUM5-KEYGEN-v1", &mut okm)
        .expect("HKDF expand");

    // Step 3: split into pk/sk halves
    let pk = okm[0..32].to_vec();
    let sk = okm[32..64].to_vec();

    // Step 4: address = hex(SHA512(pk))
    let digest = Sha512::digest(&pk);
    let addr = hex::encode(digest);

    (pk, sk, addr)
}
