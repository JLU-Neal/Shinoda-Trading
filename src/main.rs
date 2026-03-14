mod types;
mod data;
mod strategy;
mod strategies;
mod portfolio;
mod backtest;
mod metrics;

use clap::Parser;

use crate::data::load_bars_from_csv;
use crate::strategy::Strategy;
use crate::strategies::momentum::MomentumStrategy;
use crate::strategies::mean_reversion::MeanReversionStrategy;
use crate::backtest::run_backtest;

#[derive(Parser)]
#[command(name = "quant_backtest")]
#[command(about = "Shinoda Trading - US Stock Daily Bar Backtest System")]
struct Cli {
    /// Stock symbol (e.g. AAPL)
    #[arg(long)]
    symbol: String,

    /// Path to CSV data file
    #[arg(long)]
    file: String,

    /// Strategy name: momentum or mean_reversion
    #[arg(long)]
    strategy: String,

    /// Initial cash in USD (default: 100000)
    #[arg(long, default_value_t = 100_000.0)]
    initial_cash: f64,

    /// Lookback period for momentum strategy (default: 10)
    #[arg(long, default_value_t = 10)]
    lookback: usize,

    /// Entry threshold for momentum strategy (default: 0.05)
    #[arg(long, default_value_t = 0.05)]
    entry_threshold: f64,

    /// Exit threshold for momentum strategy (default: -0.03)
    #[arg(long, default_value_t = -0.03)]
    exit_threshold: f64,

    /// Window size for mean reversion strategy (default: 20)
    #[arg(long, default_value_t = 20)]
    window: usize,

    /// Buy threshold for mean reversion strategy (default: 0.95)
    #[arg(long, default_value_t = 0.95)]
    buy_threshold: f64,

    /// Sell threshold for mean reversion strategy (default: 1.05)
    #[arg(long, default_value_t = 1.05)]
    sell_threshold: f64,
}

fn build_strategy(cli: &Cli) -> Result<Box<dyn Strategy>, String> {
    match cli.strategy.as_str() {
        "momentum" => Ok(Box::new(MomentumStrategy::new(
            cli.lookback,
            cli.entry_threshold,
            cli.exit_threshold,
        ))),
        "mean_reversion" => Ok(Box::new(MeanReversionStrategy::new(
            cli.window,
            cli.buy_threshold,
            cli.sell_threshold,
        ))),
        other => Err(format!(
            "Unknown strategy '{}'. Supported strategies: momentum, mean_reversion",
            other
        )),
    }
}

fn main() {
    let cli = Cli::parse();

    let strategy = match build_strategy(&cli) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let bars = match load_bars_from_csv(&cli.file, &cli.symbol) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            std::process::exit(1);
        }
    };

    let result = run_backtest(&bars, strategy.as_ref(), cli.initial_cash);

    println!("========================================");
    println!("  Backtest Result - {} ({})", cli.symbol, strategy.name());
    println!("========================================");
    println!("  Initial Cash:      ${:.2}", result.initial_cash);
    println!("  Final Equity:      ${:.2}", result.final_equity);
    println!("  Total Return:      {:.2}%", result.total_return * 100.0);
    println!("  Annualized Return: {:.2}%", result.annualized_return * 100.0);
    println!("  Max Drawdown:      {:.2}%", result.max_drawdown * 100.0);
    println!("  Trade Count:       {}", result.trade_count);
    println!("========================================");
}
