use crate::kernel::market;
use crate::classes::shared::order::*;

pub fn update_stats(stock: Stock) {
    market::update_stats(stock);
}