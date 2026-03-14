## ADDED Requirements

### Requirement: 绩效指标计算
系统 SHALL 提供 `calculate_metrics(equity_curve, initial_cash, trade_count)` 函数，从净值曲线计算总收益率、年化收益率和最大回撤。

#### Scenario: 计算总收益率
- **WHEN** 提供净值曲线和初始资金
- **THEN** 总收益率 = `(final_equity - initial_cash) / initial_cash`

#### Scenario: 计算年化收益率
- **WHEN** 提供净值曲线和初始资金
- **THEN** 年化收益率按 252 个交易日估算：`annualized = (final / initial) ^ (252 / days) - 1`

#### Scenario: 计算最大回撤
- **WHEN** 提供净值曲线
- **THEN** 最大回撤基于净值曲线逐日计算，为历史最高点到最低谷的最大百分比跌幅

#### Scenario: 无交易时的指标
- **WHEN** 回测过程中未发生任何交易
- **THEN** 总收益率为 0，最大回撤为 0，交易次数为 0

### Requirement: BacktestResult 领域模型
系统 SHALL 定义 `BacktestResult` 结构体，包含 `initial_cash: f64`、`final_equity: f64`、`total_return: f64`、`annualized_return: f64`、`max_drawdown: f64`、`trade_count: usize` 字段。

#### Scenario: 结果输出完整
- **WHEN** 回测完成
- **THEN** BacktestResult 包含所有必需的绩效指标字段且值正确
