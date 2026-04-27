# RebrnG Sprint 1 后半程走查、文本可读性与可玩性打磨大纲 v1.0

状态：frozen

更新时间：2026-04-27

关联文档：
- [2026-04-27-reborng-sprint1-content-playability-outline.md](./2026-04-27-reborng-sprint1-content-playability-outline.md)
- [2026-04-26-reborng-launch-vertical-slice-implementation-plan.md](./2026-04-26-reborng-launch-vertical-slice-implementation-plan.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)
- [2026-04-27-reborng-phase7-frontend-style-freeze-spec.md](./2026-04-27-reborng-phase7-frontend-style-freeze-spec.md)
- [2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md](./2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md)

适用范围：`codex/sprint-0-foundation` 分支在 Sprint 1 Phase 1-5 完成后的后半程打磨。

## 文档角色

本文档冻结 Sprint 1 Phase 5 之后的开发方向。它不是新的内容扩展纲要，也不是正式版长局设计，而是把当前已通过自动验收的 8 自由窗口原型推进到“可人工走查、可读、可调、可交付候选”的打磨闭环。

当前关键事实：
- Sprint 1 原冻结大纲的 Phase 1-5 已完成：内容包扩展、三类遭遇、账本行动反馈、4 条 8 窗口可玩性验收脚本均已落地。
- 当前不应继续横向扩新系统；下一步应先处理用户可见文本、走查记录、平衡调参和候选冻结。
- 只读审查发现 UI 与 Rust 可见文案中仍存在明显 mojibake，例如 `闈掕寘`、`鑺傜偣`、`铔` 等。这会直接破坏人工走查，应作为后半程第一优先级。

## 核心结论

Sprint 1 后半程锁定为 4 个阶段：

- Phase 6：中文可读性与文本来源治理。
- Phase 7：人工走查与缺陷回收。
- Phase 8：8 窗口平衡调参。
- Phase 9：Sprint 1 可走查候选冻结。

本轮继续锁定 8 个自由窗口，不扩局长，不接 runtime AI，不做完整蛊虫、炼化、升转、杀招、升仙、仙窍、宝黄天。

## Phase 6：中文可读性与文本来源治理

目标：让所有用户可见文本恢复为可读中文，并建立后续不会再次混入 mojibake 的检查口。

必须做：
- 修复 UI、Rust 投影、本地叙事、内容 YAML 中所有用户可见 mojibake。
- 新增可选检查脚本 `scripts/check-visible-text.ps1`，扫描用户可见源码和内容文件中的乱码残留。
- 检查目标至少覆盖：`packages/ui-ledger/src/`、`apps/desktop/src/`、`crates/game-core/src/lib.rs`、`content/s0/`。
- 扫描禁止 `�`、`鑾`、`娓`、`榛`、`铔`、`鍥`、`闈`、`璐` 等已知乱码残留进入用户可见源。
- 修复后人工启动 `start-game.cmd` 检查主界面、行动按钮、线索页、因果账、Build 页、关系页、阶段收口页。

版本规则：
- 若只修 UI/投影文本，不升级 `RULES_VERSION`。
- 若修改内容 bundle 文本，统一将 `ContentManifest.version` 从 `s0.1.0` 升为 `s0.1.1`。
- 不改变规则成本、行动后果或存档结构。

禁止：
- 不借文本修复扩展新路线。
- 不接 DeepSeek 或任何 runtime AI。
- 不把本命蛊、核心蛊、辅助蛊、杀招、炼化写成新玩法按钮。

## Phase 7：人工走查与缺陷回收

目标：把自动化验收转化为真实玩家视角的走查反馈。

必须做：
- 使用根目录 `start-game.cmd` 做 4 条路径人工走查。
- 新增走查记录文档，建议路径：`docs/superpowers/reviews/2026-04-27-reborng-sprint1-playtest-review.md`。
- 走查问题按 `阻断 / 误导 / 体验弱 / 文案弱` 分类。
- 每条问题记录至少包含：路径、窗口位置、现象、预期、建议处理、是否阻断后续走查。

