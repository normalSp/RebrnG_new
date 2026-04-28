# RebrnG Sprint 3 人生重开设置层与对话流壳开发大纲 v1.0

状态：frozen candidate

更新时间：2026-04-28

关联文档：
- [2026-04-28-reborng-sprint3-reboot-dialogue-framework-freeze.md](./2026-04-28-reborng-sprint3-reboot-dialogue-framework-freeze.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)
- [2026-04-28-reborng-s0-opening-rite-aperture-start-contract.md](./2026-04-28-reborng-s0-opening-rite-aperture-start-contract.md)

参考材料：
- 根目录 `example.md` 仅作为豆包式人生重开体验参考，不纳入内容 bundle，不作为可直接导入的剧情源。

## 文档角色

本文冻结 Sprint 3 的总体开发大纲。上一份 Sprint 3 目标冻结文档回答“最终要像什么”，本文回答“接下来按什么阶段开发”。

Sprint 3 的目标是把当前 8 自由窗口账本策略底座升级成更接近豆包式人生重开体验的结构：先进行出身、天赋、属性、开窍大典设置，再进入现有 S0 青茅山循环；主界面改为“对话流正文 + 账本明细侧栏”。核心规则不改归属：Rust 仍是唯一规则真相，React 只消费投影，DeepSeek 不进入 runtime 主链。

## 版本策略

- 内容版本默认升为 `s0.2.0`。
- 规则版本默认升为 `sprint3-rules-v1`。
- 旧 Sprint 2 存档不做迁移，加载时按版本不匹配拒绝。
- 默认预设开局继续保留，用于自动测试、快速走查和回归。

## Phase 1：设置层内容合同与最小内容包

目标：先把人生重开设置层的内容形态锁住，避免代码阶段临时发明字段。

新增内容类型：
- `OriginSpec`：出身，决定初始社会关系、资源来源、安全区和压力来源。
- `TalentSpec`：天赋，决定属性修正、路线倾向、线索可见性或 IF 许可。
- `AttributeProfile`：初始属性面板，至少覆盖资质、气运、心智、根骨。
- `OpeningRiteOutcome`：开窍大典结果，决定空窍状态、资质展示、资源包和关注度。
- `InitialResourcePackage`：初始元石、基础物资、债务、人情或线索。

最小内容目标：
- 3 个青茅山安全出身。
- 10 个天赋候选。
- 4 项初始属性。
- 1 个开窍大典结果表。

模式边界：
- `canon_strict` 只允许低风险或温和修正天赋。
- 强天赋、重生者、提前抢机缘、硬改写原著锚点只允许 `sandbox_if`。

## Phase 2：Rust 设置状态与命令边界

目标：让设置层成为一等规则状态，而不是前端临时表单。

新增状态与接口：
- `RunSetupState`：保存候选项、已选项、属性面板、出身、开窍大典结果和设置完成状态。
- `SetupCommand`：提交设置阶段选择。
- `SetupResponse`：返回设置阶段结算结果。
- `SetupViewModel`：供 React 显示设置页。

新增 Tauri command：
- `create_setup_run`
- `resolve_setup_choice`
- `confirm_setup_run`

兼容要求：
- 默认预设开局必须继续存在。
- 设置完成后生成的 S0 状态必须能进入现有 `ActionCommand` 管线。
- React 不得自行计算属性、资源包、天赋效果或开窍结果。

## Phase 3：对话流投影

目标：让现有账本投影外再派生一层豆包式正文流，但不让正文流成为规则真相。

新增投影：
- `DialogueTimelineView`

最小字段语义：
- 当前阶段标题。
- 当前正文段落。
- 上一次选择标题。
- 上一次结果摘要。
- 可选行动列表。
- 最新账本变化摘要。
- 当前模式和证据门禁提示。

派生规则：
- 只允许从规则状态、内容 bundle 和因果账本派生。
- React 不生成剧情，不判断奖励，不改状态。
- 同一次行动必须同时提供正文结果、点击成功反馈和账本明细变化。

## Phase 4：UI 形态调整

目标：把主阅读层从账本页改成对话流正文，账本明细作为第二阅读层。

UI 变化：
- 主区域显示阶段标题、正文、上一选择、结果摘要和行动选项。
- 侧栏或下方折叠区显示 AP、时段、地点、资源、债务、暴露、伤势、空窍、蛊虫、线索。
- 行动按钮沿用 Rust 投影，显示叙事标题、风险提示、代价摘要和禁用原因。
- 继续遵守冷峻账本视觉方向，不复刻豆包聊天皮肤。

边界：
- UI 只消费 `SetupViewModel`、`DialogueTimelineView` 和 `LedgerViewModel`。
- UI 不持有完整 `GameState`。
- UI 不读取 YAML，不直接读写存档，不接 runtime AI。

## Phase 5：S0 8 窗口完整回归

目标：证明人生重开设置层和对话流壳没有破坏既有核心循环。

必须回归：
- 设置完成后进入 S0。
- 默认预设开局绕过设置层进入 S0。
- 4 条 Sprint 1 可玩性路径继续通过。
- 2 条窗口平衡验收继续通过。
- AP、移动、债务、暴露、伤势、黑市解锁、遭遇、存档仍按现有规则运行。
- 性能红线继续保持：单次 `resolve_action <300ms`，下一回合可交互 `<1s`。

## Phase 6：DeepSeek 离线扩写方案冻结

目标：只冻结离线内容扩写管线，不接 runtime 主链。

允许：
- 生成候选正文、行动说明、结果反馈和风声文本。
- 人工筛选、证据打标、模式许可标注。
- 写入 YAML 后由内容构建器校验并进入 bundle。

禁止：
- 玩家点击后同步调用 DeepSeek。
- DeepSeek 生成规则结果、奖励、蛊虫、传承、锚点变化或原著硬事实。
- API key、prompt、response 写入仓库、存档或运行时主链。

## 公共接口变更

- 新增设置接口：`create_setup_run`、`resolve_setup_choice`、`confirm_setup_run`。
- 新增内容索引：`origins`、`talents`、`opening_rite_outcomes`、`setup_narratives`。
- 新增投影：`SetupViewModel`、`DialogueTimelineView`。
- `GameState` 增加 setup 来源摘要，用于存档和回看：出身、天赋、属性、开窍大典结果。
- `LedgerViewModel` 保留现有账本职责，对话流不得替代账本母体。

## 验收标准

- 设置流程能稳定生成初始属性、资源包、空窍状态、关注度和内容版本引用。
- `canon_strict` 拒绝完整传承、稳定本命蛊、提前硬改写方源等核心人物。
- `sandbox_if` 允许强天赋，但仍受资源、阶段、锚点压力和世界规则门禁限制。
- 进入 S0 后，现有核心循环和存档验证全部通过。
- 同一行动在对话流、最近反馈、账本明细三处体现结果。
- 红线检查无 runtime AI proposal/narrator、无 Express 主链、前端不扫描 YAML、React 不持有完整 `GameState`。

## Assumptions

- Sprint 3 不扩成长局，不做炼化、升转、完整杀招、升仙、仙窍、宝黄天。
- `example.md` 继续只作为体验参考，不纳入提交，不作为内容源。
- 强爽点只进 `sandbox_if`；`canon_strict` 保持原著锚点保护和生存压迫。
- 每个 Phase 完成后验证、中文提交并推送当前开发分支 `codex/sprint-0-foundation`。
