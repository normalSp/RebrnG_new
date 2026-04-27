# RebrnG Sprint 2.2 点击反馈与设定收口记录

## Summary

本轮根据真人走查反馈处理两个高误导风险：移动点击后反馈不明显，以及 S0 开局没有冻结开窍大典语义。文本量不足已登记为体验弱项，留给后续 DeepSeek 离线内容扩写管线，不接入 runtime 主链。

当前基线：
- 分支：`codex/sprint-0-foundation`
- 内容版本：`s0.1.2`
- 规则版本：`sprint2-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`

## Findings And Handling

- `误导 / 已处理`：点击移动后，旧移动按钮消失或不可用，玩家容易误以为按钮锁死。本轮把“当前位置已记为”和“最近落账”前置到主界面，并让结算状态显示“已移动到：节点名”。
- `误导 / 已处理`：未冻结开窍大典开局，可能产生“未开窍也能修行”的原著设定冲突。本轮新增 `aperture_opened` 状态，S0 默认空窍已开，未开窍状态下修行会被拒绝。
- `体验弱 / 延期`：文本量偏少，剧情推进感不足。本轮只登记，不扩文本，不接 runtime AI。
- `工具 / 已冻结计划`：短期优先用 Jam 录屏回收点击问题；Tauri WebDriver 作为后续 spike；`tauri-pilot` 仅作为候选调研。

## Verification

已执行：
- `.\verify-game.cmd`

验证结果：
- 启动入口检查通过。
- 用户可见文本扫描通过，扫描范围已扩展到 specs、plans、reviews。
- Rust format、clippy、workspace tests 通过。
- 桌面 Rust check、内容 bundle 构建、前端构建通过。
- 红线检查通过：无 Express 主链、无 runtime AI proposal/narrator、React 不持有完整 `GameState`、前端不扫描 YAML。

## Next Step

继续真人走查 4 条路径，重点复核：
- 点击移动后是否能立刻理解当前位置变化。
- 开窍大典和空窍状态是否足够明确。
- 文本量不足是否已经影响主线理解到需要提前启动离线扩写。
