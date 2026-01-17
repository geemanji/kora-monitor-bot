use kora_monitor_bot::config::Config;
use kora_monitor_bot::solana::test_utils::TestAccountGenerator;
use solana_sdk::signature::{read_keypair_file, Signer};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let payer = read_keypair_file(&config.operator_keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
    
    let generator = TestAccountGenerator::new(&config.rpc_url, payer);
    let owner = Pubkey::from_str(&config.kora_node_id)?;

    println!("Creating 3 test accounts on Devnet for Kora Node: {}...", owner);

    for i in 1..=3 {
        match generator.create_abandoned_account(&owner, 32) {
            Ok(kp) => println!("Account {} created: {}", i, kp.pubkey()),
            Err(e) => eprintln!("Failed to create account {}: {}", i, e),
        }
    }

    println!("Done! Now restart your bot and run /status.");
    Ok(())
}