# RebrnG 首发纵切实现计划 v1.2

状态：frozen

更新时间：2026-04-27

关联文档：
- [2026-04-26-reborng-launch-vertical-slice-spec.md](./2026-04-26-reborng-launch-vertical-slice-spec.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)
- [2026-04-27-reborng-phase7-frontend-style-freeze-spec.md](./2026-04-27-reborng-phase7-frontend-style-freeze-spec.md)
- [2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md](./2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md)

适用范围：新项目 `D:\workspace\CodeBuddyWorkSpace\RebrnG_new`

## 文档角色

本文档把首发纵切拆成第一轮可执行开发任务。

目标不是完成正式版，而是跑通 S0 青茅山 8 回合最小闭环，并验证 Rust 核心增强版技术底座不会复刻旧项目每回合分钟级等待。

## 纵切目标

第一轮原型必须证明：

- 固定开局可以进入 8 个自由回合。
- 所有行动都通过 Rust `ActionCommand` 管线结算。
- UI 只通过 Tauri invoke 提交行动并读取账本投影。
- `canon_strict` 与 `sandbox_if` 使用同一状态结构。
- 阶段存档读回后状态和账本一致。
- 运行时 AI 不进入 `resolve_action`、Tauri command 或 UI 点击主链。
- 单次行动结算目标 `<300ms`，下一回合可交互目标 `<1s`。

## 阶段 0：项目骨架

任务：

- 安装或验证 Rust stable toolchain。
- 验证 `rustc` 与 `cargo` 可用。
- 通过 Corepack 启用 `pnpm`。
- 安装或验证 Windows Tauri 前置依赖。
- 建立 `apps/desktop` Tauri 桌面入口。
- 建立 `crates/game-core` Rust crate。
- 建立 `crates/content-tools` Rust crate。
- 建立 `packages/ui-ledger` React 账本 UI 包。
- 建立 `content/s0` 内容源目录。
- 建立基础 Cargo workspace。
- 建立 pnpm workspace，仅服务前端、Tauri 前端构建和内容构建辅助。
- 配置 `cargo test`。
- 配置前端 `typecheck` 与 Vitest 或等价测试命令。
- 不建立 `apps/web-dev`。
- 不建立 Express / 本地 HTTP API。

验收：

- `rustc --version` 可返回版本。
- `cargo --version` 可返回版本。
- `pnpm --version` 可返回版本。
- `cargo test` 可运行空测试。
- 前端 typecheck 可运行。
- Tauri 桌面壳可启动到空账本界面。
- 项目内没有回合主链 HTTP API。

## 阶段 1：Rust 核心状态类型

任务：

- 在 Rust 中定义 `GameState`。
- 定义 `RunState`、`TimeState`、`WorldSpaceState`、`CharacterState`。
- 定义 `ResourceState`、`DebtAndCreditState`、`BuildState`。
- 定义 `AssetOwnershipState`、`ApertureState`、`TradeState`。
- 定义 `KnowledgeState`、`AnchorState`。
- 定义 `ContentBundle`。
- 定义 `SaveEnvelope`。
- 为 Tauri / JSON 序列化准备稳定字段名。

验收：

- 类型能表达 S0 必需状态。
- 类型保留 S1-S4 关键字段或占位。
- 没有 `misc`、单一 `current_region`、大一统 `inventory`。
- `canon_strict` 与 `sandbox_if` 不分裂成两套状态。

## 阶段 2：ActionCommand 管线

任务：

- 定义 Rust `ActionCommand`。
- 定义 Rust `ActionResult` 与 Tauri 返回用 `ActionResponse`。
- 实现 `resolve_action` 管线。
- 实现七步管线骨架：
  - `availability_check`
  - `cost_reservation`
  - `subsystem_resolution`
  - `anchor_recalculation`
  - `effect_commit`
  - `ledger_append`
  - `projection_refresh`
- 让规则模块只能返回状态变更和账本条目。

验收：

- UI 或测试只能通过 `resolve_action` 改规则状态。
- 子系统不能直接修改全局状态。
- 管线每一步都有最小 Rust 单测。
- 管线不接受远程 AI provider 作为依赖。

## 阶段 3：内容源与 Bundle 构建

任务：

