use once_cell::sync::Lazy;
use chrono::Utc;

// TIMEKEEPING CONSTANTS
// keeps track of the start of the market, allowing us to vary time as we like
pub static MARKET_EPOCH: Lazy<i64> = Lazy::new(|| {
    return Utc::now().timestamp_nanos_opt().unwrap(); 
});

// describes the 'display' nanoseconds passed every 'real' nanosecond
// 3600 means that every simulated second describes a 'real' hour.
pub const ACCELERATION_PARAMETER: f64 = 3600.0;

#[derive(Copy, Clone)]
pub enum GRANULARITY {
    INSTANT = 0,
    SECOND = (1e9 / ACCELERATION_PARAMETER) as isize,
    MINUTE = (60.0 * 1e9 / ACCELERATION_PARAMETER) as isize,
    HOUR = ((60.0 * 60.0 * 1e9) / ACCELERATION_PARAMETER) as isize,
    DAY = ((24.0 * 60.0 * 60.0 * 1e9) / ACCELERATION_PARAMETER) as isize
}