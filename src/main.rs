use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::fs;

fn main() {
    let rpc_url = "https://api.devnet.solana.com"; // Connect to Devnet
    let client = RpcClient::new(rpc_url.to_string());

    // Load or create keypair
    let keypair = load_or_create_keypair("wallet.json");
    let public_key = keypair.pubkey();

    println!("Wallet Address: {}", public_key);

    // Fetch and display balance
    match client.get_balance(&public_key) {
        Ok(balance) => println!("Wallet Balance: {} SOL", balance as f64 / 1_000_000_000.0),
        Err(e) => eprintln!("Error fetching balance: {:?}", e),
    }
}

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
