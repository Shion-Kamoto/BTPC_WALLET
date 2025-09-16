use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentReward {
    pub height: u64,
    pub reward: u64,
}

/// Placeholder local computation; prefer fetching from node RPC in production.
pub fn current_reward_stub(height: u64) -> CurrentReward {
    // Example: linear decay placeholder, not chain-accurate.
    let base: u64 = 25 * 100_000_000;
    let dec = (height / 100_000) * 10_000; // fake units
    let reward = base.saturating_sub(dec);
    CurrentReward { height, reward }
}
