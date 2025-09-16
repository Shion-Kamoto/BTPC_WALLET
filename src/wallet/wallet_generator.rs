//! Wallet address generator using quantum-resistant algorithms

use rand::Rng;
use sha2::{Digest, Sha512};
use base64::{encode_config, URL_SAFE};
use colored::*;

/// Generate a new quantum-resistant wallet address
pub fn generate_address(prefix: Option<&str>) -> String {
    let prefix = prefix.unwrap_or("btpc");

    // Generate random bytes for the address
    let mut rng = rand::thread_rng();
    let mut random_bytes = [0u8; 32];
    rng.fill(&mut random_bytes);

    // Hash the random bytes with SHA-512
    let mut hasher = Sha512::new();
    hasher.update(random_bytes);
    let hash_result = hasher.finalize();

    // Encode in base64 URL-safe format
    let encoded = encode_config(&hash_result, URL_SAFE);

    // Format the address with prefix and checksum
    format_address(prefix, &encoded)
}

/// Format the address with prefix and checksum
fn format_address(prefix: &str, encoded: &str) -> String {
    // Take first 40 characters of the encoded string
    let main_part = &encoded[..40];

    // Simple checksum (last 4 chars of the hash)
    let checksum = &encoded[encoded.len()-4..];

    format!("{}:{}_{}", prefix, main_part, checksum)
}

/// Validate an address format
pub fn validate_address(address: &str) -> bool {
    // Simple validation for demonstration
    address.contains(':') && address.len() > 10 && address.len() < 100
}

/// Display address in a formatted way
pub fn display_address(address: &str) {
    let parts: Vec<&str> = address.split(':').collect();
    if parts.len() != 2 {
        println!("{}", "Invalid address format".red());
        return;
    }

    let prefix = parts[0];
    let rest = parts[1];

    println!("{}", "╔══════════════════════════════════════════════════════╗".bright_green());
    println!("{}", "║                 WALLET ADDRESS                     ║".bright_green().bold());
    println!("{}", "╠══════════════════════════════════════════════════════╣".bright_green());
    println!("{}: {}", "Prefix".bright_white().bold(), prefix.bright_cyan());
    println!("{}: {}", "Address".bright_white().bold(), rest.bright_yellow());
    println!("{}", "╚══════════════════════════════════════════════════════╝".bright_green());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_address() {
        let addr = generate_address(Some("btpc"));
        assert!(addr.starts_with("btpc:"));
        assert!(addr.len() > 20);
    }

    #[test]
    fn test_validate_address() {
        let addr = "btpc:abc123_xyz";
        assert!(validate_address(addr));

        let invalid_addr = "invalid";
        assert!(!validate_address(invalid_addr));
    }
}