# RebrnG 技术架构设计 v1.2

状态：frozen

更新时间：2026-04-27

关联文档：
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-architecture-input-checklist.md](./2026-04-26-reborng-architecture-input-checklist.md)
- [2026-04-26-reborng-launch-vertical-slice-spec.md](./2026-04-26-reborng-launch-vertical-slice-spec.md)

适用范围：新项目 `D:\workspace\CodeBuddyWorkSpace\RebrnG_new`

## 文档角色

本文档把正式逻辑架构翻译成首发可实现的技术架构。

本文档冻结：

- 技术栈
- 包结构
- Rust 核心边界
- Tauri invoke 接口
- Command 包络
- Tauri 运行态托管
- 行动管线实现形态
- 内容资产构建格式
- 存档格式
- 性能红线
- 测试边界

本文档不新增玩法。

## 技术栈基线

首发技术栈采用：

- 规则核心：Rust
- 桌面壳：Tauri
- 前端：React + TypeScript
- 前端构建：Vite
- 包管理：pnpm，仅服务前端、构建脚本和开发工具
- Rust 测试：cargo test
- 前端测试：Vitest 或等价前端测试工具
- 内容源格式：YAML
- 运行时内容格式：索引化 JSON bundle，例如 `s0.bundle.json`
- 存档格式：JSON，带 `schemaVersion`

选择理由：

- Rust 负责规则、内容 bundle、索引、存档迁移和性能敏感路径，避免旧项目每回合等待 Node API 与远程 AI。
- React 适合账本式 UI 的多页投影，但不拥有规则真相。
- Tauri 满足 PC 单机优先，并允许 UI 通过 `invoke` 直接调用本地 Rust 命令。
- TypeScript 只服务 UI、交互层和内容构建辅助，不承担 `GameState` 真相源。
- YAML 适合中文长文本和内容作者维护，运行时只读取已校验、已索引的 JSON bundle。

## 环境前置

底座开发开始前必须满足：

- Rust stable toolchain 通过 `rustup` 安装。
- `cargo`、`rustc` 必须可在项目 shell 中访问。
- `pnpm` 通过 Corepack 启用，不要求全局手工安装。
- Windows Tauri 前置依赖必须在脚手架前安装完成。
- 实际 scaffold 时用 `rust-toolchain.toml` 固化 Rust 通道，用 `packageManager` 与 lockfile 固化前端包管理器版本。

当前文档不硬钉 Rust patch 号和 pnpm patch 号；版本锁定交给实际底座工程文件。

首发不采用：

- Web 试玩壳作为架构约束。
- Express / 本地 HTTP API 驱动回合。
- 每回合实时调用远程 LLM 生成事件或正文。
- 运行时扫描原始 YAML、JSONL、原著语料或大体量 lore 中间产物。

## Workspace 结构

建议结构：

```text
apps/
  desktop/
crates/
  game-core/
  content-tools/
content/
  s0/
packages/
  ui-ledger/
docs/
```

### apps/desktop

职责：

- Tauri 桌面入口
- React UI 挂载
- Tauri command 注册
- 本地文件权限声明
- 桌面打包

禁止：

- 承载游戏规则。
- 绕过 Rust core 直接改存档。
- 起本地 HTTP API 作为回合主链。

### crates/game-core

职责：

- `GameState`
- `ActionCommand`
- `ActionResult`
- `ContentBundle`
- `SaveEnvelope`
- 行动结算管线
- 规则模块
- 内容门禁
- 账本投影构建
- 存档序列化、反序列化和迁移
- 性能计时与 S0 验收脚本

这是项目唯一规则真相源。

### crates/content-tools

职责：

- 读取 `content/**/*.yaml`
- 使用 `crates/game-core` 中定义的内容类型和 schema 语义
- 校验证据等级、模式许可、标签和阶段
- 构建索引化 `s0.bundle.json`
- 输出内容 manifest、bundle version 和构建报告

这是内容构建的唯一权威入口。

禁止：

- 在 pnpm / TypeScript 脚本中另造一套内容 schema 真相。
- 在运行时扫描 YAML 或 JSONL 来补充 bundle。
- 把原著全文或大体量 lore 中间产物放进运行时 bundle 构建链路。

### content/s0

职责：

- 固定开局 YAML
- 8 回合窗口表 YAML
- 青茅山节点、事件、锚点、路线入口 YAML
- canon / IF 内容标记
- 内容构建输入

禁止：

- 把原始原著全文或超大 JSONL 中间产物放入运行时读取路径。
- 让运行时按目录扫描内容源文件。

### packages/ui-ledger

职责：

- 账本式 React 组件
- `LedgerViewModel` 展示
- UI 临时状态
- 将玩家输入转成 Tauri invoke 参数

禁止：

- 直接持有或修改完整 `GameState`。
- 读取隐藏变量原值。
- 生成规则奖励、状态 patch 或锚点事实。

## Rust 核心类型边界

### GameState

`GameState` 是完整规则快照。

至少包含：

