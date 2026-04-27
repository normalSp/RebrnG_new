# RebrnG Sprint 2 可走查版候选冻结

## 文档角色
本文是 Sprint 2 的候选冻结记录，用来说明当前分支是否已经达到“可反复走查、可记录问题、可交给他人初步体验”的工程状态。它不是正式发布说明，不打 tag，不合并 `main`，不扩展新玩法系统。

## 候选基线
- 分支：`codex/sprint-0-foundation`
- 内容版本：`s0.1.2`
- 规则版本：`sprint1-rules-v2`
- 主要启动入口：`start-game.cmd`
- 工程验证入口：`verify-game.cmd`
- 桌面打包入口：`build-playtest.cmd`
- 走查登记表：`docs/superpowers/reviews/2026-04-27-reborng-sprint2-playtest-round1.md`

## 本轮完成范围
- Phase 1：已冻结 Sprint 2 走查打磨大纲，范围锁定为走查反馈、UI 可读性、验证脚本与本地走查包。
- Phase 2：已新增一键验证脚本与桌面打包脚本，并新增首轮走查问题登记表。
- Phase 3：已打磨账本 UI 可读性，重点改善行动侧栏、禁用原因、最近反馈、阶段收口与走查摘要。
- Phase 4：暂无真实点击反馈输入，本轮未进行缺陷回收。
- Phase 5：本文作为 Sprint 2 可走查候选冻结记录。

## 验收矩阵
| 项目 | 结果 | 证据 |
| --- | --- | --- |
| 启动入口检查 | 通过 | `.\start-game.cmd -CheckOnly` 已由 `.\verify-game.cmd` 调用通过 |
| 用户可见文本扫描 | 通过 | `scripts/check-visible-text.ps1` 无 mojibake 命中 |
| Rust 格式与静态检查 | 通过 | `cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings` 通过 |
| Rust 全量测试 | 通过 | `cargo test --workspace` 通过，含 4 条 Sprint 1 可玩性路径与 2 条 Phase 8 平衡验收 |
| 桌面 Rust 检查 | 通过 | `cargo check -p rebrng-desktop` 通过 |
| 内容构建 | 通过 | `pnpm content:build` 输出 `target/rebrng-content/s0.bundle.json` |
| 前端构建 | 通过 | `pnpm -r build` 通过 |
| 红线检索 | 通过 | 无 Express 主链、无 runtime AI proposal/narrator、React 不持有 `GameState`、前端不扫描 YAML |
| Tauri release exe | 部分通过 | `build-playtest.cmd` 已生成 `target/release/rebrng-desktop.exe` |
| Tauri 安装包 bundle | 阻断 | WiX 下载 `wix314-binaries.zip` 时 `timeout: global`，属于打包依赖/网络 blocker |

## 打包状态
`build-playtest.cmd` 当前能完成内容构建、前端构建与 Rust release 编译，并生成裸 exe：

- `target/release/rebrng-desktop.exe`

但 Tauri 在 MSI bundle 阶段需要下载 WiX 工具包：

- `https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip`

本机本轮执行结果为 `timeout: global`。因此当前候选版可以通过 `start-game.cmd` 进行开发走查，也可以参考裸 exe 进行本机验证；若要分发安装包，需要先解决 WiX/网络依赖。

## 剩余问题分层
### 必须修复
暂无。工程验证、自动化可玩性验收、文本扫描和 UI 构建均已通过。

### 可延期
- 行动侧栏在内容继续增加后仍可能显得密，需要真实走查后决定是否折叠低优先级行动。
- 8 窗口压强目前由自动验收保障，体感还需要人工走查记录。
- 禁用原因与后果提示已更清楚，但是否足够“玩家一眼懂”仍需外部体验反馈。

### 设计待定
- DeepSeek/AI 仍不进入 runtime 主链；若要使用，只能另开离线内容工具方案。
- 视觉素材、图标、封面与走查包美术资源仍未纳入。
- 炼化、升转、杀招、升仙、仙窍、宝黄天继续后置，不进入 Sprint 2。

## 结论
当前分支可作为 Sprint 2 可走查候选：`start-game.cmd` 与 `verify-game.cmd` 可支撑反复走查和回归验证；`build-playtest.cmd` 已能暴露并记录桌面安装包 blocker。下一步建议先由玩家实际点击 4 条路径，把问题写入 Sprint 2 Round 1 走查登记表，再进入缺陷回收，而不是继续扩新系统。
