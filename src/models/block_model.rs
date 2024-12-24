use super::{coin_base::CoinBase, hashing_model::HashingModel, transaction::Transaction};

#[derive(Debug, Clone)]
pub struct BlockModel {
    pub index: u64,
    pub nonce: u64,
    pub coin_base: CoinBase,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl BlockModel {
    pub fn create_genesis_block() -> Self {
        let index = 1;
        let nonce = 0;
        let coin_base = CoinBase {
            miner: "Lookhin".to_string(),
            reward: 50.0,
        };
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let previous_hash = "0".repeat(64);
        let transactions = Vec::new();

        let hashing_model = HashingModel {
            index,
            nonce,
            coin_base: coin_base.clone(),
            transactions: transactions.clone(),
            timestamp,
            previous_hash: previous_hash.clone(),
        };

        BlockModel {
            index,
            nonce,
            coin_base,
            transactions,
            timestamp,
            previous_hash,
            hash: hashing_model.hash_calulation(),
        }
    }
}
