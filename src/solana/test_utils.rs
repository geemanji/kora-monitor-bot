use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use solana_client::rpc_client::RpcClient;
use anyhow::Result;

pub struct TestAccountGenerator {
    client: RpcClient,
    payer: Keypair,
}

impl TestAccountGenerator {
    pub fn new(rpc_url: &str, payer: Keypair) -> Self {
        Self {
            client: RpcClient::new(rpc_url.to_string()),
            payer,
        }
    }

    /// Creates a new account owned by `owner` with some SOL.
    pub fn create_abandoned_account(&self, owner: &solana_sdk::pubkey::Pubkey, space: usize) -> Result<Keypair> {
        let new_account = Keypair::new();
        let rent = self.client.get_minimum_balance_for_rent_exemption(space)?;
        
        let instr = system_instruction::create_account(
            &self.payer.pubkey(),
            &new_account.pubkey(),
            rent,
            space as u64,
            owner,
        );

        let recent_blockhash = self.client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[instr],
            Some(&self.payer.pubkey()),
            &[&self.payer, &new_account],
            recent_blockhash,
        );

        self.client.send_and_confirm_transaction(&tx)?;
        
        Ok(new_account)
    }
}
