use std::time::{Duration, Instant};

use crate::market::{order::Stock, market::get_price};
use super::{market_maker::{*, self}, chaotic_trend_generator::*};

const TICKRATE: f64 = 1000.0;

pub fn make_market(stock: Stock) -> ! {
    println!("Making market");
    let tick_interval = Duration::new(0, (1_000_000_000.0 / TICKRATE) as u32);

    let mut last_tick = Instant::now();
    
    loop {
        chaotic_trend_generator(stock);
        market_maker::straddle(stock);

        // RATE LIMITER
        // while last_tick.elapsed() < tick_interval {}
        // last_tick += tick_interval;
        // println!("Price: {price}");
    }
}