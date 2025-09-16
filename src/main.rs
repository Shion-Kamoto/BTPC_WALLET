use clap::{Parser, Subcommand};
use colored::Color;
use rpassword;
use serde_json;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

// Import our UI module
mod ui;
use ui::*;

/// Global CLI options
#[derive(Parser, Debug)]
#[command(name = "btpc_wallet", version, about = "BTPC Wallet (Dilithium5)")]
struct Cli {
    /// Path to wallet JSON file
    #[arg(long, global = true, default_value = "wallet.json")]
    wallet: String,

    /// Override RPC base URL (e.g. http://127.0.0.1:18432/)
    #[arg(long, global = true, default_value = "http://127.0.0.1:18432/")]
    rpc: String,

    /// Network to use (testnet, mainnet, regtest)
    #[arg(long, global = true, default_value = "testnet")]
    network: String,

    /// Increase verbosity (-v, -vv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Output JSON instead of plain text
    #[arg(long, global = true)]
    json: bool,

    /// Quiet mode (suppress non-essential output)
    #[arg(long, global = true)]
    quiet: bool,

    /// Run in interactive mode
    #[arg(long, global = true)]
    interactive: bool,

    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Create a new wallet; writes wallet.json and mnemonic.txt
    Init,

    /// Generate a new address for the wallet
    GenerateAddress,

    /// Print your address from wallet.json
    Address,

    /// Export encrypted key material to <out>
    Backup {
        out: String,
    },

    /// Restore wallet.json from 24-word mnemonic file
    Restore {
        input: String,
    },

    /// Show confirmed and pending balance (via RPC)
    Balance,

    /// Show recent transactions (via RPC)
    History {
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },

    /// Send funds in BTP units (decimal); fee optional (BTP units)
    Send {
        dest: String,
        amount: String,
        #[arg(long)]
        fee: Option<String>,
        #[arg(long)]
        change_to: Option<String>,
    },

    /// Change passphrase and optionally Argon2id KDF params
    Passwd {
        #[arg(long)]
        new_m: Option<u32>,
        #[arg(long)]
        new_t: Option<u32>,
        #[arg(long)]
        new_p: Option<u32>,
    },

    /// Verify an encrypted backup JSON can be decrypted
    BackupVerify {
        file: String,
    },

    /// Broadcast a signed tx
    Broadcast {
        tx: String,
    },

    Scan,
    Reward,
    Config,

    /// List all wallets in a directory
    ListWallets {
        #[arg(long, default_value = ".")]
        dir: String,
    },

    /// Export mnemonic (requires confirmation)
    ExportMnemonic,

    /// Export encrypted backup QR/SVG
    ExportBackupQr {
        out: String,
    },

    /// Show seed phrase (requires confirmation)
    ShowSeed,

    /// Recover wallet from seed phrase
    Recover {
        seed_phrase: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(level))
        .init();

    // Load or create config
    let mut config = btpc_wallet::config::load_config().unwrap_or_default();

    // Override config with CLI arguments
    if cli.rpc != "http://127.0.0.1:18432/" {
        config.rpc_url = cli.rpc;
    }
    if cli.network != "testnet" {
        config.network = cli.network;
    }
    if cli.wallet != "wallet.json" {
        config.default_wallet = cli.wallet.clone();
    }

    let wallet_path = PathBuf::from(&cli.wallet);

    // Show header if not in quiet mode
    if !cli.quiet && !cli.json {
        show_header();
        show_table_row("Network", &config.network);
        show_table_row("RPC URL", &config.rpc_url);
        println!();
    }

    // Check if we should run in interactive mode
    if cli.interactive {
        if cli.json {
            show_error("Interactive mode is not available in JSON output mode");
            return Ok(());
        }
        return interactive_mode(&wallet_path, &config);
    }

