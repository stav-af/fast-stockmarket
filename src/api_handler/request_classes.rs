use serde::{Deserialize, Serialize};
use phf::phf_map;

use crate::globals::GRANULARITY;
use crate::market::order::Stock;

#[derive(Deserialize, Serialize)]
pub struct OrderDTO {
    pub stock: Stock,
    pub amount: u64,
    pub price: Option<f64>
}

#[derive(Deserialize, Serialize)]
pub struct IpoDTO {
    pub stock: Stock,
    pub amount: u64,
    pub price: f64 
}

#[derive(Deserialize, Serialize)]
pub struct StockQuery {
    pub stock: Stock
}

#[derive(Deserialize, Serialize)]
pub struct HistoricPriceQuery {
    pub stock: Stock,
    pub granularity: GRANULARITY,
    pub earliest_stamp: i64,
}
