# RebrnG 技术架构设计 v1.0

状态：frozen

更新时间：2026-04-26

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
- 核心类型边界
- 行动管线实现形态
- 内容资产格式
- 存档格式
- 测试边界

本文档不新增玩法。

## 技术栈基线

首发技术栈采用：

- 语言：TypeScript
- 运行时：Node.js LTS
- 包管理：pnpm workspace
- 前端：React
- 桌面壳：Tauri
- 开发试玩壳：Web Vite
- 测试：Vitest
- 内容格式：YAML 或 JSON，进入运行时前转为 typed content bundle
- 存档格式：JSON，带 schema version

选择理由：

- TypeScript 适合表达大量状态对象和管线类型。
- React 适合账本式 UI 的多页投影。
- Tauri 满足 PC 单机优先，不把首发压成 Web 小游戏。
- Vite Web 壳便于快速开发和内部试玩。
- Vitest 可覆盖纯规则模块、管线和存档迁移。

## Workspace 结构

建议采用 pnpm monorepo：

```text
apps/
  desktop/
  web-dev/
packages/
  game-core/
  content/
  ui-ledger/
  save/
  narrative/
docs/
```

### apps/desktop

职责：

- Tauri 桌面入口
- 本地文件读写权限
- 桌面打包

不承载游戏规则。

### apps/web-dev

职责：

- 内部试玩和调试入口
- Vite dev server
- 快速验证 UI 投影

不作为正式平台约束。

### packages/game-core

职责：

- 核心状态类型
- `ActionCommand` 管线
- 规则模块
- 内容门禁
- 账本事件生成
- 纯函数测试

这是项目最核心的包。

### packages/content

职责：

- 内容资产 schema
- 内容包加载
- 证据等级校验
- 模式许可校验
- 最小 S0 内容包

运行时不得直接读取未校验内容。

### packages/ui-ledger

职责：

- 账本式 UI 组件
- 页面投影模型
- UI 状态
- 展示层交互

禁止直接修改规则状态。

### packages/save

职责：

- `SaveEnvelope`
- 存档序列化
- 存档反序列化
- 存档版本迁移
- 阶段检查点

### packages/narrative

职责：

- 把账本投影和内容模板转成正文上下文
- 处理语气、局部变体和状态映射
- 为未来 LLM 接入保留边界

禁止生成规则事实。

## 核心类型边界

### GameState

`GameState` 是完整规则快照。

至少包含：

- `run`
- `time`
- `world`
- `character`
- `resources`
- `debtsAndCredit`
- `build`
- `assets`
- `aperture`
- `trade`
- `knowledge`
- `anchors`

### ActionCommand

所有玩家行动统一进入 `resolveAction(command, state, contentBundle)`。

最小字段：

- `actorId`
- `intent`
- `target`
- `declaredCost`
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

### ActionResult

最小字段：

- `accepted`
- `visibleOutcome`
- `statePatch`
- `ledgerEntries`
- `hiddenSignals`
- `projectionHints`

规则模块只返回 `ActionResult` 组成部分，不直接改 UI。

### ContentAsset

最小字段：

- `contentId`
- `stage`
- `region`
- `tags`
- `evidenceLevel`
- `modePolicy`
- `visibility`
- `effectTemplate`
- `anchorImpact`

`evidenceLevel` 至少支持：

- `canon_explicit`
- `canon_inferred`
- `if_allowed`
- `original_playable`
- `rumor_only`

### SaveEnvelope

最小字段：

- `metadata`
- `snapshot`
- `ledger`
- `checkpoints`
- `rngState`
- `migrationState`

存档文件必须带 `schemaVersion`。

## 行动管线实现

`game-core` 提供唯一入口：

```text
resolveAction(command, state, contentBundle, modePolicy) -> ActionResult
```

内部顺序固定：

1. `availabilityCheck`
2. `costReservation`
3. `subsystemResolution`
4. `anchorRecalculation`
5. `effectCommit`
6. `ledgerAppend`
7. `projectionRefresh`

每一步都应可单测。

禁止：

- UI 直接调用子系统改状态。
- 子系统绕过 `effectCommit` 改全局状态。
- narrative 包生成 `statePatch`。

## 内容数据流

内容进入运行时前必须经过：

1. schema 校验
2. 证据等级校验
3. 模式许可校验
4. 标签和阶段校验
5. bundle 构建

运行时只消费 `ContentBundle`。

首发最小内容包必须包含：

- 固定开局
- 8 回合窗口表
- 青茅山核心节点
- 月光、功绩、药堂、黑市、传承入口级内容
- 至少一次危险遭遇
- 至少一次跑路优于硬打的场景
- 阶段性成功和失败结果

## 账本 UI 数据流

UI 数据流：

```text
GameState + CausalityLedger + ContentBundle
  -> buildProjection()
  -> LedgerViewModel
  -> React UI
```

UI 只允许提交 `ActionCommand`。

UI 不允许：

- 修改 `GameState`
- 读取隐藏变量原值
- 现场生成规则奖励
- 改写内容证据等级

## 存档与迁移

存档由 `packages/save` 负责。

最低能力：

- 创建阶段存档
- 读取阶段存档
- 序列化 `SaveEnvelope`
- 反序列化 `SaveEnvelope`
- 校验 `schemaVersion`
- 校验内容包版本
- 保存和恢复 RNG 状态

首发必须测试：

- 同一存档读回后 AP、节点、债务、Build、暴露和账本一致。
- 内容包版本不一致时给出明确错误或迁移提示。

## 测试架构

测试分四层：

- 类型和 schema 测试
- 规则模块单测
- 行动管线集成测试
- S0 8 回合场景测试

首发必须有以下测试：

- `resolveAction(move)` 不把移动写成固定 AP 税。
- `canon_strict` 拒绝缺证据关键内容。
- `sandbox_if` 启用 IF 内容但仍受门禁限制。
- 战败可进入重创可续。
- Build 分层不混写求活路线和主修流派。
- SaveEnvelope 读写后关键状态一致。
- UI projection 不包含隐藏变量原值。

## 技术架构禁止项

禁止：

- 把全部状态塞进一个全局 store 且无领域边界。
- 把内容资产直接写在 React 组件里。
- 把 UI 操作写成直接改状态。
- 把 `canon_strict` 和 `sandbox_if` 写成两套状态结构。
- 把存档当成本地缓存，不做 schema version。
- 在首发实现中提前实现完整 S1-S4。

## 首发实现边界

首发技术实现只做：

- S0 固定开局
- 8 回合最小闭环
- 核心状态类型
- 行动管线
- 最小内容包
- 账本式 UI 投影
- 阶段存档
- 验收测试

只预留，不实现完整：

- 仙窍经营细节
- 宝黄天完整交易盘
- 阵和仙蛊屋完整部署
- 跨五域完整旅行
- 尊者级 IF

## 归档说明

当前归档路径：

`docs/superpowers/specs/2026-04-26-reborng-technical-architecture-design.md`
