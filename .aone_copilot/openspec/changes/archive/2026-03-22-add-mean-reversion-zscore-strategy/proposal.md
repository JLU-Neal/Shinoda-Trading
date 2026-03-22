# Change: 新增 Mean Reversion Z-Score 策略

## Why
当前系统已有基于价格/均值比率的简单均值回归策略（`MeanReversionStrategy`），但缺乏基于统计偏离度（z-score）的更精确均值回归策略。Z-Score 策略通过计算滚动窗口内的标准差来量化价格偏离程度，能更好地适应不同波动率环境，提供更科学的入场和出场信号。

## What Changes
- 新增 `src/strategies/mean_reversion_zscore.rs`，实现 `MeanReversionZScoreStrategy`
- 策略使用总体标准差计算 z-score，基于 `entry_z` 和 `exit_z` 阈值生成信号
- 构造函数返回 `Result`，对非法参数（`window < 2`、`entry_z <= 0`、`exit_z < 0`）返回错误
- 在 `src/strategies/mod.rs` 中注册新模块
- 在 `src/main.rs` 的 `build_strategy` 中新增 `mean_reversion_zscore` 分支
- 在 `BacktestArgs` 中新增 `--entry-z` 和 `--exit-z` CLI 参数

## Impact
- 影响的规范：`strategy`（新增策略需求）、`cli`（新增策略选项和参数）
- 影响的代码：`src/strategies/mod.rs`、`src/main.rs`、新增 `src/strategies/mean_reversion_zscore.rs`
