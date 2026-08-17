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

impl Blockchain {
    pub fn new() -> Blockchain {
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

        //return gen. block
        //
        Blockchain {
            blocks: vec![genesis_block],
        }
    }
    pub fn add_block() {}
    pub fn is_valid() {}

    pub fn print_chain() {}
}

pub fn calculate_hash(index: u64, timestamp: u64, data: &str, previous_hash: &str) -> String {
    let mut hasher = Sha256::new();
    let combine = format!("{} {} {} {}", index, timestamp, data, previous_hash);
    hasher.update(combine.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn main() {
    let gen_block = Blockchain::new();
    println!("{:?}", gen_block);
}
