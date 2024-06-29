use crate::globals::GRANULARITY;

#[derive(Copy, Clone)]
pub struct ObStat {
    pub tick: u64,
    pub granularity: GRANULARITY, 
    pub volume: u64,
    pub max_price: f64,
    pub min_price: f64
}

#[derive(Copy, Clone)]
pub struct Transaction {
    pub transaction_id: Option<u64>,
    pub price: f64,
    pub volume: u64,
    pub timestamp: i64
}

impl Default for ObStat {
    fn default() -> Self {
        ObStat {
            tick: 0,
            granularity: GRANULARITY::INSTANT,
            volume: 0,
            max_price: f64::MIN,
            min_price: f64::MAX,
        }
    }
}