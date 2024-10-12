use serde::{Deserialize, Serialize};
use sled::IVec;

use crate::{proof_of_work::ProofOfWork, transaction::Transaction};

//todo; move those public types to lib level
pub type Hash = String;
pub type Nonce = i64;
type Timestamp = i64;
pub type TimestampProvider = fn() -> Timestamp;
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: Timestamp,
    pre_block_hash: Hash,
    hash: Hash,
    transactions: Vec<Transaction>,
    nonce: Nonce,
    height: usize,
}
impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}
pub type HashFn = fn(&[u8]) -> Vec<u8>;
pub type SignVerifyFn = fn(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool;
pub type SignFn = fn(pkcs8: &[u8], message: &[u8]) -> Vec<u8>;
impl Block {
    fn new_block(
        ts_provider: TimestampProvider,
        pre_block_hash: Hash,
        hash: Hash,
        transactions: &[Transaction],
        nonce: Nonce,
        height: usize,
    ) -> Self {
        Self {
            timestamp: ts_provider(),
            pre_block_hash,
            hash,
            transactions: transactions.into(),
            nonce,
            height,
        }
    }

    pub fn get_pre_block_hash(&self) -> &Hash {
        &self.pre_block_hash
    }
    pub fn hash_transactions(&self, hash_fn: HashFn) -> Vec<u8> {
        let txhashs: Vec<u8> = self
            .transactions
            .iter()
            .flat_map(|tx| tx.get_id())
            .copied()
            .collect();
        hash_fn(&txhashs)

        //crate::sha256_digest(&txhashs)
    }

    pub fn get_timestamp(&self) -> &Timestamp {
        &self.timestamp
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Block, bincode::Error> {
        Ok(bincode::deserialize(bytes)?)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        Ok(bincode::serialize(self)?.to_vec())
    }
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }
    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }
    pub fn new(
        ts_provider: TimestampProvider,
        hash_fn: HashFn,
        pre_block_hash: Hash,
        transactions: &[Transaction],
        height: usize,
    ) -> Block {
        let mut block = Block::new_block(
            ts_provider,
            pre_block_hash,
            String::new(),
            transactions,
            0,
            height,
        );
        let pow = ProofOfWork::new(&block);

        //todo:return erro below
        let (nonce, hash) = pow.run(hash_fn).unwrap();
        block.nonce = nonce;
        block.hash = hash;
        block
    }
    pub fn generate_genesis_block(
        ts_provider: TimestampProvider,
        hash_fn: HashFn,
        transaction: &Transaction,
    ) -> Block {
        const NONE_HASH: &'static str = "None";
        let transactions = vec![transaction.clone()];
        return Block::new(
            ts_provider,
            hash_fn,
            String::from(NONE_HASH),
            &transactions,
            0,
        );
    }

    pub(crate) fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }
}
// impl TryFrom<Block> for IVec {
//     type Error = bincode::Error;

//     fn try_from(value: Block) -> Result<Self, Self::Error> {
//         let bytes = bincode::serialize(&value)?;
//         Ok(Self::from(bytes))
//     }
// }
