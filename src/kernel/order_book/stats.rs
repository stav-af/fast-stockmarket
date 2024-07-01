use super::record::ObStat;

pub struct Stats {
    minute_volatility: f64,
    hour_volatility: f64,
    day_volatility: f64,
    month_volatility: f64
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            minute_volatility: 0.0,
            hour_volatility: 0.0,
            day_volatility: 0.0,
            month_volatility: 0.0
        }
    }

    pub fn update_volatilities(&mut self, history_matrix: &Vec<Vec<ObStat>>) {
        self.minute_volatility = Self::calculate_volatility(&history_matrix[0]);
        self.hour_volatility = Self::calculate_volatility(&history_matrix[1]);
        self.day_volatility = Self::calculate_volatility(&history_matrix[2]);
        self.month_volatility = Self::calculate_volatility(&history_matrix[3]);
    }

    fn calculate_volatility(stats: &Vec<ObStat>) -> f64 {
        let n = stats.len() as f64;
        if n < 2.0 { return n };
        
        let return_lambda = |s: &ObStat| (s.close - s.open)/s.open;
        let avg_return: f64 = stats.iter()
            .map(return_lambda)
            .sum::<f64>() / n;

        stats.iter()
            .map(|s| return_lambda(s) - avg_return)
            .sum::<f64>() / (n - 1.0)
    }
}