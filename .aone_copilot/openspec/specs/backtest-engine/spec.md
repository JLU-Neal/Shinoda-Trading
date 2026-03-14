# backtest-engine Specification

## Purpose
TBD - created by archiving change add-daily-bar-backtest-mvp. Update Purpose after archive.
## Requirements
### Requirement: 回测引擎核心流程
系统 SHALL 提供 `run_backtest(bars, strategy, initial_cash)` 函数，接收历史数据、策略实例和初始资金，按统一成交规则逐日执行回测，返回完整的 `BacktestResult`。

#### Scenario: 完整回测流程执行
- **WHEN** 提供有效的历史数据、策略实例和初始资金
- **THEN** 系统逐日遍历数据，生成信号并按规则执行交易，返回包含所有绩效指标的结果

#### Scenario: 空数据回测
- **WHEN** 提供的历史数据为空
- **THEN** 返回初始资金不变、无交易的回测结果

### Requirement: T+1 成交规则
系统 SHALL 遵循以下成交规则：第 `t` 天收盘后生成信号，第 `t+1` 天开盘价成交。若 `t+1` 不存在则忽略该信号。

#### Scenario: 信号在下一交易日执行
- **WHEN** 第 `t` 天策略生成 `Buy` 或 `Sell` 信号且第 `t+1` 天存在
- **THEN** 以第 `t+1` 天的开盘价执行交易

#### Scenario: 最后一天信号自动失效
- **WHEN** 最后一个交易日生成信号
- **THEN** 因无下一交易日数据，信号被忽略

### Requirement: 买入执行规则
系统 SHALL 在满足以下条件时执行买入：当前无持仓、信号为 `Buy`、下一交易日存在。买入股数为 `floor(cash / next_open)`，全仓买入。若现金不足以买入 1 股则忽略交易。

#### Scenario: 全仓买入
- **WHEN** 无持仓且信号为 `Buy` 且下一交易日存在
- **THEN** 以 `floor(cash / next_open)` 股数买入，现金减少，持仓增加

#### Scenario: 现金不足无法买入
- **WHEN** 现金不足以买入 1 股
- **THEN** 忽略该买入信号，账户状态不变

#### Scenario: 已有持仓时忽略买入
- **WHEN** 当前已有持仓且信号为 `Buy`
- **THEN** 忽略该信号，账户状态不变

### Requirement: 卖出执行规则
系统 SHALL 在满足以下条件时执行卖出：当前有持仓、信号为 `Sell`、下一交易日存在。卖出时全部清仓。

#### Scenario: 全部清仓
- **WHEN** 有持仓且信号为 `Sell` 且下一交易日存在
- **THEN** 以 `next_open` 价格卖出全部持仓，现金增加，持仓清零

#### Scenario: 无持仓时忽略卖出
- **WHEN** 当前无持仓且信号为 `Sell`
- **THEN** 忽略该信号，账户状态不变

### Requirement: 每日净值计算
系统 SHALL 每日计算净值：`equity = cash + position_shares * current_close`。若无持仓则 `equity = cash`。净值曲线记录每日净值。

#### Scenario: 有持仓时净值计算
- **WHEN** 当前持有股票
- **THEN** 净值等于现金加上持仓股数乘以当日收盘价

#### Scenario: 无持仓时净值计算
- **WHEN** 当前无持仓
- **THEN** 净值等于现金

### Requirement: 资金与仓位约束
系统 SHALL 遵循以下约束：初始资金默认 100000.0 USD；不允许负现金；不允许杠杆；不允许做空；同一时刻最多持有一个仓位。

#### Scenario: 资金约束验证
- **WHEN** 回测过程中执行任何交易
- **THEN** 现金余额始终 >= 0，且不存在做空或杠杆行为

### Requirement: Portfolio 领域模型
系统 SHALL 定义 `Portfolio` 结构体，包含 `cash: f64`、`position: Option<Position>`、`equity_curve: Vec<f64>` 字段。`Position` 包含 `shares: u32`、`entry_price: f64`。

#### Scenario: Portfolio 状态跟踪
- **WHEN** 回测过程中发生交易
- **THEN** Portfolio 的 cash、position 和 equity_curve 正确更新

### Requirement: Trade 领域模型
系统 SHALL 定义 `Trade` 结构体，包含 `date: NaiveDate`、`side: String`、`price: f64`、`shares: u32` 字段，记录每次成交。

#### Scenario: 交易记录完整
- **WHEN** 执行一次买入或卖出
- **THEN** 生成一条 Trade 记录，包含成交日期、方向、价格和股数

