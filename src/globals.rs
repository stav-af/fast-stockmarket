use once_cell::sync::Lazy;
use chrono::Utc;

// TIMEKEEPING CONSTANTS
// keeps track of the start of the market, allowing us to vary time as we like
pub static MARKET_EPOCH: Lazy<i64> = Lazy::new(|| {
    return Utc::now().timestamp_nanos_opt().unwrap(); 
});

// describes the 'display' nanoseconds passed every 'real' nanosecond
// 8760 means that every simulated second describes a 'real' hour.
pub static ACCELERATION_PARAMETER: i64 = 8760;
