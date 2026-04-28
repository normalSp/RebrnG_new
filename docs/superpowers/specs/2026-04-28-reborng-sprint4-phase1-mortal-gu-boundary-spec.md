# RebrnG Sprint 4 Phase 1 凡人蛊虫玩法边界规格 v1.0

状态：frozen candidate

更新时间：2026-04-28

关联文档：
- [2026-04-28-reborng-sprint4-mortal-gu-foundation-outline.md](./2026-04-28-reborng-sprint4-mortal-gu-foundation-outline.md)
- [2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md](./2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md)
- [2026-04-28-reborng-s0-opening-rite-aperture-start-contract.md](./2026-04-28-reborng-s0-opening-rite-aperture-start-contract.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)

## 文档角色

本文冻结 Sprint 4 Phase 1 的凡人蛊虫玩法边界。本文只定义后续实现合同，不改规则代码、不改内容包、不升级 `ContentManifest.version` 或 `RULES_VERSION`。

Sprint 4 的核心修正是：月光修行不能再只依赖抽象修行痕迹，必须在后续 Phase 中接入真实蛊虫状态、炼化控制权和喂养维护压力。

## 核心结论

- 蛊虫必须成为规则层一等状态，而不是 Build 页里的普通字符串。
- `GuSpec` 与 `GuInstance` 必须分开：前者是内容定义，后者是单只蛊实例。
- `炼化归属` 不等于 `炼蛊 / 合炼 / 升转`。
- `核心蛊`、`辅助蛊`、`本命蛊` 继续分层，不允许合并成一个字段。
- S0 默认 `本命蛊：未建立`，不开放获取、绑定、改命或稳定建立玩法。
- `moonlight_gu` 是 Sprint 4 唯一稳定 canon 蛊虫闭环。

## 首发边界

Sprint 4 只做：
- 月光蛊内容定义。
- 月光蛊实例进入玩家状态。
- 月光蛊炼化 / 控制状态。
- 月光蛊支持月光修行。
- 轻量喂养维护压力展示。
- Build、账本和对话流中的蛊虫状态反馈。

Sprint 4 不做：
- 完整炼蛊系统。
- 蛊虫升转系统。
- 完整杀招库。
- 阵、仙蛊屋、升仙、仙窍、宝黄天。
- 本命蛊建立玩法。
- 酒虫稳定获取。
- 盗天核心真传或强机缘稳定获取。

## GuSpec 合同

`GuSpec` 表示一种蛊虫的内容定义。

最小字段语义：
- `gu_id`：稳定内容 ID，例如 `moonlight_gu`。
- `display_name`：中文名，例如“月光蛊”。
- `rank_band`：品转范围或当前定义品转；S0 月光蛊为一转范畴。
- `path_tags`：流派或用途标签，例如月道、攻伐、修行支撑。
- `role_tags`：核心蛊、辅助蛊、消耗蛊、侦查、治疗、防护等用途标签。
- `feeding_profile`：喂养维护类型或占位。
- `refinement_profile`：炼化 / 炼蛊 / 升转约束或占位。
- `evidence_level`：证据等级。
- `mode_permission`：`canon_strict` / `sandbox_if` 许可。

Phase 2 内容 schema 必须拒绝：
- 缺 `gu_id`。
- 缺 `display_name`。
- 缺证据等级。
- 缺模式许可。
- 非法品转。
- 重复 `gu_id`。
- `sandbox_if` 蛊虫进入 `canon_strict` 关键奖励链。

## GuInstance 合同

`GuInstance` 表示世界或玩家手里的一只具体蛊。

最小字段语义：
- `instance_id`：单只蛊实例 ID。
- `gu_id`：指向 `GuSpec`。
- `rank`：当前品转。
- `owner`：所属者或当前控制者。
- `container`：随身、空窍、落脚点、组织托管、挂单中、部署中等。
- `control_state`：未炼化、炼化中、已炼化、失控、受他人痕迹影响等。
- `condition_state`：完好、受损、残蛊、濒毁等。
- `feeding_state`：稳定、将缺、断供、偏食压力等。
- `build_role`：核心、辅助、临时、缺口替代等。

禁止：
- 把 `GuInstance` 写成普通 inventory item。
- 用一个字符串同时表示蛊名、归属、容器、损伤和喂养。
- 让前端直接推导或改写蛊虫状态。

## GuInventoryState 合同

`GuInventoryState` 表示玩家当前可追踪的蛊虫容器状态。

最小语义：
- 能列出玩家相关 `GuInstance`。
- 能区分随身、空窍、落脚点等容器。
- 能支持后续存档序列化与读取校验。
- 能为 Build 页、账本页、对话流提供只读投影。

Sprint 4 暂不要求：
- 多组织托管。
- 宝黄天挂单。
- 仙窍内部部署。
- 大规模蛊虫图鉴管理。

## RefinementState 合同

`RefinementState` 只表达炼化归属和控制权。

