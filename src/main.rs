mod types;
mod data;
mod strategy;
mod strategies;
mod portfolio;
mod backtest;
mod metrics;
mod fetcher;

use clap::{Parser, Subcommand};
use chrono::NaiveDate;

use crate::data::load_bars_from_csv;
use crate::strategy::Strategy;
use crate::strategies::momentum::MomentumStrategy;
use crate::strategies::mean_reversion::MeanReversionStrategy;
use crate::backtest::run_backtest;
use crate::fetcher::{fetch_bars_from_alpha_vantage, save_bars_to_csv};

#[derive(Parser)]
#[command(name = "quant_backtest")]
#[command(about = "Shinoda Trading - US Stock Daily Bar Backtest System")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run backtest with historical data
    Backtest(BacktestArgs),
    /// Fetch historical data from Alpha Vantage API
    Fetch(FetchArgs),
}

#[derive(Parser)]
struct BacktestArgs {
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

#[derive(Parser)]
struct FetchArgs {
    /// Stock symbol (e.g. AAPL)
    #[arg(long)]
    symbol: String,

    /// Start date (YYYY-MM-DD)
    #[arg(long)]
    start: String,

    /// End date (YYYY-MM-DD)
    #[arg(long)]
    end: String,

    /// Alpha Vantage API key
    #[arg(long)]
    api_key: String,

    /// Output CSV file path (default: data/{symbol}.csv)
    #[arg(long)]
    output: Option<String>,
}

fn build_strategy(args: &BacktestArgs) -> Result<Box<dyn Strategy>, String> {
    match args.strategy.as_str() {
        "momentum" => Ok(Box::new(MomentumStrategy::new(
            args.lookback,
            args.entry_threshold,
            args.exit_threshold,
        ))),
        "mean_reversion" => Ok(Box::new(MeanReversionStrategy::new(
            args.window,
            args.buy_threshold,
            args.sell_threshold,
        ))),
        other => Err(format!(
            "Unknown strategy '{}'. Supported strategies: momentum, mean_reversion",
            other
        )),
    }
}

fn run_backtest_command(args: &BacktestArgs) {
    let strategy = match build_strategy(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let bars = match load_bars_from_csv(&args.file, &args.symbol) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error loading data: {}", e);
            std::process::exit(1);
        }
    };

    let result = run_backtest(&bars, strategy.as_ref(), args.initial_cash);

    println!("========================================");
    println!("  Backtest Result - {} ({})", args.symbol, strategy.name());
    println!("========================================");
    println!("  Initial Cash:      ${:.2}", result.initial_cash);
    println!("  Final Equity:      ${:.2}", result.final_equity);
    println!("  Total Return:      {:.2}%", result.total_return * 100.0);
    println!("  Annualized Return: {:.2}%", result.annualized_return * 100.0);
    println!("  Max Drawdown:      {:.2}%", result.max_drawdown * 100.0);
    println!("  Trade Count:       {}", result.trade_count);
    println!("========================================");
}

fn run_fetch_command(args: &FetchArgs) {
    let start_date = match NaiveDate::parse_from_str(&args.start, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: invalid start date '{}': {}", args.start, e);
            std::process::exit(1);
        }
    };

    let end_date = match NaiveDate::parse_from_str(&args.end, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error: invalid end date '{}': {}", args.end, e);
            std::process::exit(1);
        }
    };

    if start_date > end_date {
        eprintln!("Error: start date {} is after end date {}", args.start, args.end);
        std::process::exit(1);
    }

    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| format!("data/{}.csv", args.symbol));

    println!("Fetching {} data from {} to {} ...", args.symbol, args.start, args.end);

    let bars = match fetch_bars_from_alpha_vantage(&args.api_key, &args.symbol, start_date, end_date) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
            std::process::exit(1);
        }
    };

    if bars.is_empty() {
        eprintln!("Warning: no data found for {} between {} and {}", args.symbol, args.start, args.end);
        std::process::exit(0);
    }

    match save_bars_to_csv(&bars, &output_path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error saving data: {}", e);
            std::process::exit(1);
        }
    }

    println!("========================================");
    println!("  Fetch Complete - {}", args.symbol);
    println!("========================================");
    println!("  Records:    {}", bars.len());
    println!("  Date Range: {} ~ {}", bars.first().unwrap().date, bars.last().unwrap().date);
    println!("  Saved To:   {}", output_path);
    println!("========================================");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Backtest(args) => run_backtest_command(args),
        Commands::Fetch(args) => run_fetch_command(args),
    }
}
