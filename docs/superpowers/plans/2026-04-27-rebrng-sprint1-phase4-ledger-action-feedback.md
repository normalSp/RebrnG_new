# RebrnG Sprint 1 Phase 4：账本行动反馈实施计划

## Summary
Phase 4 的目标不是新增玩法规则，而是把 Sprint 1 Phase 2-3 已经存在的 Rust 规则投影整理成更清晰的“可玩账本交互”。本阶段只做行动分组、禁用原因、可见代价、风险/后果提示、最近结果反馈、线索页可读化和阶段收口展示。

本阶段不得新增规则、不得新增内容 YAML、不得修改存档结构、不得升级 `RULES_VERSION`。React 仍只消费 Rust 投影并提交 `ActionCommand`，不能持有完整 `GameState`，不能自行扣 AP、资源、暴露或债务。

## 上位依赖
- `DESIGN.md`：冷峻账本视觉合同、颜色/字体/密度/禁忌项。
- `2026-04-27-reborng-phase7-frontend-style-freeze-spec.md`：账本母界面、状态条、页签和信息优先级。
- `2026-04-27-reborng-sprint1-content-playability-outline.md`：Sprint 1 可玩性目标与 8 窗口内容体量。
- `2026-04-26-reborng-logical-architecture-design.md`：UI 只读账本投影，不持有玩法真相。
- `2026-04-26-reborng-technical-architecture-design.md`：Tauri invoke、single active run、Rust core 规则权威。

## Key Changes
- 扩展 Rust 投影字段，而不是让 React 猜规则。
- `ActionChoiceView` 新增 `group`、`tone`、`consequence_hint`。
- `LedgerViewModel` 新增 `recent_feedback` 与 `clue_view`。
- UI 行动区改为账本分组：遭遇决断、移动去处、修行资源、情报风声、恢复交易、等待阶段。
- 禁用行动应保留可见并显示中文原因；未解锁黑市继续隐藏，不显示禁用原因。
- 最近因果在正文页与因果账页前置展示，帮助玩家理解上一步代价。
- 线索页将 `KnowledgeState.known_clues` 转成中文风声账，不暴露隐藏变量原值。
- 阶段收口继续只读 Rust `StageClosureView`，UI 不自行判断结局。

## Public Interfaces
### `ActionChoiceView`
新增字段：

```text
group: ActionChoiceGroup
tone: ActionChoiceTone
consequence_hint: String
```

建议枚举：

```text
ActionChoiceGroup = encounter | movement | cultivation | information | recovery | trade | wait
ActionChoiceTone = normal | safe | risky | danger | blocked
```

字段语义：
- `group` 只负责 UI 分组，不参与规则结算。
- `tone` 只负责风险视觉表达，不改变行动可用性。
- `consequence_hint` 是 Rust 生成的短文本，例如“推进月光痕迹”“暴露上升”“可能压缩后续窗口”。

### `RecentFeedbackView`
最小字段：

```text
title: String
summary: String
tone: ActionChoiceTone
source_kind: String
```

来源规则：
- 从最新 `LedgerEntry` 派生。
- 不进入规则真相。
- 不保存进 `SaveEnvelope` 的新增结构，只作为投影派生。

### `ClueLedgerView`
最小字段：

```text
known_clues: Vec<ClueLineView>
blackmarket_access_summary: String
```

`ClueLineView` 最小字段：

```text
id: String
label: String
summary: String
tone: ActionChoiceTone
```

线索显示约束：
- 已知线索可显示中文可读账行。
- 未发现线索不得占位暗示。
- 黑市门路继续遵守“未解锁隐藏、已解锁可显示但受时段/AP/暴露规则约束”。

## Implementation Tasks
### 1. Rust 投影测试
先写失败测试：
- 行动投影必须包含 `group / tone / consequence_hint`。
- 遭遇决断、移动、修行、侦查、恢复、交易、等待能分入不同 `group`。
- AP 不足、元石不足、时段不合、遭遇阻断必须有中文禁用原因。
- 黑市未解锁前不显示黑市行动；已解锁但时段不合时显示可理解的禁用原因。
- `recent_feedback` 来自最新 ledger entry。
- `clue_view` 能展示 `rumor_blackmarket_tail / rumor_academy_pressure / rumor_infirmary_debt / rumor_alley_probe` 等已知线索。

### 2. Rust 投影实现
- 修改 `crates/game-core/src/lib.rs` 的投影结构和派生函数。
- 不改 `resolve_action` 规则语义。
- 不改 `GameState` 存档字段。
- 不升级 `RULES_VERSION`。

### 3. TypeScript 类型同步
- 修改 `packages/ui-ledger/src/index.ts`。
- 同步新增 `ActionChoiceGroup`、`ActionChoiceTone`、`RecentFeedbackView`、`ClueLedgerView`。
- 如果 Phase 3 已新增 `yield / argue / delay / frame`，确保 union 已覆盖。

### 4. 账本 UI 展示
- 修改 `packages/ui-ledger/src/LedgerShell.tsx`。
- 行动侧栏按 `group` 分节。
- 按 `tone` 显示普通、稳妥、风险、危险、阻断状态。
- 按钮显示中文短标签、代价、风险、后果提示和禁用原因。
- 正文页显示 `recent_feedback`。
- 因果账页前置最新一笔结果。
- 线索页展示 `clue_view`，不要直接渲染原始 clue id。

### 5. 桌面壳交互文案
- 修改 `apps/desktop/src/App.tsx`。
- 结算状态优先显示行动中文 label。
- `makeCommand(choice)` 写入 `context_note: choice.label`。
- React 仍只提交 `ActionCommand`，不自行计算成本和后果。

### 6. 冷峻账本样式
- 修改 `apps/desktop/src/App.css`。
- 增加行动分组、危险态、禁用原因、最近反馈、线索账行样式。
- 遵守 `DESIGN.md` 的纸墨、青灰、暗金、朱砂风险色。
- 禁止紫色 SaaS、营销页渐变、普通 dashboard 卡片堆、纯终端 UI、泛水墨皮肤。

## Test Plan
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo check -p rebrng-desktop`
- `cargo run -p rebrng-content-tools -- build-s0 --input content/s0 --output target/rebrng-content/s0.bundle.json`
- `pnpm -r build`
- 红线检索：无 Express 主链、无 runtime AI proposal/narrator、React 不持有完整 `GameState`、React 不读写存档、不扫描 YAML、不把蛊虫/杀招写成前端技能系统。

## Done Criteria
- 玩家能一眼区分遭遇决断、移动、修行、情报、恢复/交易和等待。
- 禁用原因不再像系统错误，而像账本提示。
- 新玩家能从按钮上的代价、风险、后果提示理解为什么不能“每回合做完所有事”。
- UI 没有新增玩法真相字段，所有可玩信息来自 Rust 投影。
- Phase 4 完成后提交并推送当前分支，提交信息使用中文：`feat: 打磨 S0 账本行动反馈`。

## Assumptions
- Phase 4 默认基于 Phase 3 已完成。
- 不处理 `apps/desktop/src/assets/` 与 `docs/art/` 未跟踪目录。
- 不新增遭遇类型、不新增路线内容、不做完整蛊虫炼化或杀招系统。
- 不创建 PR，不合并 `main`，只推送 `codex/sprint-0-foundation`。
