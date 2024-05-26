use std::cmp;
use std::collections::BinaryHeap;
use std::sync::RwLock;


use super::order::*;



pub struct OrderBook {
    pub price: f64,
    stock: Stock,
    _bid: RwLock<BinaryHeap<Order>>,
    _ask: RwLock<BinaryHeap<Order>>,
}

impl OrderBook {
    pub fn new(stock: Stock) -> Self {
        let order_book = OrderBook {
            price: 0.0,
            stock: stock, 
            _bid: RwLock::new(BinaryHeap::<Order>::new()), 
            _ask: RwLock::new(BinaryHeap::<Order>::new())
        };

        order_book
    }
    
    pub fn process_order(&mut self, order: Order){
        println!("Processing order");
        match order.order_type {
            OrderType::Buy => { self._bid.write().unwrap().push(order) },
            OrderType::Sell => { self._ask.write().unwrap().push(order) }
        }
        println!("order has been placed");
        self.find_trade();
    }

    fn find_trade(&mut self) {
        loop {
            println!("Finding trade");
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
                    // let market_price = self.price;
                    // println!("Market at {market_price}")
                }
            }

            match (cmp::Ord::cmp(&buy.details.amount, &sell.details.amount)) {
                cmp::Ordering::Greater => {
                    buy.details.amount -= sell.details.amount;
                    bid.push(buy)
                }
                cmp::Ordering::Less => {
                    sell.details.amount -= buy.details.amount;
                    ask.push(sell)
                }
                _ => { }
            } 

            println!("BOOK: Bid queue has {}", bid.len());
            println!("BOOK: Ask queue has {}", ask.len());
            
        }
    }

    fn clean_book() {

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