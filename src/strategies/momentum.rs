use crate::strategy::Strategy;
use crate::types::{Bar, Signal};

pub struct MomentumStrategy {
    lookback: usize,
    entry_threshold: f64,
    exit_threshold: f64,
}

impl MomentumStrategy {
    pub fn new(lookback: usize, entry_threshold: f64, exit_threshold: f64) -> Self {
        Self {
            lookback,
            entry_threshold,
            exit_threshold,
        }
    }
}

impl Strategy for MomentumStrategy {
    fn name(&self) -> &str {
        "Momentum"
    }

    fn generate_signal(&self, bars: &[Bar], current_index: usize) -> Signal {
        if current_index < self.lookback {
            return Signal::Hold;
        }

        let current_close = bars[current_index].close;
        let lookback_close = bars[current_index - self.lookback].close;
        let momentum_return = current_close / lookback_close - 1.0;

        if momentum_return > self.entry_threshold {
            Signal::Buy
        } else if momentum_return < self.exit_threshold {
            Signal::Sell
        } else {
            Signal::Hold
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Bar;
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

    #[test]
    fn test_hold_when_insufficient_data() {
        let strategy = MomentumStrategy::new(5, 0.05, -0.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
        ];

        let signal = strategy.generate_signal(&bars, 2);
        assert_eq!(signal, Signal::Hold);
    }

    #[test]
    fn test_buy_signal() {
        let strategy = MomentumStrategy::new(5, 0.05, -0.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(), 103.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 5).unwrap(), 104.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(), 110.0),
        ];

        let signal = strategy.generate_signal(&bars, 5);
        assert_eq!(signal, Signal::Buy);
    }

    #[test]
    fn test_sell_signal() {
        let strategy = MomentumStrategy::new(5, 0.05, -0.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(), 103.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 5).unwrap(), 104.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(), 95.0),
        ];

        let signal = strategy.generate_signal(&bars, 5);
        assert_eq!(signal, Signal::Sell);
    }

    #[test]
    fn test_hold_signal() {
        let strategy = MomentumStrategy::new(5, 0.05, -0.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(), 103.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 5).unwrap(), 104.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(), 103.0),
        ];

        let signal = strategy.generate_signal(&bars, 5);
        assert_eq!(signal, Signal::Hold);
    }
}
