# RebrnG Sprint 5 月光蛊离线候选文本审校记录

## 文档角色
本文件记录 Sprint 5 首批月光蛊候选文本的审校结果。候选来源是 `scripts/deepseek-candidates.mjs generate --mock` 生成的本地 mock 候选池；该候选池只用于人工审校，不作为内容源提交。

## 候选池概况
- 生成命令：`node scripts/deepseek-candidates.mjs generate --mock --out .local/deepseek-candidates`
- 校验命令：`node scripts/deepseek-candidates.mjs validate --dir .local/deepseek-candidates`
- 候选数量：10
- 校验结果：10 valid，0 invalid
- 入库方式：人工改写后写入 YAML，并同步 Rust starter 文本

## 可入库并已人工改写
- `s0_action_claim_moonlight_gu_action_result_v001`：用于 `s0.gu.moonlight.claim`，保留“持有不等于可用”和“需要炼化归属”的核心语义。
- `s0_action_claim_moonlight_gu_ledger_feedback_v001`：合并进领取文本的账本后果，不单独新增叙事 id。
- `s0_action_refine_moonlight_gu_action_result_v001`：用于 `s0.gu.moonlight.refine`，强调炼化消耗窗口、进入空窍、后续负担开始。
- `s0_action_cultivate_moonlight_action_result_v001`：用于 `s0.action.cultivate.moonlight`，强调已炼化月光蛊、元石消耗和修行痕迹。
- `s0_action_cultivate_moonlight_blocked_unrefined_v001`：改写为 Rust 禁用原因，不新增 YAML 模板。
- `s0_action_cultivate_moonlight_blocked_no_primeval_stone_v001`：改写为 Rust 禁用原因，不新增 YAML 模板。
- `s0_action_inspect_gu_moonlight_gu_summary_v001` 与 `s0_action_inspect_gu_feeding_stable_v001`：合并为 `s0.gu.moonlight.inspect`。

## 延期候选
- `s0_action_refine_moonlight_gu_blocked_no_aperture_v001`：当前 S0 自由窗口默认空窍已开，暂不入库；若后续重做开窍大典前置互动，再回收。
- `s0_action_inspect_gu_feeding_warning_v001`：当前 Sprint 5 不展开喂养周期和警戒阈值，暂不入库，避免暗示未实现的周期风险。

## 拒绝候选
暂无。mock 候选未出现稳定本命蛊、酒虫、杀招、传承奖励、方源硬绑定、跳过炼化或跳过元石/AP代价等红线问题。

## 入库边界
- 入库文本只改变可见叙事厚度，不改变规则事实。
- 内容版本升为 `s0.4.0`。
- 规则版本保持 `sprint4-rules-v1`。
- `.local/deepseek-candidates/` 不提交。
- 后续若使用真实 DeepSeek 输出，仍必须先进入候选池并经过本文同级审校记录，不能直接写入 YAML。
