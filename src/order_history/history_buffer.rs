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
    pub histories: Vec<Vec<ObStat>>
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
        for (second_num, record) in &measurements.iter().group_by(|t| MTime::which_second(t.timestamp)) {
            let record_vec: Vec<&Transaction> = record.collect();

            let mut max_p = f64::MIN;
            let mut min_p = f64::MAX;
            let mut vol = 0;
            for t in &record_vec {
                max_p = if max_p > t.price {max_p} else {t.price};
                min_p = if min_p < t.price {min_p} else {t.price};
                vol += t.volume;
            }

            self.histories[0].push(ObStat {
                granularity: GRANULARITY::SECOND,
                tick: second_num,
                volume: vol,
                high: max_p,
                low: min_p,
                open: record_vec[0].price,
                close: record_vec.last().unwrap().price
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

            if !current_hist.is_empty() {
                next_hist.extend(Self::downgrade_granularity(current_hist, current_hist[0].granularity));
            }
        }
    }

    fn downgrade_granularity(measurements:&mut Vec<ObStat>, granularity: GRANULARITY) -> Vec<ObStat> {
        if measurements.len() == 0 {
            return Vec::<ObStat>::new()
        }

        let slice_size = granularity_max_measurements(granularity) as usize;
        let mut last_tick_in_slice = measurements[0].tick + (slice_size as u64 - 1);
        
        let last_tick = measurements.last().unwrap().tick;

        let mut ret: Vec<ObStat> = Vec::new();
        while (!measurements.is_empty()) && (last_tick >= last_tick_in_slice) {
            let index = measurements.iter()
                .position(|m| m.tick > last_tick_in_slice)
                .unwrap_or(measurements.len());
            

            let subject: Vec<ObStat> = measurements.drain(0..index).collect();
            
            let first = subject[0];
            let tick = first.tick / slice_size as u64;
            
            let mut max: f64 = f64::MIN;
            let mut min: f64 = f64::MAX;
            let mut vol: u64 = 0;

            for m in &subject {
                max = if max > m.high {max} else {m.high};
                min = if min < m.low {min} else {m.low};
                vol += m.volume;
            }

            ret.push(ObStat {
                granularity: next_granularity(granularity),
                tick: tick,
                volume: vol,
                high: max,
                low: min,
                open: first.open,
                close: subject.last().unwrap().close
            });

            last_tick_in_slice += slice_size as u64;
        }
        ret
    }

}