# RebrnG Sprint 5 月光蛊闭环沉浸文本入库与走查打磨大纲 v1.0

状态：frozen candidate

更新时间：2026-04-28

## 文档角色
本文冻结 Sprint 5 的总体方向。Sprint 5 不继续扩炼蛊、升转、杀招、升仙、仙窍、宝黄天，也不接 runtime AI；本轮只把 Sprint 4 已经实现的月光蛊闭环做成更可读、更有压迫感、更符合《蛊真人》质感的首批沉浸文本。

## 核心结论
- Sprint 5 第一优先级是沉浸文本入库，而不是新增玩法机制。
- 首批文本只覆盖月光蛊闭环：领取、重复领取、炼化、重复炼化、修行成功、未炼化禁用、元石不足、检查喂养、阶段收口余波。
- 内容版本升为 `s0.4.0`。
- 规则版本保持 `sprint4-rules-v1`，因为本轮不改 AP、元石、炼化、喂养、存档结构或行动结算。
- DeepSeek 只作为离线候选工具，默认 mock / dry-run；真实 API 只允许本机环境变量手动触发，候选不自动入库。

## 文本审校标准
候选文本必须服务五件事：
- 处境：玩家在哪里，面对什么秩序和视线。
- 选择：玩家刚做了什么，为什么这件事在蛊界有意义。
- 代价：AP、元石、债务、暴露、伤势、线索、蛊虫状态或窗口如何变化。
- 因果：这次行动如何推高或缓解下一步压力。
- 边界：不得生成规则层没有写入的奖励、蛊虫、杀招、传承、NPC命运或原著锚点变化。

禁止项：
- 把月光蛊写成普通装备。
- 把炼化写成点击升级。
- 跳过炼化、AP、元石或喂养压力。
- 在 `canon_strict` 中新增酒虫、稳定本命蛊、完整传承、杀招、方源硬绑定或尊者级硬改写。
- 保存 DeepSeek key、prompt、完整 response、模型名、thinking chain 或未审核草稿到仓库、存档或运行时账本。

## Development Phases
### Phase 1：冻结大纲与审校记录
新增本文档，并新增月光蛊候选文本审校记录，明确哪些候选被改写入库、哪些延期、哪些拒绝。

### Phase 2：生成或整理候选池
使用 `scripts/deepseek-candidates.mjs generate --mock` 生成默认候选池。真实 DeepSeek 输出只能留在 `.local/deepseek-candidates/`，不得自动进入 `content/s0/**/*.yaml`。

### Phase 3：人工审校后入库文本
更新 `content/s0/narratives/s0-local-narratives.yaml`，并同步 `crates/game-core/src/lib.rs` 的 `starter_narratives()` 与本地 fallback 文案，保证桌面运行时和 YAML bundle 一致。

### Phase 4：对话流与账本观感走查
重点走查“未领取 -> 领取 -> 未炼化禁用修行 -> 炼化 -> 修行成功 -> 检查喂养”。每一步都必须在对话流、最近反馈、Build 页或蛊虫账中给出清晰反馈。

### Phase 5：回归与候选冻结
复跑全量验证，新增 Sprint 5 候选冻结记录，记录 `s0.4.0 / sprint4-rules-v1`、入库槽位、未入库候选原因和剩余文本债务。

## Public Interfaces
- `ContentManifest.version`：升为 `s0.4.0`。
- `RULES_VERSION`：保持 `sprint4-rules-v1`。
- 叙事内容入口：`content/s0/narratives/s0-local-narratives.yaml`。
- Rust 内置 starter 同步入口：`starter_narratives()`。
- 候选工具入口：`scripts/deepseek-candidates.mjs`，候选输出目录为 `.local/deepseek-candidates/`，不入库。

## Test Plan
- `pnpm deepseek:test`
- `node scripts/deepseek-candidates.mjs generate --mock --out .local/deepseek-candidates`
- `node scripts/deepseek-candidates.mjs validate --dir .local/deepseek-candidates`
- `node scripts/deepseek-candidates.mjs redline`
- `pnpm content:build`
- `cargo test --workspace`
- `pnpm -r build`
- `powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\check-visible-text.ps1`
- `.\verify-game.cmd`

## 归档说明
Sprint 5 完成后，再决定 Sprint 6 是继续把沉浸文本扩展到黑市、药堂、传承四路径，还是推进炼蛊、升转、杀招雏形机制。
