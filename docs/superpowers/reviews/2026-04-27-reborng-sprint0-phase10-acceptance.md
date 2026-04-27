# RebrnG Sprint 0 Phase 10 验收记录

## 审查范围

本记录只覆盖 Sprint 0 Phase 10：S0 8 自由窗口闭环、阶段收口投影、存档往返、性能红线和 AI 主链红线。

本阶段不再把“网络离线场景 / 无 key 场景 / 无模型配置场景”作为玩家功能验收项。保留的架构红线是：运行时 AI 不进入 `resolve_action`、Tauri command 或 UI 点击后的同步主链。

## 新增验收

- `crates/game-core/tests/s0_eight_round_performance.rs` 使用真实行动自然推进 8 个自由窗口，不手动改 `TimeState`。
- 固定剧本覆盖学堂侦查、月光修行、功绩告示、深夜黑市遭遇、跑路、药堂恢复、再次修行、月光角移动和锚点等待。
- `BuildState.moonlight_cultivation_marks` 记录月光修行痕迹，`LedgerViewModel.stage_closure` 派生 `站稳一转根基` 或 `重创可续`。
- `SaveEnvelope` 在 8 回合末尾序列化、反序列化并通过 `validate_for_load`。

## 性能记录

本轮命令：

```powershell
cargo test -p rebrng-game-core --test s0_eight_round_performance -- --nocapture
```

输出摘要：

- actions: `17`
- total_resolve_ms: `0`
- total_projection_ms: `0`
- save_load_ms: `0`
- 单次 `resolve_action_ms < 300ms`
- 下一回合可交互预算 `< 1s`

## 结论

Phase 10 的核心闭环验收已具备工程测试入口。Sprint 0 基座可以用这组测试防止后续重新引入分钟级等待、绕过统一行动管线、或把阶段收口做成 UI 临时判断。
