# RebrnG Sprint 4 凡人蛊虫系统基座与 DeepSeek 离线候选工具总体大纲 v1.0

状态：frozen candidate

更新时间：2026-04-28

关联文档：
- [2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md](./2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md)
- [2026-04-28-reborng-sprint3-development-outline.md](./2026-04-28-reborng-sprint3-development-outline.md)
- [2026-04-28-reborng-sprint3-phase6-offline-narrative-expansion-freeze.md](./2026-04-28-reborng-sprint3-phase6-offline-narrative-expansion-freeze.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)

## 文档角色

本文冻结 Sprint 4 的总体方向。Sprint 3 已完成“人生重开设置层 + 对话流壳 + S0 回归 + 离线叙事扩写合同”，Sprint 4 不再继续只扩 UI 形态，而是补齐凡人阶段最核心的玩法底座：蛊虫必须成为一等规则状态。

Sprint 4 的目标不是扩成长局，也不是进入升仙、仙窍、宝黄天或完整杀招系统，而是让首发 S0 中最基础的月光蛊获取、炼化归属、修行使用和喂养维护压力进入 Rust 规则层、存档层和账本投影层。

## 当前基线

- 基线分支：`codex/sprint3-phase1-setup-content`
- Sprint 4 分支：`codex/sprint4-mortal-gu-foundation`
- Sprint 4 worktree：`.worktrees/sprint4-mortal-gu-foundation`
- 当前内容版本：`s0.2.0`
- 当前规则版本：`sprint3-rules-v1`
- Sprint 3 已通过 `verify-game.cmd` 全量验证。

## 版本策略

- Sprint 4 默认内容版本升为 `s0.3.0`。
- Sprint 4 默认规则版本升为 `sprint4-rules-v1`。
- 旧 Sprint 3 存档不做迁移，加载时按版本不匹配拒绝。
- 如果只修改文档或离线工具说明，不升级版本；一旦新增 `GuInstance`、蛊虫内容或修行规则，必须升级规则 / 内容版本。

## 核心目标

- 蛊虫从 Build 页的展示字符串升级为规则层一等状态。
- 月光修行必须依赖真实 `GuInstance` 与炼化 / 控制状态，不能再只是抽象修行痕迹。
- `核心蛊`、`辅助蛊`、`本命蛊` 继续保持分离，本命蛊在 S0 默认仍为“未建立”。
- 喂养维护压力进入可见状态，但 Sprint 4 不展开复杂公式。
- DeepSeek 只作为离线候选文本工具，不进入 runtime 主链。

## 非目标

Sprint 4 不做：
- 完整炼蛊系统。
- 蛊虫升转系统。
- 完整杀招库。
- 阵、仙蛊屋、升仙、仙窍、宝黄天。
- 长局扩展或 60 分钟内容扩容。
- runtime DeepSeek、runtime AI narrator、runtime AI proposal。
- DeepSeek 密钥入库、prompt/response 入库、AI 输出自动写 YAML。

## Phase 1：冻结凡人蛊虫玩法边界

目标：在代码实现前先钉住凡人阶段蛊虫系统的最小规则合同。

必须明确：
- `GuSpec`：蛊虫内容定义。
- `GuInstance`：具体单只蛊实例。
- `GuInventoryState`：玩家蛊虫容器。
- `RefinementState`：炼化归属 / 控制状态。
- `FeedingState`：喂养维护状态。
- `ConditionState`：完好、受损、残蛊等状态。
- `CoreGuSlot`、`SupportGuSlot`、`VitalGuState` 三者不能混成一个字段。

首发只要求状态语义存在，不要求完整炼蛊公式、升转公式或喂养消耗表。

## Phase 2：扩展内容 schema 与最小蛊虫内容

目标：让内容包能描述最小 `GuSpec`，并让内容构建器校验证据等级、模式许可和索引完整性。

最小内容：
- `moonlight_gu`：月光蛊，作为 `canon_strict` 下的严谨首发链路。

暂不稳定发放：
- 酒虫只能作为传闻、黑市风险线索或后续内容候选。
- 盗天类强机缘只能作为 `sandbox_if` 候选或远期传闻，不给 S0 稳定获得。
- 本命蛊相关内容只保留状态位，不开放建立玩法。

内容构建必须拒绝：
- 缺证据等级的蛊虫。
- 缺模式许可的蛊虫。
- 重复蛊虫 id。
- 非法品转。
- `sandbox_if` 蛊虫进入 `canon_strict` 关键奖励链。

## Phase 3：实现 Rust 蛊虫状态与行动闭环

目标：让蛊虫参与 S0 核心行动管线。

新增或扩展行动：
- 接触 / 领取月光蛊。
- 炼化月光蛊。
- 使用已炼化月光蛊修行。
- 检查喂养维护压力。

规则要求：
- 未开窍不能修行。
- 空窍已开但没有已炼化月光蛊时，月光修行必须禁用或给出明确原因。
- 月光蛊已炼化后，才允许推进月光修行痕迹。
- 修行仍消耗 AP 与元石，不能因为蛊虫入场而取消既有资源压力。
- 喂养压力可见，但 Sprint 4 只做轻量状态，不做复杂周期扣费。

