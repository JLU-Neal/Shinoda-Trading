use crate::strategy::Strategy;
use crate::types::{Bar, Signal};

#[derive(Debug)]
pub struct MeanReversionZScoreStrategy {
    window: usize,
    entry_z: f64,
    exit_z: f64,
}

impl MeanReversionZScoreStrategy {
    pub fn new(window: usize, entry_z: f64, exit_z: f64) -> Result<Self, String> {
        if window < 2 {
            return Err(format!("window must be >= 2, got {}", window));
        }
        if entry_z <= 0.0 {
            return Err(format!("entry_z must be > 0.0, got {}", entry_z));
        }
        if exit_z < 0.0 {
            return Err(format!("exit_z must be >= 0.0, got {}", exit_z));
        }
        Ok(Self {
            window,
            entry_z,
            exit_z,
        })
    }

    fn rolling_mean(values: &[f64]) -> f64 {
        let sum: f64 = values.iter().sum();
        sum / values.len() as f64
    }

    fn rolling_std_dev(values: &[f64], mean: f64) -> f64 {
        let variance: f64 = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
        variance.sqrt()
    }

    fn compute_zscore(values: &[f64]) -> Option<f64> {
        let mean = Self::rolling_mean(values);
        let std_dev = Self::rolling_std_dev(values, mean);
        if std_dev == 0.0 {
            return None;
        }
        let current = *values.last().unwrap();
        Some((current - mean) / std_dev)
    }
}

impl Strategy for MeanReversionZScoreStrategy {
    fn name(&self) -> &str {
        "mean_reversion_zscore"
    }

    fn generate_signal(&self, bars: &[Bar], current_index: usize) -> Signal {
        if current_index + 1 < self.window {
            return Signal::Hold;
        }

        let start = current_index + 1 - self.window;
        let window_closes: Vec<f64> = bars[start..=current_index]
            .iter()
            .map(|bar| bar.close)
            .collect();

        match Self::compute_zscore(&window_closes) {
            None => Signal::Hold,
            Some(z_score) => {
                if z_score < -self.entry_z {
                    Signal::Buy
                } else if z_score > -self.exit_z {
                    Signal::Sell
                } else {
                    Signal::Hold
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_bar(date: NaiveDate, close: f64) -> Bar {
        Bar {
            date,
            symbol: "TEST".to_string(),
            open: close,
            high: close,
            low: close,
            close,
            volume: 1000.0,
        }
    }

    fn make_date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2023, 1, day).unwrap()
    }

    #[test]
    fn test_new_valid_parameters() {
        let result = MeanReversionZScoreStrategy::new(20, 2.0, 0.5);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_window_too_small() {
        let result = MeanReversionZScoreStrategy::new(1, 2.0, 0.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("window"));
    }

    #[test]
    fn test_new_entry_z_zero() {
        let result = MeanReversionZScoreStrategy::new(20, 0.0, 0.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("entry_z"));
    }

    #[test]
    fn test_new_entry_z_negative() {
        let result = MeanReversionZScoreStrategy::new(20, -1.0, 0.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("entry_z"));
    }

    #[test]
    fn test_new_exit_z_negative() {
        let result = MeanReversionZScoreStrategy::new(20, 2.0, -0.1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exit_z"));
    }

    #[test]
    fn test_name() {
        let strategy = MeanReversionZScoreStrategy::new(5, 2.0, 0.5).unwrap();
        assert_eq!(strategy.name(), "mean_reversion_zscore");
    }

    #[test]
    fn test_hold_when_insufficient_data() {
        let strategy = MeanReversionZScoreStrategy::new(20, 2.0, 0.5).unwrap();
        let bars: Vec<Bar> = (1..=10)
            .map(|i| create_test_bar(make_date(i), 100.0))
            .collect();

        for index in 0..bars.len() {
            let signal = strategy.generate_signal(&bars, index);
            assert_eq!(signal, Signal::Hold, "index {} should be Hold", index);
        }
    }

    #[test]
    fn test_hold_when_zero_variance() {
        let strategy = MeanReversionZScoreStrategy::new(5, 2.0, 0.5).unwrap();
        let bars: Vec<Bar> = (1..=5)
            .map(|i| create_test_bar(make_date(i), 100.0))
            .collect();

        let signal = strategy.generate_signal(&bars, 4);
        assert_eq!(signal, Signal::Hold);
    }

    #[test]
    fn test_buy_signal() {
        // Window: [100, 101, 99, 100, 94]
        // mean = 98.8, variance = sum of squared diffs / 5
        // diffs: 1.2, 2.2, 0.2, 1.2, -4.8
        // squared: 1.44, 4.84, 0.04, 1.44, 23.04 → sum = 30.8
        // variance = 30.8 / 5 = 6.16, std_dev = 2.4819..
        // z = (94 - 98.8) / 2.4819 = -1.933...
        // With entry_z = 1.5, z < -1.5 → Buy
        let strategy = MeanReversionZScoreStrategy::new(5, 1.5, 0.5).unwrap();
        let bars = vec![
            create_test_bar(make_date(1), 100.0),
            create_test_bar(make_date(2), 101.0),
            create_test_bar(make_date(3), 99.0),
            create_test_bar(make_date(4), 100.0),
            create_test_bar(make_date(5), 94.0),
        ];

        let signal = strategy.generate_signal(&bars, 4);
        assert_eq!(signal, Signal::Buy);
    }

    #[test]
    fn test_sell_signal() {
        // Window: [100, 98, 97, 99, 100]
        // mean = 98.8, diffs: 1.2, -0.8, -1.8, 0.2, 1.2
        // squared: 1.44, 0.64, 3.24, 0.04, 1.44 → sum = 6.8
        // variance = 6.8 / 5 = 1.36, std_dev = 1.1661..
        // z = (100 - 98.8) / 1.1661 = 1.0290...
        // z > -exit_z (i.e. z > -0.5) → Sell
        let strategy = MeanReversionZScoreStrategy::new(5, 2.0, 0.5).unwrap();
        let bars = vec![
            create_test_bar(make_date(1), 100.0),
            create_test_bar(make_date(2), 98.0),
            create_test_bar(make_date(3), 97.0),
            create_test_bar(make_date(4), 99.0),
            create_test_bar(make_date(5), 100.0),
        ];

        let signal = strategy.generate_signal(&bars, 4);
        assert_eq!(signal, Signal::Sell);
    }

    #[test]
    fn test_hold_between_thresholds() {
        // We need z_score in range [-entry_z, -exit_z], i.e. [-2.0, -0.5]
        // Window: [100, 101, 99, 100, 98]
        // mean = 99.6, diffs: 0.4, 1.4, -0.6, 0.4, -1.6
        // squared: 0.16, 1.96, 0.36, 0.16, 2.56 → sum = 5.2
        // variance = 5.2 / 5 = 1.04, std_dev = 1.0198..
        // z = (98 - 99.6) / 1.0198 = -1.5689...
        // -2.0 <= -1.5689 <= -0.5 → Hold
        let strategy = MeanReversionZScoreStrategy::new(5, 2.0, 0.5).unwrap();
        let bars = vec![
            create_test_bar(make_date(1), 100.0),
            create_test_bar(make_date(2), 101.0),
            create_test_bar(make_date(3), 99.0),
            create_test_bar(make_date(4), 100.0),
            create_test_bar(make_date(5), 98.0),
        ];

        let signal = strategy.generate_signal(&bars, 4);
        assert_eq!(signal, Signal::Hold);
    }
}
