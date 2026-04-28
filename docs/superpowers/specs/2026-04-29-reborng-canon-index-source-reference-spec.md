# RebrnG 原著事实索引与来源引用规格

Status: frozen candidate  
Date: 2026-04-29  
Depends on:
- `2026-04-28-reborng-first-volume-story-bible.md`
- `docs/superpowers/data/first-volume-story-beats.json`
- 本地私有源文件 `reverend-insanity.txt`

## Summary
本规格冻结“剧情圣经 + 原著事实索引”的双层资料结构。剧情圣经负责第一卷宏观主线、玩家参与边界、IF 偏移和禁改事实；原著事实索引负责保存可复用的细节证据，例如开窍大典的地下溶洞、水中行走、希望蛊入体、空窍与元海、真元颜色、炼化阻力和空窍壁膜推进。

索引层只存“转述事实 + 行号来源 + 游戏用途”，不保存长段原文，不进入内容 bundle，不进入存档。后续 DeepSeek V4-Pro 扩写只能吃剧情 beat、`CanonFactCard` 和红线，不允许直接自由续写整本原文。

## Why This Layer Exists
如果把所有原著细节直接塞进剧情圣经，文档会迅速膨胀，而且主线结构会被细枝末节淹没。如果只保留宏观剧情，后续扩写又容易忘掉关键世界观质感。索引层把二者拆开：

- 剧情圣经：回答“第一卷怎样分段、玩家能影响哪里、严谨模式哪些不能改”。
- 原著索引：回答“某个剧情槽位要引用哪些具体世界观事实，哪些细节不能写错”。
- DeepSeek 候选：只基于 `NarrativeSlot + CanonFactCard + 红线` 扩写，不自由发明事实。

## Source Handling Rules
- `reverend-insanity.txt` 视为本地私有 source，不提交 Git，不进入内容包，不进入构建产物。
- `scripts/extract-canon-source-map.mjs` 只读本地 source，输出章节行号、编码与原始文件 hash。
- `first-volume-source-map.json` 只允许保存章节范围和 hash，不允许保存章节正文。
- `first-volume-canon-index.json` 只允许保存转述事实卡，不允许保存长段原文或原文摘录字段。
- 原文编码优先按 UTF-8 读取；失败时按 GB18030 / GBK 解码，并统一输出 UTF-8 JSON。

## Public Data Files
- `docs/superpowers/data/canon/first-volume-source-map.json`：第一卷 1-199 节的来源映射。
- `docs/superpowers/data/canon/first-volume-canon-index.json`：首批原著事实卡。
- `docs/superpowers/data/first-volume-story-beats.json`：每个剧情 beat 通过 `canon_index_refs` 引用事实卡。

## CanonFactCard Contract
每张事实卡必须包含：

- `fact_id`：稳定 id，供剧情 beat 和 DeepSeek target 引用。
- `volume`：卷名。
- `chapter_no`：章节序号。
- `chapter_title`：章节标题。
- `line_start` / `line_end`：本地 source map 行号范围。
- `fact_type`：事实类型，例如 `opening_rite`、`aperture`、`refinement`。
- `canon_summary`：转述事实，不使用长段原文。
- `game_design_use`：这张事实卡如何进入玩法、UI 或叙事。
- `mechanic_hooks`：后续机制钩子。
- `narrative_hooks`：后续文本槽位钩子。
- `canon_strict_rule`：严谨模式必须遵守的规则。
- `sandbox_if_allowance`：IF 模式可偏移的范围和代价。
- `forbidden_misreadings`：常见误读与禁止写法。

## SourceMapEntry Contract
每条来源映射必须包含：

- `source_id`
- `local_file`
- `encoding`
- `raw_sha256`
- `volume`
- `chapter_no`
- `chapter_title`
- `line_start`
- `line_end`

## First Batch Coverage
首批事实卡只覆盖 Beat 1-2 与系统关键事实，不一次性索引整卷。必须至少覆盖：

- 开窍大典地下溶洞、水中行走、花海与希望蛊仪式。
- 步数与资质等级的对应关系。
- 希望蛊入体并开辟空窍。
- 空窍、元海、光膜、真元容量。
- 甲乙丙丁资质与元海比例、恢复速度、修行上限的关系。
- 一转青铜、二转赤铁、三转白银等真元颜色/层级。
- 蛊师定义：开窍后用真元喂养、炼化、操控蛊虫。
- 炼化月光蛊需要真元、元石、时间，并会受到蛊虫意志抵抗。
- 空窍冲刷与小境界推进：光膜、水膜、石膜等后续机制钩子。

## DeepSeek Usage
DeepSeek V4-Pro 只能使用以下输入：

- 剧情 beat 摘要。
- `NarrativeSlot`。
- 对应 `canon_index_refs` 的事实卡转述。
- 模式门禁与红线。

DeepSeek 输出仍是离线候选，必须进入 `.local/deepseek-candidates/`，再由人工审核后入库。禁止把密钥、prompt、完整 response、模型名、thinking chain 或长段原文写入仓库。

## Validation
必须通过：

- `node scripts/extract-canon-source-map.mjs --source <本地原著> --out docs/superpowers/data/canon/first-volume-source-map.json`
- `node scripts/validate-canon-index.mjs`
- `pnpm deepseek:test`
- `node scripts/deepseek-candidates.mjs redline`

验收条件：

- 第一卷 source map 必须映射到 199 节。
- 剧情 beat 中所有 `canon_index_refs` 必须能在事实卡中找到。
- 仓库不得新增 `reverend-insanity.txt` 或长段原文摘录。
- 输出 JSON 不得出现替换符或历史已知乱码标记。
- `canon_strict` 不得把酒虫、完整花酒传承、本命蛊、方源绑定、白凝冰击败等内容写成稳定奖励。