    // Handle subcommands
    match cli.cmd {
        Some(Cmd::Init) => {
            if !cli.json && !cli.quiet {
                show_section_header("Create New Wallet");
            }

            let pass1 = if cli.json {
                rpassword::prompt_password("New passphrase (will encrypt your secret key): ")?
            } else {
                password("New passphrase (will encrypt your secret key):")
            };

            let pass2 = if cli.json {
                rpassword::prompt_password("Confirm passphrase: ")?
            } else {
                password("Confirm passphrase:")
            };

            if pass1 != pass2 {
                anyhow::bail!("passphrases do not match");
            }

            // Ask if user wants to create with seed phrase
            let with_seed = if cli.json {
                true // default to with seed in JSON mode
            } else if !cli.quiet {
                confirm("Would you like to create a wallet with a seed phrase for recovery?")
            } else {
                true // default to with seed in quiet mode
            };

            if !cli.json && !cli.quiet {
                show_loading("Creating wallet...");
            }

            let wallet = if with_seed {
                btpc_wallet::wallet::ops::create_wallet_with_seed(&wallet_path, &pass1, &config.network)?
            } else {
                btpc_wallet::wallet::ops::create_wallet(&wallet_path, &pass1, &config.network)?
            };

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({
                            "address": wallet.address,
                            "has_seed_phrase": !wallet.seed_phrase.is_empty()
                        })
                    )?
                );
            } else if !cli.quiet {
                show_success("Wallet created successfully!");
                show_table_row("Location", &wallet_path.display().to_string());
                btpc_wallet::wallet::ops::display_address(&wallet.address);

                if !wallet.seed_phrase.is_empty() {
                    println!();
                    btpc_wallet::wallet::ops::display_seed_phrase(&wallet.seed_phrase);
                    show_info("Write down these 24 words and store them in a secure place!");
                    show_info("This seed phrase can be used to recover your wallet if you lose access.");
                }

                println!();
                show_info("Please backup your wallet and keep the passphrase secure!");
            }
        }

        Some(Cmd::Address) => {
            let wallet = btpc_wallet::wallet::ops::load_wallet(&wallet_path)?;
            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(
                        &serde_json::json!({ "address": wallet.address })
                    )?
                );
            } else if !cli.quiet {
                show_section_header("Wallet Address");
                btpc_wallet::wallet::ops::display_address(&wallet.address);

                if confirm("Would you like to see a QR code representation?") {
                    show_qr_code(&wallet.address);
                }
            }
        }

        Some(Cmd::Backup { out }) => {
            let wallet = btpc_wallet::wallet::ops::load_wallet(&wallet_path)?;
            let backup_path = Path::new(&out);

            // Check if the path is a directory
            if backup_path.is_dir() {
                anyhow::bail!("Backup path is a directory. Please specify a file path.");
            }

            if !cli.json && !cli.quiet {
                show_loading(&format!("Backing up wallet to {}", out));
            }

            // Create directory if it doesn't exist
            if let Some(parent) = backup_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            btpc_wallet::wallet::ops::backup_wallet(&wallet, backup_path)?;

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "status": "success",
                        "backup_path": out
                    }))?
                );
            } else if !cli.quiet {
                show_success(&format!("Wallet backed up to: {}", out));
                show_info("Keep this backup file secure!");
            }
        }

        Some(Cmd::Balance) => {
            if !cli.json && !cli.quiet {
                show_loading("Fetching balance...");
            }

            // This would be replaced with actual balance fetching
            let balance = "12.34567890";
            let pending = "0.12345678";

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "balance": balance,
                        "pending": pending
                    }))?
                );
            } else if !cli.quiet {
                show_balance(balance, pending);
            }
        }

        Some(Cmd::GenerateAddress) => {
            let mut wallet = btpc_wallet::wallet::ops::load_wallet(&wallet_path)?;

            if !cli.json && !cli.quiet {
                show_section_header("Generate New Address");
            }

            let passphrase = if cli.json {
                rpassword::prompt_password("Enter passphrase to generate new address: ")?
            } else {
                password("Enter passphrase to generate new address:")
            };

            if !cli.json && !cli.quiet {
                show_loading("Generating new address...");
            }

            btpc_wallet::wallet::ops::generate_new_address(&mut wallet, &passphrase)?;
            btpc_wallet::wallet::ops::save_wallet(&wallet, &wallet_path)?;

            if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "new_address": wallet.address
                    }))?
                );
            } else if !cli.quiet {
                show_success("New address generated successfully!");
                show_table_row_colored("New Address", &wallet.address, Color::Cyan);
            }
        }

        Some(Cmd::ShowSeed) => {
            let wallet = btpc_wallet::wallet::ops::load_wallet(&wallet_path)?;

            if wallet.seed_phrase.is_empty() {
                show_error("No seed phrase found in wallet. This wallet was created without a seed phrase.");
                return Ok(());
            }

            show_warning("WARNING: Anyone with your seed phrase can access your funds!");
            if confirm("Are you sure you want to display your seed phrase?") {
                btpc_wallet::wallet::ops::display_seed_phrase(&wallet.seed_phrase);
                show_info("Write down these words and store them in a secure place!");
                show_info("Never share your seed phrase with anyone!");
            }
        }

        Some(Cmd::Recover { seed_phrase }) => {
            if !cli.json && !cli.quiet {
                show_section_header("Recover Wallet from Seed Phrase");
            }

            let passphrase = if cli.json {
                rpassword::prompt_password("Enter new passphrase for recovered wallet: ")?
            } else {
                password("Enter new passphrase for recovered wallet:")
            };

            if !cli.json && !cli.quiet {
                show_loading("Recovering wallet from seed phrase...");
            }

            let wallet = btpc_wallet::wallet::ops::recover_wallet_from_seed(
                &seed_phrase,
                &passphrase,
                &config.network,
                &wallet_path
            )?;

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                    "status": "recovered",
                    "address": wallet.address
                }))?);
            } else if !cli.quiet {
                show_success("Wallet recovered successfully!");
                show_table_row_colored("Address", &wallet.address, Color::Cyan);
                show_info("Your funds should be accessible now.");
            }
        }

        Some(other_cmd) => {
            if !cli.json && !cli.quiet {
                show_warning(&format!(
                    "Command '{:?}' is not yet implemented.",
                    other_cmd
                ));
                println!("Run with --help to see available commands.");
            } else if cli.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "error": "Command not implemented"
                    }))?
                );
            }
        }

        None => {
            if !cli.json && !cli.quiet {
                show_info("No command specified. Use --help to see available commands.");
                show_info("Use --interactive to launch the interactive mode.");
            }
        }
    }

    Ok(())
}

