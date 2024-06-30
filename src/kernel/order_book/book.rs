use std::cmp;
use std::collections::BinaryHeap;
use std::sync::RwLock;

use super::record::*;

use crate::kernel::market_time::market_time::MTime;
use crate::classes::shared::{order::*, transaction::*};

pub struct OrderBook {
    pub transaction_record: Vec<Transaction>,
    pub stats: ObStat,
    pub price: f64,
    stock: Stock,
    _bid: RwLock<BinaryHeap<Order>>,
    _ask: RwLock<BinaryHeap<Order>>,
}

impl OrderBook {
    pub fn new(stock: Stock) -> Self {
        let order_book = OrderBook {
            transaction_record: Vec::<Transaction>::new(),
            stats: ObStat::default(),
            price: 0.0,
            stock: stock, 
            _bid: RwLock::new(BinaryHeap::<Order>::new()), 
            _ask: RwLock::new(BinaryHeap::<Order>::new()),
        };

        order_book
    }
    
    pub fn process_order(&mut self, order: Order){
        // println!("Processing order");
        match order.order_type {
            OrderType::Buy => { self._bid.write().unwrap().push(order) },
            OrderType::Sell => { self._ask.write().unwrap().push(order) }
        }
        // println!("order has been placed");
    }

    pub fn find_trade(&mut self) {
        use OrderVariant::*;
        loop {
            // println!("Finding trade");
            let mut bid = self._bid.write().unwrap();
            let mut ask = self._ask.write().unwrap();

            if bid.is_empty() {
                return;
            }
            if ask.is_empty() {
                return;
            }

            let mut buy = bid.pop().unwrap();
            let mut sell = ask.pop().unwrap();
            match (&buy.variant, &sell.variant) {
                (Limit { price: bid_price}, Limit { price: ask_price }) 
                    => {
                        if bid_price < ask_price {
                            bid.push(buy);
                            ask.push(sell);
                            return;
                        }
                        self.price = *ask_price;
                        // println!("Sold at {}", self.price);

                    }
                (Market, Limit { price }) |
                (Limit { price }, Market ) => {
                    self.price = *price;
                }
                _ => { 
                    // let market_price = self.price;
                }
            }
            let trade_size = cmp::min(buy.details.amount, sell.details.amount);

            if buy.details.amount > trade_size {
                buy.details.amount -= trade_size;
                bid.push(buy)
            } else if sell.details.amount > trade_size {
                sell.details.amount -= trade_size;
                ask.push(sell);
            }

            self.transaction_record.push(Transaction {
                transaction_id: None,
                price: self.price,
                volume: trade_size,
                timestamp: MTime::now(),
            });
        }
    }

    pub fn clean_book(&mut self){
        let now = chrono::Utc::now().timestamp_nanos_opt().unwrap();
        let retain_condition = |o: &Order| match o.details.lifetime_nanos {
            Some(lifetime) => {
                lifetime + o.details.time > now
            },
            None => true
        };

        self._bid.write().unwrap().retain(retain_condition);
        self._ask.write().unwrap().retain(retain_condition);
    }


    #[cfg(test)]
    pub fn get_bids_for_testing(&self) -> std::sync::RwLockReadGuard<BinaryHeap<Order>> {
        self._bid.read().unwrap()
    }

    #[cfg(test)]
    pub fn get_asks_for_testing(&self) -> std::sync::RwLockReadGuard<BinaryHeap<Order>> {
        self._ask.read().unwrap()
    }  
}