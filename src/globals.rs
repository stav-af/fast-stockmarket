use once_cell::sync::Lazy;
use chrono::Utc;

// TIMEKEEPING CONSTANTS
// keeps track of the start of the market, allowing us to vary time as we like
pub static MARKET_EPOCH: Lazy<i64> = Lazy::new(|| {
    return Utc::now().timestamp_nanos_opt().unwrap(); 
});

// describes the 'display' nanoseconds passed every 'real' nanosecond
// 8760 means that every simulated second describes a 'real' hour.
pub const ACCELERATION_PARAMETER: isize = 86400;

#[derive(Copy, Clone)]
pub enum GRANULARITY {
    INSTANT = 0,
    SECOND = 1e9 as isize / ACCELERATION_PARAMETER as isize,
    MINUTE = 6e10 as isize / ACCELERATION_PARAMETER as isize,
    HOUR = 3.6e12 as isize / ACCELERATION_PARAMETER as isize,
    DAY = 8.64e13 as isize / ACCELERATION_PARAMETER as isize
}