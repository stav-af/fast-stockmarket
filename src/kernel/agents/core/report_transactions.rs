use crate::kernel::market;
use crate::classes::shared::{order::*,  transaction::*};

pub fn report_transactions(stock: Stock) {
    let transactions = market::report_transactions(stock);
    // TODO: Record these transactions somewhere
}