- 在 `crates/content-tools` 中建立内容构建入口。
- 让 `crates/content-tools` 复用 `crates/game-core` 内容类型。
- 定义 S0 YAML 内容 schema。
- 编写固定开局 YAML。
- 编写 8 回合窗口表 YAML。
- 编写青茅山核心节点 YAML。
- 编写月光、功绩、药堂、黑市、传承入口级内容 YAML。
- 实现 YAML 校验。
- 实现证据等级、模式许可、标签、阶段校验。
- 构建索引化 `s0.bundle.json`。
- Rust 运行时只加载 `s0.bundle.json`。

验收：

- `canon_strict` 缺证据关键内容无法进入 bundle 或无法运行。
- `sandbox_if` 内容带明确模式许可。
- bundle 构建输出 manifest、版本号和索引。
- 回合中不扫描 YAML、JSONL 或原始语料。
- pnpm 只负责调用内容构建命令，不拥有 schema 真相。

## 阶段 4：Tauri Invoke 边界

任务：

- 建立 Sprint 0 单 active run 托管模型。
- 实现 `create_run(input) -> ActionResponse`。
- 实现 `resolve_action(command) -> ActionResponse`。
- 实现 `load_save(slot_id) -> ActionResponse`。
- 实现 `write_save(slot_id) -> SaveWriteResult`。
- 实现 `build_projection() -> LedgerViewModel`。
- 实现 `get_content_manifest() -> ContentManifest`。
- 在 Tauri state 中托管当前 run、bundle 和存档服务。
- 为 commands 定义统一错误对象，错误分类覆盖 validation、content、save、io、internal。

验收：

- Sprint 0 只管理一个 active run。
- 不实现多 run 并行管理。
- 不实现多存档列表 UI。
- React 不能直接改 `GameState`。
- React 不能直接读写存档文件。
- `build_projection` 无规则副作用。
- commands 不调用远程 LLM。
- command 错误对象可被 UI 展示，且不泄露隐藏变量原值。

## 阶段 5：S0 时间、地图与经济

任务：

- 实现四时段。
- 实现自由窗口、锚点窗口、AP 刷新。
- 实现青茅山最小节点图。
- 实现移动成本，不使用固定 AP 税。
- 实现元石、材料、功绩、债务、人情。
- 实现暴露区间。

验收：

- 玩家可在节点间移动。
- 移动可产生窗口损耗、暴露变化或到达后 AP 压缩。
- 前 6-8 回合出现资源和时间取舍。
- 单次移动结算计时输出 `<300ms` 目标数据。

## 阶段 6：遭遇、恢复与 Build 痕迹

任务：

- 实现一次危险遭遇。
- 实现侦查和部分情报。
- 实现跑路、拖延、求饶或换场。
- 实现重创可续。
- 实现求活路线痕迹。
- 实现主修流派 / 道痕占位。
- 实现 Build 缺口显示。

验收：

- 至少一次局面中跑路优于硬打。
- 战败可进入伤势、债务、暴露或路线断裂。
- Build 页能显示玩家靠什么活和主要缺口。
- 求活路线不与主修流派 / 道痕混成一个字段。

## 阶段 7：账本式 UI 投影

前置冻结依赖：

- Phase 7 必须遵循根目录 `DESIGN.md` 与 `2026-04-27-reborng-phase7-frontend-style-freeze-spec.md` 的“冷峻账本”视觉合同。
- Phase 7 的 Build、修行、蛊虫相关投影必须遵循 `2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md` 的状态合同，不得临时把蛊虫写成普通装备或把杀招写成技能树。

任务：

- 在 Rust 中实现 `build_projection`。
- 在 React 中实现正文场景页。
- 实现节点地图页。
- 实现物资与债务页。
- 实现关系局势页占位。
- 实现空窍 / 修行页。
- 实现 Build 页。
- 实现风声与线索页。
- UI 所有行动按钮只提交 Tauri invoke。
- 修复当前 UI 源码中的中文乱码，保持 UTF-8 文本稳定。
- 建立冷峻账本基础 token，不使用紫色 SaaS、泛营销渐变、纯终端 UI 或纯水墨皮肤。

验收：

- 顶层状态条始终显示时段、窗口、AP、节点、暴露、债务压力。
- 顶层状态条同时显示伤势与当前遭遇压力。
- Build 页至少显示核心蛊 / 辅助蛊占位、求活路线、主修流派保留位、喂养维护压力与主要缺口。
- UI 不读取隐藏变量原值。
- UI 只提交 `ActionCommand`。
- UI 不调用 AI 生成正文。
- 中文正文、标签和中英文 ID 混排无乱码。
- 页面组看起来属于同一本账，而不是普通 dashboard 卡片拼贴。

