use crate::market::{
    market::{ get_price, buy, sell },
    order::Stock
};
extern crate statrs;
use statrs::{distribution::Normal, statistics::Distribution};
use std::f64::consts::PI;

// VALUES CONTROL THE TRAILING BUY/SELLS
const NUM_TRAIL_LEVELS: u64 = 50;
const TRAIL_LEVEL_GAPS: f64 = 0.01;
const VOLUME_MULTIPLIER: f64 = 10.0;
const TRAIL_GRADIENT: f64 = 0.001; // how far from mean is each buy

const STD: f64 = 1.0;

fn probability_density(distance_from_mean: f64, n: Normal) -> f64 {
    let variance = n.variance().unwrap(); // Standard deviation squared, assuming standard deviation is 1 for standard normal distribution
    // (((2.0 * PI * variance).sqrt())) * ((-0.5 * distance_from_mean.powi(2)) / variance).exp();
    
    let coefficient = 1.0 / ((2.0 * PI * variance).sqrt());
    let exponent = (-0.5 * (distance_from_mean.powi(2) / variance)).exp();
    1.0/(coefficient * exponent)
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

        sell(stock, trade_volume, Some(price + distance_from_price), Some(100));
        // buy_limit(stock, trade_volume, price + distance_from_price);
        // sell_limit(stock, trade_volume, price - distance_from_price);
        buy(stock, trade_volume, Some(price - distance_from_price), Some(100));
        // println!("MARK: Sold {trade_volume} shares at {}", price + distance_from_price);
        // println!("MARK: Bought {trade_volume} shares at {}", price - distance_from_price);
    }
}