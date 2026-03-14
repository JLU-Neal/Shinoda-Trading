## Context
Shinoda Trading 项目需要从零构建一个美股日线回测系统 MVP。当前项目为空白 Rust 项目，无任何已有代码。本设计文档记录核心技术决策，确保实现与规范一致。

## Goals / Non-Goals
- **Goals**:
  - 构建可运行的单标的日线回测系统
  - 支持动量策略和均值回归策略
  - 代码模块化，策略可插拔
  - 命令行驱动，无外部服务依赖
- **Non-Goals**:
  - 实盘交易接入
  - 多标的组合回测
  - 手续费/滑点模型
  - Web UI 或可视化

## Decisions

### 模块划分
- **Decision**: 按职责拆分为 data、types、strategy、strategies/、backtest、portfolio、metrics、main 共 8 个模块
- **Alternatives considered**: 单文件实现（过于耦合，不利于扩展）；按领域分 crate（MVP 阶段过度设计）
- **Rationale**: 单 crate 多模块在 MVP 阶段平衡了清晰度和简单性

### 策略接口设计
- **Decision**: 使用 `Strategy` trait 的动态分发（`&dyn Strategy`），策略不感知持仓状态
- **Alternatives considered**: 泛型静态分发（编译期确定策略类型）；策略感知持仓状态
- **Rationale**: 动态分发支持运行时通过 CLI 选择策略；策略不感知持仓可保持策略逻辑纯粹，由回测引擎统一决定是否执行交易

### 成交规则
- **Decision**: T 日收盘生成信号，T+1 日开盘价成交，全仓买入/全部清仓
- **Rationale**: 避免未来函数偏差（look-ahead bias），简化仓位管理逻辑

### 数据输入
- **Decision**: 仅支持本地 CSV 文件输入，symbol 由 CLI 参数传入
- **Alternatives considered**: 直接调用 API 获取数据
- **Rationale**: MVP 阶段优先离线可控，避免网络依赖和 API 限制

### 依赖选择
- **Decision**: csv + serde（数据解析）、chrono（日期处理）、clap（CLI 参数）
- **Rationale**: 均为 Rust 生态成熟稳定的 crate，社区广泛使用

## Risks / Trade-offs
- **单标的限制** → 后续扩展多标的时需重构 Portfolio 和回测引擎，但 MVP 阶段可接受
- **f64 精度** → 金融计算使用 f64 存在浮点精度问题 → 缓解：MVP 阶段可接受，后续可引入 rust_decimal
- **无手续费/滑点** → 回测结果偏乐观 → 缓解：在设计上预留扩展点，后续版本添加

## Migration Plan
不适用（全新项目，无需迁移）

## Open Questions
- 是否需要在 MVP 阶段支持交易明细导出？（当前决定：不支持，列为未来扩展点）
- 示例 CSV 数据的来源和时间范围？（建议：从 Yahoo Finance 下载 AAPL/MSFT/SPY 近 5 年日线数据）
