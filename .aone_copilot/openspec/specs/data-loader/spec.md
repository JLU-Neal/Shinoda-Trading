# data-loader Specification

## Purpose
TBD - created by archiving change add-daily-bar-backtest-mvp. Update Purpose after archive.
## Requirements
### Requirement: CSV 数据加载
系统 SHALL 提供 `load_bars_from_csv(path, symbol)` 函数，从本地 CSV 文件读取单只美股历史日线数据，并返回按日期升序排列的 `Vec<Bar>`。

#### Scenario: 成功加载合法 CSV 文件
- **WHEN** 提供一个格式正确的 CSV 文件路径和股票代码
- **THEN** 返回包含所有交易日数据的 `Vec<Bar>`，按日期升序排列

#### Scenario: CSV 文件不存在
- **WHEN** 提供的文件路径不存在
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: CSV 字段缺失或格式错误
- **WHEN** CSV 文件中存在列缺失、日期格式非法或数值解析失败
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: 价格数据非法
- **WHEN** CSV 文件中存在价格 <= 0 的记录
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: 数据为空
- **WHEN** CSV 文件不包含任何数据行
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: 日期未按升序排列
- **WHEN** CSV 文件中的日期未按升序排列
- **THEN** 返回包含明确错误信息的 `Err`

### Requirement: CSV 输入格式规范
系统 SHALL 要求 CSV 文件遵循以下格式：列头为 `date,open,high,low,close,volume`；`date` 格式为 `YYYY-MM-DD`；所有价格字段和成交量字段 MUST 存在且为有效数值；`symbol` 由外部参数传入，不从 CSV 推断。

#### Scenario: 标准格式 CSV 解析
- **WHEN** CSV 文件包含 `date,open,high,low,close,volume` 列头和有效数据行
- **THEN** 每行被正确解析为一个 `Bar` 结构体，`symbol` 字段由传入参数填充

### Requirement: Bar 领域模型
系统 SHALL 定义 `Bar` 结构体，包含 `date: NaiveDate`、`symbol: String`、`open: f64`、`high: f64`、`low: f64`、`close: f64`、`volume: f64` 字段，表示单个交易日的行情数据。

#### Scenario: Bar 结构体字段完整
- **WHEN** 从 CSV 加载一行有效数据
- **THEN** 生成的 `Bar` 实例包含所有必需字段且值与 CSV 数据一致

