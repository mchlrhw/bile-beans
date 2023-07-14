use chrono::{DateTime, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

#[derive(Serialize)]
pub struct Block {
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

impl Block {
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).expect("serialization must succeed"));

        String::from_utf8_lossy(&hasher.finalize()).into()
    }
}

pub struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block {
            timestamp: Utc::now(),
            transactions: vec![],
            proof: 100,
            previous_hash: "1".to_string(),
        };

        Self {
            chain: vec![genesis],
            current_transactions: vec![],
        }
    }

    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: u64) -> usize {
        let transaction = Transaction {
            sender: sender.to_owned(),
            recipient: recipient.to_owned(),
            amount,
        };

        self.current_transactions.push(transaction);

        self.chain.len()
    }

    pub fn new_block(&mut self, proof: u64) -> Block {
        let block = Block {
            timestamp: Utc::now(),
            transactions: self.current_transactions.drain(..).collect(),
            proof,
            previous_hash: self
                .chain
                .last()
                .expect("we must always have a previous block")
                .hash(),
        };

        block
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}
