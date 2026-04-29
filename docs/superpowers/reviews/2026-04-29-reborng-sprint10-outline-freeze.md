# RebrnG Sprint 10 指导大纲冻结记录

## 冻结范围

本轮冻结 Sprint 10 的指导大纲与内容生产接口，不进入运行时实现。

- 分支：`codex/sprint10-liquor-worm-flower-wine-outer-trail`
- 上位基线：`codex/sprint9-aperture-cultivation-loop`
- 运行时内容版本：保持 Sprint 9 当前版本，未改 `content/s0`
- 规则版本：保持 Sprint 9 当前版本，未改 `GameState` 或结算规则

## 新增产物

- `docs/superpowers/specs/2026-04-29-reborng-sprint10-liquor-worm-flower-wine-outer-trail-outline.md`
- `docs/superpowers/data/sprint10-liquor-worm-flower-wine-targets.json`
- `docs/superpowers/data/canon/first-volume-canon-index.json` 新增酒虫 / 花酒外围事实卡
- `docs/superpowers/data/first-volume-story-beats.json` 回填 Beat 3-4 的 `canon_index_refs`
- `scripts/validate-canon-index.mjs` 增加 Sprint 10 target 引用校验

## 设计结论

Sprint 10 的目标不是发放酒虫，而是让玩家接触原著核心机缘的外围压力。

`canon_strict` 下，玩家只能通过风声、误导、买酒试探、夜探失败、假入口、撤退和被盯上来感到这条线的存在。酒虫实例、花酒核心密洞、留影存声、核心历史证据和方源主机缘必须继续被锚点保护。

`sandbox_if` 下可以提供更强诱惑，例如疑似酒虫目击、错位残影或外围痕迹，但必须伴随资源亏损、暴露、追索、伤势、路线断裂或锚点压力。

## 原著索引补强

新增事实卡覆盖以下内容：

- 家族公开版本中的花酒行者传闻
- 方源依靠重生记忆追查酒虫，但并不知道精确地点
- 原时间线中醉酒蛊师与酒香引出酒虫的传闻
- 青竹酒、元石消耗、夜探与失败成本
- 酒虫警觉、嗜酒、会飞和诱饮跟踪窗口
- 花酒外围洞口、水路、岩缝和空间危险
- 花酒核心真相与证据红线

这些事实卡均为转述事实和行号引用，不包含长段原文。

## 后续实现建议

下一轮若进入 Sprint 10 可玩实现，建议新增轻量追查状态：

- `WineTrailState`：酒香试探、买酒次数、夜探痕迹、被盯程度。
- `InheritanceTrailState`：外围残线、假入口、撤退记录、锚点压力。

若新增以上状态，应升级规则版本到 `sprint10-rules-v1`；若只新增文本和 YAML 内容，可只升级内容版本到 `s0.9.0`。

## 红线

- 不稳定获得酒虫。
- 不炼化酒虫。
- 不成为花酒遗藏核心发现者。
- 不进入核心密洞。
- 不取得留影存声、骨骸、核心遗藏或可推翻族史的硬证据。
- 不改写方源主机缘。
- DeepSeek 候选必须引用事实卡，不得自由续写第一卷。

## 走查方式

本轮没有新增运行时内容，因此 `start-game.cmd` 不会出现新的酒虫 / 花酒行动。走查重点是文档与数据：

1. 查看 Sprint 10 指导大纲，确认严谨模式和 IF 模式边界。
2. 查看 `sprint10-liquor-worm-flower-wine-targets.json`，确认每个槽位都有 `canon_index_refs` 与红线。
3. 运行 `node scripts/validate-canon-index.mjs`，确认引用完整。

等后续实现可玩外围残线后，再从游戏内走查“风声 -> 买酒 -> 夜探 -> 假线 / 撤退 / 被盯”的路径。
