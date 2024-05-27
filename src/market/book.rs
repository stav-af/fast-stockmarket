use std::cmp;
use std::collections::BinaryHeap;
use std::sync::RwLock;

use chrono::Utc;

use super::order::*;
use crate::globals::GRANULARITY;
use crate::order_history::*;

const REPORT_FREQUENCY: GRANULARITY = GRANULARITY::SECOND;

pub struct OrderBook {
    pub history: history_buffer::HistoryBuffer,
    pub stats: ob_stats::ObStat,
    pub price: f64,
    stock: Stock,
    _bid: RwLock<BinaryHeap<Order>>,
    _ask: RwLock<BinaryHeap<Order>>,
}

impl OrderBook {
    pub fn new(stock: Stock) -> Self {
        let order_book = OrderBook {
            history: history_buffer::HistoryBuffer::new(),
            stats: ob_stats::ObStat::default(),
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
        loop {
            // println!("Finding trade");
            let mut bid = self._bid.write().unwrap();
            let mut ask = self._ask.write().unwrap();

            // println!("BOOK: Bid queue has {}", bid.len());
            // println!("BOOK: Ask queue has {}", ask.len());

            //println!("BOOK: Total queue is {}", bid.len() + ask.len());

            if bid.is_empty() {
                // println!("BOOK: No buy orders!");
                return;
            }
            if ask.is_empty() {
                // println!("BOOK: No sell orders!");
                return;
            }

            let mut buy = bid.pop().unwrap();
            let mut sell = ask.pop().unwrap();
            // println!("BOOK: Bid queue has {}", bid.iter().map(|o| o.details.amount).sum::<u64>());
            // println!("BOOK: Ask queue has {}", ask.iter().map(|o| o.details.amount).sum::<u64>());
            // println!("BOOK: Ask queue is {}% market orders", 
            //     100 * ask.iter().filter(|o| o.variant == OrderVariant::Market).map(|o| o.details.amount).sum::<u64>() / 
            //     (ask.iter().map(|o| o.details.amount).sum::<u64>() + 1)
            // );
            // println!("BOOK: bid queue is {}% market orders", 
            //    100 * bid.iter().filter(|o| o.variant == OrderVariant::Market).map(|o| o.details.amount).sum::<u64>() / 
            //    (bid.iter().map(|o| o.details.amount).sum::<u64>() + 1)
           // );
            use OrderVariant::*;
            match (&buy.variant, &sell.variant) {
                (Limit { price: bid_price}, Limit { price: ask_price }) 
                    => {
                        if bid_price < ask_price {
                            // println!("BOOK: Could not find viable trade");
                            bid.push(buy);
                            ask.push(sell);
                            return;
                        }
                        self.price = *ask_price;
                        println!("Sold at {}", self.price);

                    }
                ((Market, Limit { price }) |
                (Limit { price }, Market )) => {
                    self.price = *price;
                    // println!("Sold at {price}")
                }
                _ => { 
                    let market_price = self.price;
                    // println!("Market at {market_price}")
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

            self.stats.max_price = if self.stats.max_price < self.price {self.price} else {self.stats.max_price};
            self.stats.min_price = if self.stats.min_price > self.price {self.price} else {self.stats.min_price};
            self.stats.volume += trade_size;
            if Utc::now().timestamp_nanos_opt().unwrap() > self.stats.timestamp + REPORT_FREQUENCY as i64 {
                self.history.update(self.stats);
                self.stats = ob_stats::ObStat::default();
            }
            // match (cmp::Ord::cmp(&buy.details.amount, &sell.details.amount)) {
            //     cmp::Ordering::Greater => {
            //         buy.details.amount -= sell.details.amount;
            //         bid.push(buy)
            //     }
            //     cmp::Ordering::Less => {
            //         sell.details.amount -= buy.details.amount;
            //         ask.push(sell)
            //     }
            //     _ => { }
            // } 
            //println!("BOOK: SOLD!");
            //println!("BOOK: Bid queue has {}", bid.iter().map(|o| o.details.amount).sum::<u64>());
            //println!("BOOK: Ask queue has {}", ask.iter().map(|o| o.details.amount).sum::<u64>());
//
            //println!("BOOK: Ask queue is {}% market orders", 
            //    100 * ask.iter().filter(|o| o.variant == OrderVariant::Market).map(|o| o.details.amount).sum::<u64>() / 
            //    (ask.iter().map(|o| o.details.amount).sum::<u64>() + 1)
            //);
            //println!("BOOK: bid queue is {}% market orders", 
            //    100 * bid.iter().filter(|o| o.variant == OrderVariant::Market).map(|o| o.details.amount).sum::<u64>() / 
            //    (bid.iter().map(|o| o.details.amount).sum::<u64>() + 1)
            //);

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