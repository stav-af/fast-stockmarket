use once_cell::sync::Lazy;
use chrono::Utc;

pub static MARKET_EPOCH: Lazy<i64> = Lazy::new(|| {
    return Utc::now().timestamp_nanos_opt().unwrap(); 
});
