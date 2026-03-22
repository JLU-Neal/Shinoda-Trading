## MODIFIED Requirements

### Requirement: 命令行参数解析
系统 SHALL 使用 `clap` 解析命令行参数，采用子命令模式。支持 `backtest` 子命令（保留原有全部参数：`--symbol`、`--file`、`--strategy`、`--initial-cash`、`--lookback`、`--entry-threshold`、`--exit-threshold`、`--window`、`--buy-threshold`、`--sell-threshold`，新增 `--entry-z`、`--exit-z`）和 `fetch` 子命令（参数：`--symbol`、`--start`、`--end`、`--api-key`、`--output`）。`--strategy` 参数支持 `momentum`、`mean_reversion` 和 `mean_reversion_zscore` 三种策略。

#### Scenario: 使用动量策略运行回测
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy momentum`
- **THEN** 系统加载数据、使用动量策略执行回测并输出结果

#### Scenario: 使用均值回归策略运行回测
- **WHEN** 执行 `cargo run -- backtest --symbol SPY --file data/SPY.csv --strategy mean_reversion`
- **THEN** 系统加载数据、使用均值回归策略执行回测并输出结果

#### Scenario: 使用 Z-Score 均值回归策略运行回测
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy mean_reversion_zscore`
- **THEN** 系统加载数据、使用 Z-Score 均值回归策略（默认参数 window=20, entry_z=2.0, exit_z=0.5）执行回测并输出结果

#### Scenario: 自定义策略参数
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy mean_reversion_zscore --window 30 --entry-z 1.5 --exit-z 0.3`
- **THEN** 系统使用指定的策略参数执行回测

#### Scenario: 缺少必需参数
- **WHEN** 未提供子命令或子命令缺少必需参数
- **THEN** 输出 usage 帮助信息并退出

#### Scenario: 非法策略名称
- **WHEN** `--strategy` 参数值不是 `momentum`、`mean_reversion` 或 `mean_reversion_zscore`
- **THEN** 返回明确的错误信息

#### Scenario: 使用 fetch 子命令获取数据
- **WHEN** 执行 `cargo run -- fetch --symbol AAPL --start 2024-01-01 --end 2024-06-30 --api-key YOUR_KEY --output data/AAPL.csv`
- **THEN** 系统从 Alpha Vantage API 获取指定时间范围的数据并保存为 CSV 文件

#### Scenario: fetch 子命令使用默认输出路径
- **WHEN** 执行 `cargo run -- fetch --symbol AAPL --start 2024-01-01 --end 2024-06-30 --api-key YOUR_KEY` 且未指定 `--output`
- **THEN** 系统将数据保存到 `data/{symbol}.csv`（如 `data/AAPL.csv`）

#### Scenario: Z-Score 策略参数非法时报错
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy mean_reversion_zscore --window 1`
- **THEN** 返回明确的参数错误信息并退出