- `run`
- `time`
- `world`
- `character`
- `resources`
- `debts_and_credit`
- `build`
- `assets`
- `aperture`
- `trade`
- `knowledge`
- `anchors`

禁止把这些状态压进 `misc`、单一 `current_region` 或大一统 `inventory`。

### ActionCommand

所有玩家行动统一进入 Rust：

```text
resolve_action(command, state, content_bundle, mode_policy) -> ActionResponse
```

最小字段：

- `actor_id`
- `intent`
- `target`
- `declared_cost`
- `context`

`intent` 首发至少覆盖：

- `move`
- `cultivate`
- `work_merit`
- `seek_treatment`
- `investigate`
- `trade_blackmarket`
- `encounter_response`
- `rest_or_recover`

中后期预留：

- `manage_aperture`
- `trade_treasure_yellow_heaven`
- `deploy_formation`
- `activate_gu_house`

### ActionResponse

Tauri 返回给 UI 的最小结果：

- `accepted`
- `visible_outcome`
- `state_summary`
- `ledger_entries`
- `hidden_signals`
- `projection`
- `performance`

UI 默认只消费 `projection`、`visible_outcome`、`ledger_entries` 和必要的 `state_summary`。

### ContentBundle

运行时只读取构建后的 `ContentBundle`。

最小字段：

- `bundle_id`
- `bundle_version`
- `schema_version`
- `content_assets`
- `indices`
- `mode_policies`
- `anchor_policies`
- `manifest`

`content_assets` 至少保留：

- `content_id`
- `stage`
- `region`
- `tags`
- `evidence_level`
- `mode_policy`
- `visibility`
- `effect_template`
- `anchor_impact`

`evidence_level` 至少支持：

- `canon_explicit`
- `canon_inferred`
- `if_allowed`
- `original_playable`
- `rumor_only`

### SaveEnvelope

存档对象由 Rust 负责。

最小字段：

- `metadata`
- `snapshot`
- `ledger`
- `checkpoints`
- `rng_state`
- `migration_state`

存档文件必须带 `schemaVersion`、内容包版本和规则版本。

## Tauri Invoke 接口

UI 与规则核心只通过 Tauri commands 通信。

首发最小 commands：

- `create_run(input) -> ActionResponse`
- `resolve_action(command) -> ActionResponse`
- `load_save(slot_id) -> ActionResponse`
- `write_save(slot_id) -> SaveWriteResult`
- `build_projection() -> LedgerViewModel`
- `get_content_manifest() -> ContentManifest`

Command 包络：

- 成功时返回 `ActionResponse`、`LedgerViewModel`、`SaveWriteResult` 或 `ContentManifest`。
- 失败时返回明确错误对象，不返回裸字符串。
- 错误分类至少覆盖：`validation`、`content`、`save`、`io`、`internal`。
- 错误对象必须包含可展示摘要和开发诊断字段。
- 错误对象不得泄露隐藏变量原值。

约束：

- `resolve_action` 是玩家行动主入口。
- commands 内部可以持有当前运行状态，但状态写入必须经过 Rust core 的统一管线。
- `build_projection` 只能从当前状态构建展示模型，不能产生规则副作用。
- `load_save` 必须校验 `schemaVersion`、规则版本和内容包版本。
- `write_save` 只能写入 `SaveEnvelope`，不能写 UI 临时状态。

### Tauri 运行态托管

Sprint 0 只支持单个 active run。

运行态由 Tauri / Rust 托管：

- 当前 `GameState`
- 当前 `ContentBundle`
- 当前 `CausalityLedger`
- 当前 RNG 状态
- 当前存档 slot 指针

React 只保存 UI 临时状态，例如选中的账页、展开面板和输入草稿。

首发底座不实现：

- 多 run 并行管理
- 多存档列表 UI
- 跨窗口同步
- 服务端式 session 管理

禁止：

- React 直接读写存档文件。
- React 直接构造状态 patch。
- Tauri command 调用远程 LLM 后再返回 `resolve_action`。
- 在 command 中临时扫描内容目录来决定回合结果。

## 行动管线实现

Rust core 提供唯一规则入口：

```text
resolve_action(command, state, content_bundle, mode_policy) -> ActionResult
```

内部顺序固定：

1. `availability_check`
2. `cost_reservation`
3. `subsystem_resolution`
4. `anchor_recalculation`
5. `effect_commit`
6. `ledger_append`
7. `projection_refresh`

每一步都应可单测。

禁止：

- UI 直接调用子系统改状态。
- 子系统绕过 `effect_commit` 改全局状态。
- narrative / 文本层生成 `state_patch`。
- 远程 LLM 参与同步结算。

## 内容数据流

内容进入运行时前必须经过：

```text
content/**/*.yaml
  -> crates/content-tools
  -> schema 校验
  -> 证据等级校验
  -> 模式许可校验
  -> 标签和阶段校验
  -> 索引构建
  -> s0.bundle.json
```

运行时只消费 `ContentBundle`。

