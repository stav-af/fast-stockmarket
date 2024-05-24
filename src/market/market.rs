use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::RwLock;
use hashbrown::HashMap as HashbrownMap; // Optional, replace HashMap with HashbrownMap if using hashbrown
use super::order::*;
use super::book::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Market {
    stock_book: RwLock<HashbrownMap<Stock, RwLock<OrderBook>>>
}


lazy_static! {
    pub static ref MARKET: Market = Market { 
        stock_book: RwLock::new(HashbrownMap::new())
    };
}

pub fn ipo(stock: Stock, amount: u64, price: f64) {
    let mut market = MARKET.stock_book.write().unwrap();
    market.insert(
        stock,
        RwLock::new(OrderBook::new(stock, price, amount))
    );
}

pub fn buy_market(stock: Stock, amount: u64) {
    let lock =  MARKET.stock_book.read().unwrap();
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.buy_market(stock, amount);
}

pub fn buy_limit(stock: Stock, amount: u64, price: f64) {
    let lock =  MARKET.stock_book.read().unwrap();
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.buy_limit(stock, amount, price);
}

pub fn sell_market(stock: Stock, amount: u64) {
    let lock =  MARKET.stock_book.read().unwrap();
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.sell_market(stock, amount);
}

pub fn sell_limit(stock: Stock, amount: u64, price: f64) {
    let lock =  MARKET.stock_book.read().unwrap();
    let mut book = lock.get(&stock).unwrap().write().unwrap();
    book.sell_limit(stock, amount, price);
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