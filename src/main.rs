use std::time::{SystemTime, UNIX_EPOCH};

use ring::digest::{Context, SHA256};
use ring::signature::ECDSA_P256_SHA256_FIXED;

fn main() {
    println!("Hello, world!");
    //create_db_and_chain();
    // let block = custom_blockchain::blockchain::Blockchain::mine_block(
    //     Transaction::new()
    //     sha256_digest,
    //     current_timestamp,
    //     base58_decode,
    // );
    // println!("{:#?}", block);
}

fn create_db_and_chain() {
    let result = custom_blockchain::blockchain::Blockchain::create_blockchain(
        "123434231234123412312",
        sha256_digest,
        current_timestamp,
        base58_decode,
    );
    println!("{:#?}", result.is_ok());
}
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}
pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}
pub fn ecdsa_p256_sha256_sign_verify(public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_public_key =
        ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public_key);
    let result = peer_public_key.verify(message, signature.as_ref());
    result.is_ok()
}
