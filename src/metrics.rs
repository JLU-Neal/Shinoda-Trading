use crate::types::BacktestResult;

pub fn calculate_metrics(equity_curve: &[f64], initial_cash: f64, trade_count: usize) -> BacktestResult {
    if equity_curve.is_empty() || equity_curve.len() <= 1 {
        return BacktestResult {
            initial_cash,
            final_equity: initial_cash,
            total_return: 0.0,
            annualized_return: 0.0,
            max_drawdown: 0.0,
            trade_count,
        };
    }

    let final_equity = *equity_curve.last().unwrap();
    let total_return = (final_equity - initial_cash) / initial_cash;
    let days = equity_curve.len() as f64;
    let annualized_return = (final_equity / initial_cash).powf(252.0 / days) - 1.0;

    let mut max_drawdown = 0.0;
    let mut peak = equity_curve[0];

    for &value in equity_curve.iter() {
        let drawdown = (peak - value) / peak;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
        if value > peak {
            peak = value;
        }
    }

    BacktestResult {
        initial_cash,
        final_equity,
        total_return,
        annualized_return,
        max_drawdown,
        trade_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_metrics_no_change() {
        let equity_curve = vec![100000.0; 10];
        let result = calculate_metrics(&equity_curve, 100000.0, 0);
        assert_eq!(result.total_return, 0.0);
        assert_eq!(result.max_drawdown, 0.0);
    }

    #[test]
    fn test_max_drawdown() {
        let equity_curve = vec![100.0, 110.0, 90.0, 95.0];
        let result = calculate_metrics(&equity_curve, 100.0, 0);
        let expected_drawdown = (110.0 - 90.0) / 110.0;
        assert!((result.max_drawdown - expected_drawdown).abs() < 1e-10);
    }
}
