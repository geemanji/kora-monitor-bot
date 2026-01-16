use std::env;
use dotenv::dotenv;
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_url: String,
    pub teloxide_token: String,
    pub operator_keypair_path: String,
    pub kora_node_id: String,
    pub network: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        Ok(Config {
            rpc_url: env::var("RPC_URL").context("RPC_URL must be set")?,
            teloxide_token: env::var("TELOXIDE_TOKEN").context("TELOXIDE_TOKEN must be set")?,
            operator_keypair_path: env::var("OPERATOR_KEYPAIR_PATH").context("OPERATOR_KEYPAIR_PATH must be set")?,
            kora_node_id: env::var("KORA_NODE_ID").context("KORA_NODE_ID must be set")?,
            network: env::var("NETWORK").unwrap_or_else(|_| "devnet".to_string()),
        })
    }
}
