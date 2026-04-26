# RebrnG 首发纵切实现计划 v1.0

状态：frozen

更新时间：2026-04-26

关联文档：
- [2026-04-26-reborng-launch-vertical-slice-spec.md](./2026-04-26-reborng-launch-vertical-slice-spec.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)

适用范围：新项目 `D:\workspace\CodeBuddyWorkSpace\RebrnG_new`

## 文档角色

本文档把首发纵切拆成第一轮可执行开发任务。

目标不是完成正式版，而是跑通 S0 青茅山 8 回合最小闭环。

## 纵切目标

第一轮原型必须证明：

- 固定开局可以进入 8 个自由回合。
- 所有行动都通过 `ActionCommand` 管线结算。
- UI 只读取账本投影。
- `canon_strict` 与 `sandbox_if` 使用同一状态结构。
- 阶段存档读回后状态和账本一致。

## 阶段 0：项目骨架

任务：

- 建立 pnpm workspace。
- 建立 `apps/web-dev`。
- 建立 `apps/desktop` 占位。
- 建立 `packages/game-core`。
- 建立 `packages/content`。
- 建立 `packages/ui-ledger`。
- 建立 `packages/save`。
- 建立 `packages/narrative`。
- 配置 TypeScript、Vitest、基础 lint 或 typecheck 命令。

验收：

- `pnpm install` 可完成。
- `pnpm test` 可运行空测试。
- `pnpm typecheck` 可运行。
- Web dev 壳可启动到空账本界面。

## 阶段 1：核心状态类型

任务：

- 定义 `GameState`。
- 定义 `RunState`、`TimeState`、`WorldSpaceState`、`CharacterState`。
- 定义 `ResourceState`、`DebtAndCreditState`、`BuildState`。
- 定义 `AssetOwnershipState`、`ApertureState`、`TradeState`。
- 定义 `KnowledgeState`、`AnchorState`。
- 定义 `SaveEnvelope`。

验收：

- 类型能表达 S0 必需状态。
- 类型保留 S1-S4 关键字段或占位。
- 没有 `misc`、单一 `currentRegion`、大一统 `inventory`。

## 阶段 2：ActionCommand 管线

任务：

- 定义 `ActionCommand`。
- 定义 `ActionResult`。
- 实现 `resolveAction` 管线。
- 实现空的七步管线骨架。
- 让规则模块只能返回 patch 和 ledger entries。

验收：

- UI 或测试只能通过 `resolveAction` 改规则状态。
- 子系统不能直接修改全局状态。
- 管线每一步都有最小测试。

## 阶段 3：S0 时间、地图与经济

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

## 阶段 4：遭遇、恢复与 Build 痕迹

任务：

- 实现一次危险遭遇。
- 实现侦查和部分情报。
- 实现跑路、拖延、求饶或换场。
- 实现重创可续。
- 实现求活路线痕迹。
- 实现 Build 缺口显示。

验收：

- 至少一次局面中跑路优于硬打。
- 战败可进入伤势、债务、暴露或路线断裂。
- Build 页能显示玩家靠什么活和主要缺口。

## 阶段 5：内容包与模式门禁

任务：

- 定义 `ContentAsset` schema。
- 建立最小 S0 内容包。
- 实现证据等级。
- 实现 `canon_strict` 门禁。
- 实现 `sandbox_if` 门禁。

验收：

- `canon_strict` 拒绝缺证据关键内容。
- `sandbox_if` 可启用 IF 内容，但仍受阶段、空间、资源、Build 和主线保护限制。
- 实时文本或 narrative 层不能生成规则奖励。

## 阶段 6：账本式 UI 投影

任务：

- 实现 `buildProjection`。
- 实现正文场景页。
- 实现节点地图页。
- 实现物资与债务页。
- 实现关系局势页占位。
- 实现空窍 / 修行页。
- 实现 Build 页。
- 实现风声与线索页。

验收：

- 顶层状态条始终显示时段、窗口、AP、节点、暴露、债务压力。
- UI 不读取隐藏变量原值。
- UI 只提交 `ActionCommand`。

## 阶段 7：阶段存档

任务：

- 实现 `SaveEnvelope` 序列化。
- 实现 `SaveEnvelope` 反序列化。
- 保存内容包版本。
- 保存 RNG 状态。
- 保存因果账本。
- 保存阶段检查点。

验收：

- 读档后 AP、节点、债务、Build、暴露和账本一致。
- 内容包版本不一致时出现明确错误或迁移提示。

## 阶段 8：8 回合验收剧本

任务：

- 编写固定开局。
- 编写 8 回合窗口表。
- 放入月光、功绩、药堂、黑市、传承入口。
- 放入至少一次危险遭遇。
- 放入至少一种阶段成功。
- 放入至少一种重创可续失败。

验收：

- 一局可跑完 8 个自由回合。
- 至少两次资源取舍有真实意义。
- 四主路线出现可感知分化。
- 玩家不能在每个锚点前把所有事都做完。

## 不进入第一轮原型的内容

第一轮不实现：

- 完整多开局。
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

- `resolveAction(move)` 场景测试。
- `resolveAction(cultivate)` 场景测试。
- `resolveAction(seek_treatment)` 场景测试。
- `resolveAction(encounter_response)` 场景测试。
- `canon_strict` 内容门禁测试。
- `sandbox_if` 内容门禁测试。
- SaveEnvelope 往返测试。
- 8 回合闭环集成测试。

## 完成定义

首发纵切原型完成必须同时满足：

- 可从固定开局进入 8 回合。
- 可通过 UI 提交行动。
- 所有行动通过统一管线。
- 账本页能解释主要压力。
- 存档读回一致。
- 测试覆盖核心闭环。

## 归档说明

当前归档路径：

`docs/superpowers/specs/2026-04-26-reborng-launch-vertical-slice-implementation-plan.md`
