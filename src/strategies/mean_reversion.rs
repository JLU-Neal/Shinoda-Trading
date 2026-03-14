use crate::strategy::Strategy;
use crate::types::{Bar, Signal};

pub struct MeanReversionStrategy {
    window: usize,
    buy_threshold: f64,
    sell_threshold: f64,
}

impl MeanReversionStrategy {
    pub fn new(window: usize, buy_threshold: f64, sell_threshold: f64) -> Self {
        Self {
            window,
            buy_threshold,
            sell_threshold,
        }
    }
}

impl Strategy for MeanReversionStrategy {
    fn name(&self) -> &str {
        "MeanReversion"
    }

    fn generate_signal(&self, bars: &[Bar], current_index: usize) -> Signal {
        if current_index < self.window {
            return Signal::Hold;
        }

        let start_index = current_index - self.window + 1;
        let sum: f64 = bars[start_index..=current_index]
            .iter()
            .map(|bar| bar.close)
            .sum();
        
        let ma = sum / self.window as f64;
        let current_close = bars[current_index].close;

        if current_close < ma * self.buy_threshold {
            Signal::Buy
        } else if current_close >= ma * self.sell_threshold {
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
        let strategy = MeanReversionStrategy::new(5, 0.95, 1.05);
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
        let strategy = MeanReversionStrategy::new(5, 0.95, 1.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(), 103.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 5).unwrap(), 104.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(), 90.0),
        ];

        let signal = strategy.generate_signal(&bars, 5);
        assert_eq!(signal, Signal::Buy);
    }

    #[test]
    fn test_sell_signal() {
        let strategy = MeanReversionStrategy::new(5, 0.95, 1.05);
        let bars = vec![
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(), 100.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 2).unwrap(), 101.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 3).unwrap(), 102.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 4).unwrap(), 103.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 5).unwrap(), 104.0),
            create_test_bar(NaiveDate::from_ymd_opt(2023, 1, 6).unwrap(), 115.0),
        ];

        let signal = strategy.generate_signal(&bars, 5);
        assert_eq!(signal, Signal::Sell);
    }
}
