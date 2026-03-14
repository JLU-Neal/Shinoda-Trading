## ADDED Requirements

### Requirement: 可插拔策略接口
系统 SHALL 定义 `Strategy` trait，包含 `name(&self) -> &str` 和 `generate_signal(&self, bars: &[Bar], current_index: usize) -> Signal` 方法。策略只负责信号生成，不得直接修改账户、持仓或交易记录。

#### Scenario: 策略仅生成信号
- **WHEN** 回测引擎调用策略的 `generate_signal` 方法
- **THEN** 策略返回 `Buy`、`Sell` 或 `Hold` 信号，不修改任何外部状态

#### Scenario: 历史数据不足时返回 Hold
- **WHEN** 当前索引小于策略所需的计算窗口
- **THEN** 策略 MUST 返回 `Hold`

### Requirement: Signal 领域模型
系统 SHALL 定义 `Signal` 枚举，包含 `Buy`、`Sell`、`Hold` 三个变体，表示策略输出的交易信号。

#### Scenario: 信号类型覆盖
- **WHEN** 策略对任意交易日生成信号
- **THEN** 信号值 MUST 为 `Buy`、`Sell` 或 `Hold` 之一

### Requirement: 动量策略
系统 SHALL 实现 `MomentumStrategy`，包含 `lookback: usize`、`entry_threshold: f64`、`exit_threshold: f64` 参数。信号规则：若 `t < lookback` 返回 `Hold`；计算收益率 `r = close[t] / close[t - lookback] - 1.0`；若 `r > entry_threshold` 返回 `Buy`；若 `r < exit_threshold` 返回 `Sell`；否则返回 `Hold`。

#### Scenario: 动量策略数据不足时返回 Hold
- **WHEN** 当前索引 `t < lookback`
- **THEN** 返回 `Hold`

#### Scenario: 动量策略触发买入信号
- **WHEN** 收益率 `r > entry_threshold`
- **THEN** 返回 `Buy`

#### Scenario: 动量策略触发卖出信号
- **WHEN** 收益率 `r < exit_threshold`
- **THEN** 返回 `Sell`

#### Scenario: 动量策略无明确信号
- **WHEN** 收益率在 `exit_threshold` 和 `entry_threshold` 之间
- **THEN** 返回 `Hold`

### Requirement: 均值回归策略
系统 SHALL 实现 `MeanReversionStrategy`，包含 `window: usize`、`buy_threshold: f64`、`sell_threshold: f64` 参数。信号规则：若 `t < window` 返回 `Hold`；计算最近 `window` 日收盘价均值 `ma`；若 `close[t] < ma * buy_threshold` 返回 `Buy`；若 `close[t] >= ma * sell_threshold` 返回 `Sell`；否则返回 `Hold`。

#### Scenario: 均值回归策略数据不足时返回 Hold
- **WHEN** 当前索引 `t < window`
- **THEN** 返回 `Hold`

#### Scenario: 均值回归策略触发买入信号
- **WHEN** 当前收盘价低于均值乘以 `buy_threshold`
- **THEN** 返回 `Buy`

#### Scenario: 均值回归策略触发卖出信号
- **WHEN** 当前收盘价高于或等于均值乘以 `sell_threshold`
- **THEN** 返回 `Sell`

#### Scenario: 均值回归策略无明确信号
- **WHEN** 当前收盘价在两个阈值之间
- **THEN** 返回 `Hold`
