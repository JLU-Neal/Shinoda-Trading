# Change: 添加美股日线回测系统 MVP

## Why
Shinoda Trading 需要一个最小可用的回测系统，用于对美股历史日线数据执行基础交易策略回测。当前项目尚无任何回测能力，本变更将从零构建核心回测框架，支持动量策略与均值回归策略，为后续策略研发和实盘交易奠定基础。

## What Changes
- 新增 CSV 数据加载模块，支持从本地文件读取美股日线 OHLCV 数据
- 新增可插拔策略接口（Strategy trait），解耦策略与回测引擎
- 实现两种内置策略：动量策略（Momentum）和均值回归策略（MeanReversion）
- 新增回测引擎，按 T 日收盘生成信号、T+1 日开盘成交的规则执行交易
- 新增绩效指标计算模块，输出总收益率、年化收益率、最大回撤等指标
- 新增 CLI 入口，支持通过命令行参数指定股票、数据文件、策略及策略参数
- 定义核心领域模型：Bar、Signal、Position、Portfolio、Trade、BacktestResult

## Impact
- 影响的规范：data-loader、strategy、backtest-engine、metrics、cli
- 影响的代码：新建完整 Rust 项目结构，包括 Cargo.toml、src/ 下所有模块、data/ 示例数据目录