## 阶段 8：阶段存档

任务：

- Rust 实现 `SaveEnvelope` 序列化。
- Rust 实现 `SaveEnvelope` 反序列化。
- 保存规则版本。
- 保存内容包版本。
- 保存 RNG 状态。
- 保存因果账本。
- 保存阶段检查点。
- 通过 Tauri 文件权限写入本地存档。

验收：

- 读档后 AP、节点、债务、Build、暴露和账本一致。
- 内容包版本不一致时出现明确错误或迁移提示。
- React 不直接读写存档文件。

## 阶段 9：AI 断链与本地叙事兜底

任务：

- 明确首发运行时不接远程 LLM。
- 正文使用策展模板、内容 bundle 和账本投影。
- AI 只保留为离线内容候选或后台草稿工具的未来接口。
- 不在 `resolve_action`、Tauri commands 或 UI 点击链路中等待 AI。

验收：

- 删除或不创建 runtime AI provider。
- AI 缺失、无 key、无模型配置不作为玩家功能验收项；验收只检查运行时没有 AI provider、proposal 或 narrator 进入回合主链。
- 没有 proposal / narrator 远程请求参与回合主链。

## 阶段 10：8 回合验收剧本与性能脚本

任务：

- 编写固定开局。
- 编写 8 回合窗口表。
- 放入月光、功绩、药堂、黑市、传承入口。
- 放入至少一次危险遭遇。
- 放入至少一种阶段成功。
- 放入至少一种重创可续失败。
- 编写 S0 8 回合性能脚本。
- 记录每回合 `resolve_action`、projection、save/load 耗时。
- 输出 `resolve_action_ms`、`projection_ms`、`save_load_ms`、`bundle_load_ms`。

验收：

- 一局可跑完 8 个自由回合。
- 至少两次资源取舍有真实意义。
- 四主路线出现可感知分化。
- 玩家不能在每个锚点前把所有事都做完。
- 性能报告中没有分钟级等待。
- 单次行动结算目标 `<300ms`，下一回合可交互目标 `<1s`。
- 性能字段来自 Rust command 本地计时，不由 React 估算。

## 不进入第一轮原型的内容

第一轮不实现：

- 完整多开局。
- Web 试玩壳。
- Express / 本地 HTTP API。
- 运行时 AI proposal。
- 运行时 AI narrator。
- 完整跨图旅行。
- 完整升仙。
- 完整仙窍经营。
- 完整宝黄天交易。
- 完整阵和仙蛊屋。
- 尊者级 IF。
- 完整数值平衡。

这些只保留架构接口。

## 测试矩阵

最低测试：

- Rust `resolve_action(move)` 场景测试。
- Rust `resolve_action(cultivate)` 场景测试。
- Rust `resolve_action(seek_treatment)` 场景测试。
- Rust `resolve_action(encounter_response)` 场景测试。
- Rust `canon_strict` 内容门禁测试。
- Rust `sandbox_if` 内容门禁测试。
- Rust `SaveEnvelope` 往返测试。
- Tauri command 边界测试。
- React UI projection 展示测试。
- 8 回合闭环集成测试。
- 8 回合性能测试。
- 运行时 AI 主链红线检索。

## 完成定义

首发纵切原型完成必须同时满足：

- 可从固定开局进入 8 回合。
- 可通过 React UI 提交行动。
- 所有行动通过 Tauri invoke 进入 Rust 统一管线。
- 账本页能解释主要压力。
- 存档读回一致。
- 运行时 AI 不阻塞玩家行动主链。
- 测试覆盖核心闭环。
- 性能验收没有分钟级等待。

## 归档说明

当前归档路径：

`docs/superpowers/specs/2026-04-26-reborng-launch-vertical-slice-implementation-plan.md`

## Phase 7 本命蛊补充验收

Phase 7 Build / 修行投影必须显式显示 `本命蛊：未建立`。

该字段仅为首发保留位，不开放获取、绑定、改命或替换玩法；实现者不得把本命蛊与核心蛊、辅助蛊、求活路线或杀招按钮合并成同一字段。

新增验收：

- Build 页至少显示核心蛊 / 辅助蛊占位、本命蛊占位、求活路线、主修流派保留位、喂养维护压力与主要缺口。
- `canon_strict` 下不会生成缺证据的本命蛊绑定事件。
- UI 文案不得暗示 S0 可刷出本命蛊。
