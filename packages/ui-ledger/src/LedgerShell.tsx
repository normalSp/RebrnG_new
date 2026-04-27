import type { CSSProperties } from "react";
import type { ActionCommand, ActionIntent, LedgerViewModel } from "./index";

export interface LedgerShellProps {
  projection: LedgerViewModel | null;
  status: string;
  onCreateRun: () => Promise<void>;
  onResolveAction: (command: ActionCommand) => Promise<void>;
  onWriteSave: () => Promise<void>;
  onLoadSave: () => Promise<void>;
}

const panelStyle = {
  border: "1px solid #d7c7a0",
  borderRadius: "12px",
  padding: "16px",
  background: "rgba(255, 250, 235, 0.9)",
} satisfies CSSProperties;

const zeroDeclaredCost = {
  ap: 0,
  primeval_stones: 0,
  exposure_risk: 0,
};

function makeCommand(intent: ActionIntent, target?: string): ActionCommand {
  return {
    actor: "player",
    intent,
    target: target ?? null,
    declared_cost: zeroDeclaredCost,
  };
}

export function LedgerShell({
  projection,
  status,
  onCreateRun,
  onResolveAction,
  onWriteSave,
  onLoadSave,
}: LedgerShellProps) {
  const hasRun = projection !== null;
  const hasActiveEncounter = projection?.active_encounter_id != null;
  const canAct =
    projection !== null &&
    projection.window_type === "free" &&
    !hasActiveEncounter &&
    projection.available_ap > 0;
  const canEncounterDecision =
    projection !== null &&
    projection.window_type === "free" &&
    hasActiveEncounter &&
    projection.available_ap > 0;

  return (
    <main className="ledger-shell">
      <section className="hero">
        <p className="eyebrow">RebrnG Sprint 0</p>
        <h1>青茅山账本底座</h1>
        <p>
          规则状态由 Rust 托管，React 只读取账本投影。阶段 5 先验证时间、节点移动、
          资源债务和暴露都能从同一条行动管线结算。
        </p>
      </section>

      <section style={panelStyle}>
        <div className="toolbar">
          <button type="button" onClick={onCreateRun}>
            新建 Sprint 0 单局
          </button>
          <button type="button" disabled={!hasRun} onClick={onWriteSave}>
            写入 slot_0
          </button>
          <button type="button" onClick={onLoadSave}>
            读取 slot_0
          </button>
          <span>{status}</span>
        </div>

        {projection ? (
          <div className="ledger-grid">
            <article>
              <h2>正文场景</h2>
              <p>{projection.scene_text}</p>
            </article>

            <article>
              <h2>窗口与位置</h2>
              <dl>
                <dt>章节日</dt>
                <dd>第 {projection.current_day} 日</dd>
                <dt>时段</dt>
                <dd>{projection.current_period}</dd>
                <dt>窗口</dt>
                <dd>{projection.window_type}</dd>
                <dt>窗口 ID</dt>
                <dd>{projection.window_id}</dd>
                <dt>AP</dt>
                <dd>{projection.available_ap}</dd>
                <dt>节点</dt>
                <dd>{projection.current_node_id}</dd>
              </dl>
            </article>

            <article>
              <h2>资源账</h2>
              <dl>
                <dt>元石</dt>
                <dd>{projection.primeval_stones}</dd>
                <dt>材料</dt>
                <dd>{projection.materials}</dd>
                <dt>功绩</dt>
                <dd>{projection.merit}</dd>
                <dt>债务压力</dt>
                <dd>{projection.debt_pressure}</dd>
                <dt>暴露</dt>
                <dd>{projection.exposure}</dd>
              </dl>
            </article>

            <article>
              <h2>遭遇与伤势</h2>
              <dl>
                <dt>伤势</dt>
                <dd>{projection.injury_level}</dd>
                <dt>当前遭遇</dt>
                <dd>{projection.active_encounter_id ?? "none"}</dd>
                <dt>已知风险</dt>
                <dd>{projection.active_encounter_known_risk ?? "none"}</dd>
              </dl>
            </article>

            <article>
              <h2>Build 痕迹</h2>
              <p>{projection.build_summary}</p>
            </article>

            <article>
              <h2>行动</h2>
              <div className="actions">
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("scout", "academy_gate"))}
                >
                  观察学堂
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("cultivate", "academy_gate"))}
                >
                  月光修行
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("move", "moonlight_corner"))}
                >
                  去月光角
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("move", "merit_notice"))}
                >
                  去功绩告示
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("scout", "merit_notice"))}
                >
                  查功绩
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("move", "infirmary_lane"))}
                >
                  去药堂侧巷
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("recover", "infirmary_lane"))}
                >
                  药堂恢复
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("move", "blackmarket_hint"))}
                >
                  摸黑市暗口
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("trade", "blackmarket_hint"))}
                >
                  黑市换料
                </button>
                <button
                  type="button"
                  disabled={!canEncounterDecision}
                  onClick={() =>
                    onResolveAction(
                      makeCommand("retreat", projection.active_encounter_id ?? undefined),
                    )
                  }
                >
                  跑路
                </button>
                <button
                  type="button"
                  disabled={!canEncounterDecision}
                  onClick={() =>
                    onResolveAction(
                      makeCommand("confront", projection.active_encounter_id ?? undefined),
                    )
                  }
                >
                  硬顶
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() => onResolveAction(makeCommand("wait"))}
                >
                  等过时段
                </button>
              </div>
            </article>

            <article>
              <h2>因果账</h2>
              <ol>
                {projection.ledger_entries.map((entry, index) => (
                  <li key={`${entry.kind}-${index}`}>{entry.text}</li>
                ))}
              </ol>
            </article>

            <article>
              <h2>性能</h2>
              <p>resolve_action: {projection.performance.resolve_action_ms}ms</p>
              <p>projection: {projection.performance.projection_ms}ms</p>
              <p>save/load: {projection.performance.save_load_ms}ms</p>
            </article>
          </div>
        ) : (
          <p className="empty">还没有 active run。先新建单局。</p>
        )}
      </section>
    </main>
  );
}
