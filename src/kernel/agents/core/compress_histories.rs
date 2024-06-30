use crate::kernel::market::compress_histories;
use crate::kernel::market_time::market_time::MTime;

use crate::classes::shared::{order::*,  transaction::*};
use crate::globals::*;

pub fn process_transactions(stock: Stock) {
    let transactions = compress_histories(stock);
    // TODO: Record these transactions somewhere
}