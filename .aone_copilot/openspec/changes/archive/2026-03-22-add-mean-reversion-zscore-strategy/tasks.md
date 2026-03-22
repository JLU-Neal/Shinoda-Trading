## 1. 策略实现
- [x] 1.1 创建 `src/strategies/mean_reversion_zscore.rs`，实现 `MeanReversionZScoreStrategy` 结构体和 `new()` 参数校验
- [x] 1.2 实现 `Strategy` trait（`name()` 和 `generate_signal()`），包含滚动均值、总体标准差、z-score 计算和信号生成
- [x] 1.3 编写单元测试（数据不足、零方差、买入/卖出/持有信号、参数校验）

## 2. 模块注册与 CLI 集成
- [x] 2.1 在 `src/strategies/mod.rs` 中注册 `mean_reversion_zscore` 模块
- [x] 2.2 在 `src/main.rs` 的 `BacktestArgs` 中新增 `--entry-z` 和 `--exit-z` 参数
- [x] 2.3 在 `src/main.rs` 的 `build_strategy` 中新增 `mean_reversion_zscore` 分支

## 3. 验证
- [x] 3.1 编译通过，运行 `cargo test` 确保所有测试通过（28 tests passed）
