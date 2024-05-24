


mod tests {
    //gpt says i don't need this, rust analyzer disagrees :(
    use crate::market::{
        order::{Stock, OrderVariant, BuyOrder, SellOrder},
        market::{ipo, buy_market, sell_market, sell_limit, get_market},
        book::OrderBook
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
        sell_market(stock, market_order_size);
        sell_limit(stock, limit_order_size, limit_order_price);

       
        _assert_top_ask(&stock, &OrderVariant::Market, market_order_size, 0.0);
        
        buy_market(stock, market_order_size);
        _assert_top_ask(&stock, &_limit, limit_order_size, limit_order_price);
    
        buy_market(stock, limit_order_size);
        _assert_top_ask(&stock, &_limit, ipo_size, ipo_price);
    }


    #[cfg(test)]
    fn _assert_top_ask(stock: &Stock, variant: &OrderVariant, amount: u64, price: f64){
        // unwrap  all the way into market
        let market = get_market().write().unwrap();
        let book = market.get(&stock).unwrap().write().unwrap();
        
        let ask_queue = book.get_asks_for_testing();
        let ask = ask_queue.peek();

        match variant {
            OrderVariant::Market => _assert_market_sell(ask, amount),
            OrderVariant::Limit { price: _ } => _assert_limit_sell(ask, amount, price) 
        }
    }

    #[cfg(test)]
    fn _assert_market_sell(ask: Option<&SellOrder>, amount: u64){
        match ask {
            Some(order) => {
                assert!(order.variant == OrderVariant::Market, "Expected a Market order, but found {:?}", order);
                assert!(order.details.amount == amount);
            }
            None => panic!("Expected a market sell order, found None")
        }
    }
    
    #[cfg(test)]
    fn _assert_limit_sell(ask: Option<&SellOrder>, amount: u64, price: f64){
        match ask {
            Some(order) => {
                assert!(order.variant == OrderVariant::Limit { price }, "Expected a Limit order at {price}, found {:?}", order);
                assert!(order.details.amount == amount, "Expected Limit order with {amount} shares left, found {:?}", order);
            }
            None => panic!("Expected a market sell order, found None")
        }
    }
}