use std::sync::RwLock;

use lazy_static::lazy_static;
use hashbrown::HashMap as HashbrownMap; // Optional, replace HashMap with HashbrownMap if using hashbrown
use chrono::Utc;

use super::order::*;
use super::book::*;

use crate::globals::GRANULARITY;
use crate::order_history::history_buffer::HistoryBuffer;
use crate::order_history::ob_stats::Transaction;
use crate::timekeeper::market_time::MTime;

pub struct Market {
    stock_book: RwLock<HashbrownMap<Stock, RwLock<StockRecord>>>
}

pub struct StockRecord {
    pub order_book: OrderBook,
    pub history: HistoryBuffer
}

impl StockRecord {
    fn new(stock: Stock) -> Self {
        StockRecord {
            order_book: OrderBook::new(stock),
            history: HistoryBuffer::new()
        }
    }
}

lazy_static! {
    pub static ref MARKET: Market = Market { 
        stock_book: RwLock::new(HashbrownMap::new())
    };
}

pub fn ipo(stock: Stock, amount: u64, price: f64) {
    {
        let mut market = MARKET.stock_book.write().unwrap();
        market.insert(
            stock,
            RwLock::new(StockRecord::new(stock))
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
        book.write().unwrap().order_book.clean_book();
    }
}

pub fn compress_histories(_: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    for (_, book) in market_lock.iter() {
        let mut stock_record = book.write().unwrap();
        let transactions = &mut stock_record.order_book.transaction_record;

        let last_second_timestamp = MTime::which_second(transactions.last().unwrap().timestamp) * GRANULARITY::SECOND as u64;

        let index = transactions.iter()
            .position(|x| x.timestamp > last_second_timestamp as i64)
            .unwrap_or(transactions.len());

        let whole_seconds = transactions.drain(0..index).collect::<Vec<Transaction>>();
        stock_record.history.process_transactions(whole_seconds);
    }
}

pub fn find_trades(_: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    for (_, book) in market_lock.iter() {
        book.write().unwrap().order_book.find_trade();
    }
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
    let book = &mut lock.get(&stock).unwrap().write().unwrap().order_book;
    book.process_order(order);
}

pub fn get_price(stock: Stock) -> f64 {
    let lock =  MARKET.stock_book.read().unwrap();
    let book = &lock.get(&stock).unwrap().read().unwrap().order_book;
    book.price
}

// ONLY FOR USAGE IN UNIT TESTS
pub fn get_market() -> &'static RwLock<hashbrown::HashMap<Stock, RwLock<StockRecord>>> {
    return &MARKET.stock_book;
}