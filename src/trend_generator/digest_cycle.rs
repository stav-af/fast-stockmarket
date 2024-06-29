use std::time::{Duration, Instant};
use std::thread;

use crate::market::order::Stock;
use crate::market::market::{clean_books, find_trades, compress_histories};

use super::{market_maker::straddle, chaotic_trend_generator::generate_trend};

// ticks per second, should describe the max tickrate
const TICKRATE: f64 = 10000.0;

pub fn make_market(stock: Stock) {
    
    dispatch(generate_trend, stock, TICKRATE);
    dispatch(straddle, stock, TICKRATE);
    dispatch(find_trades, stock, TICKRATE);
    dispatch(clean_books, stock, TICKRATE/100.0);
    dispatch(compress_histories, stock, TICKRATE/10.0)
}

fn dispatch(f: fn(Stock) -> (), stock: Stock, tickrate: f64){
    // dispatches a function f acting on a stock stock, tickrate times per second.
    // designed to be ran in it's own thread
    thread::spawn( move || {
        let tick_interval = Duration::new(0, (1_000_000_000.0 / tickrate) as u32);
        let mut last_tick = Instant::now();
        loop {
            f(stock);

            // RATELIMIT
            while last_tick.elapsed() < tick_interval {}
            last_tick += tick_interval;
        }
    });
}