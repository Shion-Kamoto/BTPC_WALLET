use zeroize::Zeroize;

pub fn hex_lower(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Parse a human decimal amount (e.g., "1.23456789") into base units (u64).
/// 1 BTP = 100_000_000 base units.
pub fn parse_amount_to_units(s: &str) -> anyhow::Result<u64> {
    let parts: Vec<&str> = s.trim().split('.').collect();
    if parts.len() == 1 {
        return parts[0].parse::<u64>().map_err(|e| anyhow::anyhow!(e));
    }
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("invalid amount format"));
    }
    let whole = parts[0].parse::<u64>().map_err(|e| anyhow::anyhow!(e))?;
    let frac_str = format!("{:0<8}", parts[1]); // pad/right-fill to 8 decimals
    if frac_str.len() > 8 {
        return Err(anyhow::anyhow!("too many decimals (max 8)"));
    }
    let frac = frac_str[..8]
        .parse::<u64>()
        .map_err(|e| anyhow::anyhow!(e))?;
    whole
        .checked_mul(100_000_000)
        .and_then(|w| w.checked_add(frac))
        .ok_or_else(|| anyhow::anyhow!("overflow"))
}

pub fn zeroize_vec(mut v: Vec<u8>) {
    v.zeroize();
}
