## ADDED Requirements

### Requirement: Alpha Vantage API 数据获取
系统 SHALL 提供 `fetch_bars_from_alpha_vantage(api_key, symbol, start_date, end_date)` 函数，通过 Alpha Vantage TIME_SERIES_DAILY API 获取指定股票在指定时间范围内的历史日线数据，并返回按日期升序排列的 `Vec<Bar>`。

#### Scenario: 成功获取指定时间范围的数据
- **WHEN** 提供有效的 API Key、股票代码（如 "AAPL"）、起始日期和结束日期
- **THEN** 返回该时间范围内所有交易日的 `Vec<Bar>`，按日期升序排列

#### Scenario: API Key 无效
- **WHEN** 提供无效的 API Key
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: 股票代码不存在
- **WHEN** 提供不存在的股票代码
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: 网络请求失败
- **WHEN** 网络连接不可用或 API 服务不可达
- **THEN** 返回包含明确错误信息的 `Err`

#### Scenario: API 返回错误信息
- **WHEN** Alpha Vantage API 返回错误响应（如超出速率限制）
- **THEN** 返回包含 API 错误信息的 `Err`

### Requirement: API 数据保存为 CSV
系统 SHALL 提供 `save_bars_to_csv(bars, output_path)` 函数，将 `Vec<Bar>` 保存为与现有 CSV 加载格式兼容的文件，列头为 `date,open,high,low,close,volume`。

#### Scenario: 成功保存数据到 CSV 文件
- **WHEN** 提供非空的 `Vec<Bar>` 和有效的输出路径
- **THEN** 生成的 CSV 文件包含正确的列头和所有数据行，格式与 `load_bars_from_csv` 兼容

#### Scenario: 输出路径不可写
- **WHEN** 提供的输出路径不可写（如权限不足或目录不存在）
- **THEN** 返回包含明确错误信息的 `Err`
