use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::Sha256;

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
}

fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    let combine = format!("{} {} {} {}", index, timestamp, data, previous_hash);
    hasher.update(combine.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn main() {
    // genesis block
    let index = 0;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let data = String::from("Genesis Block");
    let previous_hash = String::from("0");
    let hash = calculate_hash(index, timestamp, &data, &previous_hash);

    let genesis_block = Block {
        index: index,
        timestamp: timestamp,
        data: data,
        previous_hash: previous_hash,
        hash: hash,
    };
    // block 1
    let block1_index = 1;
    let block1_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let block1_data = String::from("Transfer from genesis block");
    let genesis_block_hash = genesis_block.hash.clone();
    let block1_hash = calculate_hash(index, timestamp, &block1_data, &genesis_block_hash);

    let block1 = Block {
        index: block1_index,
        timestamp: block1_timestamp,
        data: block1_data,
        previous_hash: genesis_block_hash,
        hash: block1_hash,
    };

    let block_chain = Blockchain {
        blocks: vec![genesis_block, block1],
    };

    println!("{:#?}", block_chain);
}
