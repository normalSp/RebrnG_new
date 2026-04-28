# Sprint 6 可走查候选冻结记录

## 候选基线

- 分支：`codex/sprint6-route-narrative-polish`
- 内容版本：`s0.5.0`
- 规则版本：`sprint4-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`
- 本轮性质：四路线沉浸文本扩写，不改规则、不扩系统

## 完成内容

- 冻结 Sprint 6 总体大纲。
- 新增 Sprint 6 目标槽位清单。
- 扩展 DeepSeek 离线候选工具，支持 `--targets <file>`。
- 使用真实 DeepSeek V4-Pro 生成 15 个离线候选。
- 完成候选审校，拒绝不合世界观、现代化或规则越权文本。
- 入库功绩、药堂、黑市、传承和学堂公开压力文本。
- 同步 YAML 内容源与 Rust starter 内容。

## 验收矩阵

| 项目 | 结论 |
| --- | --- |
| 候选工具测试 | `pnpm deepseek:test` 通过，8 项测试全绿 |
| DeepSeek 候选校验 | `node scripts/deepseek-candidates.mjs validate --dir .local/deepseek-candidates/sprint6` 通过，15 个候选有效 |
| runtime 红线 | `node scripts/deepseek-candidates.mjs redline` 通过，runtime 路径无 DeepSeek/API 调用 |
| 内容构建 | `pnpm content:build` 通过，输出 `s0.5.0` |
| Rust 回归 | `.\verify-game.cmd` 通过，包含 Rust fmt、clippy、workspace tests 和 desktop check |
| 前端构建 | `pnpm -r build` 通过 |
| 可见文本扫描 | `scripts/check-visible-text.ps1` 通过 |

## 剩余问题

### 必须修复

- 暂无。

### 可延期

- 功绩审计文本当前仍通过同一 scout target 映射，未来可考虑让行动 id 进入叙事选择，但本轮不改规则管线。
- 黑市和传承仍是 8 窗口内的短反馈，长剧情扩写留给后续内容 Sprint。

### 设计待定

- Sprint 7 是继续补厚巷道试探与旁支落脚点，还是推进炼蛊/升转/杀招雏形，需要另行选择。

## 红线

- 不提交 `.local/deepseek-candidates/`。
- 不保存 DeepSeek 密钥、prompt、完整 response、模型名或 thinking chain。
- 不接 runtime AI。
- 不新增高阶系统。
