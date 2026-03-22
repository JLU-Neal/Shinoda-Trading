## Context
当前回测系统仅支持从本地 CSV 文件加载数据。用户需要手动获取并准备 CSV 格式的历史行情数据，使用门槛较高。Alpha Vantage 提供免费的美股历史日线数据 API，可以直接集成到系统中。

## Goals / Non-Goals
- **Goals**:
  - 通过 Alpha Vantage TIME_SERIES_DAILY API 获取指定股票的历史日线数据
  - 支持用户指定时间范围（起始日期和结束日期）进行过滤
  - 将获取的数据保存为与现有 CSV 格式兼容的文件，可直接用于回测
- **Non-Goals**:
  - 不实现实时数据流或 WebSocket 连接
  - 不实现 API Key 的加密存储或配置文件管理
  - 不实现多数据源切换或数据源抽象层

## Decisions

### 使用同步 HTTP 客户端（reqwest blocking）
- **决策**: 使用 `reqwest::blocking` 而非异步模式
- **理由**: 数据获取是一次性操作，不需要并发请求，同步模式更简单，避免引入 `tokio` 异步运行时的复杂性
- **替代方案**: 使用 `tokio` + `reqwest` 异步模式 → 对于单次 API 调用过度设计

### fetch 作为独立子命令
- **决策**: 将 `fetch` 设计为独立的 CLI 子命令，与 `backtest` 并列
- **理由**: 数据获取和回测是两个独立的操作，用户可能只想下载数据而不立即回测；子命令模式更清晰
- **替代方案**: 在 backtest 命令中添加 `--fetch` 标志 → 职责混合，不够清晰

### 输出为 CSV 文件
- **决策**: fetch 命令将 API 数据转换并保存为 CSV 文件，格式与现有 `load_bars_from_csv` 兼容
- **理由**: 复用现有的 CSV 数据加载流程，保持架构简单；用户可以检查和编辑下载的数据

## Risks / Trade-offs
- **API 速率限制**: 免费 API 每分钟 5 次调用，每天 500 次 → 对于单次获取足够，但需在文档中提醒用户
- **API Key 安全**: API Key 通过命令行参数传入，可能出现在 shell 历史中 → 可接受的 MVP 风险，后续可通过环境变量支持
- **数据完整性**: Alpha Vantage 免费版返回的数据量有限（默认 100 条，full 模式返回全部历史） → 使用 `outputsize=full` 获取完整数据后按时间范围过滤

## Open Questions
- 无
