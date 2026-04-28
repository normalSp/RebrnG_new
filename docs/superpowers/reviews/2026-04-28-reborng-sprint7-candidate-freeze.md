# RebrnG Sprint 7 候选冻结记录

Status: walkthrough candidate  
Date: 2026-04-28  
Branch: codex/sprint7-first-volume-story-outline

## Candidate Baseline

- Content version: `s0.6.0`
- Rules version: `sprint4-rules-v1`
- Runtime AI: not connected
- Startup entry: `start-game.cmd`
- Verification entry: `verify-game.cmd`

## Completed Scope

- 新增第一卷剧情大纲冻结文档：`docs/superpowers/specs/2026-04-28-reborng-sprint7-first-volume-story-outline.md`
- 新增第一卷节拍清单：`docs/superpowers/data/sprint7-first-volume-beats.json`
- 新增 Sprint 7 叙事候选槽位：`docs/superpowers/data/sprint7-narrative-targets.json`
- 将开篇试制文本入库到 `content/s0/narratives/s0-local-narratives.yaml`
- 同步 Rust 内置 `starter_narratives()`，保证桌面运行时和 YAML bundle 一致
- 将 `ContentManifest.version` 与 `STARTER_CONTENT_VERSION` 升为 `s0.6.0`

## Narrative Scope

本轮只实现“开窍大典到学堂初压”的长正文试制：

- 学堂门前开局场景
- 月光蛊领取
- 月光蛊炼化
- 月光修行
- 学堂门前风声与方源邻近余波
- 月光角侦查
- 学堂公开压力触发、忍让、争辩、硬顶

本轮未实现：

- 完整第一卷可玩内容
- 空窍冲刷、小境界推进、蛊虫升转、完整炼蛊、杀招雏形
- 酒虫稳定获得、完整花酒行者传承、方源硬绑定、提前改写青茅山结局

## DeepSeek Candidate Note

已尝试使用 Sprint 7 目标槽位执行真实 DeepSeek 离线候选生成：

```powershell
node scripts/deepseek-candidates.mjs generate --allow-network --targets docs/superpowers/data/sprint7-narrative-targets.json --out .local/deepseek-candidates/sprint7
```

本次调用超过本轮工程等待窗口后未产出候选文件，因此未把任何 DeepSeek 原始候选纳入提交。入库文本为人工按冻结槽位试制文本，仍遵守离线候选规则：不提交 key、prompt、完整 response、模型名或 thinking chain。

## Canon Risk Review

- `canon_strict` 没有稳定发放酒虫。
- `canon_strict` 没有完整开放花酒行者传承。
- 方源只作为学堂风声中的危险邻近余波出现，没有绑定、合作、控制或命运改写。
- 月光蛊仍需要领取、炼化和元石/AP 修行代价。
- 本命蛊仍保持“未建立”语义，不在 Sprint 7 中开放。

## Verification Matrix

- `pnpm content:build`: passed, output `s0.6.0`
- `powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\check-visible-text.ps1`: passed
- `node scripts/deepseek-candidates.mjs redline`: passed
- `.\verify-game.cmd`: passed
- `git diff --check`: passed with only existing Windows LF/CRLF warnings

## Walkthrough Focus

走查时重点确认：

- 新开局进入设置层后，开窍大典不再像背景设定，而是第一卷入口。
- 确认开局进入 S0 后，学堂门前正文有明显长正文密度。
- 领取月光蛊后，文本能说明“领取不等于炼化”。
- 炼化月光蛊后，文本能说明归属、空窍和后续喂养维护压力。
- 月光修行后，文本能说明元石、窗口和学堂视线压力。
- 触发学堂公开压力后，忍让、争辩、硬顶都不是单纯胜负，而是不同后账。
