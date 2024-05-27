use chrono::Utc;

use crate::globals::MARKET_EPOCH;

pub struct MTime {}

impl MTime {
    pub fn now() -> i64 {
        Utc::now().timestamp_nanos_opt().unwrap() - *MARKET_EPOCH
    }
}

