use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::Sha256;

struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    let combine = format!("{} {} {} {}", index, timestamp, data, previous_hash);
    hasher.update(combine.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn main() {
    let index = 0;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let data = String::from("Genesis Block");
    let previous_hash = String::from("0");
    let hash = calculate_hash(index, timestamp, &data, &previous_hash);

    let block1 = Block {
        index: index,
        timestamp: timestamp,
        data: data,
        previous_hash: previous_hash,
        hash: hash,
    };

    let index1 = 0;
    let data1 = String::from("Transferred from Block 1");
    let prev_hash1 = block1.hash.clone();
    let new_hash = calculate_hash(index1, timestamp, &data1, &prev_hash1);

    let block2 = Block {
        index: index1,
        timestamp: timestamp,
        data: data1,
        previous_hash: prev_hash1,
        hash: new_hash,
    };

    println!("Block #{}", block1.index);
    println!("Timestamp: {}", block1.timestamp);
    println!("Data: {}", block1.data);
    println!("Previous Hash: {}", block1.previous_hash);
    println!("Hash: {}", block1.hash);
    println!();
    println!("Block #{}", block2.index);
    println!("Timestamp: {}", block2.timestamp);
    println!("Data: {}", block2.data);
    println!("Previous Hash: {}", block2.previous_hash);
    println!("Hash: {}", block2.hash);
}
