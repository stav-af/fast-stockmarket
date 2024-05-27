use chrono::Utc;

use crate::globals::{GRANULARITY, MARKET_EPOCH};


#[derive(Copy, Clone)]
pub struct ObStat {
    pub timestamp: i64,
    pub granularity: GRANULARITY, 
    pub volume: u64,
    pub max_price: f64,
    pub min_price: f64
}

impl Default for ObStat {
    fn default() -> Self {
        ObStat {
            timestamp: Utc::now().timestamp_nanos_opt().unwrap() - *MARKET_EPOCH,
            granularity: GRANULARITY::INSTANT,
            volume: 0,
            max_price: f64::MIN,
            min_price: f64::MAX,
        }
    }
}