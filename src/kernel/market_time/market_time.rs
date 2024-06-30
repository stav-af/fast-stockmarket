use chrono::Utc;

use crate::globals::{GRANULARITY, MARKET_EPOCH};

pub struct MTime {}

impl MTime {
    pub fn now() -> i64 {
        Utc::now().timestamp_nanos_opt().unwrap() - *MARKET_EPOCH
    }

    pub fn which_second(timestamp: i64) -> u64 {
        (timestamp / GRANULARITY::SECOND as i64) as u64
    }

    pub fn current_second() -> u64 {
        let now = Self::now();
        (now / GRANULARITY::SECOND as i64) as u64
    }

}

