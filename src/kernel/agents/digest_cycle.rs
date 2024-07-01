use std::time::{Duration, Instant};
use std::thread::{spawn, sleep};

use crate::classes::shared::order::*;

use super::core::update_stats::update_stats;
use super::trend::{chaotic_trend_generator::*, market_maker::*};
// TODO: Refactor this into somewhere else
use super::core::{clean_books::*, compress_histories::*, find_trades::*};

// ticks per second, should describe the max tickrate
const TICKRATE: f64 = 10000.0;

pub fn make_market(stock: Stock) {
    dispatch(generate_trend, stock, TICKRATE);
    dispatch(straddle, stock, TICKRATE);
    dispatch(find_trades, stock, TICKRATE);
    dispatch(clean_books, stock, TICKRATE/100.0);
    dispatch(process_transactions, stock, TICKRATE/10.0);
    dispatch(update_stats, stock, TICKRATE/10.0);
}

fn dispatch(f: fn(Stock) -> (), stock: Stock, tickrate: f64){
    // dispatches a function f acting on a stock stock, tickrate times per second.
    // designed to be ran in it's own thread
    spawn(move || {
        let tick_interval = Duration::new(0, (1_000_000_000.0 / tickrate) as u32);
        let mut last_tick = Instant::now();
        loop {
            f(stock);

            // RATELIMIT
            let now = Instant::now();
            if now < last_tick + tick_interval {
                sleep(last_tick + tick_interval - now);
            }
            last_tick += tick_interval;
        }
    });
}