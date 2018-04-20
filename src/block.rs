use std::time::{SystemTime, UNIX_EPOCH};
use crypto_hash::digest;
use crypto_hash::Algorithm;
use to_json::ToJSON;
use calculate_proof::calculate_proof;

#[derive(Clone, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub block_height: u64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof: u64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
}

impl Block {
    pub fn with_proof(&self, proof: u64) -> Block {
        Block {
            index: self.index.clone(),
            timestamp: self.timestamp.clone(),
            proof: proof,
            transactions: self.transactions.clone(),
            previous_block_hash: self.previous_block_hash.clone()
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Transaction {
    pub id: String,
    pub timestamp: u64,
    pub payload: String,
}

fn hex_string_from(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

impl Blockchain {
    pub fn add(&self, block: Block) -> Blockchain {
        let mut new_chain = self.clone();
        new_chain.block_height += 1;
        new_chain.blocks.push(block);
        new_chain
    }

    pub fn generate_next_block(&self) -> Block {
        let prev = self.blocks.last().expect("heribert is always there");
        let new_block = Block {
            index: prev.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_secs(),
            proof: 0,
            transactions: Vec::new(),
            previous_block_hash: hex_string_from(digest(Algorithm::SHA256, prev.to_json().as_bytes())),
        };
        calculate_proof(&new_block, 6)
    }

    pub fn new() -> Blockchain {
        return Blockchain {
            block_height: 1,
            blocks: vec![
                Block {
                    index: 1,
                    timestamp: 0,
                    proof: 1917336,
                    transactions: vec![
                        Transaction {
                            id: String::from("b3c973e2-db05-4eb5-9668-3e81c7389a6d"),
                            timestamp: 0,
                            payload: String::from("I am Heribert Innoq"),
                        },
                    ],
                    previous_block_hash: String::from("0"),
                },
            ],
        };
    }
}

#[test]
fn constructor_creates_correct_genesis_block() {
    let blockchain = Blockchain::new();
    assert_eq!(blockchain.blocks[0].proof, 1917336);
}
