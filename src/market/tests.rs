


mod tests {
    //gpt says i don't need this, rust analyzer disagrees :(
    use crate::market::{
        market::{buy, find_trades, get_market, ipo, sell}, 
        order::{Order, OrderType::*, OrderVariant, Stock}
    };

    #[test]
    fn test_sell_order_precedence_e2e() {
        // these vars should persist e2e
        let _limit = OrderVariant::Limit { price:0.0 }; //avoid reinitialization for readabilitys 
        let market_order_size = 10;
        
        let limit_order_size = 12;
        let limit_order_price = 9.5;
        
        let ipo_size = 100;
        let ipo_price = 10.0;

        let stock: Stock = Stock::AAPL;

        // ipo, then offer sells at a better price, see if they're at the front of the ask queue
        ipo(stock, ipo_size, ipo_price);
        sell(stock, market_order_size,None, None);
        sell(stock, limit_order_size, Some(limit_order_price), None);
        
        find_trades(stock);
        _assert_top_ask(&stock, &OrderVariant::Market, market_order_size, 0.0);
        
        buy(stock, market_order_size, None, None);
        find_trades(stock);
        _assert_top_ask(&stock, &_limit, limit_order_size, limit_order_price);
    
        buy(stock, limit_order_size, None, None);
        find_trades(stock);
        _assert_top_ask(&stock, &_limit, ipo_size, ipo_price);
    }


    #[test]
    fn test_clean_book_works() {
        let stock = Stock::AAPL;
        let lifetime = 100;

        // put some unmatched orders on, sleep, clean, assert they're empty
        ipo(stock, 0, 0.0);
        buy(stock, 2, Some(100.9), Some(lifetime));
        std::thread::sleep(std::time::Duration::from_nanos((lifetime * 20) as u64));
        {
            let market = get_market().read().unwrap();
            let mut book = &mut market.get(&stock).unwrap().write().unwrap().order_book;

            book.clean_book();
        }
        _assert_no_bids(&stock);
    }

    #[cfg(test)]
    fn _assert_top_ask(stock: &Stock, variant: &OrderVariant, amount: u64, price: f64){
        // unwrap  all the way into market
        let market = get_market().read().unwrap();
        let book = &market.get(&stock).unwrap().read().unwrap().order_book;
        
        let ask_queue = book.get_asks_for_testing();
        let ask = ask_queue.peek();

        match variant {
            OrderVariant::Market => _assert_market_sell(ask, amount),
            OrderVariant::Limit { price: _ } => _assert_limit_sell(ask, amount, price) 
        }
    }

    #[cfg(test)]
    fn _assert_no_bids(stock: &Stock) {
        let market = get_market().read().unwrap();
        let book = &market.get(&stock).unwrap().read().unwrap().order_book;

        let bid_queue = book.get_bids_for_testing();
        println!("empty {}", bid_queue.is_empty());
        println!("len {}", bid_queue.len());
        assert!(bid_queue.is_empty(), "Expected an empty bid queue, but the queue has length {}", bid_queue.len());
    }

    #[cfg(test)]
    fn _assert_market_sell(ask: Option<&Order>, amount: u64){
        match ask {
            Some(order) => {
                assert!(order.variant == OrderVariant::Market, "Expected a Market order, but found {:?}", order);
                assert!(order.details.amount == amount);
            }
            None => panic!("Expected a market sell order, found None")
        }
    }
    
    #[cfg(test)]
    fn _assert_limit_sell(ask: Option<&Order>, amount: u64, price: f64){
        match ask {
            Some(order) => {
                assert!(order.variant == OrderVariant::Limit { price }, "Expected a Limit order at {price}, found {:?}", order);
                assert!(order.details.amount == amount, "Expected Limit order with {amount} shares left, found {:?}", order);
            }
            None => panic!("Expected a market sell order, found None")
        }
    }
}