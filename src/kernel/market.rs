use std::sync::RwLock;

use lazy_static::lazy_static;
use hashbrown::HashMap as HashbrownMap; // Optional, replace HashMap with HashbrownMap if using hashbrown
use circular_buffer::CircularBuffer;

use super::order_book::{book::*, record::*, stats::*};
use super::market_time::market_time::*;

use crate::classes::api::response_classes::StockHistoryDTO;
use crate::classes::shared::order::{self, *};
use crate::classes::shared::transaction::Transaction;
use crate::globals::*;

pub struct Market {
    stock_book: RwLock<HashbrownMap<Stock, RwLock<StockRecord>>>
}

pub struct StockRecord {
    pub order_book: OrderBook,
    pub history: HistoryBuffer,
    pub stats: Stats,
    pub recent_transactions: CircularBuffer<100, Transaction>
}

impl StockRecord {
    fn new(stock: Stock) -> Self {
        StockRecord {
            order_book: OrderBook::new(stock),
            history: HistoryBuffer::new(),
            stats: Stats::new(),
            recent_transactions: CircularBuffer::<100, Transaction>::new()
        }
    }

    fn update_stats(&mut self) {
        self.stats.update_stats(&self.history._historic_data)
    }

    fn report_transactions(&mut self){
        // push all transactions with an associated Id to recent_transactions
        // to be polled by buy/sell user requests
        self.recent_transactions.extend(
            self.order_book.transaction_record.iter().filter(
                |t| t.buy_id != None || t.sell_id != None)
            )
    }
}

lazy_static! {
    pub static ref MARKET: Market = Market { 
        stock_book: RwLock::new(HashbrownMap::new())
    };
}

pub fn ipo(stock: Stock, amount: u64, price: f64, id: Option<u64>) {
    {
        let mut market = MARKET.stock_book.write().unwrap();
        market.insert(
            stock,
            RwLock::new(StockRecord::new(stock))
        );
    }
   
    place_order(stock, amount, OrderType::Sell, Some(price), None, id)
}

pub fn buy(stock: Stock, amount: u64, price: Option<f64>, lifetime: Option<i64>, id: Option<u64>){
    place_order(stock, amount, OrderType::Buy, price, lifetime, id)    
}


pub fn sell(stock: Stock, amount: u64, price: Option<f64>, lifetime: Option<i64>, id: Option<u64>){
    place_order(stock, amount, OrderType::Sell, price, lifetime, id)
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

pub fn report_transactions(stock: Stock) -> Vec<Transaction>{
    let lock =  MARKET.stock_book.read().unwrap();
    let record = &mut lock.get(&stock).unwrap().write().unwrap();
    record.report_transactions();

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

pub fn get_order_status(stock: Stock, id: u64, order_type: OrderType) -> OrderStatus { 
    let executed = search_transaction_buffer(stock, id, order_type);
    let pending = search_orderbook(stock, id, order_type);  
   
    match (executed, pending) {
        (Some(price), false) => OrderStatus::Executed { price: price },
        (Some(_), true) => OrderStatus::PartiallyFilled,
        (None, true) => OrderStatus::Pending,
        _ => panic!()
    }
}

fn search_transaction_buffer(stock: Stock, id: u64, order_type: OrderType) -> Option<f64> {
    let lock =  MARKET.stock_book.read().unwrap();
    let record = &lock.get(&stock).unwrap().read().unwrap();
    
    let search_criteria: &dyn Fn(Transaction) -> bool;

    let sell_cond = |t: Transaction| t.sell_id == Some(id);
    let buy_cond = |t: Transaction| t.buy_id == Some(id);
    match order_type {
        OrderType::Sell => search_criteria = &sell_cond,
        OrderType::Buy => search_criteria = &buy_cond
    }

    let (sum, count) = record.recent_transactions.iter()
        .filter(|&t| search_criteria(*t))
        .fold((0.0, 0), |(sum, count), t| (sum + t.price, count + 1));

    if count > 0 {
        let avg_price = sum / count as f64;
        Some(avg_price)
    } else {
        None
    }
}

fn search_orderbook(stock: Stock, id: u64, order_type: OrderType) -> bool {
    let lock =  MARKET.stock_book.read().unwrap();
    let record = &lock.get(&stock).unwrap().read().unwrap();

    match order_type {
        OrderType::Buy => record.order_book.is_pending_bid(id),
        OrderType::Sell => record.order_book.is_pending_ask(id) 
    }
}

pub fn update_stats(stock: Stock) {
    let market_lock =  MARKET.stock_book.read().unwrap();
    let record = &mut market_lock.get(&stock).unwrap().write().unwrap();
    record.update_stats()
}


fn place_order(stock: Stock, amount: u64, order_type: OrderType, price: Option<f64>, lifetime: Option<i64>, id: Option<u64>){
    if amount <= 0 {
        return;
    }
    // println!("placing order");
    use OrderVariant::*;
    let order = Order {
        id: id,
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

pub fn get_stock_history(stock: Stock, granularity: GRANULARITY, count: usize) -> Vec<StockHistoryDTO> {
    let lock =  MARKET.stock_book.read().unwrap();
    let history = &lock.get(&stock).unwrap().read().unwrap().history._historic_data;

    let requested_hist = &history[granularity_index(granularity)];
    let n = requested_hist.len();
    
    let subj: &[ObStat];
    if n <= count {
        subj = &requested_hist[..];
    } else {
        subj = &requested_hist[n-14..];
    }

    subj.iter().map(|x| Into::<StockHistoryDTO>::into(*x)).collect()
}

// ONLY FOR USAGE IN UNIT TESTS
pub fn get_market() -> &'static RwLock<hashbrown::HashMap<Stock, RwLock<StockRecord>>> {
    return &MARKET.stock_book;
}