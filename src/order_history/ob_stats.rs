use crate::timekeeper::market_time::MTime;
use crate::globals::GRANULARITY;

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
            timestamp: MTime::now(),
            granularity: GRANULARITY::INSTANT,
            volume: 0,
            max_price: f64::MIN,
            min_price: f64::MAX,
        }
    }
}