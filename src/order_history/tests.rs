mod tests {
    use crate::{globals::{GRANULARITY, ACCELERATION_PARAMETER}, order_history::{history_buffer::HistoryBuffer, ob_stats::{ObStat, Transaction}}};

    #[test]
    fn test_histories_intializes_empty() {
        let hist = HistoryBuffer::new();

        let hist_len = hist.histories.len(); 
        assert!(hist_len == 4, "Expected 4 histories, but found {} histories", hist_len);
    
        for (i, h) in hist.histories.iter().enumerate() {
            assert!(h.len() == 0, "Expected new history to be empty, found len {}, at histories {}", h.len(), i)
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

        assert!(h.histories[0].len() != 0, "Expected process_transactions to append to h[0] (Seconds history)");
        assert!(h.histories[1..4].iter().map(|x| x.len()).sum::<usize>() == 0, "Expected all histories apart from seconds to be empty");

        let ob_stat = h.histories[0][0];
        assert!(ob_stat.volume == 100);
        assert!(ob_stat.tick == 0);
        assert!(ob_stat.max_price == 9.0);
        assert!(ob_stat.min_price == 0.0);
    }

    #[test]
    fn test_obstat_gets_compressed() {
        let mut h = HistoryBuffer::new();
        
        // one hour one day one minute and one second, in seconds
        let magic = 86400 + 3661;
        for i in 0..magic {
            h.histories[0].push(ObStat {
                tick: i as u64,
                granularity: GRANULARITY::SECOND,
                volume: 100,
                max_price: 10.0,
                min_price: 1.0,
            });
        };

        h.compress();

        let s = &h.histories[0];
        let m = &h.histories[1];
        let hr = &h.histories[2];
        let d = &h.histories[3];

        assert!(s.len() == 1, "Expected 1 element in Seconds, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(m.len() == 1, "Expected 1 element in Minutes, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(hr.len() == 1, "Expected 1 element in Hours, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
        assert!(d.len() == 1, "Expected 1 element in Days, got s, m, h, d, {}, {}, {}, {}", s.len(), m.len(), hr.len(), d.len());
    }
}