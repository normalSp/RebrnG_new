# RebrnG Sprint 2 走查反馈回收与可交付走查版大纲 v1.0

状态：frozen candidate

更新时间：2026-04-27

关联文档：
- [2026-04-27-reborng-sprint1-walkthrough-candidate-freeze.md](../reviews/2026-04-27-reborng-sprint1-walkthrough-candidate-freeze.md)
- [2026-04-27-reborng-sprint1-playtest-review.md](../reviews/2026-04-27-reborng-sprint1-playtest-review.md)
- [2026-04-27-reborng-sprint1-content-playability-outline.md](./2026-04-27-reborng-sprint1-content-playability-outline.md)
- [2026-04-27-reborng-phase7-frontend-style-freeze-spec.md](./2026-04-27-reborng-phase7-frontend-style-freeze-spec.md)

适用范围：`codex/sprint-0-foundation` 分支在 Sprint 1 可走查候选冻结之后的 Sprint 2。

## 文档角色

本文档冻结 Sprint 2 的目标和边界。Sprint 2 不是长局扩展、不是高阶系统启动，也不是 AI 接入轮；它是一次游戏开发流程里的走查回收轮，目标是把 8 自由窗口候选版从“自动验收能跑”推进到“可以稳定点击、记录、修复、再次验证、交给别人体验”。

当前关键事实：
- Sprint 1 候选已冻结，内容版本为 `s0.1.2`，规则版本为 `sprint1-rules-v2`。
- 8 自由窗口、4 条可玩性路径、2 条平衡验收、文本扫描和启动入口均已通过。
- 剩余风险主要是主观 UI 阅读负担、按钮理解成本、8 窗口压强体感，以及可交付走查包是否能顺利生成。

## 核心结论

Sprint 2 锁定为 5 个阶段：

- Phase 1：冻结 Sprint 2 走查打磨大纲。
- Phase 2：建立可重复走查与验证入口。
- Phase 3：打磨走查 UI 可读性。
- Phase 4：回收真实走查缺陷。
- Phase 5：冻结 Sprint 2 走查版候选。

本轮继续锁定 8 个自由窗口，不扩成 60-120 分钟纵切，不做炼化、升转、完整杀招、升仙、仙窍、宝黄天，不接 runtime AI。

## Phase 1：冻结 Sprint 2 走查打磨大纲

必须新增本文件，并把本轮边界写清楚：
- 只处理走查反馈、UI 可读性、验证脚本、可交付走查包。
- 不新增路线、不新增高阶系统、不引入 runtime AI。
- 不纳入 `apps/desktop/src/assets/`、`docs/art/`、`output/`，除非另开视觉资产任务。

## Phase 2：建立可重复走查与验证入口

必须新增：
- `verify-game.cmd` / `verify-game.ps1`：一键执行文本扫描、Rust 检查、内容构建、前端构建和全量测试。
- `build-playtest.cmd` / `build-playtest.ps1`：尝试生成 Tauri 桌面走查包；若 Windows 打包依赖缺失，必须输出明确 blocker。
- `docs/superpowers/reviews/2026-04-27-reborng-sprint2-playtest-round1.md`：作为实际点击走查的问题登记表。

## Phase 3：打磨走查 UI 可读性

必须遵守：
- React 只消费 `LedgerViewModel`，不持有完整 `GameState`。
- 不改 AP、元石、债务、暴露、伤势、遭遇代价。
- 不改存档结构，不升级 `RULES_VERSION`。

优先改善：
- 行动侧栏的分组和可读顺序。
- 禁用原因、代价、风险、后果提示的视觉层级。
- 最近反馈、阶段收口和存档状态的可见性。
- 移动端或窄窗口下的账本阅读稳定性。

## Phase 4：回收真实走查缺陷

问题分类固定为：
- `阻断`：无法继续走查、无法启动、关键按钮不可用、路径无法抵达。
- `误导`：玩家被 UI 或文案引向错误理解，例如黑市像商店、传承像稳定奖励。
- `体验弱`：可玩但阅读负担高、压强体感不稳、按钮层级不够清晰。
- `文案弱`：语气、短标签、反馈文本不够冷峻或不够清楚。

处理规则：
- 本轮只必须修 `阻断` 和高风险 `误导`。
- `体验弱 / 文案弱` 可低风险修补，也可记录到下一轮。
- 若改规则语义，升级 `RULES_VERSION`；若改内容包文本或数据，升级 `ContentManifest.version`。

## Phase 5：冻结 Sprint 2 走查版候选

必须新增候选总结文档，记录：
- 当前提交、内容版本、规则版本。
- 验证矩阵和打包结果。
- 剩余问题分层。
- 是否可交给外部体验。

默认不打 Git tag、不合并 `main`、不创建 PR，继续推送 `codex/sprint-0-foundation`。

## 测试计划

每个代码阶段必须运行：
- `.\verify-game.cmd`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo check -p rebrng-desktop`
- `pnpm content:build`
- `pnpm -r build`

每轮候选冻结必须额外确认：
- `.\start-game.cmd -CheckOnly` 通过。
- 用户可见文本扫描无 mojibake。
- 4 条 Sprint 1 可玩性路径仍通过。
- 2 条 Phase 8 平衡验收仍通过。
- 红线检索无 Express 主链、无 runtime AI proposal/narrator、React 不持有完整 `GameState`、前端不扫描 YAML。

## Assumptions

- Sprint 2 第一优先级是可走查、可记录、可反复验证，不是扩内容量。
- 本轮仍以 PC 单机 Tauri 桌面走查为准，不做 Web 试玩壳。
- DeepSeek / AI 只作为后续离线内容工具另开冻结方案，不进入 runtime 主链。
