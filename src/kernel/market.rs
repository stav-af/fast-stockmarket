use std::sync::RwLock;

use lazy_static::lazy_static;
use hashbrown::HashMap as HashbrownMap; // Optional, replace HashMap with HashbrownMap if using hashbrown

use super::order_book::{book::*, record::*, stats::*};
use super::market_time::market_time::*;

use crate::classes::shared::order::*;
use crate::classes::shared::transaction::Transaction;
use crate::globals::*;

pub struct Market {
    stock_book: RwLock<HashbrownMap<Stock, RwLock<StockRecord>>>
}

pub struct StockRecord {
    pub order_book: OrderBook,
    pub history: HistoryBuffer,
    pub stats: Stats
}

impl StockRecord {
    fn new(stock: Stock) -> Self {
        StockRecord {
            order_book: OrderBook::new(stock),
            history: HistoryBuffer::new(),
            stats: Stats {}
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

pub fn clean_books(stock: Stock) {
    let lock =  MARKET.stock_book.read().unwrap();
    let book = &mut lock.get(&stock).unwrap().write().unwrap().order_book;
    book.clean_book();
}

pub fn find_trades(stock: Stock) {
    let lock =  MARKET.stock_book.read().unwrap();
    let book = &mut lock.get(&stock).unwrap().write().unwrap().order_book;
    book.find_trade();
}

pub fn compress_histories(stock: Stock) -> Vec<Transaction>{
    let lock =  MARKET.stock_book.read().unwrap();
    let record = &mut lock.get(&stock).unwrap().write().unwrap();

    let transactions = &mut record.order_book.transaction_record;
    let last_second_timestamp = MTime::which_second(transactions.last().unwrap().timestamp) * GRANULARITY::SECOND as u64;

    let index = transactions.iter()
        .position(|x| x.timestamp > last_second_timestamp as i64)
        .unwrap_or(transactions.len());

    let whole_seconds = transactions.drain(0..index).collect::<Vec<Transaction>>();

    let stock_record = &mut record.history;
    stock_record.process_transactions(&whole_seconds);
    stock_record.compress();

    whole_seconds
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
    let lock =  MARKET.stock_book.read().unwrap();
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