#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub fee: f64,
    pub signature: String,
    pub timestamp: u64,
}
