use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::message::Message;
use std::{env, fs};

fn main() {
    let rpc_url = "https://api.devnet.solana.com"; // Connect to Devnet
    let client = RpcClient::new(rpc_url.to_string());

    // Load or create keypair
    let keypair = load_or_create_keypair("wallet.json");
    let public_key = keypair.pubkey();
    println!("Public Key: {}", public_key);

    // Check command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "airdrop" => airdrop(&client, &public_key),
            "send" => {
                if args.len() < 4 {
                    eprintln!("Usage: cargo run send <recipient_pubkey> <amount_in_sol>");
                    return;
                }
                let recipient = args[2].clone();
                let amount: f64 = args[3].parse().expect("Invalid amount");
                send_sol(&client, &keypair, &recipient, amount);
            }
            _ => eprintln!("Unknown command"),
        }
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
fn airdrop(client: &RpcClient, public_key: &Pubkey) {
    println!("💸 Requesting 1 SOL airdrop...");
    match client.request_airdrop(public_key, 1_000_000_000) {
        Ok(tx_signature) => {
            println!("✅ Airdrop successful! Transaction: {}", tx_signature);
            check_balance(client, public_key);
        }
        Err(e) => eprintln!("❌ Airdrop failed: {:?}", e),
    }
}

// Function to send SOL

fn send_sol(client: &RpcClient, sender: &Keypair, recipient: &str, amount_sol: f64) {
    let sender_pubkey = sender.pubkey();
    let recipient_pubkey: Pubkey = recipient.parse().expect("Invalid recipient public key");

    let lamports = (amount_sol * 1_000_000_000.0) as u64;

    let instruction = system_instruction::transfer(&sender_pubkey, &recipient_pubkey, lamports);
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[instruction.clone()], // Clone to avoid move
        Some(&sender_pubkey),
        &[sender],
        recent_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("✅ Transaction successful! Signature: {}", signature),
        Err(e) => eprintln!("❌ Transaction failed: {:?}", e),
    }
}
