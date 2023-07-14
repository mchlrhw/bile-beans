use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
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

    pub fn new_transaction(&mut self, transaction: Transaction) -> usize {
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

fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let guess = format!("{last_proof}{proof}");

    let mut hasher = Sha256::new();
    hasher.update(guess);
    let guess_hash = String::from_utf8_lossy(&hasher.finalize()).to_string();

    guess_hash.starts_with("0000")
}

pub fn proof_of_work(last_proof: u64) -> u64 {
    let mut proof = 0;
    while !valid_proof(last_proof, proof) {
        proof += 1;
    }

    proof
}
