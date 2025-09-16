# BTPC Wallet (Dilithium5)

A secure, command-line cryptocurrency wallet built with Rust, featuring post-quantum cryptography using Dilithium5 signatures.

## Features

- 🔐 **Post-Quantum Security**: Dilithium5 digital signatures
- 💾 **Encrypted Storage**: AES-256-GCM encrypted wallet files
- 🌱 **Seed Phrase Recovery**: 24-word mnemonic backup system
- 📱 **Interactive Mode**: User-friendly terminal interface
- 🔧 **Multi-Network Support**: Testnet, Mainnet, and custom networks
- 📊 **Balance Tracking**: View confirmed and pending balances
- 📜 **Transaction History**: Recent transaction monitoring
- 📍 **Address Management**: Generate new addresses
- 💰 **Send Funds**: Secure transaction processing
- 🛡️ **Backup & Restore**: Encrypted wallet backups
- 📟 **QR Code Support**: Address visualization

## Installation

### Prerequisites
- Rust 1.60+ and Cargo
- Bitcoin PoC (BTPC) node access

### Build from Source
```bash
git clone <your-repo-url>
cd btpc-wallet
cargo build --release

Install Globally
cargo install --path .

Quick Start

1. Create a New Wallet
btpc_wallet init

2. View Your Address
btpc_wallet address

3. Check Balance
btpc_wallet balance

4. Send Funds
btpc_wallet send <recipient_address> <amount>

Command Reference
Wallet Management
Command	Description	Example
init	Create new wallet	btpc_wallet init
address	Show wallet address	btpc_wallet address
balance	Check balance	btpc_wallet balance
generate-address	Generate new address	btpc_wallet generate-address
show-seed	Display seed phrase	btpc_wallet show-seed
Transactions
Command	Description	Example
send	Send funds	btpc_wallet send bc1q... 1.5
history	Transaction history	btpc_wallet history --limit 20
broadcast	Broadcast raw transaction	btpc_wallet broadcast <raw_tx>
Backup & Recovery
Command	Description	Example
backup	Create encrypted backup	btpc_wallet backup my_wallet.backup
restore	Restore from seed phrase	btpc_wallet restore "word1 word2 ..."
backup-verify	Verify backup file	btpc_wallet backup-verify backup.file
export-mnemonic	Export mnemonic phrase	btpc_wallet export-mnemonic
Configuration
Command	Description	Example
passwd	Change passphrase	btpc_wallet passwd
config	View configuration	btpc_wallet config
list-wallets	List available wallets	btpc_wallet list-wallets
Interactive Mode
Launch the interactive terminal interface:

btpc_wallet --interactive

Interactive Menu Options:
Check Balance - View available and pending balances

View Address - Display your wallet address with QR code option

Generate New Address - Create a new receiving address

Send Funds - Initiate a transaction

Transaction History - View recent transactions

Backup Wallet - Create encrypted backup

Settings - Configure network and preferences

Exit - Close the application

Network Configuration
Default Networks
Testnet: http://127.0.0.1:18432/ (default)

Mainnet: http://127.0.0.1:8332/

Regtest: http://127.0.0.1:18443/

Custom Network Setup

# Use custom RPC URL
btpc_wallet --rpc http://your-node:8332/ balance

# Change network
btpc_wallet --network mainnet address

# Custom wallet file
btpc_wallet --wallet /path/to/wallet.json balance

Security Features
Encryption
Private keys encrypted with AES-256-GCM

Argon2id key derivation function

Passphrase-protected wallet files

Best Practices
Always backup your seed phrase and store it securely offline

Use strong passphrases (12+ characters, mixed characters)

Keep backups in multiple secure locations

Verify backups can be restored

File Structure

~/
├── wallet.json                 # Default wallet file (encrypted)
├── .config/btpc_wallet/
│   └── config.json            # Application configuration
└── mnemonic.txt               # Seed phrase backup (if exported)

Advanced Usage
JSON Output Mode

btpc_wallet --json balance
# Output: {"balance": "12.34567890", "pending": "0.12345678"}

Verbose Logging

btpc_wallet -v balance      # Basic debug info
btpc_wallet -vv balance     # Detailed tracing

Multiple Wallets

# Work with different wallet files
btpc_wallet --wallet my_wallet.json balance
btpc_wallet --wallet savings.json balance

# List available wallets
btpc_wallet list-wallets --dir ./wallets

Recovery Procedures
From Seed Phrase

btpc_wallet recover "word1 word2 word3 ... word24"

From Backup File

# Restore from encrypted backup
btpc_wallet restore --input backup.file

# Restore from encrypted backup
btpc_wallet restore --input backup.file

# Specify correct path
btpc_wallet --wallet /correct/path/wallet.json balance

"RPC connection failed"

Check your node is running

Verify RPC URL and credentials

Ensure network matches your node

Debug Mode

RUST_LOG=debug btpc_wallet -vv balance

Building from Source

git clone <repository>
cd btpc-wallet
cargo build
cargo test

Architecture

src/
├── main.rs          # CLI interface and command handling
├── lib.rs           # Library exports
├── config.rs        # Configuration management
├── wallet/          # Wallet operations
│   ├── mod.rs
│   └── ops.rs       # Core wallet functions
└── ui/             # User interface components
    └── mod.rs      # Terminal UI helpers


Support
For issues and questions:

Check this README and --help output

Review debug logs with -vv flag

Ensure your BTPC node is properly configured

License
MIT License - See LICENSE file for details.

Disclaimer
This is experimental software. Always:

Test with small amounts first

Maintain secure backups

Use in conjunction with hardware security for large amounts

Keep software updated with security patches

Remember: Your seed phrase is the key to your funds. Never share it with anyone and store it securely!
