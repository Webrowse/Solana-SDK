use solana_sdk::signature::{Keypair, Signer};
use std::{fs::File, io::Write};
use solana_sdk::bs58;

fn main() {
    // Generate a new keypair
    let keypair = Keypair::new();

    // Convert the keypair to JSON
    let keypair_json = bs58::encode(keypair.to_bytes()).into_string();
    // Save to a file
    let mut file = File::create("wallet.json").expect("Failed to create file");
    file.write_all(keypair_json.as_bytes()).expect("Failed to write keypair");

    // Print public key
    println!("✅ Wallet created!");
    println!("Public Key: {}", keypair.pubkey());
}
