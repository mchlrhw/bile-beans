use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

fn hash(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s);

    hex::encode(hasher.finalize())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        Self {
            sender,
            recipient,
            amount,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: String,
}

impl Block {
    fn hash(&self) -> String {
        hash(&serde_json::to_string(self).expect("serialization must succeed"))
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

    pub fn chain(&self) -> Vec<Block> {
        self.chain.clone()
    }

    pub fn last_proof(&self) -> u64 {
        self.chain
            .last()
            .expect("we must always have a previous block")
            .proof
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

        self.chain.push(block.clone());

        block
    }

    pub fn resolve_conflicts(&mut self, chains: &[Vec<Block>]) -> Vec<Block> {
        let mut new_chain: Option<Vec<Block>> = None;
        let mut longest = self.chain.len();

        for chain in chains {
            if chain.len() > longest && is_valid(chain) {
                longest = chain.len();
                new_chain = Some(chain.to_owned());
            }
        }

        if let Some(new_chain) = new_chain {
            self.chain = new_chain;
        }

        self.chain.clone()
    }
}

fn is_valid(chain: &[Block]) -> bool {
    for (block_a, block_b) in chain.iter().tuple_windows() {
        if block_b.previous_hash != block_a.hash() {
            return false;
        }

        if !valid_proof(block_a.proof, block_b.proof) {
            return false;
        }
    }

    true
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

fn valid_proof(last_proof: u64, proof: u64) -> bool {
    let guess = hash(&format!("{last_proof}{proof}"));

    guess.starts_with("0000")
}

pub fn proof_of_work(last_proof: u64) -> u64 {
    for proof in 0.. {
        if valid_proof(last_proof, proof) {
            return proof;
        }
    }

    panic!("we should always find a proof")
}
