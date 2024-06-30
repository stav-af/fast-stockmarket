#[derive(Copy, Clone)]
pub struct Transaction {
    pub transaction_id: Option<u64>,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64
}
