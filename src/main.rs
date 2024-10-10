use std::time::{SystemTime, UNIX_EPOCH};

use ring::digest::{Context, SHA256};

fn main() {
    println!("Hello, world!");
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
