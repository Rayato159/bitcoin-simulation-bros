use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

use crate::models::transaction::Transaction;

pub fn generate_random_transactions() -> Vec<Transaction> {
    let mut rng = rand::thread_rng(); // Create the random number generator

    let names = vec![
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
