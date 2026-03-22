# Change: 集成 Alpha Vantage API 获取美股历史日线数据

## Why
当前系统仅支持从本地 CSV 文件加载历史行情数据，用户需要手动准备数据文件。集成 Alpha Vantage 免费 API 后，用户可以直接通过命令行指定股票代码和时间范围，自动从网络获取真实的美股历史日线数据，大幅降低数据准备的门槛。

## What Changes
- 新增 `src/fetcher.rs` 模块，封装 Alpha Vantage API 调用逻辑
- 新增 `fetch` 子命令，支持通过 `--symbol`、`--start`、`--end`、`--api-key` 参数从 API 获取数据并保存为 CSV
- 新增依赖：`reqwest`（HTTP 客户端）、`serde_json`（JSON 解析）、`tokio`（异步运行时）
- CLI 从扁平参数模式改为子命令模式（`backtest` 和 `fetch`）

## Impact
- 影响的规范：`data-loader`（新增 API 数据获取能力）、`cli`（新增 fetch 子命令，重构为子命令模式）
- 影响的代码：`Cargo.toml`、`src/main.rs`、新增 `src/fetcher.rs`
