use std::{
    env::current_dir,
    sync::{Arc, RwLock},
};

use sled::{transaction::TransactionResult, Db, Tree};

use crate::{
    block::{Block, HashFn, TimestampProvider},
    transaction::Transaction,
};

const DATA_FOLDER: &'static str = "Data";
const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";
const BLOCKS_TREE: &str = "blocks";

#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>, // hash of last block
    db: Db,
}

impl Blockchain {
    pub fn create_blockchain(
        genesis_address: &str,
        hash_fn: HashFn,
        ts_provider: TimestampProvider,
        decoder: fn(&str) -> Vec<u8>,
    ) -> Result<Blockchain, std::io::Error> {
        let db = sled::open(current_dir()?.join(DATA_FOLDER))?;
        let block_tree = db.open_tree(BLOCKS_TREE)?;
        let data = block_tree.get(TIP_BLOCK_HASH_KEY)?;

        let tip_hash = match data {
            Some(data) => String::from_utf8(data.to_vec()).expect("could not get tip hash"),

            None => {
                let coinbase_tx =
                    Transaction::new_coinbase_tx(hash_fn, decoder, genesis_address).unwrap();
                let block = Block::generate_genesis_block(ts_provider, hash_fn, &coinbase_tx);
                Self::update_blocks_tree(&block_tree, &block);
                String::from(block.get_hash())
            }
        };

        Ok(Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        })
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash().as_str();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }
}

// pub fn create_blockchain(genesis_address: &str) -> Blockchain {
//     let db = sled::open(current_dir().unwrap().join("data")).unwrap();
//     let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
//     let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
//     let tip_hash;
//     if data.is_none() {
//         let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
//         let block = Block::generate_genesis_block(&coinbase_tx);
//         Self::update_blocks_tree(&blocks_tree, &block);
//         tip_hash = String::from(block.get_hash());
//     } else {
//         tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
//     }
//     Blockchain {
//         tip_hash: Arc::new(RwLock::new(tip_hash)),
//         db,
//     }
// }

// struct Blockchain {}

// impl Blockchain {
//     pub fn new_blockchain() -> Blockchain {
//         let db = sled::open(current_dir().unwrap().join("data")).unwrap();
//         let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
//         let tip_bytes = blocks_tree
//             .get(TIP_BLOCK_HASH_KEY)
//             .unwrap()
//             .expect("No existing blockchain found. Create one first.");
//         let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();
//         Blockchain {
//             tip_hash: Arc::new(RwLock::new(tip_hash)),
//             db,
//         }
//     }
//     pub fn get_db(&self) -> &Db {
//         &self.db
//     }
//     pub fn get_tip_hash(&self) -> String {
//         //todo: return result below
//         self.tip_hash.read().unwrap().clone()
//     }
//     pub fn set_tip_hash(&self, new_tip_hash: &str) {
//         let mut tip_hash = self.tip_hash.write().unwrap();
//         *tip_hash = String::from(new_tip_hash)
//     }
//     pub fn iterator(&self) -> BlockchainIterator {
//         BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
//     }
// }
// pub struct BlockchainIterator {
//     db: Db,
//     current_hash: String,
// }
// impl BlockchainIterator {
//     fn new(tip_hash: String, db: Db) -> BlockchainIterator {}
//     pub fn next(&mut self) -> Option<Block> {}
// }
// pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
//     for trasaction in transactions {
//         if trasaction.verify(self) == false {
//             panic!("ERROR: Invalid transaction")
//         }
//     }
//     let best_height = self.get_best_height();
//     let block = Block::new_block(self.get_tip_hash(), transactions, best_height + 1);
//     let block_hash = block.get_hash();
//     let blocks_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
//     Self::update_blocks_tree(&blocks_tree, &block);
//     self.set_tip_hash(block_hash);
//     block
// }
