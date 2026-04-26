import type { ActionCommand, LedgerViewModel } from "./index";
import type { CSSProperties } from "react";

export interface LedgerShellProps {
  projection: LedgerViewModel | null;
  status: string;
  onCreateRun: () => Promise<void>;
  onResolveAction: (command: ActionCommand) => Promise<void>;
}

const panelStyle = {
  border: "1px solid #d7c7a0",
  borderRadius: "12px",
  padding: "16px",
  background: "rgba(255, 250, 235, 0.9)",
} satisfies CSSProperties;

export function LedgerShell({
  projection,
  status,
  onCreateRun,
  onResolveAction,
}: LedgerShellProps) {
  const canAct = projection !== null && projection.available_ap > 0;

  return (
    <main className="ledger-shell">
      <section className="hero">
        <p className="eyebrow">RebrnG Sprint 0</p>
        <h1>青茅山账本底座</h1>
        <p>
          规则状态由 Rust 托管，React 只读取账本投影。这里还不是完整 8 回合，只是底座能否
          稳定走通的第一块青砖。
        </p>
      </section>

      <section style={panelStyle}>
        <div className="toolbar">
          <button type="button" onClick={onCreateRun}>
            新建 Sprint 0 单局
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
              <h2>状态条</h2>
              <dl>
                <dt>时段</dt>
                <dd>{projection.current_period}</dd>
                <dt>窗口</dt>
                <dd>{projection.window_type}</dd>
                <dt>AP</dt>
                <dd>{projection.available_ap}</dd>
                <dt>节点</dt>
                <dd>{projection.current_node_id}</dd>
                <dt>暴露</dt>
                <dd>{projection.exposure}</dd>
                <dt>债务压力</dt>
                <dd>{projection.debt_pressure}</dd>
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
                  onClick={() =>
                    onResolveAction({
                      actor: "player",
                      intent: "scout",
                      target: "academy_gate",
                      declared_cost: {
                        ap: 1,
                        primeval_stones: 0,
                        exposure_risk: 0,
                      },
                    })
                  }
                >
                  观察风声
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() =>
                    onResolveAction({
                      actor: "player",
                      intent: "cultivate",
                      target: "academy_gate",
                      declared_cost: {
                        ap: 1,
                        primeval_stones: 0,
                        exposure_risk: 0,
                      },
                    })
                  }
                >
                  月光修行
                </button>
                <button
                  type="button"
                  disabled={!canAct}
                  onClick={() =>
                    onResolveAction({
                      actor: "player",
                      intent: "move",
                      target: "infirmary_lane",
                      declared_cost: {
                        ap: 1,
                        primeval_stones: 0,
                        exposure_risk: 1,
                      },
                    })
                  }
                >
                  去药堂侧巷
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
            </article>
          </div>
        ) : (
          <p className="empty">还没有 active run。先新建单局。</p>
        )}
      </section>
    </main>
  );
}