允许状态：
- `unrefined`：未炼化，不能作为玩家稳定修行支撑。
- `refining`：炼化中，后续可承接时间 / AP / 风险消耗。
- `refined`：已炼化，玩家可以稳定调用。
- `unstable`：控制不稳，可能带来失败、反噬或暴露。
- `foreign_trace`：受他人痕迹影响，后续可接更高阶设定。

明确区分：
- `炼化`：建立控制权。
- `炼蛊`：制作或合炼蛊虫。
- `升转`：提高品转或改变阶段。

Sprint 4 只实现炼化归属闭环，不做完整炼蛊或升转。

## FeedingState 合同

`FeedingState` 表达喂养维护压力。

允许状态：
- `stable`：当前稳定。
- `warning`：将缺或开始有维护压力。
- `starving`：断供风险。
- `special_need`：特殊食性或偏食压力。

Sprint 4 只要求可见，不要求复杂周期公式。后续如实现周期扣费，必须通过时间 / AP / 资源 / 债务 / 风险接口，而不是前端临时扣数。

## ConditionState 合同

`ConditionState` 表达蛊虫损伤状态。

允许状态：
- `intact`：完好。
- `damaged`：受损。
- `crippled`：残蛊。
- `near_destroyed`：濒毁。

用途：
- 战败后果。
- 遭遇反噬。
- 炼化失败。
- 维护不足。

Sprint 4 不做复杂耐久系统，只保留状态边界和投影语义。

## Build 三层关系

### 核心蛊

核心蛊是当前路线、行动或杀招雏形的组合中心。

S0 中，月光蛊可以成为月光路线核心候选，但必须通过真实 `GuInstance` 和炼化状态支撑。

### 辅助蛊

辅助蛊是支撑路线、杀招、恢复或经营的配套位。

S0 默认可以为空，不强行给玩家补支撑蛊。

### 本命蛊

本命蛊是更深层生命 / 空窍绑定状态，不等同核心蛊。

S0 默认：
- `status = not_established`
- 显示为“本命蛊：未建立”
- 不开放获取、绑定、改命或替换玩法

后续若允许核心蛊与本命蛊重合，必须通过显式关系字段表达，不能复用同一个字段。

## 月光蛊闭环边界

`moonlight_gu` 是 Sprint 4 唯一稳定 canon 蛊虫闭环。

后续 Phase 3 最小闭环应满足：
- 玩家空窍已开。
- 玩家接触或领取月光蛊实例。
- 玩家完成炼化归属。
- 月光蛊进入空窍或等价可调用容器。
- 月光修行行动检查已炼化月光蛊。
- 修行仍消耗 AP 与元石。
- 行动反馈显示蛊虫状态变化。

如果没有已炼化月光蛊：
- 月光修行必须禁用，或给出明确禁用原因。
- 不允许静默推进 `moonlight_cultivation_marks`。

## 强机缘边界

酒虫：
- 可作为传闻、黑市风险、传承诱惑或后续 IF 候选。
- 不在 Sprint 4 稳定获得。

盗天类机缘：
- 只允许作为远期 IF 候选或文本反例。
- 不进入 S0 canon 稳定奖励。

花酒行者传承：
- 仍保持残线、传闻、半真半假和高风险诱惑。
- 不在 Sprint 4 变成稳定奖励通道。

本命蛊：
- 只显示“未建立”。
- 不通过天赋、黑市、传承或月光线稳定建立。

## UI 与存档边界

UI：
- React 只消费 Rust 投影。
- React 不持有完整 `GameState`。
- React 不直接修改蛊虫状态。
- React 不把蛊虫做成普通装备栏。

存档：
- `GuInventoryState` 后续必须进入 `SaveEnvelope.snapshot`。
- `GuSpec` 只通过内容版本引用，不直接塞入存档。
- 旧 `sprint3-rules-v1` 存档在实现阶段必须拒绝加载。

## 验收标准

Phase 1 只验收文档，不验收代码实现。

必须满足：
- 本文档出现 `GuSpec`、`GuInstance`、`GuInventoryState`、`RefinementState`、`FeedingState`、`ConditionState`。
- 本文档明确区分炼化、炼蛊、升转。
- 本文档明确 `核心蛊`、`辅助蛊`、`本命蛊` 三者分层。
- 本文档明确 `moonlight_gu` 是唯一稳定 canon 闭环。
- 本文档明确酒虫、盗天类机缘、本命蛊建立不在 Sprint 4 稳定发放。
- 本阶段不改 `content/s0`。
- 本阶段不改 Rust 规则状态。
- 本阶段不升级 `ContentManifest.version`。
- 本阶段不升级 `RULES_VERSION`。

## 工具链说明

本阶段同时修复本机 `rg` 工具链。

结论：
- `rg` 比 PowerShell 原生全文搜索更适合代码库检索。
- 当前 Codex 应用包内 `WindowsApps/.../rg.exe` 会因 ACL 限制在 PowerShell 中出现 `Access is denied`。
- 已通过 `cargo install ripgrep --locked` 安装独立 ripgrep 到用户 Cargo bin。
- 后续 shell 若仍优先命中 WindowsApps 版本，应将 `C:\Users\11411\.cargo\bin` 放到 PATH 前面，或在命令中临时前置该路径。
