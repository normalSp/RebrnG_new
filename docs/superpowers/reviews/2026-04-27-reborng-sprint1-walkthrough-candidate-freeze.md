# RebrnG Sprint 1 可走查候选冻结总结

## Summary

本文件冻结 `codex/sprint-0-foundation` 当前状态为 Sprint 1 可走查候选。冻结级别为“候选文档冻结”：不打 Git tag、不合并 `main`、不创建 PR，不宣称正式版本发布。

本轮只归档候选状态和验收结果，不改规则、不调数值、不扩内容、不接 runtime AI。

## Candidate Baseline

- 分支：`codex/sprint-0-foundation`
- 冻结前基线提交：`26788f0 test: 增加 Sprint 1 窗口平衡验收`
- 内容版本：`s0.1.2`
- 规则版本：`sprint1-rules-v2`
- 启动入口：`start-game.cmd`
- 当前范围：8 个自由窗口的青茅山首发纵切候选
- 已完成阶段：Phase 6 中文治理、Phase 7 走查记录、Phase 8 窗口平衡调参

## Acceptance Matrix

| 验收项 | 结果 | 证据 |
| --- | --- | --- |
| 启动入口可用 | 通过 | `.\start-game.cmd -CheckOnly` |
| 用户可见文本扫描 | 通过 | `scripts/check-visible-text.ps1` |
| Rust 格式检查 | 通过 | `cargo fmt --check` |
| Rust lint | 通过 | `cargo clippy --workspace --all-targets -- -D warnings` |
| 全量 Rust 测试 | 通过 | `cargo test --workspace` |
| 桌面壳 Rust 检查 | 通过 | `cargo check -p rebrng-desktop` |
| 内容 bundle 构建 | 通过 | `pnpm content:build`，输出 `s0.1.2` |
| 前端/桌面前端构建 | 通过 | `pnpm -r build` |
| 4 条 Sprint 1 可玩性路径 | 通过 | 月光根基、黑市跑路、药堂债务恢复、传承诱惑撤退 |
| 2 条 Phase 8 平衡验收 | 通过 | 日中 2 AP 基线、重伤恢复消耗窗口 |
| 性能红线 | 通过 | 现有验收断言 `resolve_action <300ms`、下一回合可交互 `<1s` |
| runtime AI 红线 | 通过 | 检索无 runtime AI `proposal/narrator` 主链 |
| React 边界 | 通过 | 检索前端无完整 `GameState` 持有、无 YAML 扫描、无直接文件读写 |

## Remaining Issues

### 必须修复

- 暂无。当前自动验收、文本扫描、启动入口检查和构建链均通过。

### 可延期

- 主观 UI 阅读负担：行动侧栏、禁用原因、代价提示是否足够一眼看懂，仍需你在桌面窗口中实际点击确认。
- 8 窗口压强体感：自动测试已证明存在 AP、元石、债务、暴露、伤势取舍，但真实体感是否“稳中偏紧”仍需人工走查判断。
- 按钮理解成本：黑市、药堂、传承、遭遇决断的短标签是否足够冷峻清晰，需要人工体验后再决定是否改文案。

### 设计待定

- DeepSeek / AI：继续保持不接 runtime 主链；如需使用，只作为后续离线内容工具另开冻结方案。
- 视觉素材：`apps/desktop/src/assets/` 与 `docs/art/` 当前不纳入候选冻结。
- 长局扩展：炼化、升转、完整杀招、升仙、仙窍、宝黄天仍不进入本候选范围。

## Freeze Decision

当前状态可以作为 Sprint 1 可走查候选交给人工继续点击验证。下一步建议优先让你使用 `start-game.cmd` 走一遍 4 条路径，并把主观问题按“阻断 / 误导 / 体验弱 / 文案弱”回填到后续走查记录。
