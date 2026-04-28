# RebrnG Sprint 5 Candidate Freeze Review

## 文档角色
本文档是 Sprint 5 的候选冻结记录，用于说明当前分支已经完成“月光蛊闭环沉浸文本入库与走查打磨”的预期范围。它不是正式发布记录，不创建 tag，不合并 `main`，不创建 PR。

本轮只调整文本、内容版本和候选审校记录，不改变 AP、元石、炼化、喂养、存档结构或行动结算语义。

## 候选基线
- 分支：`codex/sprint5-moonlight-narrative-polish`
- 输入基线：`a8b4b82 docs: 冻结 Sprint 4 候选`
- 内容版本：`s0.4.0`
- 规则版本：`sprint4-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`
- DeepSeek 离线工具：`scripts/deepseek-candidates.mjs`
- DeepSeek 候选输出目录：`.local/deepseek-candidates/`，该目录不入库

## 已完成范围
- Phase 1：冻结 Sprint 5 总体大纲与审校标准，明确本轮只处理月光蛊闭环文本，不扩规则系统。
- Phase 2：使用现有 DeepSeek 离线工具生成 mock 候选池，候选保持 `needs_review`，不自动入库。
- Phase 3：人工审校并改写入库月光蛊闭环文本，同步 `content/s0` YAML 与 Rust starter 文本。
- Phase 4：将走查重点固定为“未领取 -> 领取 -> 未炼化禁用修行 -> 炼化 -> 修行成功 -> 检查喂养”的即时反馈链。
- Phase 5：复跑全量验证并冻结当前候选状态。

## 入库槽位
- `s0.gu.moonlight.claim`：领取月光蛊，强调制度登记、空窍牵引和蛊虫不是普通装备。
- `s0.gu.moonlight.refine`：炼化月光蛊，强调真元牵引、归属建立和控制权代价。
- `s0.gu.moonlight.inspect`：检查喂养压力，强调当前未断供但维护负担已进入账本。
- `s0.action.cultivate.moonlight`：月光修行成功，强调已炼化月光蛊、元石消耗和学堂比较压力。
- `s0.action.cultivate.moonlight_corner`：月光角修行成功，强调避开视线不等于避开制度。
- 修行禁用与资源不足反馈：由 Rust fallback 与行动投影文本补强，避免玩家误以为按钮失效。
- 阶段收口文本：补入月光蛊余波，说明炼化、喂养和制度压力还会继续追索。

## 候选审校结论
本轮 mock 候选共 `10` 条，全部通过候选 schema 校验。实际入库文本不是直接复制候选，而是按原著风险、规则一致性和当前 S0 状态重新人工改写。

已合并或部分吸收的方向：
- 领取、炼化、修行成功、未炼化禁用、元石不足、检查喂养、阶段收口余波。

暂缓或拒绝的方向：
- 重复领取提示暂缓入库为独立 narrative 槽位，当前由规则 fallback 处理。
- 炼化 AP 不足暂缓扩写为长正文，保留清晰禁用原因即可。
- 喂养警告不在本轮制造实际断供危机，避免误导玩家以为复杂喂养周期已经开放。

详细记录见：`docs/superpowers/reviews/2026-04-28-reborng-sprint5-moonlight-candidate-review.md`。

## 验收矩阵
- 启动入口检查：`.\start-game.cmd -CheckOnly` 已由 `.\verify-game.cmd` 覆盖并通过。
- 全量工程验证：`.\verify-game.cmd` 通过。
- DeepSeek 工具测试：`pnpm deepseek:test` 通过，`5/5` tests passed。
- DeepSeek 候选校验：`node scripts/deepseek-candidates.mjs validate --dir .local/deepseek-candidates` 通过，`10` valid，`0` invalid。
- DeepSeek runtime 红线：`node scripts/deepseek-candidates.mjs redline` 通过，扫描 `4` 类 runtime 路径，命中数 `0`。
- 内容构建：`pnpm content:build` 已由 `verify-game.cmd` 覆盖，输出 `s0.qingmao.foundation v s0.4.0`。
- Rust 回归：`cargo fmt --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace`、`cargo check -p rebrng-desktop` 已由 `verify-game.cmd` 覆盖并通过。
- 前端构建：`pnpm -r build` 已由 `verify-game.cmd` 覆盖并通过。
- 文本治理：`scripts/check-visible-text.ps1` 已由 `verify-game.cmd` 覆盖，未发现用户可见乱码残留。
- 架构红线：未发现 Express 主链、runtime AI proposal/narrator、React 持有完整 `GameState`、前端扫描 YAML、前端直接读写存档。

## 冻结结论
当前 Sprint 5 可以冻结为“月光蛊闭环文本走查候选”。它已经把 Sprint 4 的月光蛊规则状态转化为更容易被玩家理解的正文反馈和账本反馈，同时保持 `RULES_VERSION` 不变，说明本轮没有改变规则语义。

本候选仍不是正式发布版。它适合继续人工走查月光蛊领取、炼化、修行、检查喂养与阶段收口观感，但不应被视为完整蛊虫、炼蛊、升转或杀招系统。

## 剩余问题分层
### 必须修复
暂无。当前自动验证未暴露阻断项或高误导项。

### 可延期
- 重复领取、炼化 AP 不足、喂养临界警告仍主要依赖规则 fallback 或短提示，后续可按真实走查反馈决定是否扩成长正文。
- 本轮只覆盖月光蛊闭环，黑市、药堂、传承、学堂压力等路线仍需要后续同等文本厚度。
- DeepSeek 真实 API 输出尚未纳入本轮验收，当前工具链以 mock/dry-run 和人工审校流程为准。

### 设计待定
- Sprint 6 优先方向需要再定：继续做黑市/药堂/传承文本沉浸，还是推进凡人蛊虫的炼蛊、升转、杀招雏形机制。
- 酒虫、盗天相关机缘、本命蛊建立仍不得进入稳定 `canon_strict` 奖励链，只能作为传闻、IF 候选或后续系统。
- 若后续扩写更多正文，仍必须走候选、审校、YAML 入库、bundle 校验的离线流程，不能接入 runtime AI 主链。

## 后续建议
建议下一步用 `start-game.cmd` 进行一次专门的月光蛊走查：
- 开局 Build 页是否清楚显示“月光蛊：未领取”和“本命蛊：未建立”。
- 领取后是否能立即理解“这只是制度内接触/登记，不是已经稳定掌控”。
- 未炼化时修行禁用原因是否足够清楚。
- 炼化后核心蛊候选、空窍容器和喂养压力是否没有和本命蛊混写。
- 修行成功和阶段收口是否能让玩家感到资源、制度、蛊虫维护压力仍在逼近。
