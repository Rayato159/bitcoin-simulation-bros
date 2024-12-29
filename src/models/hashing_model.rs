use super::{coin_base::CoinBase, transaction::Transaction};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct HashingModel {
    pub index: u64,
    pub nonce: u64,
    pub coin_base: CoinBase,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
    pub previous_hash: String,
}

impl HashingModel {
    pub fn hash_calulation(&self) -> String {
        let mut hasher = Sha256::new();

        let transaction_data: String =
            self.transactions
                .iter()
                .fold(String::new(), |mut acc, transaction| {
                    acc.push_str(&format!(
                        "{}{}{}{}{}",
                        transaction.sender,
                        transaction.receiver,
                        transaction.amount,
                        transaction.fee,
                        transaction.signature,
                    ));
                    acc
                });

        hasher.update(format!(
            "{}{}{}{}{}{}{}",
            self.index,
            self.nonce,
            self.coin_base.miner,
            transaction_data,
            self.coin_base.reward,
            self.timestamp,
            self.previous_hash
        ));

        let hash = hasher.finalize();

        format!("{:x}", hash)
    }
}
