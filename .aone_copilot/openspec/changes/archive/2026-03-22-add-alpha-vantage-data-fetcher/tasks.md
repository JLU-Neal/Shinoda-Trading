## 1. 依赖与项目配置
- [x] 1.1 在 `Cargo.toml` 中添加 `reqwest`（blocking 特性）和 `serde_json` 依赖

## 2. 数据获取模块（可与 3 并行）
- [x] 2.1 创建 `src/fetcher.rs`，实现 `fetch_bars_from_alpha_vantage` 函数
- [x] 2.2 在 `src/fetcher.rs` 中实现 `save_bars_to_csv` 函数
- [x] 2.3 为 `fetcher.rs` 编写单元测试（JSON 解析、时间范围过滤、CSV 保存）

## 3. CLI 子命令重构
- [x] 3.1 将 `src/main.rs` 从扁平参数重构为子命令模式（`backtest` + `fetch`）
- [x] 3.2 实现 `fetch` 子命令的参数定义和执行流程
- [x] 3.3 确保 `backtest` 子命令保持原有行为不变

## 4. 集成验证
- [x] 4.1 编译通过，运行 `cargo test` 确保所有测试通过
- [x] 4.2 手动验证 `fetch` 子命令可正常获取数据（需要有效的 API Key）
