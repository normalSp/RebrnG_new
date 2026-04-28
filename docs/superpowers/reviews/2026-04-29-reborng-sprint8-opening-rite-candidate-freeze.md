# RebrnG Sprint 8 开窍大典与空窍基础机制可走查候选冻结

## Summary
本候选冻结记录 Sprint 8 的可走查状态：设置层确认后进入开窍大典强制锚点，完成地下溶洞、涉水、希望蛊入体三段后，生成凡人空窍基础状态并进入现有 S0 自由窗口。

- 分支：`codex/sprint8-opening-rite-aperture-foundation`
- 内容版本：`s0.7.0`
- 规则版本：`sprint8-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`

## Completed Scope
- 新增 `MortalApertureState`，保存资质、涉水步数、元海容量、当前真元、真元质量、空窍壁膜、小境界、恢复说明和开窍阶段。
- 设置确认后进入 `s0_opening_rite_anchor`，不直接进入自由窗口。
- 三段开窍行动 `enter_opening_cave -> cross_opening_river -> receive_hope_gu` 全部走 `ActionCommand` 管线。
- 完成开窍后进入 `day1_morning_free`，并允许后续月光蛊领取、炼化和修行闭环继续运行。
- 内容层将开窍结果 profile 扩展为可校验字段，并将内容版本升为 `s0.7.0`。
- UI 增加“空窍账”，Build 页同时显示空窍账与蛊虫账，继续分离核心蛊、辅助蛊、本命蛊。
- 新增 Sprint 8 规格与 DeepSeek 候选槽位，候选槽位引用 `canon_index_refs`。

## Verification Matrix
- `verify-game.cmd`：通过。
- `cargo test --workspace`：通过。
- `cargo clippy --workspace --all-targets -- -D warnings`：通过。
- `cargo check -p rebrng-desktop`：通过。
- `pnpm content:build`：通过，输出 `s0.7.0`。
- `pnpm -r build`：通过。
- `scripts/check-visible-text.ps1`：通过。
- `node scripts/validate-canon-index.mjs`：通过，第一卷 source map、事实卡和 beat 引用有效。
- `start-game.cmd -CheckOnly`：通过。
- `git diff --check`：无格式错误，仅有 Windows 换行提示。

## Walkthrough Focus
本轮走查建议从 `start-game.cmd` 新开局开始：

1. 选择 1 个出身和 3 个天赋。
2. 确认后检查是否进入“开窍大典”强制锚点，而不是直接进入自由窗口。
3. 依次点击进入地下溶洞、涉水前行、接纳希望蛊。
4. 检查对话流是否讲清地下溶洞、涉水、希望蛊入体和空窍初开。
5. 检查 Build 页和账本侧栏中的“空窍账”：资质、步数、元海、真元、壁膜、小境界是否清晰。
6. 完成开窍后继续领取月光蛊、炼化月光蛊、进行月光修行，确认旧 S0 核心循环没有断。

## Remaining Deferred Work
- 空窍冲刷、小境界推进公式仍未开放，建议 Sprint 9 后续处理。
- 真元恢复、元石补充、资质对修行效率的量化影响仍是状态钩子级。
- 开窍大典长正文仍可继续通过 DeepSeek 离线候选扩写，但不得接入 runtime 主链。
- 酒虫、本命蛊建立、炼蛊升转、杀招、升仙、仙窍、宝黄天仍不在本轮范围。

