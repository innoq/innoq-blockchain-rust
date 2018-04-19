pub struct Block {
    index: u64,
    timestamp: u64,
    proof: u64,
    transactions: [Transaction],
    previous_block_hash: String,
}

pub struct Transaction {
    id: String,
    timestamp: u64,
    payload: String,
}
