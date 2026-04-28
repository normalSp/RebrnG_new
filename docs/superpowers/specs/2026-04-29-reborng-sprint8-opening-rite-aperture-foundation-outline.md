# RebrnG Sprint 8 开窍大典强锚点与凡人空窍基础机制大纲

## Summary
Sprint 8 把第一卷 Beat 1-2 从“剧情圣经 + 原著事实索引”落到可玩入口：设置层确认后不再直接进入自由窗口，而是先进入 `s0_opening_rite_anchor`。玩家经历地下溶洞、涉水、希望蛊入体与空窍初开后，才进入现有 S0 青茅山 8 自由窗口。

本 Sprint 的目标不是完整空窍冲刷、升转或杀招，而是建立凡人修行的底座状态：资质、涉水步数、元海容量、当前真元、真元质量、空窍壁膜、小境界和恢复说明都成为可保存、可投影、可测试的一等状态。

## Canon References
本 Sprint 必须引用 Sprint 7.2 原著事实索引，不允许把开窍大典写成普通属性抽卡。

- `opening_rite_cave_walk`：开窍大典在地下溶洞和水中行走中体现资质。
- `opening_rite_flower_sea_and_spiritual_spring`：希望蛊、花海与灵泉是仪式质感来源。
- `aptitude_step_ranges`：涉水步数与甲乙丙丁资质区间绑定。
- `hope_gu_enters_body`：希望蛊入体后开辟空窍。
- `aperture_primeval_sea_capacity`：资质对应元海容量，是修行速度与上限的基础。
- `aptitude_primeval_sea_and_cultivation_limits`：资质影响恢复、修行压力和资源依赖。
- `gu_master_definition`：蛊师以真元喂养、炼化、操控蛊虫。
- `primeval_essence_rank_colors`：一转青铜、二转赤铁、三转白银等真元质量为后续阶段保留。
- `aperture_wall_small_realm_progression`：光膜、水膜、石膜等空窍壁膜推进留作后续机制钩子。

## Runtime Flow
- `confirm_setup_run` 生成设置摘要和资源预览后，进入 `s0_opening_rite_anchor`。
- 开窍锚点不消耗自由窗口 AP，不增加 `free_rounds_elapsed`。
- 锚点行动顺序固定为 `enter_opening_cave -> cross_opening_river -> receive_hope_gu`。
- `receive_hope_gu` 完成后写入 `MortalApertureState`，同步 `aperture_opened = true`，然后进入 `day1_morning_free`。
- 未完成开窍前，领取月光蛊、炼化月光蛊、检查月光蛊、月光修行都必须被拒绝或禁用，并显示“开窍大典尚未完成”的中文原因。

## State Contract
`MortalApertureState` 是 Sprint 8 新增的规则状态，最小字段为：

- `opened`：空窍是否已开。
- `aptitude_grade`：甲、乙、丙、丁或未定。
- `opening_steps`：开窍大典涉水步数。
- `primeval_sea_capacity_percent`：元海容量百分比。
- `primeval_essence_current`：当前真元。
- `primeval_essence_quality`：当前真元质量，Sprint 8 只开放一转青铜。
- `aperture_wall_state`：空窍壁膜，Sprint 8 初始为光膜。
- `minor_realm`：小境界，Sprint 8 初始为一转初阶。
- `recovery_profile`：恢复说明，暂不展开复杂恢复公式。
- `opening_phase`：开窍锚点阶段，用于投影和存档回看。

## Aptitude Baseline
Sprint 8 先冻结代表值，不在本轮做平衡公式：

- 丁等：`10-19` 步，元海容量 `25%`。
- 丙等：`20-29` 步，元海容量 `44%`。
- 乙等：`30-39` 步，元海容量 `66%`。
- 甲等：`40-49` 步，元海容量 `88%`。

这些值用于状态、投影、文本和测试。后续若要调整平衡，必须升级规则版本并补回归。

## Mode Boundary
- `canon_strict`：只能出现可信的资质、空窍、希望蛊和月光蛊制度链；不能把强 IF 异象写成稳定奖励。
- `sandbox_if`：可以出现开窍异象、异常梦兆或压力线索，但只影响关注度、后续线索或锚点压力；不得直接生成酒虫、本命蛊、完整传承或越阶收益。

## UI Contract
- 对话流主区必须让玩家明确知道：地下溶洞、涉水、希望蛊入体、空窍初开、元海容量。
- 账本侧栏必须新增“空窍账”，展示资质、步数、元海、当前真元、真元质量、壁膜、小境界和恢复说明。
- Build 页必须继续分离 `核心蛊 / 辅助蛊 / 本命蛊`，空窍账不能替代蛊虫账。

## Non-Goals
- 不做完整空窍冲刷和小境界推进公式。
- 不做炼蛊、升转、杀招、本命蛊建立、升仙、仙窍、宝黄天。
- 不稳定发放酒虫或花酒行者主传承。
- 不接 runtime DeepSeek，不新增 AI 设置页或网络调用。

## Verification
- 设置确认后必须先进入 `s0_opening_rite_anchor`，不能直接进入自由窗口。
- 三段开窍锚点自然完成后，必须进入 `day1_morning_free`。
- `MortalApertureState` 必须可序列化并通过 `SaveEnvelope` 读回。
- `pnpm content:build` 输出内容版本 `s0.7.0`。
- `cargo test --workspace`、`pnpm -r build`、`scripts/check-visible-text.ps1`、`node scripts/validate-canon-index.mjs` 必须通过。