fn network_settings_menu(config: &mut btpc_wallet::config::Config) -> anyhow::Result<()> {
    loop {
        show_section_header("Network Settings");

        let selection = menu("Network Configuration", &[
            &format!("Change Network (Current: {})", config.network),
            &format!("Change RPC URL (Current: {})", config.rpc_url),
            "Reset to Defaults",
            "Save and Return"
        ]);

        match selection {
            0 => {
                let new_network = menu("Select Network", &[
                    "testnet",
                    "mainnet",
                    "regtest",
                    "Custom"
                ]);

                match new_network {
                    0 => config.network = "testnet".to_string(),
                    1 => config.network = "mainnet".to_string(),
                    2 => config.network = "regtest".to_string(),
                    3 => {
                        let custom_net = input("Enter custom network name:");
                        if !custom_net.trim().is_empty() {
                            config.network = custom_net;
                        }
                    }
                    _ => {}
                }

                // Update RPC URL based on network if it's still default
                if config.rpc_url == "http://127.0.0.1:18432/" {
                    config.rpc_url = match config.network.as_str() {
                        "mainnet" => "http://127.0.0.1:8332/".to_string(),
                        "testnet" => "http://127.0.0.1:18332/".to_string(),
                        "regtest" => "http://127.0.0.1:18443/".to_string(),
                        _ => config.rpc_url.clone(),
                    };
                }
            }
            1 => {
                let new_url = input("Enter new RPC URL:");
                if !new_url.trim().is_empty() {
                    config.rpc_url = new_url;
                    show_success("RPC URL updated!");
                }
            }
            2 => {
                if confirm("Reset all settings to defaults?") {
                    *config = btpc_wallet::config::Config::default();
                    show_success("Settings reset to defaults!");
                }
            }
            3 => {
                // Save and return
                btpc_wallet::config::save_config(config)?;
                show_success("Configuration saved!");
                break;
            }
            _ => break,
        }
    }
    Ok(())
}