`crates/content-tools` 必须复用 `crates/game-core` 内容类型，不得在构建器中发明另一套字段语义。

首发最小内容包必须包含：

- 固定开局
- 8 回合窗口表
- 青茅山核心节点
- 月光、功绩、药堂、黑市、传承入口级内容
- 至少一次危险遭遇
- 至少一次跑路优于硬打的场景
- 阶段性成功和失败结果

性能约束：

- 启动时加载 bundle。
- 回合中只查索引。
- 不扫描原始内容源。
- 不读取 `100MB+` lore 中间文件。

## 账本 UI 数据流

UI 数据流：

```text
React UI
  -> Tauri invoke(ActionCommand)
  -> Rust game-core
  -> ActionResponse / LedgerViewModel
  -> React UI
```

UI 只允许提交 `ActionCommand`。

UI 不允许：

- 修改 `GameState`
- 读取隐藏变量原值
- 现场生成规则奖励
- 改写内容证据等级
- 调用远程 AI 阻塞回合

## 叙事与 AI 边界

首发运行时正文来源：

- 策展内容模板
- 账本投影
- Rust core 提供的可见因果与后果摘要

AI / LLM 允许用于：

- 离线内容候选生成
- 离线风格润色
- 离线 canon 审查辅助
- 后台非阻塞生成草稿

AI / LLM 禁止用于：

- `resolve_action` 同步链路
- 关键奖励生成
- 原著硬事实生成
- 存档状态生成
- 内容证据等级改写
- 玩家点击后必须等待的正文生成

如果 AI 不可用、断网或超时，S0 8 回合必须仍可完整游玩。

## 存档与迁移

存档由 Rust core 负责。

最低能力：

- 创建阶段存档
- 读取阶段存档
- 序列化 `SaveEnvelope`
- 反序列化 `SaveEnvelope`
- 校验 `schemaVersion`
- 校验内容包版本
- 保存和恢复 RNG 状态
- 执行未来迁移或返回明确不可迁移错误

首发必须测试：

- 同一存档读回后 AP、节点、债务、Build、暴露和账本一致。
- 内容包版本不一致时给出明确错误或迁移提示。

## 性能红线

首发必须满足：

- 单次 `resolve_action` 规则结算目标 `<300ms`。
- 下一回合可交互目标 `<1s`。
- bundle 启动加载可被计时并记录。
- 存档读写可被计时并记录。
- 8 回合验收脚本输出每回合耗时。

`performance` 至少保留以下计时口径：

- `resolve_action_ms`
- `projection_ms`
- `save_load_ms`
- `bundle_load_ms`

Rust command 内部使用本地计时采集这些指标，并随 `ActionResponse` 或相关返回对象传回 UI / 测试脚本。

禁止把性能问题解释为“文本生成慢所以正常”。

旧项目教训：

- 不允许玩家每回合等待 AI proposal。
- 不允许玩家每回合等待 AI narrator。
- 不允许两次 `45s` 级远程超时串联进入主链。
- 不允许运行时读取大体量原始 lore 文件来临时构建回合事实。

## 测试架构

测试分五层：

- Rust 类型和 schema 测试
- Rust 规则模块单测
- Rust 行动管线集成测试
- Tauri command 边界测试
- S0 8 回合场景与性能测试

首发必须有以下测试：

- `resolve_action(move)` 不把移动写成固定 AP 税。
- `canon_strict` 拒绝缺证据关键内容。
- `sandbox_if` 启用 IF 内容但仍受门禁限制。
- 战败可进入重创可续。
- Build 分层不混写求活路线和主修流派。
- `SaveEnvelope` 读写后关键状态一致。
- UI projection 不包含隐藏变量原值。
- 断网或无 AI 配置时仍能跑完 S0 8 回合。
- S0 8 回合性能报告中单回合无分钟级等待。

## 技术架构禁止项

禁止：

- 把全部状态塞进一个前端全局 store 且无领域边界。
- 把内容资产直接写在 React 组件里。
- 把 UI 操作写成直接改状态。
- 把 `canon_strict` 和 `sandbox_if` 写成两套状态结构。
- 把存档当成本地缓存，不做 schema version。
- 在首发实现中提前实现完整 S1-S4。
- 保留 Web Vite 试玩壳作为首发架构约束。
- 保留 Express / 本地 HTTP API 作为回合主链。
- 让远程 AI 参与同步回合结算。

## 首发实现边界

首发技术实现只做：

- S0 固定开局
- 8 回合最小闭环
- Rust 核心状态类型
- Rust 行动管线
- YAML 到 JSON bundle 的最小内容构建
- Tauri invoke commands
- 账本式 UI 投影
- 阶段存档
- 验收测试和性能计时

只预留，不实现完整：

- 仙窍经营细节
- 宝黄天完整交易盘
- 阵和仙蛊屋完整部署
- 跨五域完整旅行
- 尊者级 IF

## 归档说明

当前归档路径：

`docs/superpowers/specs/2026-04-26-reborng-technical-architecture-design.md`
