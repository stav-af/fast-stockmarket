use itertools::Itertools;

use crate::{globals::GRANULARITY, timekeeper::market_time::MTime};

use super::ob_stats::{ObStat, Transaction};


const fn granularity_max_measurements(granularity: GRANULARITY) -> usize {
    (next_granularity(granularity) as isize/granularity as isize) as usize
}

const fn granularity_index(granularity: GRANULARITY) -> usize {
     match granularity {
        GRANULARITY::SECOND => 0,
        GRANULARITY::MINUTE => 1,
        GRANULARITY::HOUR => 2,
        GRANULARITY::DAY => 3,
        _ => panic!()
    }
    
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
    pub _live_data: Vec<Vec<ObStat>>,
    pub _historic_data: Vec<Vec<ObStat>>
}

impl HistoryBuffer {
    pub fn new() -> Self {
        // these will be second, minute, hour and day respectively.
        let live_data = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let historic_data = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        Self {
            _live_data: live_data,
            _historic_data: historic_data
        }
    }

    pub fn process_transactions(&mut self, measurements: Vec<Transaction>){
        // take a list of transactions, convert to _live_data, group by.
        for (second_num, record) in &measurements.iter().group_by(|t| MTime::which_second(t.timestamp)) {
            let record_vec: Vec<&Transaction> = record.collect();

            let (min_p, max_p, vol) = record_vec.iter()
                .fold((f64::MAX, f64::MIN, 0), |(min, max, vol), r| (min.min(r.price), max.max(r.price), vol + r.volume));

            self._live_data[0].push(ObStat {
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
        // cycles over the _live_data, when there are more 'seconds' measurements than seconds in a minute
        // the 'seconds' meausrements are compressed and pushed to the 'minute' array and so on
        let len = self._live_data.len();

        // println!("Seconds: {}", self._live_data[0].len());
        // println!("Minutes: {}", self._live_data[1].len());    
        // println!("Hours  : {}", self._live_data[2].len());
        // println!("Days   : {}", self._live_data[3].len());
        
        for i in 0..(len - 1) {
            let (current_hist, next_hist) = self._live_data.split_at_mut(i + 1);

            let current_hist = &mut current_hist[i];
            let next_hist = &mut next_hist[0];

            if !current_hist.is_empty() {
                let granularity = current_hist[0].granularity;
                let (compressed, uncompressed) = Self::downgrade_granularity(current_hist, granularity);

                next_hist.extend(compressed);
                if !uncompressed.is_empty() {self._historic_data[granularity_index(granularity)] = uncompressed}
            }
        }
    }

    /// Takes a list of ObStat, groups by measurements falling into a granularity one lower (e.g groups all seconds in the same minute)
    /// Then returns that list of ObStat, and the list used to compress into that obStat
    fn downgrade_granularity(measurements:&mut Vec<ObStat>, granularity: GRANULARITY) -> (Vec<ObStat>, Vec<ObStat>) {
        let mut target: Vec<ObStat> = Vec::new();
        let mut subject: Vec<ObStat> = Vec::new();

        if measurements.len() == 0 {
            return (target, subject)
        }

        let slice_size = granularity_max_measurements(granularity) as usize;
        let mut last_tick_in_slice = measurements[0].tick + (slice_size as u64 - 1);
        
        let last_tick = measurements.last().unwrap().tick;

        while (!measurements.is_empty()) && (last_tick >= last_tick_in_slice) {
            let index = measurements.iter()
                .position(|m| m.tick > last_tick_in_slice)
                .unwrap_or(measurements.len());
            

            subject = measurements.drain(0..index).collect();
            let (max, min, vol) = subject.iter().fold((f64::MIN, f64::MAX, 0), |(max, min, vol), m| {
                (max.max(m.high), min.min(m.low), vol + m.volume)
            }); 
 
            let first = subject[0];
            let tick: u64 = first.tick / slice_size as u64;
            target.push(ObStat {
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
        (target, subject) 
    }

}