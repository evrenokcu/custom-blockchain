use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};

use crate::block::{Block, Hash, HashFn, Nonce};

pub struct ProofOfWork<'a> {
    block: &'a Block,
    target: BigInt,
}

#[derive(Debug)]
pub enum MiningError {
    NonceExhausted,
}

impl<'a> ProofOfWork<'a> {
    const TARGET_BITS: i32 = 8;
    const MAX_NONCE: i64 = i64::MAX;

    pub fn new(block: &'a Block) -> Self {
        let target = BigInt::from(1) << (256 - Self::TARGET_BITS);
        Self { block, target }
    }

    fn prepare_data(&self, nonce: Nonce, hash_fn: HashFn) -> Vec<u8> {
        [
            self.block.get_pre_block_hash().as_bytes(),
            &self.block.hash_transactions(hash_fn),
            &self.block.get_timestamp().to_be_bytes(),
            &Self::TARGET_BITS.to_be_bytes(),
            &nonce.to_be_bytes(),
        ]
        .concat()
    }

    pub fn run(&self, hash_fn: HashFn) -> Result<(Nonce, Hash), MiningError> {
        //todo: log instead of print
        println!("Mining the block...");

        (0..Self::MAX_NONCE)
            .find_map(|nonce| {
                let data = self.prepare_data(nonce, hash_fn);
                let hash = hash_fn(&data);
                let hash_int = BigInt::from_bytes_be(Sign::Plus, &hash);

                if hash_int < self.target {
                    let encoded_hash = HEXLOWER.encode(&hash);
                    println!("Hash found: {}", encoded_hash);
                    Some(Ok((nonce, encoded_hash)))
                } else {
                    None
                }
            })
            .unwrap_or_else(|| {
                println!("Max nonce reached without finding a valid hash.");
                Err(MiningError::NonceExhausted)
            })
    }
}
