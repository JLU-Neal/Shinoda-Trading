# cli Specification

## Purpose
TBD - created by archiving change add-daily-bar-backtest-mvp. Update Purpose after archive.
## Requirements
### Requirement: 命令行参数解析
系统 SHALL 使用 `clap` 解析命令行参数。必需参数：`--symbol`（股票代码）、`--file`（CSV 文件路径）、`--strategy`（策略名称，支持 `momentum` 和 `mean_reversion`）。可选参数：`--initial-cash`（默认 100000）、`--lookback`、`--entry-threshold`、`--exit-threshold`、`--window`、`--buy-threshold`、`--sell-threshold`。

#### Scenario: 使用动量策略运行回测
- **WHEN** 执行 `cargo run -- --symbol AAPL --file data/AAPL.csv --strategy momentum`
- **THEN** 系统加载数据、使用动量策略执行回测并输出结果

#### Scenario: 使用均值回归策略运行回测
- **WHEN** 执行 `cargo run -- --symbol SPY --file data/SPY.csv --strategy mean_reversion`
- **THEN** 系统加载数据、使用均值回归策略执行回测并输出结果

#### Scenario: 自定义策略参数
- **WHEN** 执行 `cargo run -- --symbol AAPL --file data/AAPL.csv --strategy momentum --lookback 20 --entry-threshold 0.05 --exit-threshold -0.02`
- **THEN** 系统使用指定的策略参数执行回测

#### Scenario: 缺少必需参数
- **WHEN** 未提供 `--symbol`、`--file` 或 `--strategy` 中的任一参数
- **THEN** 输出 usage 帮助信息并退出

#### Scenario: 非法策略名称
- **WHEN** `--strategy` 参数值不是 `momentum` 或 `mean_reversion`
- **THEN** 返回明确的错误信息

### Requirement: 回测结果输出
系统 SHALL 在回测完成后，将结果以可读格式输出到标准输出，包含：初始资金、最终净值、总收益率、年化收益率、最大回撤、交易次数。

#### Scenario: 结果格式化输出
- **WHEN** 回测成功完成
- **THEN** 在标准输出中显示所有绩效指标，格式清晰可读

### Requirement: 主流程编排
系统 SHALL 按以下顺序执行主流程：解析 CLI 参数 → 加载 CSV 数据 → 校验数据合法性 → 构造策略实例 → 执行回测 → 输出结果。

#### Scenario: 端到端执行
- **WHEN** 提供有效的命令行参数和数据文件
- **THEN** 系统按顺序完成数据加载、策略构造、回测执行和结果输出

