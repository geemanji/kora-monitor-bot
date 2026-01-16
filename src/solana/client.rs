use solana_client::rpc_client::RpcClient;
use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_client::rpc_config::{RpcProgramAccountsConfig, RpcAccountInfoConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::account::Account;
use solana_account_decoder::UiAccountEncoding;
use std::sync::Arc;
use std::time::Duration;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountAnalysis {
    pub pubkey: String,
    pub lamports: u64,
    pub data_len: usize,
    pub owner: String,
    pub is_rent_exempt: bool,
    pub min_rent_exempt_balance: u64,
}

pub struct SolanaClient {
    pub rpc: Arc<RpcClient>,
}

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        let timeout = Duration::from_secs(30);
        let rpc = RpcClient::new_with_timeout_and_commitment(
            rpc_url.to_string(),
            timeout,
            CommitmentConfig::confirmed(),
        );
        
        SolanaClient {
            rpc: Arc::new(rpc),
        }
    }

    pub fn get_slot(&self) -> Result<u64, ClientError> {
        self.rpc.get_slot()
    }

    pub fn get_sponsored_accounts(&self, kora_node_id: &str) -> Result<Vec<(Pubkey, Account)>, ClientError> {
        let program_id = Pubkey::from_str(kora_node_id)
            .map_err(|_| ClientError::new_with_kind(ClientErrorKind::Custom("Invalid Kora Node ID".into())))?;

        let config = RpcProgramAccountsConfig {
            filters: None,
            account_config: RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                data_slice: None,
                commitment: Some(CommitmentConfig::confirmed()),
                min_context_slot: None,
            },
            with_context: false,
        };

        self.rpc.get_program_accounts_with_config(&program_id, config)
    }

    pub fn analyze_account(&self, pubkey: &Pubkey, account: &Account) -> Result<AccountAnalysis, ClientError> {
        let data_len = account.data.len();
        let min_rent = self.rpc.get_minimum_balance_for_rent_exemption(data_len)?;
        
        Ok(AccountAnalysis {
            pubkey: pubkey.to_string(),
            lamports: account.lamports,
            data_len,
            owner: account.owner.to_string(),
            is_rent_exempt: account.lamports >= min_rent,
            min_rent_exempt_balance: min_rent,
        })
    }

    pub fn analyze_accounts(&self, accounts: Vec<(Pubkey, Account)>) -> Vec<AccountAnalysis> {
        accounts.into_iter()
            .filter_map(|(pk, acc)| {
                self.analyze_account(&pk, &acc).ok()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::account::Account;

    #[test]
    fn test_analyze_account_basic() {
        // We can't easily test RpcClient methods without a mock or live connection,
        // but we can structure logic to be more testable if needed.
        // For now, let's verify the struct can be instantiated.
        let pk = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let analysis = AccountAnalysis {
            pubkey: pk.to_string(),
            lamports: 1000,
            data_len: 10,
            owner: owner.to_string(),
            is_rent_exempt: true,
            min_rent_exempt_balance: 500,
        };
        
        assert_eq!(analysis.pubkey, pk.to_string());
        assert_eq!(analysis.lamports, 1000);
        assert!(analysis.is_rent_exempt);
    }
}

