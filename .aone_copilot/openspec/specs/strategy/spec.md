# strategy Specification

## Purpose
TBD - created by archiving change add-daily-bar-backtest-mvp. Update Purpose after archive.
## Requirements
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

### Requirement: Z-Score 均值回归策略
系统 SHALL 实现 `MeanReversionZScoreStrategy`，包含 `window: usize`、`entry_z: f64`、`exit_z: f64` 参数。策略使用滚动窗口内收盘价的总体标准差计算 z-score，信号规则：若 `t + 1 < window` 返回 `Hold`；若标准差为 0 返回 `Hold`；计算 `z = (close[t] - mean) / std_dev`；若 `z < -entry_z` 返回 `Buy`；若 `z > -exit_z` 返回 `Sell`；否则返回 `Hold`。`name()` MUST 返回 `"mean_reversion_zscore"`。

#### Scenario: Z-Score 策略数据不足时返回 Hold
- **WHEN** 当前索引 `t + 1 < window`
- **THEN** 返回 `Hold`

#### Scenario: Z-Score 策略零方差时返回 Hold
- **WHEN** 滚动窗口内所有收盘价完全相同（标准差为 0）
- **THEN** 返回 `Hold`，不得发生除零错误

#### Scenario: Z-Score 策略触发买入信号
- **WHEN** 当前 z-score 小于 `-entry_z`（价格显著低于均值）
- **THEN** 返回 `Buy`

#### Scenario: Z-Score 策略触发卖出信号
- **WHEN** 当前 z-score 大于 `-exit_z`（价格回归均值附近）
- **THEN** 返回 `Sell`

#### Scenario: Z-Score 策略无明确信号
- **WHEN** 当前 z-score 在 `-entry_z` 和 `-exit_z` 之间
- **THEN** 返回 `Hold`

### Requirement: Z-Score 策略参数校验
系统 SHALL 在 `MeanReversionZScoreStrategy::new()` 中校验参数合法性，返回 `Result<Self, String>`。参数约束：`window >= 2`、`entry_z > 0.0`、`exit_z >= 0.0`。非法参数 MUST 返回错误而非 panic。

#### Scenario: 合法参数构造成功
- **WHEN** 提供 `window = 20, entry_z = 2.0, exit_z = 0.5`
- **THEN** 构造成功，返回 `Ok(strategy)`

#### Scenario: 非法窗口参数
- **WHEN** 提供 `window = 1`
- **THEN** 返回 `Err`，包含明确错误信息

#### Scenario: 非法阈值参数
- **WHEN** 提供 `entry_z <= 0` 或 `exit_z < 0`
- **THEN** 返回 `Err`，包含明确错误信息

