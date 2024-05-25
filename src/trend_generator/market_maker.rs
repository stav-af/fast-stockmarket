use crate::{market::{order::Stock, market::{buy_limit, sell_limit, get_price}}};
extern crate statrs;
use statrs::{distribution::{Normal}, statistics::Distribution};
use std::f64::consts::PI;

// VALUES CONTROL THE TRAILING BUY/SELLS
const NUM_TRAIL_LEVELS: u64 = 500;
const TRAIL_LEVEL_GAPS: f64 = 0.001;
const VOLUME_MULTIPLIER: f64 = 1000.0;
const TRAIL_GRADIENT: f64 = 0.0001; // how far from mean is each buy

const STD: f64 = 0.0001;

fn probability_density(distance_from_mean: f64, n: Normal) -> f64 {
    let variance = n.variance().unwrap(); // Standard deviation squared, assuming standard deviation is 1 for standard normal distribution
    1.0 / (((2.0 * PI * variance).sqrt())) * (-0.5 * distance_from_mean.powi(2)).exp()
}

pub fn straddle(stock: Stock) {
    // provide buy and sell limit orders to the market, at a normal distribution
    // centered at the current stock price
    let normal = Normal::new(0.0, STD as f64).unwrap();
    let price = get_price(stock);

    for i in 1..NUM_TRAIL_LEVELS + 1 {
        let distance = i as f64 * TRAIL_GRADIENT;
        let volume = probability_density(distance, normal);
        // println!("MM: volume {volume}");

        let trade_volume = (volume * VOLUME_MULTIPLIER) as u64;
        let distance_from_price = i as f64 * TRAIL_LEVEL_GAPS;

        buy_limit(stock, trade_volume, price - distance_from_price);
        // buy_limit(stock, trade_volume, price + distance_from_price);
        // sell_limit(stock, trade_volume, price - distance_from_price);
        sell_limit(stock, trade_volume, price + distance_from_price);
        // println!("MARK: Sold {trade_volume} shares at {}", price + distance_from_price);
        // println!("MARK: Bought {trade_volume} shares at {}", price - distance_from_price);
    }
}