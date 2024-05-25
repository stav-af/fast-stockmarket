use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PriceDTO {
    pub price: f64,
    pub timestamp: i64
}