use super::record::ObStat;

pub struct Stats {
    minute_volatility: f64,
    hour_volatility: f64,
    day_volatility: f64,
    month_volatility: f64
}

impl Stats {
    fn new() -> Self {
        Stats {
            minute_volatility: 0.0,
            hour_volatility: 0.0,
            day_volatility: 0.0,
            month_volatility: 0.0
        }
    }

    fn update_volatilities(history_matrix: &Vec<ObStat>) {
        
    }
}