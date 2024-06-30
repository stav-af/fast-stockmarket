


mod tests {
    //gpt says i don't need this, rust analyzer disagrees :(
    use crate::kernel::market::*;
    use crate::kernel::order_book::record::*;
    use crate::classes::shared::{order::*, transaction::*};
    use crate::globals::*;

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


    #[test]
    fn test__live_data_intializes_empty() {
        let hist = HistoryBuffer::new();

        let hist_len = hist._live_data.len(); 
        assert!(hist_len == 4, "Expected 4 _live_data, but found {} histories", hist_len);
    
        for (i, h) in hist._live_data.iter().enumerate() {
            assert!(h.len() == 0, "Expected new history to be empty, found len {}, at _live_data {}", h.len(), i)
        }
    }

    #[test]
    fn test_process_transaction_updates_seconds() {
        let mut h = HistoryBuffer::new();

        let vals = 0..10;
        h.process_transactions(
            &vals.map(|i| Transaction {
                transaction_id: None,
                price: i as f64,
                volume: 10,
                timestamp: 1,
            }).collect::<Vec<Transaction>>()
        );
        h.compress();

        assert!(h._live_data[0].len() != 0, "Expected process_transactions to append to h[0] (Seconds history)");
        assert!(h._live_data[1..4].iter().map(|x| x.len()).sum::<usize>() == 0, "Expected all histories apart from seconds to be empty");

        let ob_stat = h._live_data[0][0];
        assert!(ob_stat.volume == 100);
        assert!(ob_stat.tick == 0);
        assert!(ob_stat.high == 9.0);
        assert!(ob_stat.low == 0.0);
    }

    #[test]
    fn test_compress_correctly_compresses() {
        let mut h = HistoryBuffer::new();
        
        // one hour one day one minute and one second, in seconds
        let magic = 86400 + 3661;
        for i in 0..magic {
            h._live_data[0].push(ObStat {
                tick: i as u64,
                granularity: GRANULARITY::SECOND,
                volume: 100,
                high: 10.0,
                low: 1.0,
                open: 0.0,
                close: 0.0
            });
        };

        h.compress();

        let s = &h._live_data[0];
        let m = &h._live_data[1];
        let hr = &h._live_data[2];
        let d = &h._live_data[3];

        assert!(s.len() == 1, "Expected 1 element in Seconds, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(m.len() == 1, "Expected 1 element in Minutes, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(hr.len() == 1, "Expected 1 element in Hours, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(d.len() == 1, "Expected 1 element in Days, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
    
        assert!(d[0].volume == 100 * 86400, "Expected 100 volume per second for a day 8,640,000 total, found {}", d[0].volume);
        assert!(d[0].high == 10.0);
        assert!(d[0].low == 1.0);
    }

    #[test]
    fn test_compress_handles_gaps() {
        let mut h = HistoryBuffer::new();
        
        // one hour one day one minute and one second, in seconds
        let magic = 31;
        for i in 0..magic {
            h._live_data[0].push(ObStat {
                tick: i as u64 * 2,
                granularity: GRANULARITY::SECOND,
                volume: 100,
                high: 10.0,
                low: 1.0,
                open: 0.0,
                close: 0.0
            });
        };

        h.compress();

        let s = &h._live_data[0];
        let m = &h._live_data[1];

        assert!(s.len() == 1, "Expected 1 element in Seconds, got s, m, {}, {}", s.len(), m.len());
        assert!(m.len() == 1);

        assert!(m[0].volume == 3000);
    }

    #[test]
    fn test_reporting_open_close_prices() {
        let mut h = HistoryBuffer::new();

        let vals = 0..10;
        h.process_transactions(
            &vals.map(|i| Transaction {
                transaction_id: None,
                price: i as f64,
                volume: 10,
                timestamp: 1,
            }).collect::<Vec<Transaction>>()
        );
        h.compress();

        let ob_stat = h._live_data[0][0];
        assert!(ob_stat.volume == 100);
        assert!(ob_stat.tick == 0);
        assert!(ob_stat.high == 9.0);
        assert!(ob_stat.low == 0.0);
        assert!(ob_stat.open == 0.0);
        assert!(ob_stat.close == 9.0); 
    }

    #[test]
    fn test_historical_data_populated_correctly(){
        let mut h = HistoryBuffer::new();
        
        // one hour one day one minute and one second, in seconds
        let magic = 90;
        for i in 0..magic {
            h._live_data[0].push(ObStat {
                tick: i,
                granularity: GRANULARITY::SECOND,
                volume: 100,
                high: 10.0,
                low: 1.0,
                open: 0.0,
                close: 0.0
            });
        };

        h.compress();

        let l_s = &h._live_data[0];
        let l_m = &h._live_data[1];

        let h_s = &&h._historic_data[0];
        let h_m = &h._historic_data[1];

        assert!(l_s.len() == 30, "Expected 30 live seconds, found: {}", l_s.len());
        assert!(l_m.len() == 1);
        assert!(h_s.len() == 60);
        assert!(h_m.len() == 0)
    }
}