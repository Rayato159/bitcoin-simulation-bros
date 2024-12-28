use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

use crate::models::{
    block_model::BlockModel, coin_base::CoinBase, hashing_model::HashingModel,
    transaction::Transaction,
};

const DIFFICULTY: usize = 4;
const INIT_MINING_REWARD: f64 = 50.0;
const DECRESE_RATE: f64 = 0.5;

pub fn generate_random_transactions() -> Vec<Transaction> {
    let mut rng = rand::thread_rng(); // Create the random number generator

    let names = [
        "Satoshi", "Alice", "Bob", "Charlie", "Dave", "Eve", "Frank", "Grace", "Hank", "Ivy",
    ];

    // Generate a random number of transactions (1 to 3)
    let transaction_count = rng.gen_range(1..=3);

    // Generate the transactions
    (0..transaction_count)
        .map(|_| {
            // Random sender and receiver
            let sender_index = rng.gen_range(0..names.len());
            let mut receiver_index = rng.gen_range(0..names.len());
            while receiver_index == sender_index {
                receiver_index = rng.gen_range(0..names.len());
            }

            Transaction {
                sender: names[sender_index].to_string(),
                receiver: names[receiver_index].to_string(),
                amount: rng.gen_range(0.0..=10.0), // Random amount (0 to 10)
                fee: 0.5,                          // Fixed fee
                signature: (0..64) // Generate a random 64-character alphanumeric signature
                    .map(|_| rng.sample(Alphanumeric) as char)
                    .collect(),
                timestamp: Utc::now().timestamp() as u64, // Current timestamp
            }
        })
        .collect()
}

pub fn mine_block(last_block: &mut BlockModel) -> BlockModel {
    let target = "0".repeat(DIFFICULTY);

    loop {
        let hashing_model = HashingModel {
            index: last_block.index,
            nonce: last_block.nonce,
            coin_base: last_block.coin_base.clone(),
            transactions: last_block.transactions.clone(),
            timestamp: last_block.timestamp,
            previous_hash: last_block.previous_hash.clone(),
        };

        let hash = hashing_model.hash_calulation();
        last_block.hash = hash.clone();

        if hash.starts_with(&target) {
            let reward = if last_block.index % 4 == 0 {
                last_block.coin_base.reward - (INIT_MINING_REWARD * DECRESE_RATE)
            } else {
                last_block.coin_base.reward
            };

            let new_block_hashing_model = HashingModel {
                index: last_block.index + 1,
                nonce: 0,
                coin_base: CoinBase {
                    miner: "Lookhin".to_string(),
                    reward,
                },
                transactions: generate_random_transactions(),
                timestamp: chrono::Utc::now().timestamp() as u64,
                previous_hash: hash,
            };

            return BlockModel {
                index: new_block_hashing_model.index,
                nonce: 0,
                coin_base: new_block_hashing_model.coin_base.clone(),
                transactions: new_block_hashing_model.transactions.clone(),
                timestamp: new_block_hashing_model.timestamp,
                previous_hash: new_block_hashing_model.previous_hash.clone(),
                hash: new_block_hashing_model.hash_calulation(),
            };
        }

        last_block.nonce += 1;
    }
}
