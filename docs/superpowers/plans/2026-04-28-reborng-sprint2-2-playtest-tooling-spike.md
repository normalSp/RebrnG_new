# RebrnG Sprint 2.2 走查工具 Spike 计划

## Summary

本计划用于评估“让我也能更像真人一样看见、点击、截图”的工具链。结论先冻结为轻量 spike：短期优先使用 Jam 录屏回收真人走查问题；工程自动化优先调研 Tauri 官方 WebDriver 路线；`tauri-pilot` 只作为候选研究，不直接进入主分支。

## 工具分层

- `Jam`：最适合当前真人走查回收。你录一圈，我读取截图、点击事件、console 和可见状态，能定位“点了但不知道成功没”的反馈问题。
- `Tauri WebDriver / tauri-driver`：官方可自动化桌面壳的方向，适合未来做启动、点击、截图断言。
- `Playwright MCP / browser-use`：适合 Vite 浏览器页面或 Web 试玩壳，但当前正式入口是 Tauri command + Rust active run，不能完全代表桌面主链。
- `tauri-pilot`：看起来更贴近 AI inspect/click Tauri 的目标，但成熟度和维护风险需要单独试验，不作为本轮依赖。

## Spike 验收

首个 spike 只验证四件事：

- 能启动 Tauri 桌面壳。
- 能点击“新开一局”。
- 能点击一个移动行动。
- 能截图或读取 DOM，断言当前位置和最近反馈发生变化。

## 非目标

- 不用 spike 替代真人体验判断。
- 不引入 runtime AI。
- 不把不稳定插件纳入主线构建。
- 不为了工具测试重开 Web 试玩壳。
