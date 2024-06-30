use crate::globals::GRANULARITY;

#[derive(Copy, Clone)]
pub struct ObStat {
    pub tick: u64,
    pub granularity: GRANULARITY, 
    pub volume: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64
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
            high: f64::MIN,
            low: f64::MAX,
            open: 0.0,
            close: 0.0
        }
    }
}

struct ObStatBuff<const SLOTS: usize> {
    _curr: usize,
    _stats: [ObStat; SLOTS]
}
