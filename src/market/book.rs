use std::cmp::{self, Ordering};
use std::{collections::BinaryHeap, fmt::Binary};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Mutex, RwLock};
use std::thread;

use tailcall::tailcall;

use super::order::{*, self};

pub struct OrderBook {
    pub price: f64,
    stock: Stock,
    _bid: RwLock<BinaryHeap<BuyOrder>>,
    _ask: RwLock<BinaryHeap<SellOrder>>,
}

impl OrderBook {
    pub fn new(stock: Stock, price: f64, amount: u64) -> Self {
        let mut order_book = OrderBook {
            price: price,
            stock: stock, 
            _bid: RwLock::new(BinaryHeap::<BuyOrder>::new()), 
            _ask: RwLock::new(BinaryHeap::<SellOrder>::new())
        };

        order_book.sell_limit(stock, amount, price);
        order_book
    }
    

    fn find_trade(&mut self) {
        loop {
            
            // println!("Finding trade");
            let mut bid = self._bid.write().unwrap();
            let mut ask = self._ask.write().unwrap();

            // println!("BOOK: Bid queue has {}", bid.len());
            // println!("BOOK: Ask queue has {}", ask.len());

            //println!("BOOK: Total queue is {}", bid.len() + ask.len());

            if bid.is_empty() {
                // println!("No buy orders!");
                return;
            }
            if ask.is_empty() {
                // println!("No sell orders!");
                return;
            }

            let mut buy = bid.pop().unwrap();
            let mut sell = ask.pop().unwrap();

            use OrderVariant::*;
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

            match (cmp::Ord::cmp(&buy.details.amount, &sell.details.amount)) {
                Ordering::Greater => {
                    buy.details.amount -= sell.details.amount;
                    bid.push(buy)
                }
                Ordering::Less => {
                    sell.details.amount -= buy.details.amount;
                    ask.push(sell)
                }
                _ => { }
            } 

            // println!("BOOK: Bid queue has {}", bid.len());
            // println!("BOOK: Ask queue has {}", ask.len());
            
        }
    }

    pub fn buy_market(&mut self, stock: Stock, amount: u64) {
        self._bid.write().unwrap().push( BuyOrder {
            variant: OrderVariant::Market,
            details: OrderDetails { 
                time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(), 
                    stock: stock,
                    amount: amount
            }
        });

        let _ = self.find_trade(); 
    }


    pub fn buy_limit(&mut self, stock: Stock, amount: u64, price: f64) {
        self._bid.write().unwrap().push( BuyOrder {
            variant: OrderVariant::Limit { price: price },
            details: OrderDetails { 
                time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(), 
                    stock: stock,
                    amount: amount
            }
        });

        let _ = self.find_trade(); 
    }


    pub fn sell_market(&mut self, stock: Stock, amount: u64) {
        self._ask.write().unwrap().push( SellOrder {
                variant: OrderVariant::Market,
                details: OrderDetails { 
                    time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(), 
                    stock: stock,
                    amount: amount
            }
        });

        let _ = self.find_trade(); 
    }


    pub fn sell_limit(&mut self,stock: Stock, amount: u64, price: f64) {
        self._ask.write().unwrap().push( SellOrder {
            variant: OrderVariant::Limit { price: price },
            details: OrderDetails { 
                time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(), 
                stock: stock,
                amount: amount
            }
        });


        let _ = self.find_trade(); 
    }

    #[cfg(test)]
    pub fn get_bids_for_testing(&self) -> std::sync::RwLockReadGuard<BinaryHeap<BuyOrder>> {
        self._bid.read().unwrap()
    }

    #[cfg(test)]
    pub fn get_asks_for_testing(&self) -> std::sync::RwLockReadGuard<BinaryHeap<SellOrder>> {
        self._ask.read().unwrap()
    }  
}