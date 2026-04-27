# RebrnG Sprint 1 Phase 7 人工走查记录

## Summary

本记录用于 Sprint 1 后半程 Phase 7：在 Phase 6 中文可读性治理之后，对当前 8 自由窗口纵切做一次走查回收。走查目标不是新增规则或扩内容，而是确认当前基座是否具备继续进入 Phase 8 平衡调参的条件。

本轮采用“启动入口检查 + 四条固定路径自动验收 + 待用户主观手感补充”的方式记录。当前 Codex 终端环境无法可靠替代你在 Tauri 桌面窗口中的真实点击体验，因此本记录只把可自动验证的阻断、规则链路、路径终点和文本护栏写成结论；视觉节奏、按钮理解成本和压迫感强弱仍建议你用 `start-game.cmd` 亲自走一遍后补充。

## Review Method

- 启动入口：执行 `.\start-game.cmd -CheckOnly`，确认脚本、pnpm、cargo 入口可用。
- 四条路径：执行 `cargo test -p rebrng-game-core --test sprint1_playability_acceptance -- --nocapture`。
- 文本治理：沿用 Phase 6 新增的 `scripts/check-visible-text.ps1`，确认用户可见源无已知 mojibake 残留。
- 规则边界：本轮不改 AP、元石、债务、暴露、伤势、遭遇代价，不升级 `RULES_VERSION`。

## Path Results

### 月光根基

- 覆盖测试：`sprint1_moonlight_foundation_path_has_real_ap_and_resource_tradeoff`
- 结论：通过。
- 已验证：`canon_strict` 下可以通过侦查、月光修行和学堂压力处理走到阶段锚点；至少 2 次月光修行留下根基痕迹；元石消耗存在；阶段收口可进入 `foundation_established`。
- 待人工确认：行动提示是否足够让玩家理解“修行会消耗元石”和“学堂压力不是纯惩罚，而是制度内竞争压力”。

### 黑市跑路

- 覆盖测试：`sprint1_blackmarket_retreat_path_survives_without_turning_market_into_shop`
- 结论：通过。
- 已验证：黑市未解锁前不可见；通过风声解锁后才能接近；深夜进入暗口触发 `blackmarket_extortion`；选择跑路后存活、无重伤、暴露上升，且黑市遭遇不会重复触发成刷资源入口。
- 待人工确认：黑市“隐藏门路”在 UI 上是否足够像线索解锁，而不是突然冒出的商店按钮。

### 药堂债务恢复

- 覆盖测试：`sprint1_infirmary_debt_recovery_path_turns_injury_into_debt_pressure`
- 结论：通过。
- 已验证：硬顶黑市勒索可制造重伤；药堂恢复能把重伤降到轻伤；恢复会增加药堂债与人情债；投影中债务压力可见。
- 待人工确认：债务压力在状态条、关系局势和因果账之间是否形成足够清晰的闭环。

### 传承诱惑撤退

- 覆盖测试：`sprint1_inheritance_temptation_path_withdraws_from_high_risk_if_rumor`
- 结论：通过。
- 已验证：`sandbox_if` 下可以接近传承残线并获得 `rumor_inheritance_bamboo`；接触传承残线会带来暴露上升；撤回后没有稳定奖励收益；最终仍能自然进入锚点。
- 待人工确认：传承线的“诱惑大但不稳”是否在文案和按钮后果提示里足够明显。

## Findings

### 阻断

- 暂未发现自动验收层面的阻断。启动入口检查通过，四条 8 窗口路径均可自然抵达 `s0_anchor_pending`。

### 误导

- 暂未发现可自动确认的高误导问题。黑市隐藏、传承 IF、药堂债务和月光根基四条路径的核心边界均由 Rust 规则投影驱动。

### 体验弱

- 需要你在桌面窗口中确认行动侧栏的阅读负担。自动测试能证明按钮存在、禁用原因存在、路径能跑通，但不能证明玩家第一次看到时是否能快速理解“下一步该怕什么”。
- 需要你确认 8 窗口压强是否达到“稳中偏紧”。自动验收证明存在资源/债务/暴露变化，但真实体感是否太松或太硬，要等 Phase 8 前人工反馈。

### 文案弱

- Phase 6 后源码与内容包已通过 mojibake 扫描，暂未发现真实文件乱码。
- 终端输出仍可能因为 PowerShell 编码显示出伪乱码；这不是源码内容问题。人工走查应以桌面 UI 显示为准。

## Go / No-Go

当前状态可以进入 Phase 8：8 窗口平衡调参。建议 Phase 8 只围绕压强、AP、元石、债务、暴露和伤势压窗做小步调参，不新增炼化、升转、完整杀招、升仙、仙窍或宝黄天系统。

进入 Phase 8 前，仍建议你用 `start-game.cmd` 做一次真实点击走查，并把主观问题按“阻断 / 误导 / 体验弱 / 文案弱”补到本记录或下一份走查回收文档。
