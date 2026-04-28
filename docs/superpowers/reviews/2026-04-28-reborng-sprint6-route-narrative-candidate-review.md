# Sprint 6 四路线候选文本审校记录

## 审校范围

- 候选来源：DeepSeek V4-Pro 离线候选工具
- 候选目录：`.local/deepseek-candidates/sprint6`
- 目标清单：`docs/superpowers/data/sprint6-narrative-targets.json`
- 候选数量：15
- 入库范围：功绩、药堂、黑市、传承、学堂公开压力
- 规则版本：`sprint4-rules-v1`
- 内容版本：`s0.5.0`

## 审校原则

本轮只把候选文本当作草稿方向，不直接复制入库。入库文本必须满足：

- 不改 AP、元石、债务、暴露、伤势、遭遇或蛊虫规则。
- 不生成稳定奖励、酒虫、本命蛊、完整传承、杀招或原著核心机缘。
- 不使用现代词、商业词、爽文胜利词或脱离青茅山语境的物品。
- 每段反馈都服务处境、代价、因果和下一步压力。

## Findings

### 可吸收方向

- 功绩线候选能提示“制度内积累也被看见”，方向可用。
- 药堂线候选能提示“恢复会被记账”，方向可用。
- 黑市线候选能提示“暗口不是公开店铺”，方向可用。
- 传承线候选能提示“半真半假与风险诱惑”，方向可用。
- 学堂公开压力候选能提示“忍让、争辩、硬顶都不是免费选择”，方向可用。

### 必须改写

- 药堂询价候选出现“现银、两、月息四成”等不符合当前资源层表达的词，已改写为元石、人情、欠条。
- 功绩候选出现“同事”等现代组织语感，已改写为族老、学堂、同辈和审计视线。
- 黑市硬顶候选出现“武器被夺”等当前 S0 未建模资产，已改写为元石、伤势和暗口记脸。
- 学堂公开压力触发候选一条输出为“待生成”，拒绝直接入库，改由人工补写。

### 拒绝项

- 任何暗示无代价收益、稳定大额奖励、完整传承、原著核心机缘提前到手的候选均不入库。
- 任何带现代口吻、网文吐槽、系统播报味过重的句子均不入库。
- 任何会让 `canon_strict` 漂成强 IF 的文本均不入库。

## 入库结论

本轮采用“AI 方向 + 人工改写”方式入库。最终写入的文本覆盖：

- `s0.action.scout.merit_notice`
- `s0.action.scout.merit_audit`
- `s0.action.scout.infirmary_lane`
- `s0.action.scout.clan_alley_rumor`
- `s0.action.scout.inheritance_rumor`
- `s0.action.recover.default`
- `s0.action.recover.heavy_to_light`
- `s0.action.recover.light_to_healthy`
- `s0.action.trade.blackmarket_hint`
- `s0.encounter.blackmarket_extortion.trigger`
- `s0.encounter.blackmarket_extortion.retreat`
- `s0.encounter.blackmarket_extortion.confront`
- `s0.encounter.academy_public_pressure.trigger`
- `s0.encounter.academy_public_pressure.yield`
- `s0.encounter.academy_public_pressure.argue`
- `s0.encounter.academy_public_pressure.confront`

## 红线确认

- DeepSeek 未进入 runtime 主链。
- 候选文件未提交。
- 密钥未写入仓库、文档、测试或存档。
- 入库文本未改变规则事实。

