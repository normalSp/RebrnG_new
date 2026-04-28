# RebrnG Sprint 4 Candidate Freeze Review

## 文档角色
本文件是 Sprint 4 Phase 6 的候选冻结记录，用于说明当前分支已经达到“可走查候选”状态。它不是正式版本发布记录，不创建 tag，不合并 `main`，不创建 PR。

本轮不修改规则、不扩内容、不接 runtime AI、不调整数值，只记录验证证据、剩余风险和下一步建议。

## 候选基线
- 分支：`codex/sprint4-mortal-gu-foundation`
- 输入提交：`fffd2d1 feat: 增加 DeepSeek 离线候选工具`
- 内容版本：`s0.3.1`
- 规则版本：`sprint4-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`
- DeepSeek 离线工具：`scripts/deepseek-candidates.mjs`
- DeepSeek 候选输出目录：`.local/deepseek-candidates/`，该目录不入库

## 已完成范围
- Phase 1：冻结凡人蛊虫玩法边界，明确 `GuSpec / GuInstance / GuInventoryState / RefinementState / FeedingState / ConditionState` 的 Sprint 4 最小合同。
- Phase 2：扩展内容 schema，加入 `ContentGuSpec` 与稳定 `canon_strict` 月光蛊内容 `moonlight_gu`。
- Phase 3：实现 Rust 月光蛊状态与行动闭环，覆盖领取、炼化、检查喂养、已炼化后修行。
- Phase 4：更新账本、Build 与对话流展示，让月光蛊状态、容器、喂养压力、核心蛊候选、本命蛊未建立可见。
- Phase 5：新增 DeepSeek 离线候选工具，固定 mock/dry-run 默认、显式联网、候选 JSON 校验和 runtime 红线。
- Phase 6：执行回归验证并冻结当前可走查候选。

## 验收矩阵
- 启动入口检查：`.\start-game.cmd -CheckOnly` 通过。
- 全量工程验证：`.\verify-game.cmd` 通过。
- DeepSeek 工具测试：`pnpm deepseek:test` 通过，`5/5` tests passed。
- DeepSeek runtime 红线：`node scripts/deepseek-candidates.mjs redline` 通过，扫描 `4` 类 runtime 路径，命中数 `0`。
- 内容构建：`pnpm content:build` 通过，输出 `s0.qingmao.foundation v s0.3.1` 到 `target/rebrng-content/s0.bundle.json`。
- 前端构建：`pnpm -r build` 通过。
- 文本治理：`scripts/check-visible-text.ps1` 已由 `verify-game.cmd` 覆盖，未发现用户可见乱码残留。
- Rust 回归：`cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、`cargo check -p rebrng-desktop` 已由 `verify-game.cmd` 覆盖。
- 架构红线：未发现 Express 主链、runtime AI proposal/narrator、React 持有完整 `GameState`、前端扫描 YAML、前端直接读写存档。

## 冻结结论
当前 Sprint 4 可以冻结为“可走查候选”。它已经把月光蛊从抽象 Build 文案推进到可保存、可投影、可验证的一等规则状态，并且没有破坏既有 S0 8 窗口循环、设置层、对话流壳、账本 UI 和离线内容工具边界。

本候选仍不是正式发布版。它适合继续人工走查月光蛊领取、炼化、修行、喂养提示、Build 页和对话流反馈，但不应被视为完整凡人蛊虫系统。

## 剩余问题分层
### 必须修复
暂无。当前自动验证未暴露阻断项或高误导项。

### 可延期
- 月光蛊闭环的真人体感仍需走查：领取 0 AP、炼化 1 AP、修行 1 AP/1 元石是否在 8 窗口内压强合适。
- 文本沉浸感仍偏短，DeepSeek 离线候选工具已就绪，但候选文本尚未经过人工审校入库。
- `build-playtest` 打包链不在本轮重新收口；本候选仍以开发启动入口和工程验证为主。
- 蛊虫喂养目前是可见压力与状态接口，不是完整周期系统。

### 设计待定
- Sprint 5 优先方向需要再选：继续用 DeepSeek 离线候选扩写 S0 文本，或推进炼化/炼蛊/升转/杀招雏形。
- 酒虫、盗天相关机缘、本命蛊建立仍不能进入稳定 `canon_strict` 奖励链，只能作为传闻、IF 候选或后续系统。
- 后续若让离线候选入库，必须先经过证据等级、模式许可、原著风险、规则一致性和文本质量审核。

## 后续建议
建议下一步先由人工走查 Sprint 4 候选，重点观察：
- 新开局后月光蛊“未领取 / 未炼化 / 已炼化”的可见性是否足够。
- 修行禁用原因是否让玩家理解“没有炼化月光蛊不能稳定修行”。
- 炼化月光蛊后的核心蛊候选、本命蛊未建立、喂养压力是否没有混写。
- DeepSeek 离线候选是否应优先用于补足领取、炼化、修行失败、喂养检查等关键文本槽位。

通过人工走查后，再决定 Sprint 5 是内容沉浸扩写优先，还是继续推进凡人蛊虫的炼化、喂养、升转与杀招前置系统。