4 条固定走查路径：
- 月光根基：侦查、月光修行、处理学堂压力，最终观察是否能理解“站稳一转根基”。
- 黑市跑路：先解锁黑市，再深夜进入暗口，触发勒索并跑路，观察黑市是否仍像隐藏门路而不是商店。
- 药堂债务：通过危险分支制造伤势，去药堂恢复，观察债务和人情压力是否被玩家理解。
- 传承诱惑：`sandbox_if` 下接近传承残线并撤退，观察半真半假、高风险、无稳定收益是否表达清楚。

修复边界：
- 只修阻断和高误导问题。
- 体验弱与文案弱可记录到后续批次，不强行同轮修完。
- 不新增完整战斗、完整商店、完整炼化、完整杀招或长局系统。

## Phase 8：8 窗口平衡调参

目标：让 8 个自由窗口内出现真实取舍，同时避免过早普遍暴毙。

必须检查：
- 8 窗口内至少出现两次真实取舍，来源可以是 AP、元石、债务、暴露、伤势、时段或路线机会。
- 谨慎玩法不应在 3-4 窗口内普遍暴毙。
- 玩家不能稳定在锚点前把所有想做的事情都做完。
- 跑路、侦查、恢复、等待都必须有实际价值，而不是陪衬按钮。
- 黑市不能变成开局可见商店；传承不能变成稳定高收益任务链。

允许调整：
- 初始资源、行动 AP 成本、元石消耗、暴露增量、药堂债务、人情债、伤势压窗、遭遇决断代价。
- 投影中的代价提示、风险提示和禁用原因。

版本规则：
- 只改提示文本不升级 `RULES_VERSION`。
- 修改规则成本、行动后果、伤势压窗或存档语义时，升级 `RULES_VERSION` 到下一档，并保留旧存档拒绝加载或迁移提示。

## Phase 9：Sprint 1 可走查候选冻结

目标：形成一个可交给人工持续走查的 Sprint 1 候选状态。

候选冻结必须满足：
- `start-game.cmd` 可以启动桌面走查。
- 4 条 Sprint 1 自动验收路径通过。
- 人工走查记录存在，并且阻断项为 0。
- 用户可见中文无已知 mojibake 残留。
- 性能仍满足单次 `resolve_action <300ms`、下一回合可交互 `<1s`。
- runtime AI 不进入 `resolve_action`、Tauri command 或 UI 点击主链。

候选冻结输出：
- 一份 Sprint 1 走查候选总结文档。
- 当前分支提交并推送。
- 明确剩余问题清单，区分必须修复、可延期、设计待定。

## 公共接口与边界

- 新增脚本：`scripts/check-visible-text.ps1`，只做文本检查，不修改文件。
- 内容版本：Phase 6 若修 `content/s0` 文本，`ContentManifest.version` 升为 `s0.1.1`。
- 规则版本：只有 Phase 8 修改结算语义时升级 `RULES_VERSION`。
- UI 边界不变：React 只消费 `LedgerViewModel`，不持有完整 `GameState`，不读写存档，不扫描 YAML，不接 runtime AI。
- AI 边界不变：DeepSeek/AI 只作为后续“离线内容工具”另开设计，不混入本轮。

## 测试计划

每个代码阶段完成后必须运行：
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo check -p rebrng-desktop`
- `pnpm content:build`
- `pnpm -r build`

文本治理必须额外运行：
- `powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\check-visible-text.ps1`
- 人工打开 `start-game.cmd`，确认主界面、行动区、正文页、因果页、线索页、Build 页没有乱码。

红线检索必须确认：
- 无 Express 主链。
- 无 runtime AI proposal / narrator。
- React 不持有完整 `GameState`。
- 运行时不扫描 YAML / JSONL。
- 黑市未解锁前不可见。
- 蛊虫、炼化、杀招未被写成前端技能系统。

## Assumptions

- Sprint 1 后半程继续锁定 8 个自由窗口，不扩成 60-120 分钟纵切。
- 先修用户可见中文与走查问题，再做平衡调参。
- 本轮不接 DeepSeek 到运行时；如需 AI，只作为后续离线内容工具单独冻结方案。
- `apps/desktop/src/assets/`、`docs/art/`、`output/` 不纳入本轮，除非另开视觉资产或导出物任务。

## 归档说明

当前归档路径：
`docs/superpowers/specs/2026-04-27-reborng-sprint1-post-phase5-playtest-polish-outline.md`
