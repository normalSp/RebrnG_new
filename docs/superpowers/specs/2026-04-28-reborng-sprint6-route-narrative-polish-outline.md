# RebrnG Sprint 6 四路线沉浸文本扩写总体大纲

## 文档状态

- 状态：frozen candidate
- 所属阶段：Sprint 6
- 上位依赖：
  - `2026-04-28-reborng-sprint5-moonlight-narrative-polish-outline.md`
  - `2026-04-28-reborng-sprint3-phase6-offline-narrative-expansion-freeze.md`
  - `2026-04-28-reborng-deepseek-v4-pro-offline-candidate-guide.md`

## Summary

Sprint 6 做“沉浸文本横向补厚”，优先覆盖 `功绩 / 药堂 / 黑市 / 传承` 四条路线，并补厚 `学堂公开压力` 这一横向制度遭遇。目标是让玩家在 8 个自由窗口内更清楚地感到：制度内积累并不安全，恢复会被记账，黑市门路伴随暴露，传承残线半真半假，学堂压力不是单纯战斗按钮。

本 Sprint 使用真实 DeepSeek V4-Pro 生成离线候选文本，但 DeepSeek 仍只属于离线候选工具。所有候选必须先写入 `.local/deepseek-candidates/`，通过 schema 校验和人工审校后，才允许改写入 `content/s0/**/*.yaml` 并同步 Rust starter 文本。

版本策略固定为：

- `ContentManifest.version`：升为 `s0.5.0`
- `RULES_VERSION`：保持 `sprint4-rules-v1`
- 不迁移旧内容版本存档
- 不改 AP、元石、债务、暴露、伤势、遭遇代价、蛊虫状态或存档结构

## 目标体验

Sprint 6 的文本扩写必须服务五个目标：

- 处境：说明玩家为什么被推到这个行动上，而不是只写“你做了某事”。
- 选择：让玩家看懂当前路线的诱惑和限制。
- 代价：把 AP、窗口、元石、债务、暴露、伤势、线索可信度写成世界内压力。
- 因果：行动结果要能回扣到账本、关系、路线和下一步压力。
- 下一步：每段结果都应暗示下一步风险或机会，而不是孤立结束。

## Sprint 6 内容范围

### 功绩线

功绩文本强调制度内积累并不等于安全白送。功绩告示、审计压力、被看见的稳健、被登记的功劳都要出现。

可写：

- 功绩告示的冷硬格式
- 稳健行动带来的小收益
- 审计视线、同辈比较和家族秩序压力

不可写：

- 功绩兑换成稳定大额奖励
- 制度内路线完全安全
- 绕过 AP 或窗口代价

### 药堂线

药堂文本强调恢复与债务粘合。疗伤、询价、药味、账册、人情债和后续追索都要让玩家看见。

可写：

- 询价时的冷淡、记账和人情压力
- 恢复伤势的代价来源
- 药堂债在关系页和行动反馈中的余波

不可写：

- 免费治疗
- 药堂变成传统安全区
- 直接解除长期后果

### 黑市线

黑市文本强调隐藏门路、暗口接触、交易暴露和遭遇代价。黑市不能写成开局可见商店，也不能写成宝黄天前身。

可写：

- 门路来自风声和尾巴
- 暗口交易不是公开买卖
- 勒索、跑路、硬顶和暴露后果

不可写：

- 黑市稳定公开交易所
- 无风险翻身
- 缺线索时直接展示入口

### 传承线

传承文本强调半真半假和高风险诱惑。S0 只允许查验、接近、撤退和风险反馈，不给稳定奖励。

可写：

- 残线、竹影、传闻的真假混杂
- 查验带来的暴露与窗口损失
- 撤退后仍留下线索和不安

不可写：

- 稳定获得完整传承
- 直接夺取原著核心机缘
- canon_strict 中改写方源或花酒行者硬事实

### 学堂公开压力

学堂公开压力不是完整战斗系统，而是制度压迫的横向遭遇。文本要补厚触发、忍让、争辩和硬顶的差异。

可写：

- 修行痕迹被看见后的公开压力
- 忍让保命但折损名声
- 争辩缓解但增加被盯风险
- 硬顶换来伤势或暴露

不可写：

- 学堂压力变成爽文打脸
- 单次硬顶稳定获利
- 公开改写原著核心人物硬事实

## DeepSeek 候选工具扩展

新增目标槽位清单：

- `docs/superpowers/data/sprint6-narrative-targets.json`

工具接口：

- `node scripts/deepseek-candidates.mjs generate --targets <file>`
- 未传 `--targets` 时保留 Sprint 5 月光蛊默认 mock 行为
- 真实生成命令固定为：

```powershell
node scripts/deepseek-candidates.mjs generate --allow-network --targets docs/superpowers/data/sprint6-narrative-targets.json --out .local/deepseek-candidates/sprint6
```

安全边界：

- DeepSeek 只读取本机环境变量
- 不保存密钥
- 不保存完整 prompt
- 不保存完整 response
- 不保存模型名
- 不保存 thinking chain
- 不写入存档
- 不进入 Tauri command、React 点击链路或 Rust 规则结算

## 入库规则

候选文本按四档审校：

- `可入库`：符合世界观、证据、模式、规则代价和文风，可人工润色后写入 YAML。
- `需改写`：方向可用，但有文风、压强、代价表达或信息量问题。
- `仅 sandbox_if`：有 IF 强度或原著锚点偏移，不能进 canon_strict。
- `拒绝`：现代吐槽、爽文化直给、无代价奖励、硬改原著事实、泄露隐藏数值或规则事实乱编。

入库后必须同步：

- `content/s0/narratives/s0-local-narratives.yaml`
- `crates/game-core/src/lib.rs` 中的 starter narrative
- `content/s0/manifest.yaml` 版本
- `STARTER_CONTENT_VERSION`

## 验收标准

- DeepSeek 工具支持 `--targets` 并通过测试。
- Sprint 6 候选池能从目标清单生成并通过候选 schema 校验。
- 入库文本覆盖功绩、药堂、黑市、传承和学堂公开压力。
- `pnpm content:build` 输出 `s0.5.0`。
- 月光根基、功绩稳健、药堂债务恢复、黑市跑路、传承诱惑撤退都能跑到阶段收口。
- runtime 红线保持：无 DeepSeek/API 调用、无 Express 主链、React 不持有完整 `GameState`、前端不扫描 YAML、不直接读写存档。

## 禁做项

- 不做炼蛊、升转、杀招、升仙、仙窍、宝黄天。
- 不改规则数值。
- 不新增 `ActionIntent`。
- 不新增 `GameState` 字段。
- 不新增 Tauri command。
- 不新增 runtime AI 设置页。
- 不把候选文件直接当内容源。

