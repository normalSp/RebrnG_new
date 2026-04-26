# RebrnG Rust 核心增强版架构冻结审查

状态：frozen

更新时间：2026-04-27

审查范围：
- `docs/superpowers/specs/2026-04-26-reborng-technical-architecture-design.md`
- `docs/superpowers/specs/2026-04-26-reborng-launch-vertical-slice-implementation-plan.md`
- `docs/superpowers/reviews/2026-04-26-reborng-architecture-readiness-review.md`

## 审查结论

当前架构文档足够支撑后续底座开发。

结论理由：

- 技术栈已冻结为 `Rust 核心 + Tauri invoke + React 账本 UI`。
- 规则真相、内容 bundle、存档迁移和性能敏感路径已归入 Rust。
- UI 只消费账本投影并提交 `ActionCommand`。
- Web 试玩壳、Express / 本地 HTTP API、运行时 AI proposal、运行时 AI narrator 均已明确排除出首发主链。
- 性能红线已经明确到 `resolve_action <300ms` 与下一回合可交互 `<1s`。

本审查不要求继续扩写玩法规格。

## 已补冻结口径

### 1. 环境前置

底座开发开始前必须满足：

- Rust stable toolchain 通过 `rustup` 安装。
- `rustc` 与 `cargo` 可在项目 shell 中访问。
- `pnpm` 通过 Corepack 启用。
- Windows Tauri 前置依赖在脚手架前安装完成。

当前机器只读检查显示：

- `node v22.10.0` 可用。
- `npm 10.9.0` 可用。
- `rustc` / `cargo` 当前不可用。
- `pnpm` 当前不可用。

因此，实际底座开发的第一步必须是安装或启用工具链。

### 2. 内容构建入口

内容构建冻结为：

```text
content/**/*.yaml
  -> crates/content-tools
  -> s0.bundle.json
```

`crates/content-tools` 是 YAML 到运行时 bundle 的唯一权威构建器。

约束：

- `crates/content-tools` 必须复用 `crates/game-core` 内容类型。
- pnpm 可以调用构建命令，但不拥有 schema 真相。
- 运行时只读取已构建 bundle，不扫描 YAML、JSONL 或原著语料。

### 3. Tauri 运行态托管

Sprint 0 只支持单个 active run。

由 Tauri / Rust 托管：

- 当前 `GameState`
- 当前 `ContentBundle`
- 当前 `CausalityLedger`
- 当前 RNG 状态
- 当前存档 slot 指针

React 只保留 UI 临时状态，不保存完整规则状态。

### 4. Command 包络

Tauri commands 成功时返回：

- `ActionResponse`
- `LedgerViewModel`
- `SaveWriteResult`
- `ContentManifest`

失败时返回明确错误对象。

错误分类至少覆盖：

- `validation`
- `content`
- `save`
- `io`
- `internal`

错误对象必须能给 UI 展示摘要，也能给开发调试信息，但不得泄露隐藏变量原值。

### 5. 性能计时

Rust command 内部负责本地计时。

`performance` 至少保留：

- `resolve_action_ms`
- `projection_ms`
- `save_load_ms`
- `bundle_load_ms`

React 不负责估算规则耗时。

## 仍需在实际开发中落地的文件

底座开发时应创建但本轮不创建：

- `rust-toolchain.toml`
- `Cargo.toml`
- `crates/game-core/`
- `crates/content-tools/`
- `apps/desktop/`
- `packages/ui-ledger/`
- `content/s0/`
- `package.json`
- `pnpm-lock.yaml`

这些属于实际底座开发，不属于本轮文档冻结。

## 风险与处理方式

### 1. Rust / pnpm 当前未安装

风险：

- 无法立即运行 `cargo test` 或 `pnpm`。

处理：

- 底座开发 Sprint 0 的第一步安装或启用工具链。
- 本轮不伪造测试结果，不运行不存在的 Rust 工程测试。

### 2. Rust/Tauri 边界复杂度高于 TS-only

风险：

- 过早扩玩法会拖慢底座验证。

处理：

- Sprint 0 只做单 active run、最小 commands、最小 bundle 和最小账本投影。
- 不做完整 8 回合内容，不做多存档列表，不做多 run 管理。

### 3. 内容工具链可能重新发明 schema

风险：

- `crates/content-tools` 与 `crates/game-core` 字段语义分裂。

处理：

- 明确 `crates/content-tools` 必须复用 `crates/game-core` 类型。
- pnpm 只做命令编排，不做 schema 真相源。

## 架构冻结结论

可以冻结 Rust 核心增强版架构基线。

下一步进入：

1. 提交本轮文档冻结。
2. 新建底座开发分支或 worktree。
3. 开始 Sprint 0：工具链安装、Tauri/Rust/React 骨架、最小 Rust core、最小 content bundle、Tauri invoke 和性能计时。

不建议在 Sprint 0 前继续横向扩写高阶玩法规格。

## 归档说明

当前归档路径：

`docs/superpowers/reviews/2026-04-27-reborng-rust-architecture-freeze-review.md`
