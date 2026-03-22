## ADDED Requirements

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
