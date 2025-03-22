use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use std::{env, fs};

fn main() {
    let rpc_url = "https://api.devnet.solana.com"; // Connect to Devnet
    let client = RpcClient::new(rpc_url.to_string());

    // Load or create keypair
    let keypair = load_or_create_keypair("wallet.json");
    let public_key = keypair.pubkey();

    // Check command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "airdrop" {
        airdrop_sol(&client, &public_key);
    } else {
        check_balance(&client, &public_key);
    }
}

// Function to load or create a wallet
fn load_or_create_keypair(path: &str) -> Keypair {
    match fs::read_to_string(path) {
        Ok(data) => {
            println!("🔹 Wallet found! Loading existing keypair.");
            Keypair::from_base58_string(&data)
        }
        Err(_) => {
            println!("🚀 No wallet found! Creating a new one...");
            let new_keypair = Keypair::new();
            let encoded_key = new_keypair.to_base58_string();
            fs::write(path, encoded_key).expect("Failed to save wallet file");
            println!("✅ New wallet saved to '{}'", path);
            new_keypair
        }
    }
}

// Function to fetch wallet balance
fn check_balance(client: &RpcClient, public_key: &Pubkey) {
    match client.get_balance(public_key) {
        Ok(balance) => println!("💰 Wallet Balance: {} SOL", balance as f64 / 1_000_000_000.0),
        Err(e) => eprintln!("❌ Error fetching balance: {:?}", e),
    }
}

// Function to request 1 SOL airdrop
fn airdrop_sol(client: &RpcClient, public_key: &Pubkey) {
    println!("💸 Requesting 1 SOL airdrop...");
    match client.request_airdrop(public_key, 1_000_000_000) {
        Ok(tx_signature) => {
            println!("✅ Airdrop successful! Transaction: {}", tx_signature);
            check_balance(client, public_key);
        }
        Err(e) => eprintln!("❌ Airdrop failed: {:?}", e),
    }
}
