use std::sync::RwLock;

use lazy_static::lazy_static;
use hashbrown::HashMap as HashbrownMap; // Optional, replace HashMap with HashbrownMap if using hashbrown
use chrono::Utc;

use super::order::*;
use super::book::*;

use crate::globals::GRANULARITY;
use crate::order_history::ob_stats::ObStat;
use crate::timekeeper::market_time::MTime;

pub struct Market {
    stock_book: RwLock<HashbrownMap<Stock, RwLock<OrderBook>>>
}


lazy_static! {
    pub static ref MARKET: Market = Market { 
        stock_book: RwLock::new(HashbrownMap::new())
    };
}
impl Market {
    
}
pub fn ipo(stock: Stock, amount: u64, price: f64) {
    {
        let mut market = MARKET.stock_book.write().unwrap();
        market.insert(
            stock,
            RwLock::new(OrderBook::new(stock))
        );
    }
   
    place_order(stock, amount, OrderType::Sell, Some(price), None)
}

pub fn buy(stock: Stock, amount: u64, price: Option<f64>, lifetime: Option<i64>){
    place_order(stock, amount, OrderType::Buy, price, lifetime)    
}

pub fn sell(stock: Stock, amount: u64, price: Option<f64>, lifetime: Option<i64>){
    place_order(stock, amount, OrderType::Sell, price, lifetime)    
}

pub fn clean_books(_: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    for (_, book) in market_lock.iter() {
        book.write().unwrap().clean_book();
    }
}

pub fn compress_histories(_: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    for (_, book) in market_lock.iter() {
        book.write().unwrap().history.compress();
    }
}

pub fn find_trades(_: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    for (_, book) in market_lock.iter() {
        book.write().unwrap().find_trade();
    }
}

pub fn get_historical_data<'a>(granularity: GRANULARITY, earliest_stamp: i64, stock: Stock) -> Option<Vec<ObStat>> {
    let market_lock =  MARKET.stock_book.read().unwrap();
    let book = market_lock.get(&stock).unwrap().read().unwrap();
    
    let history = &book.history;
    return history.get_historical_data(granularity, earliest_stamp);
}

fn place_order(stock: Stock, amount: u64, order_type: OrderType, price: Option<f64>, lifetime: Option<i64>){
    if amount <= 0 {
        return;
    }

    // println!("placing order");

    use OrderVariant::*;
    let order = Order {
        order_type: order_type,
        variant: match price {
            Some(p) => Limit { price: (p) },
            None => Market
        },
        details: OrderDetails {
            time: MTime::now(),
            stock: stock,
            amount: amount,
            lifetime_nanos: lifetime
        }
    };
    // println!("awaiting market lock");
    let lock =  MARKET.stock_book.read().unwrap();
    // println!("awaiting book lock");
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.process_order(order);
}

pub fn get_price(stock: Stock) -> f64 {
    let lock =  MARKET.stock_book.read().unwrap();
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.price
}

// ONLY FOR USAGE IN UNIT TESTS
pub fn get_market() -> &'static RwLock<hashbrown::HashMap<Stock, RwLock<OrderBook>>> {
    return &MARKET.stock_book;
}