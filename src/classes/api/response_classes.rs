use serde::{Deserialize, Serialize};

use crate::{globals::GRANULARITY, kernel::order_book::record::ObStat};

#[derive(Deserialize, Serialize)]
pub struct PriceDTO {
    pub price: f64,
    pub timestamp: i64
}

#[derive(Deserialize, Serialize)]
pub struct StockHistoryDTO {
    pub tick: u64,
    pub granularity: GRANULARITY,
    pub volume: u64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64
}

impl From<ObStat> for StockHistoryDTO {
    fn from(internal: ObStat) -> Self {
        StockHistoryDTO {
            tick: internal.tick,
            granularity: internal.granularity,
            volume: internal.volume, 
            high: internal.high,
            low: internal.low, 
            open: internal.open,
            close: internal.close
        }
    }
}