## 1. 项目脚手架搭建
- [x] 1.1 创建 Cargo.toml，配置项目名称和依赖（csv, serde, chrono, clap）
- [x] 1.2 创建 src/ 目录结构和所有模块文件
- [x] 1.3 创建 data/ 目录并准备示例 CSV 数据文件

## 2. 领域模型定义
- [x] 2.1 在 src/types.rs 中定义 Bar、Signal、Position、Portfolio、Trade、BacktestResult 结构体和枚举

## 3. 数据加载模块
- [x] 3.1 在 src/data.rs 中实现 load_bars_from_csv 函数
- [x] 3.2 实现 CSV 解析、字段校验、日期升序验证和错误处理

## 4. 策略模块
- [x] 4.1 在 src/strategy.rs 中定义 Strategy trait
- [x] 4.2 在 src/strategies/momentum.rs 中实现 MomentumStrategy
- [x] 4.3 在 src/strategies/mean_reversion.rs 中实现 MeanReversionStrategy
- [x] 4.4 在 src/strategies/mod.rs 中导出策略模块

## 5. 回测引擎
- [x] 5.1 在 src/portfolio.rs 中实现 Portfolio 状态管理（买入、卖出、净值计算）
- [x] 5.2 在 src/backtest.rs 中实现 run_backtest 函数，包含 T+1 成交规则

## 6. 绩效指标模块
- [x] 6.1 在 src/metrics.rs 中实现 calculate_metrics 函数（总收益率、年化收益率、最大回撤）

## 7. CLI 入口
- [x] 7.1 在 src/main.rs 中使用 clap 定义命令行参数
- [x] 7.2 实现主流程编排：参数解析 → 数据加载 → 策略构造 → 回测执行 → 结果输出

## 8. 测试
- [x] 8.1 编写 data loader 单元测试
- [x] 8.2 编写策略信号生成单元测试
- [x] 8.3 编写绩效指标计算单元测试
- [x] 8.4 端到端验证通过（cargo build + cargo test 12/12 + cargo run 两种策略均正常）
