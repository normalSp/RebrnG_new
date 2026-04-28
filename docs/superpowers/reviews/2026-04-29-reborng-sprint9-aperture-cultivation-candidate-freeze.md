# RebrnG Sprint 9 空窍冲刷与真元轻公式候选冻结

## Summary
本记录冻结 Sprint 9 的可走查候选状态。Sprint 9 已把 Sprint 8 的空窍展示状态推进为可结算的凡人一转初期修行基座：真元当前值、自然恢复、调息回元、炼化消耗、月光修行消耗、空窍壁膜冲刷进度与首个小境界跃迁都已进入 Rust 规则层。

本轮仍不实现酒虫稳定获得、花酒传承奖励、完整炼蛊升转、杀招、升仙、仙窍或宝黄天。公式是首发轻量玩法基线，用于让玩家理解资质、真元、元海和修行压力之间的关系，不声称还原原著精确数值。

## Candidate Baseline
- 分支：`codex/sprint9-aperture-cultivation-loop`
- 内容版本：`s0.8.0`
- 规则版本：`sprint9-rules-v1`
- 启动入口：`start-game.cmd`
- 验证入口：`verify-game.cmd`
- 上位规格：`docs/superpowers/specs/2026-04-29-reborng-sprint9-aperture-cultivation-loop-outline.md`

## Frozen Formula
- 资质恢复：丁等 `4`、丙等 `7`、乙等 `11`、甲等 `15` 真元/窗口。
- 炼化月光蛊：消耗 `1 AP + 8 真元`，不扣元石。
- 月光修行：消耗 `1 AP + 1 元石 + 10 真元`。
- 空窍冲刷：丁等 `6`、丙等 `8`、乙等 `11`、甲等 `14` 进度/次。
- 小境界跃迁：冲刷进度达到 `100` 时，从 `一转初阶 / 光膜` 推进到 `一转中阶 / 水膜`，当前只开放首个跃迁。
- 调息回元：新增行动“调息回元”，消耗 `1 AP`，按资质恢复真元，且不超过元海容量。
- 窗口恢复：自由窗口推进时自动恢复一次真元，且不超过元海容量。

## Verification Matrix
- `cargo fmt --check`：通过。
- `cargo clippy --workspace --all-targets -- -D warnings`：通过。
- `cargo test --workspace`：通过，包含 Sprint 9 新增 `9` 个空窍修行回归测试。
- `cargo check -p rebrng-desktop`：通过。
- `pnpm content:build`：通过，输出 `s0.8.0` 内容 bundle。
- `pnpm -r build`：通过。
- `scripts/check-visible-text.ps1`：通过。
- `node scripts/validate-canon-index.mjs`：通过，source map `199` 节，事实卡 `14` 张，beat 引用 `27` 条。

## Walkthrough Route
1. 运行 `start-game.cmd`。
2. 点击“新开一局”，选择出身与 3 个天赋。
3. 走完开窍大典三段锚点，确认空窍账显示资质、元海容量、当前真元、自然恢复与壁膜状态。
4. 进入自由窗口后，领取月光蛊。
5. 炼化月光蛊，观察真元减少 `8`，月光蛊进入空窍并显示已炼化。
6. 点击“调息回元”，观察当前真元按资质恢复，但不超过元海容量。
7. 点击月光修行，观察消耗 `1 AP + 1 元石 + 10 真元`，空窍冲刷进度上升。
8. 在空窍账、Build 页、对话流和最近反馈中确认：真元不足、自然恢复、冲刷进度和小境界提示都能被看见。

## Remaining Risks
- `必须修复`：暂无，当前自动验收通过。
- `可延期`：首个小境界跃迁在 8 窗口内通常不一定自然触发，后续长局或专门修行线需要再平衡。
- `设计待定`：酒虫对修行效率的影响、空窍壁膜多阶段推进、炼蛊升转、杀招消耗、长修行跳时。
- `原著风险`：本轮没有开放酒虫稳定获得、花酒完整传承、本命蛊建立或无代价越阶收益，canon 严谨边界保持不变。

## Next Recommendation
下一步更适合进入 Sprint 10：在当前修行底座稳定后，接入“酒虫/花酒外围残线”的剧情与机制边界。建议先冻结酒虫作为诱惑、传闻、残线或高代价 IF 的处理方式，再决定是否实现真实获得、强化修行效率或只做第一阶段外围线索。
