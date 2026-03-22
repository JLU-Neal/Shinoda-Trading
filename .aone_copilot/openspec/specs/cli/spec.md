# cli Specification

## Purpose
TBD - created by archiving change add-daily-bar-backtest-mvp. Update Purpose after archive.
## Requirements
### Requirement: 命令行参数解析
系统 SHALL 使用 `clap` 解析命令行参数，采用子命令模式。支持 `backtest` 子命令（保留原有全部参数：`--symbol`、`--file`、`--strategy`、`--initial-cash`、`--lookback`、`--entry-threshold`、`--exit-threshold`、`--window`、`--buy-threshold`、`--sell-threshold`）和 `fetch` 子命令（参数：`--symbol`、`--start`、`--end`、`--api-key`、`--output`）。

#### Scenario: 使用动量策略运行回测
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy momentum`
- **THEN** 系统加载数据、使用动量策略执行回测并输出结果

#### Scenario: 使用均值回归策略运行回测
- **WHEN** 执行 `cargo run -- backtest --symbol SPY --file data/SPY.csv --strategy mean_reversion`
- **THEN** 系统加载数据、使用均值回归策略执行回测并输出结果

#### Scenario: 自定义策略参数
- **WHEN** 执行 `cargo run -- backtest --symbol AAPL --file data/AAPL.csv --strategy momentum --lookback 20 --entry-threshold 0.05 --exit-threshold -0.02`
- **THEN** 系统使用指定的策略参数执行回测

#### Scenario: 缺少必需参数
- **WHEN** 未提供子命令或子命令缺少必需参数
- **THEN** 输出 usage 帮助信息并退出

#### Scenario: 非法策略名称
- **WHEN** `--strategy` 参数值不是 `momentum` 或 `mean_reversion`
- **THEN** 返回明确的错误信息

#### Scenario: 使用 fetch 子命令获取数据
- **WHEN** 执行 `cargo run -- fetch --symbol AAPL --start 2024-01-01 --end 2024-06-30 --api-key YOUR_KEY --output data/AAPL.csv`
- **THEN** 系统从 Alpha Vantage API 获取指定时间范围的数据并保存为 CSV 文件

#### Scenario: fetch 子命令使用默认输出路径
- **WHEN** 执行 `cargo run -- fetch --symbol AAPL --start 2024-01-01 --end 2024-06-30 --api-key YOUR_KEY` 且未指定 `--output`
- **THEN** 系统将数据保存到 `data/{symbol}.csv`（如 `data/AAPL.csv`）

### Requirement: 回测结果输出
系统 SHALL 在回测完成后，将结果以可读格式输出到标准输出，包含：初始资金、最终净值、总收益率、年化收益率、最大回撤、交易次数。

#### Scenario: 结果格式化输出
- **WHEN** 回测成功完成
- **THEN** 在标准输出中显示所有绩效指标，格式清晰可读

### Requirement: 主流程编排
系统 SHALL 根据子命令分发执行流程：`backtest` 子命令按原有流程执行（解析参数 → 加载 CSV → 构造策略 → 执行回测 → 输出结果）；`fetch` 子命令执行数据获取流程（解析参数 → 调用 API → 过滤时间范围 → 保存 CSV → 输出摘要）。

#### Scenario: backtest 子命令端到端执行
- **WHEN** 提供有效的 backtest 子命令参数和数据文件
- **THEN** 系统按顺序完成数据加载、策略构造、回测执行和结果输出

#### Scenario: fetch 子命令端到端执行
- **WHEN** 提供有效的 fetch 子命令参数
- **THEN** 系统从 API 获取数据、按时间范围过滤、保存为 CSV 并输出获取摘要（股票代码、数据条数、时间范围、保存路径）

