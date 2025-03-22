# Solana Wallet CLI

A simple **Rust-based Solana CLI tool** to create a wallet, check balance, and airdrop SOL on Devnet.

## 🚀 Features
- Generate a **new Solana wallet**
- Load an **existing wallet**
- Check **wallet balance**
- Airdrop **SOL on Devnet**

## 📦 Installation
Make sure you have **Rust and Cargo** installed:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:
```sh
git clone <your-repo-url>
cd solana_wallet
```

Install dependencies:
```sh
cargo build
```

## 🔑 Generate a New Wallet
Run the following command:
```sh
cargo run generate_wallet
```

This will **create a new keypair** and save it as `wallet.json`.

## 💰 Check Wallet Balance
To check the balance of your wallet:
```sh
cargo run check_balance
```
If the `wallet.json` file is missing, a **new wallet will be generated automatically**.

## 🚀 Airdrop SOL (Devnet)
To request **1 SOL** from the Devnet faucet:
```sh
cargo run airdrop_sol
```
⚠️ You need **Devnet SOL** to proceed with transactions. Use the airdrop command to get some.

## 🔧 Configuration
This tool uses **Solana Devnet** by default. You can change the RPC URL in `main.rs`:
```rust
let rpc_url = "https://api.devnet.solana.com";
```

## 📜 Notes
- **Keypair is stored in `wallet.json`**.
- **Use Devnet for testing** (not real SOL).
- **Ensure wallet.json is not shared publicly**.

## 🛠 Next Steps
- Send SOL transactions.
- Implement error handling.
- Improve logging and CLI usability.

---

