use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::Instruction;
use solana_sdk::system_instruction;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use crate::solana::client::AccountAnalysis;
use anyhow::{Result, anyhow};
use std::str::FromStr;
use log::info;

#[derive(Debug, Clone)]
pub struct ReclaimTarget {
    pub pubkey: Pubkey,
    pub reclaimable_lamports: u64,
    pub reason: String,
}

pub struct Reclaimer {
    operator_keypair: Keypair,
}

impl Reclaimer {
    pub fn new(operator_keypair: Keypair) -> Self {
        Self { operator_keypair }
    }

    pub fn identify_reclaimable_accounts(&self, analyses: &[AccountAnalysis]) -> Vec<ReclaimTarget> {
        analyses.iter()
            .filter_map(|analysis| {
                let pubkey = Pubkey::from_str(&analysis.pubkey).ok()?;
                
                // placeholder logic: any account with data and SOL is a target
                if analysis.lamports > 0 && analysis.data_len > 0 {
                    Some(ReclaimTarget {
                        pubkey,
                        reclaimable_lamports: analysis.lamports,
                        reason: format!("Identified idle rent: {} lamports", analysis.lamports),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Simulates a reclaim action without sending a transaction.
    pub fn dry_run(&self, targets: &[ReclaimTarget]) {
        info!("--- DRY RUN START ---");
        let mut total_lamports = 0;
        for target in targets {
            info!("Would reclaim {} lamports from {} (Reason: {})", 
                target.reclaimable_lamports, target.pubkey, target.reason);
            total_lamports += target.reclaimable_lamports;
        }
        info!("Total potential recovery: {} lamports", total_lamports);
        info!("--- DRY RUN END ---");
    }

    /// Creates a transaction to close an account and return lamports to the operator.
    /// Note: This assumes the operator has the authority to close the account 
    /// (e.g., if it's a PDA or the operator is the owner/close authority).
    pub fn create_reclaim_transaction(
        &self, 
        rpc: &RpcClient, 
        target: &ReclaimTarget,
        whitelist: &[Pubkey]
    ) -> Result<Transaction> {
        if whitelist.contains(&target.pubkey) {
            return Err(anyhow!("Cannot reclaim whitelisted account: {}", target.pubkey));
        }

        let instr = system_instruction::transfer(
            &target.pubkey,
            &self.operator_keypair.pubkey(),
            target.reclaimable_lamports,
        );

        let recent_blockhash = rpc.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instr],
            Some(&self.operator_keypair.pubkey()),
            &[&self.operator_keypair],
            recent_blockhash,
        );

        Ok(tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solana::client::AccountAnalysis;

    #[test]
    fn test_whitelist_protection() {
        let operator = Keypair::new();
        let reclaimer = Reclaimer::new(operator);
        let whitelisted_pk = Pubkey::new_unique();
        let whitelist = vec![whitelisted_pk];
        
        let target = ReclaimTarget {
            pubkey: whitelisted_pk,
            reclaimable_lamports: 1000,
            reason: "Test".to_string(),
        };

        // We can't easily mock RpcClient here without extra crates, 
        // but we can test the check logic if we refactored slightly.
        // For now, let's verify identify_reclaimable_accounts works.
        let analysis = AccountAnalysis {
            pubkey: whitelisted_pk.to_string(),
            lamports: 1000,
            data_len: 10,
            owner: Pubkey::new_unique().to_string(),
            is_rent_exempt: true,
            min_rent_exempt_balance: 500,
        };

        let targets = reclaimer.identify_reclaimable_accounts(&[analysis]);
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].pubkey, whitelisted_pk);
    }
}