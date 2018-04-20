use crypto_hash::{Algorithm, digest};
use std::ops::Range;
use rayon::prelude::*;

use block::Block;
use to_json::ToJSON;

fn prove_range(block: &Block, number_of_zeroes: usize, range: Range<u64>) -> Option<Block> {
    for proof in range {
        let new_block = block.with_proof(proof);
        let block_as_json = new_block.to_json();
        let block_sha256 = digest(Algorithm::SHA256, block_as_json.as_bytes());
        let all_zero = block_sha256.into_iter()
            .take(number_of_zeroes / 2)
            .all(|value| value.eq(&0));
        if all_zero {
            return Some(new_block);
        }
    }
    None
}

fn calculate_proof(block: &Block, number_of_zeroes: usize) -> Block {
    let chunk_size = 10_000;
    let found = (0..(u64::max_value() / chunk_size))
        .into_par_iter()
        .map(|num| {
            let bottom = num * chunk_size;
            let top = (num + 1) * chunk_size;
            prove_range(block, number_of_zeroes, bottom..top)
        })
        .find_any(|obj| obj.is_some())
        .expect("Could not find proof for block")
        .unwrap();
    found
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

    let proven_block = calculate_proof(&original_block, 4);

    let block_as_json = proven_block.to_json();
    let block_sha256 = digest(Algorithm::SHA256, block_as_json.as_bytes());
    assert_eq!(0, block_sha256[0]);
    assert_eq!(0, block_sha256[1]);
}
