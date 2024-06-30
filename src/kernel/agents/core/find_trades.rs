use crate::kernel::market;
use crate::classes::shared::order::*;

pub fn find_trades(stock: Stock) {
    market::find_trades(stock);
}