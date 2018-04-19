#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub block_height: u64,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub timestamp: u64,
    pub payload: String,
}

impl Blockchain {
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
