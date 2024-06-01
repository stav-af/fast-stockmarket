use serde::{Deserialize, Serialize};

use crate::{globals::GRANULARITY, market::order::Stock};

#[derive(Deserialize, Serialize)]
pub struct PriceDTO {
    pub price: f64,
    pub timestamp: i64
}

#[derive(Serialize, Deserialize)]
pub struct HistoricPriceDTO {
    pub granularity: GRANULARITY,
    pub earliest_stamp: i64,
    pub data: Vec<PriceDTO>
}