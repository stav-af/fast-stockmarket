use itertools::Itertools;

use crate::{globals::GRANULARITY, timekeeper::market_time::MTime};

use super::ob_stats::{ObStat, Transaction};


const fn granularity_max_measurements(granularity: GRANULARITY) -> usize {
    (next_granularity(granularity) as isize/granularity as isize) as usize
}

const fn next_granularity(granularity: GRANULARITY) -> GRANULARITY {
    match granularity {
        GRANULARITY::SECOND => GRANULARITY::MINUTE,
        GRANULARITY::MINUTE => GRANULARITY::HOUR,
        GRANULARITY::HOUR => GRANULARITY::DAY,
        _ => GRANULARITY::INSTANT
    }
    
}

pub struct HistoryBuffer {
    histories: Vec<Vec<ObStat>>
}

impl HistoryBuffer {
    pub fn new() -> Self {
        // these will be second, minute, hour and day respectively.
        let histories = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        Self {
            histories,
        }
    }

    pub fn process_transactions(&mut self, measurements: Vec<Transaction>){
        // take a list of transactions, convert to histories, group by.
        for (second_num, record) in &measurements.into_iter().group_by(|t| MTime::which_second(t.timestamp)) {
            let mut max_p = f64::MIN;
            let mut min_p = f64::MAX;
            let mut vol = 0;
            for t in record {
                max_p = if max_p > t.price {max_p} else {t.price};
                min_p = if min_p < t.price {min_p} else {t.price};
                vol += t.volume;
            }

            self.histories[0].push(ObStat {
                granularity: GRANULARITY::SECOND,
                tick: second_num,
                volume: vol,
                max_price: max_p,
                min_price: min_p
            })
        }
    }

    pub fn compress(&mut self) {
        // cycles over the histories, when there are more 'seconds' measurements than seconds in a minute
        // the 'seconds' meausrements are compressed and pushed to the 'minute' array and so on
        let len = self.histories.len();

        // println!("Seconds: {}", self.histories[0].len());
        // println!("Minutes: {}", self.histories[1].len());    
        // println!("Hours  : {}", self.histories[2].len());
        // println!("Days   : {}", self.histories[3].len());
        
        for i in 0..(len - 1) {
            let (current_hist, next_hist) = self.histories.split_at_mut(i + 1);
            let current_hist = &mut current_hist[i];
            let next_hist = &mut next_hist[0];

            if !current_hist.is_empty() && current_hist.len() > granularity_max_measurements(current_hist[0].granularity) {
                next_hist.extend(Self::downgrade_granularity(current_hist, current_hist[0].granularity));
            }
        }
    }

    fn downgrade_granularity(measurements:&mut Vec<ObStat>, granularity: GRANULARITY) -> Vec<ObStat> {
        let slice_size = granularity_max_measurements(granularity) as usize;

        let mut ret: Vec<ObStat> = Vec::new();
        while measurements.len() > slice_size {
            let mut subject = measurements.drain(0..slice_size).peekable();

            let _timestamp = subject.peek().unwrap().tick;
            let mut max: f64 = f64::MIN;
            let mut min: f64 = f64::MAX;
            let mut vol: u64 = 0;

            for m in subject {
                max = if max > m.max_price {max} else {m.max_price};
                min = if min < m.min_price {min} else {m.min_price};
                vol += m.volume;
            }

            ret.push(ObStat {
                granularity: next_granularity(granularity),
                tick: 0,
                volume: vol,
                max_price: max,
                min_price: min
            });
        }
        ret
    }
}