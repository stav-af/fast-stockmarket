use actix_web::{post, web, App, HttpServer, Result};
use serde::{Deserialize, Serialize};
use phf::phf_map;

use crate::market::order::{Stock};

#[derive(Deserialize, Serialize)]
pub struct OrderDTO {
    pub stock_name: String,
    pub amount: u64,
    pub price: Option<f64>
}

#[derive(Deserialize, Serialize)]
pub struct IpoDTO {
    pub stock_name: String,
    pub amount: u64,
    pub price: f64 
}

#[derive(Deserialize, Serialize)]
pub struct StockQuery {
    pub stock_name: String
}

pub static stockmap: phf::Map<&'static str, Stock> = phf_map! {
    "MSFT" => Stock::MSFT,
    "AAPL" => Stock::AAPL,
    "three" => Stock::GOOGL,
};
