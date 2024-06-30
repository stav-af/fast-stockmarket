use crate::kernel::market;
use crate::classes::shared::order::*;

pub fn clean_books(stock: Stock) {
    let book = market::clean_books(stock);
}