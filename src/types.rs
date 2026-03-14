use chrono::NaiveDate;

#[derive(Debug)]
pub struct Bar {
    pub date: NaiveDate,
    pub symbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct Position {
    pub shares: u32,
    pub entry_price: f64,
}

pub struct Portfolio {
    pub cash: f64,
    pub position: Option<Position>,
    pub equity_curve: Vec<f64>,
}

pub struct Trade {
    pub date: NaiveDate,
    pub side: String,
    pub price: f64,
    pub shares: u32,
}

pub struct BacktestResult {
    pub initial_cash: f64,
    pub final_equity: f64,
    pub total_return: f64,
    pub annualized_return: f64,
    pub max_drawdown: f64,
    pub trade_count: usize,
}
