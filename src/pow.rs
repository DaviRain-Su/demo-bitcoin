//!  pow.rs

use crate::types::Block;
use num_bigint::BigUint;
use num_traits::One;
use sha2::{Digest, Sha256};

///
pub const TARGET_BITS: u16 = 24;
/// ProofOfWork is a struct that holds a block and a target
pub struct ProofOfWork {
    /// block is a pointer to a block
    pub block: Block,
    /// target is a pointer to a target
    pub target: BigUint,
}

fn calca_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash_result = hasher.finalize().to_vec();
    let mut hash1 = [0; 32];
    hash1.copy_from_slice(&hash_result);
    hash1
}

impl ProofOfWork {
    /// new is a constructor for ProofOfWork
    pub fn new(b: Block) -> ProofOfWork {
        let one: BigUint = One::one();
        let target = one << (256u16 - TARGET_BITS);
        ProofOfWork { block: b, target }
    }

    /// prepare_data is a method that returns a byte array
    pub fn prepar_data(&self, nonce: i64) -> Vec<u8> {
        let mut data = vec![];
        data.extend_from_slice(&self.block.prev_block_hash);
        data.extend_from_slice(&self.block.data);
        data.extend_from_slice(&self.block.timestamp.to_be_bytes());
        data.extend_from_slice(&TARGET_BITS.to_be_bytes());
        data.extend_from_slice(&nonce.to_be_bytes());
        data
    }

    ///
    pub fn run(&self) -> (i64, [u8; 32]) {
        let mut hash = [0u8; 32];
        let mut nonce = 0i64;
        println!(
            "Mining the block containing \"{}\"",
            String::from_utf8_lossy(&self.block.data)
        );
        while nonce < i64::max_value() {
            let data = self.prepar_data(nonce);
            hash = calca_hash(&data);
            let hash_int = BigUint::from_bytes_be(&hash);
            if hash_int < self.target {
                println!("\r{}", hex::encode(data));
                break;
            } else {
                nonce += 1;
            }
        }
        (nonce, hash)
    }

    ///
    pub fn validate(&self) -> bool {
        let data = self.prepar_data(self.block.nonce);
        let hash = calca_hash(&data);
        let hash_int = BigUint::from_bytes_be(&hash);
        hash_int.cmp(&self.target).is_lt()
    }
}