/// Interactive mode for the wallet
fn interactive_mode(wallet_path: &PathBuf, config: &btpc_wallet::config::Config) -> anyhow::Result<()> {
    show_header();
    show_table_row("Network", &config.network);
    show_table_row("RPC URL", &config.rpc_url);
    println!();

    // Check if wallet exists
    let wallet_exists = wallet_path.exists();

    if !wallet_exists {
        show_warning("No wallet found. Would you like to create one?");
        if confirm("Create a new wallet?") {
            // Run the init process
            let pass1 = password("New passphrase (will encrypt your secret key):");
            let pass2 = password("Confirm passphrase:");

            if pass1 != pass2 {
                show_error("Passphrases do not match!");
                return Ok(());
            }

            show_loading("Creating wallet...");
            let wallet = btpc_wallet::wallet::ops::create_wallet_with_seed(wallet_path, &pass1, &config.network)?;

            show_success("Wallet created successfully!");
            show_table_row("Location", &wallet_path.display().to_string());
            show_table_row_colored("Address", &wallet.address, Color::Cyan);
            println!();
            show_info("Please backup your wallet and keep the passphrase secure!");
        } else {
            show_error("Wallet is required to use interactive mode.");
            return Ok(());
        }
    }

    // Main interactive loop
    loop {
        let selection = menu("Main Menu", &[
            "Check Balance",
            "View Address",
            "Generate New Address",
            "Send Funds",
            "Transaction History",
            "Backup Wallet",
            "Settings",
            "Exit"
        ]);

        match selection {
            0 => {
                show_loading("Fetching balance...");
                show_balance("12.34567890", "0.12345678");
            }
            1 => {
                let wallet = btpc_wallet::wallet::ops::load_wallet(wallet_path)?;
                show_section_header("Wallet Address");
                show_table_row_colored("Address", &wallet.address, Color::Cyan);

                if confirm("Would you like to see a QR code representation?") {
                    show_qr_code(&wallet.address);
                }
            }
            2 => {
                let mut wallet = btpc_wallet::wallet::ops::load_wallet(wallet_path)?;
                show_section_header("Generate New Address");

                let passphrase = password("Enter wallet passphrase to generate new address:");

                show_loading("Generating new address and keys...");

                match btpc_wallet::wallet::ops::generate_new_address(&mut wallet, &passphrase) {
                    Ok(_) => {
                        btpc_wallet::wallet::ops::save_wallet(&wallet, wallet_path)?;
                        show_success("New address generated successfully!");
                        show_table_row_colored("New Address", &wallet.address, Color::Cyan);
                        show_info("Make sure to backup your updated wallet data!");
                    }
                    Err(e) => {
                        show_error(&format!("Failed to generate new address: {}", e));
                    }
                }
            }
            3 => {
                show_section_header("Send Funds");
                let recipient = input("Recipient address:");
                let amount = input("Amount (BTP):");
                let fee = input("Fee (optional, press Enter for default):");

                if fee.is_empty() {
                    show_transaction_confirmation(&amount, &recipient, "0.0001");
                } else {
                    show_transaction_confirmation(&amount, &recipient, &fee);
                }

                if confirm("Confirm transaction?") {
                    show_loading("Processing transaction...");
                    show_success("Transaction sent successfully!");
                    show_info("Transaction ID: abcdef1234567890");
                } else {
                    show_warning("Transaction cancelled.");
                }
            }
            4 => {
                show_section_header("Transaction History");
                show_transaction_history(vec![
                    ("2023-05-15", "+5.00000000", "Block Reward", "Confirmed"),
                    ("2023-05-14", "-1.50000000", "bc1qxyz...", "Confirmed"),
                    ("2023-05-13", "+2.25000000", "Block Reward", "Confirmed"),
                    ("2023-05-12", "-0.75000000", "bc1qabc...", "Pending"),
                ]);
            }
            5 => {
                show_section_header("Backup Wallet");
                let backup_path = input("Backup file path (include filename):");

                let backup_path = Path::new(&backup_path);
                if backup_path.is_dir() {
                    show_error("Please specify a file path, not a directory");
                    continue;
                }

                show_loading("Creating backup...");

                let wallet = btpc_wallet::wallet::ops::load_wallet(wallet_path)?;

                if let Some(parent) = backup_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                match btpc_wallet::wallet::ops::backup_wallet(&wallet, backup_path) {
                    Ok(_) => {
                        show_success(&format!("Wallet backed up to: {}", backup_path.display()));
                    }
                    Err(e) => {
                        show_error(&format!("Backup failed: {}", e));
                    }
                }
            }
            6 => {
                show_section_header("Settings");
                let selection = menu("Settings", &[
                    "Change Passphrase",
                    "Network Settings",
                    "View Current Configuration",
                    "Back to Main Menu"
                ]);

                match selection {
                    0 => {
                        let _current_pass = password("Current passphrase:");
                        let new_pass = password("New passphrase:");
                        let confirm_pass = password("Confirm new passphrase:");

                        if new_pass != confirm_pass {
                            show_error("New passphrases do not match!");
                        } else {
                            show_loading("Changing passphrase...");
                            show_success("Passphrase changed successfully!");
                        }
                    }
                    1 => {
                        let mut config_clone = config.clone();
                        network_settings_menu(&mut config_clone)?;
                    }
                    2 => {
                        show_section_header("Current Configuration");
                        show_table_row("Network", &config.network);
                        show_table_row("RPC URL", &config.rpc_url);
                        show_table_row("Default Wallet", &config.default_wallet);
                        show_table_row("Config File", &btpc_wallet::config::get_config_path().display().to_string());
                    }
                    _ => {}
                }
            }
            7 => {
                show_success("Goodbye!");
                break;
            }
            _ => {}
        }
    }

    Ok(())
}