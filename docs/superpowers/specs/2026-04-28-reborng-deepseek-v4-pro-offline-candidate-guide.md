# RebrnG DeepSeek V4-Pro 离线候选使用指导 v1.0

状态：draft guidance

更新日期：2026-04-28

关联文档：
- [2026-04-28-reborng-sprint4-mortal-gu-foundation-outline.md](./2026-04-28-reborng-sprint4-mortal-gu-foundation-outline.md)
- [2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md](./2026-04-27-reborng-mortal-gu-refinement-killer-move-contract-spec.md)
- [2026-04-28-reborng-sprint3-reboot-dialogue-framework-freeze.md](./2026-04-28-reborng-sprint3-reboot-dialogue-framework-freeze.md)
- [2026-04-26-reborng-logical-architecture-design.md](./2026-04-26-reborng-logical-architecture-design.md)
- [2026-04-26-reborng-technical-architecture-design.md](./2026-04-26-reborng-technical-architecture-design.md)

外部资料：
- [DeepSeek V4-Pro 模型卡](https://huggingface.co/deepseek-ai/DeepSeek-V4-Pro)
- [DeepSeek V4 Preview Release](https://api-docs.deepseek.com/news/news260424)
- [DeepSeek Models & Pricing](https://api-docs.deepseek.com/quick_start/pricing)
- [Thinking Mode](https://api-docs.deepseek.com/guides/thinking_mode)
- [Context Caching](https://api-docs.deepseek.com/guides/kv_cache)
- [JSON Output](https://api-docs.deepseek.com/guides/json_mode)
- [DeepSeek Privacy Policy](https://cdn.deepseek.com/policies/en-US/deepseek-privacy-policy.html?os=___)

## 文档角色

本文只定义 DeepSeek V4-Pro 在 RebrnG Sprint 4 中的离线使用方法：怎么准备上下文、怎么喂提示词、怎么产出候选、怎么审校候选，以及哪些东西绝对不能进入 runtime。

Sprint 4 的主线仍然是凡人蛊虫系统基座：`GuSpec -> GuInstance -> 炼化状态 -> 月光修行准入 -> 账本 / 对话流反馈`。DeepSeek V4-Pro 只服务 Phase 5 离线候选文本和设定审校，不是规则引擎、不是剧情实时生成器、不是内容事实来源。

## 核心结论

- DeepSeek V4-Pro 适合做“长上下文设定审校 + 离线候选正文生成”，不适合直接接入玩家点击后的 runtime。
- 1M context 应该用于长设定包审校、设定归纳、候选批量一致性检查；日常生成应使用较小上下文：稳定前缀 + 相关证据卡 + 当前目标槽位。
- 不把整本《蛊真人》原文直接喂给 hosted API。原文只在本地处理，先抽成 `canon_cards`、风格规则、禁错项、章节范围和证据等级，再给模型使用。
- DeepSeek 输出永远是 `needs_review` 候选，不能自动写入 `content/s0/**/*.yaml`，不能自动升级版本，不能生成规则事实。
- Hosted API 会处理用户输入、prompt、上传文件、聊天历史等输入内容，并可能用于服务改进；因此不得上传密钥、私人数据、完整版权文本或足以替代原作的大段原文。

## DeepSeek V4-Pro 强在哪里

### 1. 长上下文能力适合做设定审校

官方模型卡称 DeepSeek V4-Pro 是 MoE 模型，约 1.6T 总参数、49B 激活参数，支持 1M token 上下文。它的价值不是让模型一次读完整本书后随意发挥，而是让它能在同一次任务中看到更多设定卡、规则红线、候选文本和审校维度。

对 RebrnG 来说，最适合的用法是：
- 批量审校一组 S0 候选文本是否冲突。
- 对比多个 `canon_cards` 和候选正文，找硬冲突、软漂移、现代口吻和爽文超模。
- 在长设定包内抽出“可入库事实”和“只可传闻/IF 的内容”。

不适合的用法是：
- 把 1M context 当作无限记忆。
- 把整本原文直接塞给 hosted API。
- 让模型临场决定奖励、蛊虫、传承、锚点变化或资源结算。

### 2. CSA + HCA 提升长上下文效率

模型卡说明 V4 系列使用 Compressed Sparse Attention 与 Heavily Compressed Attention 组合，用于提高长上下文效率；在 1M token 场景下，V4-Pro 相比 V3.2 单 token 推理 FLOPs 和 KV cache 都大幅降低。

项目侧的实际含义是：长审校任务更可行，但仍然要做上下文预算。即使模型支持 1M，也应该优先喂结构化证据，而不是无筛选地堆文本。

### 3. Thinking / Non-Thinking 模式可以分工

DeepSeek V4 API 支持 Thinking 与 Non-Thinking 模式。模型卡还把 V4-Pro 分成 Non-think、Think High、Think Max 这类推理强度。

推荐分工：
- `deepseek-v4-flash` Non-Thinking：短文本改写、格式整理、低风险批量草稿。
- `deepseek-v4-flash` Thinking：低成本批量初审，找明显现代词、格式错误、缺字段。
- `deepseek-v4-pro` Thinking High：正式候选生成、复杂世界观一致性检查、跨卡片审校。
- `deepseek-v4-pro` Thinking Max：只用于高风险任务，例如原著锚点冲突审校、`canon_strict` / `sandbox_if` 边界判定、长上下文设定归纳。

Thinking 模式的 `reasoning_content` 不应保存到候选文件，也不应写入存档、YAML 或日志。候选只保留最终审核所需字段。

### 4. Context Caching 适合固定提示词前缀

DeepSeek Context Caching 会对重复前缀产生 cache hit。项目提示词应固定前缀顺序，把经常复用的内容放在前面，把每次变化的目标槽位放在最后。

推荐顺序：
1. 项目红线。
2. Sprint 4 蛊虫合同。
3. `canon_strict` / `sandbox_if` 门禁。
4. 文风规则。
5. JSON schema。
6. 相关证据卡。
7. 本次目标槽位和候选任务。

这样同一批候选生成或审校任务更容易命中缓存，也能减少“提示词结构漂移”带来的输出不稳定。

### 5. JSON Output 适合候选落盘

DeepSeek JSON Output 支持通过 `response_format: { "type": "json_object" }` 请求结构化 JSON。项目候选文件必须优先使用 JSON，便于后续人工审核、diff、脚本检查和内容构建器门禁。

注意：
- prompt 中必须明确出现 `json`。
- 必须给出完整 JSON 示例。
- `max_tokens` 要留足，避免 JSON 中途截断。
- JSON 模式仍可能返回空内容或格式问题，所以离线工具必须有解析失败处理。

## 使用边界

### 允许

- 生成候选正文、行动说明、结果反馈、传闻风声、账本反馈文案。
- 审校候选文本是否违反 Sprint 4 红线。
- 将本地抽取后的 `canon_cards`、规则合同和候选文本组合成审校任务。
- 输出 `.local/deepseek-candidates/` 下的待审 JSON 或 Markdown 摘要。
- 给人工审核者列出风险项、证据不足项、建议删改项。

### 禁止

- runtime 调用 DeepSeek。
- `resolve_action`、Tauri command、React 点击链路调用 DeepSeek。
- DeepSeek 生成或修改 `GameState`。
- DeepSeek 生成规则结算、奖励、蛊虫实例、传承、锚点变化或原著硬事实。
- DeepSeek 输出自动写入 `content/s0/**/*.yaml`。
- 保存完整 prompt、完整 response、API key、模型名、请求日志或 Thinking chain。
- 上传完整版权原文、私人数据、密钥、未公开策划资料中不必要的敏感内容。

## 原文处理策略

### 不直接上传全文

《蛊真人》原文应留在本地资料库或人工阅读流程里，不直接上传到 hosted API。原因有三层：
- 版权风险：完整或大段原文可能构成可替代原作的文本输入。
- 隐私与控制风险：DeepSeek 隐私政策说明服务会处理用户输入、prompt、上传文件和聊天历史，并可能用于服务改进。
- 工程风险：全文输入会让模型把“语料”误当“可自由复述文本”，更容易生成长段贴近原文的内容。

### 本地抽成 canon_cards

本地抽卡不是摘要越多越好，而是把原文信息压成可审计、可追责、不可替代原文的设定事实。

推荐字段：

```json
{
  "card_id": "canon.s0.gu.moonlight_gu.v001",
  "scope": "s0_qingmao",
  "entity_type": "gu",
  "display_name": "月光蛊",
  "evidence_level": "canon_explicit",
  "source_ref": {
    "work": "蛊真人",
    "chapter_range": "第一卷相关学堂与月光蛊修行段落",
    "local_note_id": "local-only"
  },
  "facts": [
    "月光蛊是一转阶段的月道攻击蛊。",
    "青茅山古月一族学堂阶段，月光蛊与学员修行、资源压力和家族培养体系相关。",
    "月光蛊必须经过炼化归属，不能只作为装备字符串生效。"
  ],
  "implications": [
    "月光修行行动必须检查已炼化月光蛊。",
    "使用月光蛊修行仍然消耗 AP 和元石。",
    "月光蛊可以是核心蛊候选，但不能自动成为本命蛊。"
  ],
  "forbidden_misreads": [
    "不得把月光蛊写成无主角代价的稳定爽文奖励。",
    "不得把月光蛊升级、合炼或路线扩展自动开放到 Sprint 4。",
    "不得因为玩家持有月光蛊而跳过空窍、炼化、资源和喂养压力。"
  ],
  "mode_permission": ["canon_strict", "sandbox_if"],
  "upload_ok": true,
  "review_status": "approved_card"
}
```

### 证据卡类型

建议先维护这些最小卡片类型：
- `world_redline_card`：世界观不可破坏红线。
- `character_card`：人物身份、动机、边界和禁写项。
- `gu_card`：蛊虫品转、用途、炼化、喂养、证据等级。
- `place_card`：地点资源、风险、安全边界。
- `phase_contract_card`：Sprint 4 当前阶段规则合同。
- `style_card`：文风、叙事视角、句式禁忌。
- `forbidden_misread_card`：常见误读和强制拒绝项。
- `candidate_target_card`：本次要生成或审校的目标槽位。

### 章节范围写法

`source_ref.chapter_range` 只写章节范围或本地笔记编号，不贴长原文。必要时可以保留极短术语或专名，但不要复制大段正文。

好的写法：
- `chapter_range: "第一卷学堂发放与月光蛊修行相关段落"`
- `fact: "月光蛊与学堂阶段修行压力相关。"`

不好的写法：
- 粘贴多个自然段原文。
- 让模型“模仿这一段原文继续写”。
- 上传可让模型重构原作场景的大段细节。

## Prompt 总体结构

### 固定前缀

每次请求保持同一套前缀。变量只放在最后。

```text
你是 RebrnG 的离线候选文本与设定审校助手。

项目红线：
- 你只产出待审候选，不产出规则事实。
- 你不得生成 runtime 结算、奖励、蛊虫实例、传承、锚点变化。
- 你不得把 sandbox_if 内容混入 canon_strict。
- 你不得复述、仿写或拼接版权原文。

Sprint 4 合同：
- 当前稳定闭环只有 moonlight_gu。
- 月光修行必须要求空窍已开、月光蛊已炼化且可调用。
- 修行仍消耗 1 AP 与 1 元石。
- 月光蛊可为核心蛊候选，不能自动成为本命蛊。
- 喂养压力 Sprint 4 只做轻量状态，不做周期扣费公式。

模式门禁：
- canon_strict: 只允许 canon_explicit / canon_inferred 内容进入关键链。
- sandbox_if: 可以提出 IF 候选，但必须显式标记，不能进入 canon_strict 关键奖励链。

文风规则：
- 第二人称。
- 冷峻克制。
- 利益优先。
- 因果清楚。
- 每段必须服务处境、选择、代价或后果。
- 不使用现代网文吐槽、系统口吻、热血口号或轻松喜剧化表达。

输出必须是 json。
```

### 变量后缀

变量后缀只包含本次任务需要的卡片和目标槽位。

```text
相关证据卡：
<CANON_CARDS_JSON>

目标：
- target_content_id: s0.action.cultivate.moonlight
- target_slot: action_result
- mode: canon_strict
- evidence: canon_inferred
- state_assumptions:
  - 空窍已开
  - 月光蛊已炼化
  - 消耗 1 AP 与 1 元石

请生成一个待审候选，严格符合下面 schema。
```

## 候选生成 Schema

最小候选 JSON：

```json
{
  "candidate_id": "s0_action_cultivate_moonlight_v001",
  "target_content_id": "s0.action.cultivate.moonlight",
  "target_slot": "action_result",
  "mode": "canon_strict",
  "evidence": "canon_inferred",
  "candidate_text": "你压下呼吸，将月光蛊收入空窍深处的牵引里。月华没有替你省下元石，也没有替你抹平资质，只在一次次催动中留下更清楚的痕迹。学堂的脚步声离你不远，能多走一寸，就少一分被人看轻的余地。",
  "state_assumptions": ["空窍已开", "月光蛊已炼化", "消耗 1 AP 与 1 元石"],
  "risk_notes": ["不得生成新蛊虫、传承或额外奖励", "不得暗示本命蛊已建立"],
  "review_status": "needs_review",
  "review_notes": ""
}
```

字段要求：
- `candidate_id`：稳定、可 diff、可人工追踪。
- `target_content_id`：对应未来可能入库的内容槽位，不等于已经入库。
- `target_slot`：如 `action_result`、`action_label`、`ledger_feedback`、`rumor_text`。
- `mode`：必须是 `canon_strict` 或 `sandbox_if`。
- `evidence`：必须是 `canon_explicit`、`canon_inferred`、`project_inferred`、`sandbox_if` 之一。
- `candidate_text`：候选正文，不得包含 JSON 外解释。
- `state_assumptions`：本段文字成立所依赖的规则前提。
- `risk_notes`：人工审核者必须关注的风险。
- `review_status`：默认 `needs_review`。

## 审校 Prompt

审校任务比生成任务更适合 V4-Pro Thinking High / Max。审校输出必须指出风险类型，不只给“通过/不通过”。

```text
你是 RebrnG 的 canon 审校助手。

请审校候选文本，不要重写全文，先判断风险。

审校维度：
- hard_conflict: 是否冲突原著硬事实、项目硬规则或 Sprint 4 合同。
- soft_drift: 是否有世界观漂移、动机漂移、人物口吻漂移。
- overpowered: 是否出现无代价爽文、提前获得强机缘、跳过资源压力。
- modern_tone: 是否出现现代词、系统口吻、吐槽感、轻浮表达。
- evidence_gap: 是否有证据不足却写成事实的内容。
- runtime_leak: 是否暗示 DeepSeek 或 AI 在 runtime 结算规则。
- copyright_risk: 是否过度贴近原文表达或复述场景。

输出必须是 json。
```

审校输出 Schema：

```json
{
  "candidate_id": "s0_action_cultivate_moonlight_v001",
  "overall": "pass_with_notes",
  "hard_conflicts": [],
  "soft_drifts": [],
  "overpowered_flags": [],
  "modern_tone_flags": [],
  "evidence_gaps": [],
  "runtime_leaks": [],
  "copyright_risks": [],
  "required_edits": [],
  "review_notes": "候选只描写月光修行痕迹与资源压力，没有生成新奖励；注意后续入库时由规则层提供 AP/元石扣减。"
}
```

判定标准：
- `fail`：出现硬冲突、runtime 结算、未授权奖励、明显原文复述。
- `needs_revision`：文风不稳、证据不足、语气现代、隐含超模。
- `pass_with_notes`：可入人工复核，但仍要列出注意事项。
- `pass`：低风险短文本，且证据与规则前提完整。

## 文风控制

### RebrnG 文风目标

正文应该像玩家正在承受一个冷静、现实、逐步收紧的世界，而不是被系统奖励推着爽。

必须保留：
- 第二人称视角。
- 处境压力。
- 资源代价。
- 选择后果。
- 学堂、家族、债务、伤势、暴露、修行窗口等可落账压力。
- 对“收益”的克制表达。

必须避免：
- “恭喜获得”“系统提示”“逆天改命从此开始”。
- 过度热血、口号化、爽文胜利宣告。
- 现代网络语、吐槽腔、轻松喜剧腔。
- 把原著人物写成无条件帮玩家。
- 把传闻写成确定机缘。

### 候选文本长度

按槽位控制长度：
- `action_label`：8 到 18 个汉字。
- `action_hint`：1 句，说明代价或风险。
- `action_result`：80 到 180 个汉字。
- `dialogue_paragraph`：120 到 260 个汉字。
- `ledger_feedback`：20 到 60 个汉字，必须能对应具体账本变化。
- `rumor_text`：60 到 140 个汉字，必须保留不确定性。

### 文风检查清单

每段候选都必须回答至少两个问题：
- 玩家现在处在什么压力里？
- 这次行动付出了什么？
- 这次行动改变了什么？
- 这次行动留下了什么后果？
- 这段内容凭什么能在 `canon_strict` 中成立？

## Sprint 4 目标槽位建议

### `claim_moonlight_gu`

可生成：
- 领取月光蛊的结果正文。
- 学堂压力反馈。
- 账本短反馈。
- 未炼化状态提示。

必须包含：
- 只领取一只 `moonlight_gu`。
- 状态为未炼化。
- 不重复发放。
- 不暗示立即可用于修行。

禁止包含：
- 额外蛊虫。
- 酒虫或传承实物奖励。
- 本命蛊建立。
- 家族无条件偏爱。

### `refine_moonlight_gu`

可生成：
- 炼化成功正文。
- 炼化失败或不可行动原因。
- 容器转为空窍的反馈。
- 控制状态变为已炼化的反馈。

必须包含：
- 要求空窍已开。
- 消耗 1 AP。
- 成功后月光蛊进入空窍。
- 控制状态变为 `refined`。

禁止包含：
- 跳过炼化过程。
- 炼化即提升品转。
- 炼化即获得杀招。
- 炼化即建立本命蛊。

### `cultivate_moonlight`

可生成：
- 月光修行成功正文。
- 无已炼化月光蛊时的拒绝原因。
- 元石不足时的拒绝原因。
- 喂养压力提醒。

必须包含：
- 空窍已开。
- 月光蛊已炼化且可调用。
- 消耗 1 AP 与 1 元石。
- 推进月光修行痕迹。

禁止包含：
- 取消 AP 或元石压力。
- 直接提升大境界。
- 生成新蛊方、新杀招或新奖励。
- 暗示月光蛊已经成为本命蛊。

### `inspect_gu`

可生成：
- 月光蛊状态检查文案。
- 账本明细解释。
- 喂养 warning 文案。

必须包含：
- 蛊名。
- 品转。
- 容器。
- 炼化状态。
- 损伤状态。
- 喂养状态。
- Build 角色。

禁止包含：
- 把检查写成免费修复。
- 把 warning 写成周期扣费已经开启。
- 把 Build 核心蛊等同本命蛊。

## `canon_strict` 与 `sandbox_if`

### `canon_strict`

DeepSeek 在 `canon_strict` 下只能做低风险表达补全。

允许：
- 月光蛊领取、炼化、修行相关的候选文案。
- 学堂压力、元石压力、行动窗口压力。
- 原著明确或高可信推断能支撑的环境反馈。
- 不改变原著硬事实的传闻和风声。

禁止：
- 稳定获得酒虫。
- 稳定获得花酒行者传承。
- 盗天类强机缘落地。
- 提前绑定、击杀、结盟或硬改写方源等核心人物。
- 用缺证据原创蛊方产出关键奖励。

### `sandbox_if`

DeepSeek 在 `sandbox_if` 下可以更大胆，但必须显式标记为 IF 候选。

允许：
- 传闻、残线、错误情报、风险提示。
- 原创补完的低阶候选。
- 更高自由度的分支草稿。

仍然禁止：
- 抹掉资源、修为、空窍、炼化、喂养和风险代价。
- 未标记就混入 `canon_strict`。
- 自动写入内容 YAML。
- 让 IF 内容成为默认稳定主线。

## 1M Context 的正确用法

### 适合 1M 的任务

- 一次性审校一个章节包的候选文本。
- 对多个候选槽位做统一文风检查。
- 对比 `canon_cards`、规则合同、候选输出，找冲突。
- 从本地抽出的长设定卡包中归纳可用红线。
- 检查重复候选是否互相矛盾。

### 不适合 1M 的任务

- 单条行动结果生成。
- 简单 label 改写。
- 已有 schema 的格式整理。
- 上传未经筛选的原文。
- 让模型从大上下文里自由找奖励点。

### 推荐上下文预算

| 任务 | 推荐模型 | 推荐上下文 |
| --- | --- | --- |
| 单条行动结果候选 | `deepseek-v4-flash` 或 `deepseek-v4-pro` High | 5K 到 30K |
| 月光蛊闭环候选批量生成 | `deepseek-v4-pro` High | 30K 到 120K |
| S0 候选包审校 | `deepseek-v4-pro` High / Max | 120K 到 500K |
| 长设定卡归纳 | `deepseek-v4-pro` Max | 300K 到 1M |
| 格式整理 / JSON 修复 | `deepseek-v4-flash` Non-Thinking | 2K 到 20K |

原则：上下文越长，越要减少“开放式创作”指令，增加“审校、归类、拒绝、标注风险”的指令。

## 离线工具工作流

### Step 1：本地抽卡

输入：
- 本地原文阅读笔记。
- 现有 RebrnG 文档。
- 内容 YAML。
- 规则合同。

输出：
- `canon_cards/*.json`
- `style_cards/*.json`
- `redline_cards/*.json`

这些卡片可以进入离线 prompt，但不自动进入内容 bundle。

### Step 2：组装任务

按目标槽位选择最小证据集：
- 只生成月光修行结果，就不要喂酒虫、传承、升仙、仙窍等远期卡片。
- 只审校 `claim_moonlight_gu`，就不要喂完整蛊虫图鉴。
- 只做文风整理，就不要开放新设定。

### Step 3：生成候选

输出到：

```text
.local/deepseek-candidates/
```

文件名建议：

```text
2026-04-28_s0_action_cultivate_moonlight_v001.json
```

候选文件只保存审核所需结果，不保存完整 prompt、完整 response、key、模型名或请求日志。

### Step 4：DeepSeek 自审

把候选 JSON、相关证据卡和 Sprint 4 红线再喂给 V4-Pro Thinking High / Max 做审校。

自审输出可以作为同目录旁路文件：

```text
2026-04-28_s0_action_cultivate_moonlight_v001.review.json
```

### Step 5：人工审核

人工审核者决定：
- 拒绝。
- 要求重写。
- 保留为 backlog。
- 手工改写后准备入库。

只有人工审核后的文本，才能由内容作者手写进 YAML。

### Step 6：Rust 内容构建器门禁

入库后仍然必须过内容构建器：
- `mode_permission`
- `evidence_level`
- ID 唯一性。
- 非法品转。
- `sandbox_if` 不得进入 `canon_strict` 关键奖励链。
- 文本槽位与行动状态匹配。

DeepSeek 不能绕过这一步。

## DeepSeek 离线工具约束

脚本入口建议：

```text
scripts/deepseek-candidates.mjs
```

只读环境变量：
- `DEEPSEEK_API_KEY`
- `DEEPSEEK_BASE_URL`
- `DEEPSEEK_MODEL`

默认行为：
- 无 key 时 dry-run / mock。
- 有 key 时仅输出到 `.local/deepseek-candidates/`。
- `.local/` 必须 gitignored。

不得：
- 写入 `content/s0/**/*.yaml`。
- 修改内容版本或规则版本。
- 生成规则事实。
- 保存完整 prompt / response。
- 保存 API key。
- 保存 Thinking chain。
- 把模型名写入候选文件。

可保存：
- `candidate_id`
- `target_content_id`
- `target_slot`
- `mode`
- `evidence`
- `candidate_text`
- `state_assumptions`
- `risk_notes`
- `review_status`
- `review_notes`

## 红线检索

每次引入或修改离线工具后，必须检索 runtime 路径：

```text
crates/game-core
apps/desktop
packages/ui-ledger
content/s0
```

这些路径不得出现：
- `DEEPSEEK_API_KEY`
- `DEEPSEEK_BASE_URL`
- `deepseek-v4`
- `api.deepseek.com`
- `chat.completions`
- 任何 DeepSeek SDK 调用

允许出现 DeepSeek 的位置：
- `docs/**`
- `scripts/deepseek-candidates.mjs`
- 离线测试文件
- `.local/**`，且不入库

## 提示词模板

### 生成模板

```text
你是 RebrnG 的离线候选文本生成助手。

你必须遵守：
1. 只输出待审候选。
2. 不生成规则事实。
3. 不扩展未给出的世界观。
4. 不复述或仿写版权原文。
5. 不把 sandbox_if 混入 canon_strict。
6. 不生成奖励、蛊虫实例、传承或锚点变化。

Sprint 4 当前合同：
- 唯一稳定 canon 蛊虫闭环是 moonlight_gu。
- claim_moonlight_gu 只领取一只未炼化月光蛊。
- refine_moonlight_gu 消耗 1 AP，要求空窍已开，成功后容器为 aperture，控制状态为 refined。
- cultivate_moonlight 要求空窍已开、月光蛊已炼化且可调用，消耗 1 AP 与 1 元石，推进月光修行痕迹。
- VitalGuState 默认 not_established，月光蛊不能自动成为本命蛊。

文风：
- 第二人称。
- 冷峻克制。
- 利益优先。
- 因果清楚。
- 每段服务处境、选择、代价或后果。

输出必须是 json，格式如下：
{
  "candidate_id": "",
  "target_content_id": "",
  "target_slot": "",
  "mode": "",
  "evidence": "",
  "candidate_text": "",
  "state_assumptions": [],
  "risk_notes": [],
  "review_status": "needs_review",
  "review_notes": ""
}

相关证据卡：
{{CANON_CARDS_JSON}}

目标槽位：
{{TARGET_SLOT_JSON}}
```

### 审校模板

```text
你是 RebrnG 的离线 canon 审校助手。

你只审校，不扩写。
你必须指出候选是否违反：
- 原著硬事实。
- Sprint 4 蛊虫合同。
- canon_strict / sandbox_if 门禁。
- 文风规则。
- 版权与 hosted API 安全边界。
- runtime AI 禁止边界。

输出必须是 json，格式如下：
{
  "candidate_id": "",
  "overall": "pass | pass_with_notes | needs_revision | fail",
  "hard_conflicts": [],
  "soft_drifts": [],
  "overpowered_flags": [],
  "modern_tone_flags": [],
  "evidence_gaps": [],
  "runtime_leaks": [],
  "copyright_risks": [],
  "required_edits": [],
  "review_notes": ""
}

相关证据卡：
{{CANON_CARDS_JSON}}

Sprint 4 红线：
{{SPRINT4_REDLINE_JSON}}

候选：
{{CANDIDATE_JSON}}
```

### 抽卡模板

抽卡任务推荐本地模型或本地人工流程完成。如果必须用 hosted API，只能喂人工整理的短摘录和事实表，不喂长原文。

```text
你是 RebrnG 的本地 canon card 整理助手。

请把下面的人工笔记整理成 canon card。
不要补充笔记外事实。
不要复述原文。
不确定项写入 forbidden_misreads 或 review_notes。
输出必须是 json。

人工笔记：
{{LOCAL_NOTES}}

目标 schema：
{
  "card_id": "",
  "scope": "",
  "entity_type": "",
  "display_name": "",
  "evidence_level": "",
  "source_ref": {
    "work": "",
    "chapter_range": "",
    "local_note_id": ""
  },
  "facts": [],
  "implications": [],
  "forbidden_misreads": [],
  "mode_permission": [],
  "upload_ok": false,
  "review_status": "needs_review"
}
```

## 示例候选

### 可接受：月光修行成功

```json
{
  "candidate_id": "s0_action_cultivate_moonlight_v001",
  "target_content_id": "s0.action.cultivate.moonlight",
  "target_slot": "action_result",
  "mode": "canon_strict",
  "evidence": "canon_inferred",
  "candidate_text": "你把一枚元石扣在掌心，催动空窍里那点冷白月华。月光蛊已被炼化，却不会替你省下任何代价；真元与元石一同沉下去，只换来月光修行痕迹上缓慢的一寸推进。学堂的比较不会因此消失，只是下一次被点名时，你少一分空口解释的余地。",
  "state_assumptions": ["空窍已开", "月光蛊已炼化", "消耗 1 AP 与 1 元石"],
  "risk_notes": ["不得生成新蛊虫、传承或额外奖励", "不得暗示本命蛊已建立"],
  "review_status": "needs_review",
  "review_notes": ""
}
```

### 可接受：未炼化拒绝

```json
{
  "candidate_id": "s0_action_cultivate_moonlight_blocked_unrefined_v001",
  "target_content_id": "s0.action.cultivate.moonlight.blocked_unrefined",
  "target_slot": "blocked_reason",
  "mode": "canon_strict",
  "evidence": "project_inferred",
  "candidate_text": "月光蛊还没有真正归你驱使。它可以被登记在账本里，却不能替你推进月光修行；没有炼化归属，催动只会变成空窍里的牵扯和风险。",
  "state_assumptions": ["玩家拥有未炼化月光蛊", "空窍已开或即将检查空窍状态"],
  "risk_notes": ["不得把持有等同可用", "不得跳过 refine_moonlight_gu"],
  "review_status": "needs_review",
  "review_notes": ""
}
```

### 不可接受：超模爽文

```json
{
  "candidate_id": "bad_example",
  "target_content_id": "s0.action.cultivate.moonlight",
  "target_slot": "action_result",
  "mode": "canon_strict",
  "evidence": "canon_inferred",
  "candidate_text": "你一催动月光蛊，体内真元暴涨，月光自动凝成杀招雏形，族老们立刻意识到你是万中无一的天才。",
  "state_assumptions": [],
  "risk_notes": [],
  "review_status": "fail",
  "review_notes": "违反 Sprint 4 边界：无代价提升、生成杀招、强改 NPC 反应。"
}
```

## 人工审核表

每条候选入库前至少检查：

| 检查项 | 通过标准 |
| --- | --- |
| 目标槽位 | `target_content_id` 与 `target_slot` 明确 |
| 模式 | `canon_strict` / `sandbox_if` 正确 |
| 证据 | 证据等级存在，且不把低证据写成硬事实 |
| 规则前提 | `state_assumptions` 与 Rust 规则一致 |
| 资源压力 | 不跳过 AP、元石、炼化、喂养等成本 |
| 蛊虫边界 | 不生成新蛊虫、新品转、新杀招、新传承 |
| 本命蛊边界 | 不把月光蛊自动写成本命蛊 |
| 文风 | 第二人称、克制、因果清楚 |
| 版权 | 不复述或仿写原文表达 |
| runtime | 不暗示 AI 参与运行时结算 |

## 测试要求

文档与离线工具完成后，至少保留这些测试思路：

- 无 key mock 通过。
- 有 key 时只写 `.local/deepseek-candidates/`。
- 候选不自动入 YAML。
- 候选 JSON 缺 `mode`、缺 `evidence`、缺 `review_status` 时失败。
- 候选出现 `review_status: approved` 时失败，工具只能产出 `needs_review`。
- runtime 红线检索不出现 DeepSeek/API 调用。
- `.local/` 不入库。

## 推荐执行顺序

1. 先冻结本文档，明确 DeepSeek 只做离线候选。
2. 建立最小 `canon_cards`：月光蛊、空窍、炼化、学堂压力、元石压力、文风规则、Sprint 4 红线。
3. 用 mock 模式跑一条 `cultivate_moonlight` 候选，验证 schema。
4. 接入真实 API 前，先完成 `.local/` 输出与红线检索测试。
5. 用 `deepseek-v4-flash` 批量产出短候选。
6. 用 `deepseek-v4-pro` Thinking High 审校候选包。
7. 高风险争议项才使用 Thinking Max。
8. 人工审核后手写入 YAML，再由内容构建器和 Rust 测试把关。

## 首批交付清单

Sprint 4 如果要真正开始使用 DeepSeek，第一批不要追求“能写很多剧情”，而是先把候选生产链做窄、做稳。

### 最小 canon_cards

首批只需要这些卡：
- `canon.s0.gu.moonlight_gu.v001`：月光蛊事实、炼化边界、禁错项。
- `canon.s0.aperture.opened.v001`：空窍已开、修行准入、不能和本命蛊混写。
- `canon.s0.refinement.ownership.v001`：炼化归属与持有不同。
- `canon.s0.academy_pressure.v001`：学堂压力、比较、资源紧张。
- `canon.s0.resource.primeval_stone.v001`：元石消耗和资源压力。
- `project.sprint4.redline.v001`：Sprint 4 禁止扩展项。
- `style.reborng.cold_second_person.v001`：第二人称、冷峻克制、因果清楚。

这些卡足够支撑 `claim_moonlight_gu`、`refine_moonlight_gu`、`cultivate_moonlight`、`inspect_gu` 的候选文本，不需要喂酒虫、传承、升转、杀招或后期仙界设定。

### 最小候选批次

首批只生成 8 到 12 条：
- `claim_moonlight_gu.action_result`
- `claim_moonlight_gu.ledger_feedback`
- `refine_moonlight_gu.action_result`
- `refine_moonlight_gu.blocked_no_aperture`
- `cultivate_moonlight.action_result`
- `cultivate_moonlight.blocked_unrefined`
- `cultivate_moonlight.blocked_no_primeval_stone`
- `inspect_gu.moonlight_gu_summary`
- `inspect_gu.feeding_stable`
- `inspect_gu.feeding_warning`

每条候选都必须保持 `review_status: needs_review`。首批目标是验证流程和文风，不是扩写内容量。

### 最小审校批次

首批审校只看四类风险：
- 是否跳过炼化。
- 是否取消 AP 或元石消耗。
- 是否生成额外奖励、蛊虫、杀招或传承。
- 是否把月光蛊写成本命蛊。

通过这些风险后，再扩展到现代口吻、证据不足、版权风险和跨槽位一致性。

## 多窗口协作边界

为了避免文档窗口和规则实现窗口互相打架，建议 Sprint 4 同时开工时按文件所有权分工。

### DeepSeek 文档窗口只负责

- `docs/superpowers/specs/**`
- 后续如有需要，可新增 `docs/ai/**` 或 `docs/content-pipeline/**`
- 不修改 `crates/game-core/**`
- 不修改 `content/s0/**/*.yaml`
- 不运行会改写内容 bundle 的生成命令

### 规则实现窗口负责

- `crates/game-core/**`
- `crates/content-tools/**`
- `content/s0/**`
- `packages/ui-ledger/**`
- `apps/desktop/**`
- 规则测试、内容构建和 UI 回归

### 交接方式

DeepSeek 文档窗口只交付：
- 使用原则。
- prompt 模板。
- schema。
- 审校清单。
- 红线检索项。
- 首批候选槽位建议。

规则实现窗口根据文档自行决定：
- Rust 类型如何实现。
- YAML schema 如何落地。
- 测试如何组织。
- UI 如何投影。
- 何时升级内容和规则版本。

如果两个窗口都需要修改同一份 Sprint 4 大纲，只允许添加引用、验收项或说明文字，不在同一段落里同时重写规则合同。

## Assumptions

- Sprint 4 唯一稳定 canon 蛊虫闭环是 `moonlight_gu`。
- 酒虫、花酒行者传承、盗天类机缘只作为传闻、风险或 `sandbox_if` 候选，不稳定发放。
- DeepSeek V4-Pro 的 hosted API 只处理本地抽出的证据卡和候选文本，不处理完整原文。
- DeepSeek 产物永远是待审候选，人工审核和 Rust 内容构建器才是入库门槛。
- 如果未来改为本地部署 DeepSeek V4-Pro，也仍然保留同一套候选、审校、入库门禁，不让模型直接成为规则真相。
