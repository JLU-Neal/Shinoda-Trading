use crate::types::{Bar, BacktestResult, Portfolio, Signal};
use crate::strategy::Strategy;

pub fn run_backtest(bars: &[Bar], strategy: &dyn Strategy, initial_cash: f64) -> BacktestResult {
    if bars.is_empty() {
        return BacktestResult {
            initial_cash,
            final_equity: initial_cash,
            total_return: 0.0,
            annualized_return: 0.0,
            max_drawdown: 0.0,
            trade_count: 0,
        };
    }

    let mut portfolio = Portfolio::new(initial_cash);
    let mut trade_count: usize = 0;

    for i in 0..bars.len() {
        let signal = strategy.generate_signal(bars, i);

        if i + 1 < bars.len() {
            let next_open = bars[i + 1].open;
            match signal {
                Signal::Buy => {
                    if portfolio.buy(bars[i + 1].date, next_open).is_some() {
                        trade_count += 1;
                    }
                }
                Signal::Sell => {
                    if portfolio.sell(bars[i + 1].date, next_open).is_some() {
                        trade_count += 1;
                    }
                }
                Signal::Hold => {}
            }
        }

        portfolio.record_equity(bars[i].close);
    }

    crate::metrics::calculate_metrics(&portfolio.equity_curve, initial_cash, trade_count)
}
