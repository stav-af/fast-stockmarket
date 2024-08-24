use statrs::statistics::Statistics;

use super::record::ObStat;

pub struct Stats {
    minute_volatility: f64,
    hour_volatility: f64,
    day_volatility: f64,
    month_volatility: f64,
    
    minute_ema: f64,
    five_minute_ema: f64,
    fiftenn_minute_ema: f64,

    hour_ema: f64,
    day_ema: f64,
    week_ema: f64,

    bollinger_upper: f64,
    bollinger_lower: f64,

    rsi: f64
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            minute_volatility: 0.0,
            hour_volatility: 0.0,
            day_volatility: 0.0,
        
            minute_ema: 0.0,
            five_minute_ema: 0.0,
            fiftenn_minute_ema: 0.0,

            hour_ema: 0.0,
            day_ema: 0.0,
            week_ema: 0.0,

            bollinger_upper: 0.0,
            bollinger_lower: 0.0,

            month_volatility: 0.0,
            rsi: 0.0
        }
    }

    pub fn update_stats(&mut self, history_matrix: &Vec<Vec<ObStat>>) {
        self.minute_volatility = Self::calculate_volatility(&history_matrix[0]);
        self.hour_volatility = Self::calculate_volatility(&history_matrix[1]);
        self.day_volatility = Self::calculate_volatility(&history_matrix[2]);
        self.month_volatility = Self::calculate_volatility(&history_matrix[3]);

        self.minute_ema = Self::calculate_ema(&history_matrix[0], 120);
        self.five_minute_ema = Self::calculate_ema(&history_matrix[1], 10);
        self.minute_ema = Self::calculate_ema(&history_matrix[0], 60);
        self.minute_ema = Self::calculate_ema(&history_matrix[0], 60);
        
        self.rsi = Self::calculate_rsi(&history_matrix[3]);
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

    fn calculate_rsi(stats: &Vec<ObStat>) -> f64 {
        let n = stats.len();
        let subj: &[ObStat];
        if n <= 14 {
            subj = &stats[..];
        } else {
            subj = &stats[n-14..]
        }
        // mean gains and losses
        let (loss, gain, nl, ng) = subj.iter()
            .fold((0.0, 0.0, 0, 0), |(loss, gain, nl, ng), s| {
                let returns = s.open - s.close;
                if returns.is_sign_positive() {
                    (loss, gain + returns, nl, ng + 1)
                } else {
                    (loss - returns, gain, nl + 1, ng)
                }
            });
        
        let avg_gain = gain / ng as f64;
        let avg_loss = loss / nl as f64;

        let rs = avg_gain / avg_loss;
        100.0 / (1.0 + rs)
    }

    fn calculate_sma(stats: &Vec<ObStat>) -> f64 {
        let (sum, n) = stats.iter()
            .fold((0.0, 0), |(sum, n), ob| (sum + ob.close, n + 1));
        
        sum/n as f64
    }

    fn calculate_ema(stats: &Vec<ObStat>, period: usize) -> f64 {
        return 0.0;
        let elems = &stats[stats.len() - (2*period)..]; 
        
        let sma_elems = &elems[..period];
        let ema_elems = &elems[period..];
        
        let ema_intial = Self::calculate_sma(&sma_elems.to_vec());
        let k = 2.0/(period as f64 + 1.0);

        ema_elems.iter()
            .fold(ema_intial, |prev_ema, stat| ((stat.close - prev_ema)/k + prev_ema)) 
    }

    fn calculate_bb(stats: &Vec<ObStat>) -> (f64, f64) {
        return (0.0, 0.0);
        let std = stats.iter().map(|ob| ob.close).std_dev();
        let mean = Self::calculate_sma(stats);

        (mean - 2.0 * std, mean + 2.0 * std)        
    }
    // ema {1, 5, 10, 15 min, 1h, 1 week}
    // bb
}