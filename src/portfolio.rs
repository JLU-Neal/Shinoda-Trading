use crate::types::{Portfolio, Position, Trade};
use chrono::NaiveDate;

impl Portfolio {
    pub fn new(initial_cash: f64) -> Self {
        Portfolio {
            cash: initial_cash,
            position: None,
            equity_curve: Vec::new(),
        }
    }

    pub fn buy(&mut self, date: NaiveDate, price: f64) -> Option<Trade> {
        if self.position.is_some() {
            return None;
        }
        let shares = (self.cash / price) as u32;
        if shares == 0 {
            return None;
        }
        self.cash -= shares as f64 * price;
        self.position = Some(Position {
            shares,
            entry_price: price,
        });
        Some(Trade {
            date,
            side: "BUY".to_string(),
            price,
            shares,
        })
    }

    pub fn sell(&mut self, date: NaiveDate, price: f64) -> Option<Trade> {
        if self.position.is_none() {
            return None;
        }
        let shares = self.position.as_ref().unwrap().shares;
        self.cash += shares as f64 * price;
        self.position = None;
        Some(Trade {
            date,
            side: "SELL".to_string(),
            price,
            shares,
        })
    }

    pub fn record_equity(&mut self, current_close: f64) {
        let equity = self.cash
            + match &self.position {
                Some(p) => p.shares as f64 * current_close,
                None => 0.0,
            };
        self.equity_curve.push(equity);
    }

    pub fn current_equity(&self, current_close: f64) -> f64 {
        self.cash
            + match &self.position {
                Some(p) => p.shares as f64 * current_close,
                None => 0.0,
            }
    }
}
