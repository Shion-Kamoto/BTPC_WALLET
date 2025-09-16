use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub network: String,
    pub rpc_url: String,
    pub default_wallet: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: "testnet".to_string(),
            rpc_url: "http://127.0.0.1:18432/".to_string(),
            default_wallet: "wallet.json".to_string(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("btpc_wallet")
        .join("config.json")
}

pub fn load_config() -> anyhow::Result<Config> {
    let config_path = get_config_path();

    if config_path.exists() {
        let config_data = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    } else {
        // Create default config if it doesn't exist
        let config = Config::default();
        save_config(&config)?;
        Ok(config)
    }
}

pub fn save_config(config: &Config) -> anyhow::Result<()> {
    let config_path = get_config_path();

    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config_data = serde_json::to_string_pretty(config)?;
    fs::write(config_path, config_data)?;

    Ok(())
}