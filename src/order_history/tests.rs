mod tests {
    use crate::{globals::{GRANULARITY, ACCELERATION_PARAMETER}, order_history::{history_buffer::HistoryBuffer, ob_stats::{ObStat, Transaction}}};

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
            vals.map(|i| Transaction {
                transaction_id: None,
                price: i as f64,
                volume: 10,
                timestamp: 1,
            }).collect()
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
            vals.map(|i| Transaction {
                transaction_id: None,
                price: i as f64,
                volume: 10,
                timestamp: 1,
            }).collect()
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
}