## Phase 4：更新账本、Build 与对话流

目标：让玩家在对话流和账本里看懂蛊虫状态变化。

必须展示：
- 玩家拥有的蛊虫。
- 月光蛊是否已炼化。
- 蛊虫容器：随身、空窍、落脚点等。
- 控制状态、损伤状态、喂养状态。
- 核心蛊候选、辅助蛊状态、本命蛊未建立。
- 最近一次蛊虫相关行动的结果摘要。

UI 边界：
- React 只消费 Rust 投影。
- React 不直接修改蛊虫状态。
- React 不把蛊虫写成普通装备槽。
- React 不把杀招写成技能树按钮。

## Phase 5：DeepSeek 离线候选工具

目标：建立离线候选文本生产入口，服务后续沉浸文本扩写，不影响 runtime。

工具要求：
- 只读取本地环境变量：`DEEPSEEK_API_KEY`、`DEEPSEEK_BASE_URL`、`DEEPSEEK_MODEL`。
- 默认支持 dry-run / mock 模式，无 key 时测试仍可通过。
- 输出候选 JSON 或 Markdown 到 gitignored 本地目录。
- 候选必须人工审核后才允许写入 `content/s0/**/*.yaml`。
- 候选不能自动入库，不能自动改内容版本，不能自动生成规则事实。

运行时红线：
- `resolve_action` 不调用 DeepSeek。
- Tauri command 不调用 DeepSeek。
- React 点击链路不调用 DeepSeek。
- `SaveEnvelope` 不保存 key、prompt、response、模型名或未审核草稿。

## Phase 6：回归与 Sprint 4 候选冻结

目标：证明凡人蛊虫基座没有破坏既有 Sprint 1 / Sprint 3 可玩路径。

必须回归：
- 8 自由窗口脚本。
- Sprint 1 四条可玩性路径。
- Sprint 1 Phase 8 平衡验收。
- Sprint 3 设置层进入 S0 的回归。
- 存档写入 / 读取 / 版本拒绝。
- 对话流、最近反馈、账本明细。
- runtime AI 红线检索。

候选冻结文档必须记录：
- 当前内容版本。
- 当前规则版本。
- 月光蛊闭环是否可走查。
- DeepSeek 是否仍只存在于离线工具。
- 剩余不做项：炼蛊、升转、杀招、升仙、仙窍、宝黄天。

## Public Interfaces

新增内容接口：
- `GuSpec`
- `gu_specs` 内容索引
- `gu_count` 构建诊断

新增状态接口：
- `GuInstance`
- `GuInventoryState`
- `RefinementState`
- `FeedingState`
- `ConditionState`

新增或扩展投影：
- `GuLedgerView`
- `BuildLedgerView` 中的核心蛊 / 辅助蛊 / 本命蛊继续分字段展示
- `DialogueTimelineView` 中显示蛊虫行动结果

新增离线工具接口：
- `DEEPSEEK_API_KEY`
- `DEEPSEEK_BASE_URL`
- `DEEPSEEK_MODEL`
- 本地候选输出目录，必须 gitignored

## Test Plan

内容测试：
- `GuSpec` id 唯一。
- `moonlight_gu` 可被构建并进入索引。
- 缺证据等级、缺模式许可、非法品转必须失败。
- `sandbox_if` 蛊虫不得进入 `canon_strict` 关键奖励链。

规则测试：
- 无已炼化月光蛊时不能月光修行。
- 炼化月光蛊后可推进月光修行痕迹。
- 修行仍消耗 AP 与元石。
- 喂养压力可见但不展开复杂公式。

存档测试：
- 蛊虫实例、容器、控制状态、损伤状态、喂养状态可序列化并读回。
- 核心蛊、辅助蛊、本命蛊分层不丢失。
- 旧 `sprint3-rules-v1` 存档拒绝加载。

UI 测试：
- 对话流、最近反馈、Build 页、账本明细都能看到蛊虫状态变化。
- React 不持有完整 `GameState`。
- React 不直接读写存档。
- React 不扫描 YAML。

DeepSeek 工具测试：
- 无 key 时 dry-run / mock 测试通过。
- 有 key 时只生成本地候选文件。
- 候选不自动写入 YAML。
- runtime 目录红线检索不出现 DeepSeek/API 调用。

完整验证：
- `verify-game.cmd`
- `cargo test --workspace`
- `pnpm content:build`
- `pnpm -r build`
- `scripts/check-visible-text.ps1`

## Assumptions

- Sprint 4 首要价值是把蛊虫系统做成可定性、可定量的底座。
- 月光蛊是 Sprint 4 的唯一稳定 canon 蛊虫闭环。
- DeepSeek 是离线工具，不是 runtime 功能。
- 用户提供过的密钥不写入仓库、不写入文档、不写入测试。
- Sprint 4 完成后，再决定 Sprint 5 是扩写沉浸文本，还是继续推进炼蛊 / 杀招组合。
