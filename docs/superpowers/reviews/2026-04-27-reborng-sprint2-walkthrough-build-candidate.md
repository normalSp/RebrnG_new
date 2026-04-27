# RebrnG Sprint 2 走查版候选冻结

## 文档角色
本文是 Sprint 2 Phase 5 的候选冻结记录，用来说明当前分支是否已经达到“可稳定走查、可记录问题、可反复验证、可交给他人初步体验”的状态。它不是正式发布说明，不打 tag，不创建 PR，不合并 `main`，不扩展新玩法系统。

## 候选基线
- 分支：`codex/sprint-0-foundation`
- 候选输入提交：`8268d10 docs: 记录 Sprint 2 首轮走查反馈`
- 内容版本：`s0.1.2`
- 规则版本：`sprint1-rules-v2`
- 启动入口：`start-game.cmd`
- 工程验证入口：`verify-game.cmd`
- 桌面打包入口：`build-playtest.cmd`
- 首轮走查登记：`docs/superpowers/reviews/2026-04-27-reborng-sprint2-playtest-round1.md`

## 已完成范围
- Phase 1：已冻结 Sprint 2 走查打磨大纲，范围锁定为走查反馈、UI 可读性、验证脚本与本地走查包。
- Phase 2：已建立 `verify-game.cmd` 与 `build-playtest.cmd`，用于一键验收和桌面走查包尝试构建。
- Phase 3：已完成账本 UI 可读性打磨，行动分组、代价、风险、后果、禁用原因、最近反馈和阶段收口更适合走查。
- Phase 4：已记录首轮代理走查反馈；未发现 `阻断` 或 `高误导`，真人点击体感仍待你补充。
- Phase 5：本文冻结当前走查版候选。

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
| Tauri release exe | 通过 | `target/release/rebrng-desktop.exe` 已生成 |
| Tauri MSI 走查包 | 部分通过 | `target/release/bundle/msi/RebrnG_0.1.0_x64_en-US.msi` 已生成 |
| Tauri NSIS 打包 | 阻断 | 下载 `nsis-3.11.zip` 时 `timeout: global`，导致 `build-playtest.cmd` 最终返回失败 |

## 打包状态
`build-playtest.cmd` 当前能够完成内容构建、前端构建、Rust release 编译，并生成以下本地走查产物：

- `target/release/rebrng-desktop.exe`
- `target/release/bundle/msi/RebrnG_0.1.0_x64_en-US.msi`

但 Tauri 在 NSIS bundle 阶段需要下载：

- `https://github.com/tauri-apps/binary-releases/releases/download/nsis-3.11/nsis-3.11.zip`

本轮结果为 `timeout: global`，所以脚本最终退出码仍为失败。结论是：当前候选可以通过 `start-game.cmd` 做开发走查，也已有 MSI 产物可供本机尝试；若要把打包脚本变成全绿交付链，需要解决 NSIS 下载/缓存问题，或后续明确只产出 MSI。

## 剩余问题分层
### 必须修复
暂无。工程验证、自动化可玩性验收、文本扫描、UI 构建和核心红线均已通过。

### 可延期
- 真人点击体感仍待补充：行动侧栏信息密度、最近反馈醒目度、阶段收口提示是否足够清楚。
- 8 窗口压强目前由自动验收保障，仍需要你实际走 4 条路径确认体感是否“稳中偏紧”。
- `build-playtest.cmd` 的 NSIS 阶段仍有网络依赖 blocker；这不阻断 `start-game.cmd` 走查，但阻断完全绿色的双安装包产出。

### 设计待定
- DeepSeek/AI 仍不进入 runtime 主链；若要使用，只能另开离线内容工具方案。
- 视觉素材、图标、封面与走查包美术资源仍未纳入。
- 炼化、升转、杀招、升仙、仙窍、宝黄天继续后置，不进入 Sprint 2。

## 结论
当前分支可冻结为 Sprint 2 走查版候选。它已经满足 `start-game.cmd` 启动走查、`verify-game.cmd` 反复回归、Round 1 问题登记和本地 MSI 产物尝试的要求。下一步建议由你实际点击 4 条路径，把主观体验反馈补入 Round 1 走查记录；如出现 `阻断` 或 `高误导`，再进入 Sprint 2 第二轮缺陷回收，而不是继续横向扩新系统。
