#[derive(Copy, Clone)]
pub struct Transaction {
    pub transaction_id: Option<u64>,
    pub buy_id: Option<u64>,
    pub sell_id: Option<u64>,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64
}

impl Transaction {
    pub fn default() -> Self {
        Transaction {
            transaction_id: None,
            buy_id: None,
            sell_id: None,
            price: 0.0,
            volume: 0,
            timestamp: 0,
        }
    }
}