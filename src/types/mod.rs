//! demo bitcoin types

use std::fmt::Display;

use crate::pow::ProofOfWork;
use anyhow::Result;
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

/// demo bitcoin hash
pub type Hash = [u8; 32];

#[derive(Debug, Clone)]
/// demo Bitcoin block
pub struct Block {
    /// 当前时间戳，也就是区块创建的时间
    pub timestamp: i64,
    /// 区块存储的实际有效信息，也就是交易
    pub data: Vec<u8>,
    /// 前一个块的哈希，即父哈希
    pub prev_block_hash: Hash,
    /// 当前块的哈希
    pub hash: Hash,
    /// nonce
    pub nonce: i64,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp =
            OffsetDateTime::from_unix_timestamp(self.timestamp).expect("convert timestamp error");
        let pow = ProofOfWork::new(self.clone());
        writeln!(f, "Time: {}", timestamp)?;
        writeln!(f, "Prev. hash: {}", hex::encode(self.prev_block_hash))?;
        writeln!(f, "Data: {}", String::from_utf8_lossy(&self.data))?;
        writeln!(f, "Hash: {}", hex::encode(self.hash))?;
        writeln!(f, "PoW: {}", pow.validate())
    }
}

impl Block {
    /// 创建新块时，需要把上一个块的哈希作为参数传进来
    pub fn new(data: Vec<u8>, prev_block_hash: Hash) -> Self {
        let now = OffsetDateTime::now_utc();
        let timestamp = now.unix_timestamp();
        let mut block = Block {
            timestamp,
            data,
            prev_block_hash,
            hash: [0; 32],
            nonce: 0,
        };

        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;
        block
    }

    /// 计算块的哈希
    pub fn hash(data: &[u8], prev_block_hash: &[u8], timestamp: i64) -> Hash {
        let mut input = Vec::new();
        input.extend_from_slice(prev_block_hash);
        input.extend_from_slice(data);
        input.extend_from_slice(&timestamp.to_be_bytes());
        let mut hasher = Sha256::new();
        hasher.update(input);
        let hash_result = hasher.finalize().to_vec();
        let mut hash = [0; 32];
        hash.copy_from_slice(&hash_result);
        hash
    }
}

/// blockchain
pub struct Blockchain {
    /// blocks
    pub blocks: Vec<Block>,
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for block in &self.blocks {
            s.push_str(&format!("{}\n", block));
        }
        write!(f, "{}", s)
    }
}

impl Blockchain {
    /// genesis block
    pub fn new_genesis_block() -> Self {
        let genesis_block = Block::new("Genesis Block".as_bytes().to_vec(), [0; 32]);
        Self {
            blocks: vec![genesis_block],
        }
    }

    /// add block
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev_block = self.blocks.last().ok_or(anyhow::anyhow!("no block"))?;
        let new_block = Block::new(data.as_bytes().to_vec(), prev_block.hash);
        self.blocks.push(new_block);
        Ok(())
    }
}
