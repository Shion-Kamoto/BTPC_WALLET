use crate::utils::hex_lower;
use pqcrypto_dilithium::dilithium5::{
    keypair, PublicKey as DilithiumPublicKey, SecretKey as DilithiumSecretKey,
};
use pqcrypto_traits::sign::PublicKey;
use pqcrypto_traits::sign::SecretKey;
use sha2::{Digest, Sha512};

pub struct Keypair {
    pub pk: Vec<u8>,
    pub sk: Vec<u8>,
}

pub fn generate_keypair() -> Keypair {
    let (pk, sk): (DilithiumPublicKey, DilithiumSecretKey) = keypair();
    Keypair {
        pk: pk.as_bytes().to_vec(),
        sk: sk.as_bytes().to_vec(),
    }
}

pub fn derive_address_from_pk(pk: &[u8]) -> String {
    let digest = Sha512::digest(pk);
    hex_lower(&digest)
}
