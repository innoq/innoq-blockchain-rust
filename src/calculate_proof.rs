use crypto_hash::{Algorithm, digest};
use std::ops::Index;

use block::Block;
use to_json::ToJSON;

fn calculate_proof(block: &Block, number_of_zeroes: usize) -> Block {
    for proof in 0..u64::max_value() {
        let new_block = block.with_proof(proof);
        let block_as_json = new_block.to_json();
        let block_sha256 = digest(Algorithm::SHA256, block_as_json.as_bytes());
        let all_zero = (0..number_of_zeroes).all(|value| block_sha256.index(value).eq(&0));
        if all_zero {
            return new_block;
        }
    }
    panic!("Could not find proof for block");
}

#[test]
fn calculate_proof_matches() {
    use block::Transaction;
    let transaction = Transaction {
        id: String::from("Some id"),
        timestamp: 12345678,
        payload: String::from("Some payload"),
    };
    let original_block = Block {
        index: 33,
        timestamp: 33,
        proof: 0,
        transactions: vec![transaction],
        previous_block_hash: String::from("previous_hash"),
    };

    let expected_proof: u64 = 8334;

    let proven_block = calculate_proof(&original_block, 2);
    assert_eq!(expected_proof, proven_block.proof);
}