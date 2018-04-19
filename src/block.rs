pub struct Blocks {
    pub blocks: Vec<Block>,
    pub block_height: u64,
}

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof: u64,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: String,
}

pub struct Transaction {
    pub id: String,
    pub timestamp: u64,
    pub payload: String,
}
