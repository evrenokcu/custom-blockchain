use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{block::HashFn, wallet};

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}
impl Transaction {
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    pub(crate) fn new_coinbase_tx(
        hash_fn: HashFn,
        decoder: fn(&str) -> Vec<u8>,
        to: &str,
    ) -> Result<Transaction, bincode::Error> {
        const SUBSIDY: i32 = 10;
        let txout = TXOutput::new(SUBSIDY, to, decoder);
        let mut tx_input = TXInput::default();
        tx_input.signature = Uuid::new_v4().as_bytes().to_vec();
        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_input],
            vout: vec![txout],
        };
        tx.id = tx.hash(hash_fn)?;
        Ok(tx)
    }
    // fn lock(&mut self, address: &str, decoder: fn(&[u8]) -> Vec<u8>) {
    //     let payload = base58_decode(address);
    //     let payload = decoder(address);
    //     let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
    //     self.pub_key_hash = pub_key_hash;
    // }

    fn hash(&self, hash_fn: HashFn) -> Result<Vec<u8>, bincode::Error> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        Ok(hash_fn(tx_copy.serialize()?.as_slice()))
        //crate::sha256_digest(tx_copy.serialize().unwrap().as_slice())
    }
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        Ok(bincode::serialize(self)?.to_vec())
    }
}
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TXInput {
    txid: Vec<u8>,
    vout: usize,
    signature: Vec<u8>,
    pub_key: Vec<u8>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TXOutput {
    value: i32,
    pub_key_hash: Vec<u8>,
}

impl TXOutput {
    pub fn new(value: i32, address: &str, decoder: fn(&str) -> Vec<u8>) -> TXOutput {
        let mut output = TXOutput {
            value,
            pub_key_hash: vec![],
        };
        output.lock(address, decoder);
        return output;
    }
    fn lock(&mut self, address: &str, decoder: fn(&str) -> Vec<u8>) {
        let payload = decoder(address);
        let pub_key_hash = payload[1..payload.len() - wallet::ADDRESS_CHECK_SUM_LEN].to_vec();
        self.pub_key_hash = pub_key_hash;
    }
}
