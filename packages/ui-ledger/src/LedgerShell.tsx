import { useState } from "react";
import type {
  ActionChoiceGroup,
  ActionChoiceTone,
  ActionChoiceView,
  ActionCommand,
  LedgerViewModel,
} from "./index";

export interface LedgerShellProps {
  projection: LedgerViewModel | null;
  status: string;
  onCreateRun: () => Promise<void>;
  onResolveAction: (command: ActionCommand) => Promise<void>;
  onWriteSave: () => Promise<void>;
  onLoadSave: () => Promise<void>;
}

type LedgerPage =
  | "scene"
  | "map"
  | "resources"
  | "build"
  | "relations"
  | "save"
  | "clues"
  | "ledger";

const zeroDeclaredCost = {
  ap: 0,
  primeval_stones: 0,
  exposure_risk: 0,
};

const pages: Array<{ id: LedgerPage; label: string }> = [
  { id: "scene", label: "正文" },
  { id: "map", label: "节点" },
  { id: "resources", label: "物资债务" },
  { id: "build", label: "修行 Build" },
  { id: "relations", label: "关系局势" },
  { id: "save", label: "存档边界" },
  { id: "clues", label: "风声线索" },
  { id: "ledger", label: "因果账" },
];

const actionGroupOrder: ActionChoiceGroup[] = [
  "encounter",
  "movement",
  "cultivation",
  "information",
  "recovery",
  "trade",
  "wait",
];

const actionGroupLabels: Record<ActionChoiceGroup, string> = {
  encounter: "遭遇决断",
  movement: "移动去处",
  cultivation: "修行资源",
  information: "情报风声",
  recovery: "恢复债务",
  trade: "交易门路",
  wait: "等待阶段",
};

function makeCommand(choice: ActionChoiceView): ActionCommand {
  return {
    actor: "player",
    intent: choice.intent,
    target: choice.target ?? null,
    declared_cost: zeroDeclaredCost,
    context_note: choice.label,
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
  const [activePage, setActivePage] = useState<LedgerPage>("scene");
  const hasRun = projection !== null;

  return (
    <main className="ledger-shell" aria-label="RebrnG 青茅山账本">
      <header className="ledger-top">
        <div>
          <p className="ledger-kicker">RebrnG Sprint 2</p>
          <h1>青茅山冷账</h1>
        </div>
        <div className="ledger-run-controls" aria-label="运行控制">
          <button type="button" onClick={onCreateRun}>
            新开一局
          </button>
          <button type="button" disabled={!hasRun} onClick={onWriteSave}>
            写入 slot_0
          </button>
          <button type="button" onClick={onLoadSave}>
            读取 slot_0
          </button>
        </div>
      </header>

      <section className="ledger-status-strip" aria-label="当前压力">
        {projection ? (
          projection.status_markers.map((marker) => (
            <div className={`ledger-status-item is-${marker.tone}`} key={marker.label}>
              <span>{marker.label}</span>
              <strong>{marker.value}</strong>
            </div>
          ))
        ) : (
          <div className="ledger-status-empty">
            尚无 active run，规则状态仍未开账。
          </div>
        )}
      </section>

      <div className="ledger-system-line" role="status">
        {status}
      </div>

      {projection ? <CurrentLocationNotice projection={projection} /> : null}
      {projection ? <WalkthroughSummary projection={projection} /> : null}

      {projection ? (
        <div className="ledger-layout">
          <nav className="ledger-tabs" aria-label="账页">
            {pages.map((page) => (
              <button
                type="button"
                className={page.id === activePage ? "is-active" : ""}
                key={page.id}
                onClick={() => setActivePage(page.id)}
              >
                {page.label}
              </button>
            ))}
          </nav>

          <section className="ledger-leaf">{renderPage(activePage, projection)}</section>

          <aside className="ledger-actions" aria-label="行动">
            <div className="ledger-actions-heading">
              <span>行动账</span>
              <small>{projection.next_anchor_pressure}</small>
            </div>
            <ActionGroups
              choices={projection.action_choices}
              onResolveAction={onResolveAction}
            />
          </aside>
        </div>
      ) : (
        <section className="ledger-empty-state">
          <h2>账页未启</h2>
          <p>
            先让 Rust 创建单局。React 只拿到账本投影，不持有完整规则状态，也不在本地结算代价。
          </p>
        </section>
      )}
    </main>
  );
}

function WalkthroughSummary({ projection }: { projection: LedgerViewModel }) {
  const feedback = projection.recent_feedback?.summary ?? "尚未产生新的因果反馈";
  const location = currentNodeTitle(projection);

  return (
    <section className="ledger-walkthrough-summary" aria-label="走查摘要">
      <div>
        <span>当前位置</span>
        <strong>{location}</strong>
        <small>
          {projection.current_period} / {projection.available_ap} AP / {projection.window_id}
        </small>
      </div>
      <div>
        <span>最近反馈</span>
        <strong>{feedback}</strong>
        <small>{projection.next_anchor_pressure}</small>
      </div>
      <div>
        <span>风险</span>
        <strong>
          伤势 {injuryLabel(projection.injury_level)} / 暴露 {projection.exposure}
        </strong>
        <small>债务压力 {projection.debt_pressure}</small>
      </div>
      <div>
        <span>阶段收口</span>
        <strong>{projection.stage_closure.title}</strong>
        <small>{projection.stage_closure.summary}</small>
      </div>
    </section>
  );
}

function CurrentLocationNotice({ projection }: { projection: LedgerViewModel }) {
  const location = currentNodeTitle(projection);
  const feedback = projection.recent_feedback?.summary ?? projection.scene_text;

  return (
    <section className="ledger-location-notice" aria-live="polite">
      <div>
        <span>当前位置已记为</span>
        <strong>{location}</strong>
        <small>{projection.current_node_id}</small>
      </div>
      <p>{feedback}</p>
    </section>
  );
}

function ActionGroups({
  choices,
  onResolveAction,
}: {
  choices: ActionChoiceView[];
  onResolveAction: (command: ActionCommand) => Promise<void>;
}) {
  return (
    <div className="ledger-action-groups">
      {actionGroupOrder.map((group) => {
        const groupChoices = choices.filter((choice) => choice.group === group);
        if (!groupChoices.length) {
          return null;
        }

        const enabledCount = groupChoices.filter((choice) => choice.enabled).length;

        return (
          <section className="ledger-action-group" key={group}>
            <div className="ledger-action-group-title">
              <h3>{actionGroupLabels[group]}</h3>
              <small>
                {enabledCount}/{groupChoices.length} 可用
              </small>
            </div>
            <div className="ledger-action-list">
              {groupChoices.map((choice) => (
                <button
                  type="button"
                  className={`ledger-action is-${choice.tone}`}
                  disabled={!choice.enabled}
                  key={choice.id}
                  onClick={() => onResolveAction(makeCommand(choice))}
                >
                  <span className="ledger-action-title">
                    <span>{choice.label}</span>
                    <i>{toneLabel(choice.tone)}</i>
                  </span>
                  <span className="ledger-action-meta">
                    <small>代价：{choice.cost_hint}</small>
                    <em>风险：{choice.risk_hint}</em>
                  </span>
                  <strong>后果：{choice.consequence_hint}</strong>
                  {!choice.enabled && choice.disabled_reason ? (
                    <b>受阻：{choice.disabled_reason}</b>
                  ) : null}
                </button>
              ))}
            </div>
          </section>
        );
      })}
    </div>
  );
}

function renderPage(page: LedgerPage, projection: LedgerViewModel) {
  switch (page) {
    case "map":
      return <MapPage projection={projection} />;
    case "resources":
      return <ResourcesPage projection={projection} />;
    case "build":
      return <BuildPage projection={projection} />;
    case "relations":
      return <RelationsPage projection={projection} />;
    case "save":
      return <SavePage projection={projection} />;
    case "clues":
      return <CluesPage projection={projection} />;
    case "ledger":
      return <CausalityPage projection={projection} />;
    case "scene":
    default:
      return <ScenePage projection={projection} />;
  }
}

function ScenePage({ projection }: { projection: LedgerViewModel }) {
  return (
    <article className="ledger-page">
      <p className="ledger-page-label">正文场景</p>
      <h2>
        第 {projection.current_day} 日 / {projection.current_period} /{" "}
        {projection.current_node_id}
      </h2>
      {projection.recent_feedback ? (
        <RecentFeedback feedback={projection.recent_feedback} />
      ) : null}
      <p className="scene-text">{projection.scene_text}</p>
      <div className={`danger-note is-${projection.stage_closure.status}`}>
        <strong>{projection.stage_closure.title}</strong>
        <span>{projection.stage_closure.summary}</span>
      </div>
      <div className="narrative-boundary">
        <strong>运行时 AI：</strong>
        {projection.narrative_boundary.runtime_ai_enabled ? "已接入" : "未接入"}
        <span>{projection.narrative_boundary.source}</span>
      </div>
      {projection.active_encounter_id ? (
        <div className="danger-note is-danger">
          <strong>遭遇压身：</strong>
          {projection.active_encounter_known_risk ?? projection.active_encounter_id}
        </div>
      ) : null}
    </article>
  );
}

function RecentFeedback({
  feedback,
}: {
  feedback: NonNullable<LedgerViewModel["recent_feedback"]>;
}) {
  return (
    <div className={`recent-feedback is-${feedback.tone}`}>
      <small>{feedback.source_kind}</small>
      <strong>{feedback.title}</strong>
      <p>{feedback.summary}</p>
    </div>
  );
}

function MapPage({ projection }: { projection: LedgerViewModel }) {
  return (
    <article className="ledger-page">
      <p className="ledger-page-label">节点地图</p>
      <h2>{projection.node_view.current_region_id}</h2>
      <div className="node-ledger">
        {projection.node_view.visible_nodes.map((node) => (
          <div className={node.current ? "node-row is-current" : "node-row"} key={node.id}>
            <span>{node.title}</span>
            <small>{node.id}</small>
            <em>安全：{node.safety}</em>
          </div>
        ))}
      </div>
    </article>
  );
}

function ResourcesPage({ projection }: { projection: LedgerViewModel }) {
  return (
    <article className="ledger-page">
      <p className="ledger-page-label">物资与债务</p>
      <h2>能用的少，欠下的会回来。</h2>
      <dl className="ledger-rows">
        <Row label="元石" value={projection.primeval_stones} />
        <Row label="材料" value={projection.materials} />
        <Row label="功绩" value={projection.merit} />
        <Row label="债务压力" value={projection.debt_pressure} />
        <Row label="暴露" value={projection.exposure} />
      </dl>
    </article>
  );
}

function BuildPage({ projection }: { projection: LedgerViewModel }) {
  const build = projection.build_view;

  return (
    <article className="ledger-page">
      <p className="ledger-page-label">空窍 / 修行 / Build</p>
      <h2>求活路线不等于流派，本命蛊不等于核心蛊。</h2>
      <dl className="ledger-rows">
        <Row label="求活路线" value={build.survival_route} />
        <Row label="主修流派" value={build.main_path} />
        <Row label="道痕保留" value={build.dao_mark_note} />
        <Row label="核心蛊" value={build.core_gu} />
        <Row label="辅助蛊" value={build.support_gu} />
        <Row label="本命蛊" value={build.vital_gu} />
        <Row label="喂养维护" value={build.maintenance_pressure} />
        <Row label="主要缺口" value={build.gap_summary} />
      </dl>
    </article>
  );
}

function RelationsPage({ projection }: { projection: LedgerViewModel }) {
  const relation = projection.relationship_view;

  return (
    <article className="ledger-page">
      <p className="ledger-page-label">关系局势</p>
      <h2>庇护、利用、债与门路，都先记在同一本账里。</h2>
      <dl className="ledger-rows">
        <Row label="家族秩序" value={relation.family_pressure} />
        <Row label="药堂债" value={relation.infirmary_debt} />
        <Row label="人情债" value={relation.favor_debt} />
        <Row label="黑市门路" value={relation.blackmarket_access} />
      </dl>
    </article>
  );
}

function SavePage({ projection }: { projection: LedgerViewModel }) {
  const save = projection.save_view;

  return (
    <article className="ledger-page">
      <p className="ledger-page-label">存档 / 阶段检查点</p>
      <h2>能读回当前局势，但不把每个选择都变成回退点。</h2>
      <dl className="ledger-rows">
        <Row label="存档格式" value={save.save_version} />
        <Row label="规则版本" value={save.rules_version} />
        <Row label="内容版本" value={save.content_version} />
        <Row label="RNG 状态" value={save.rng_state} />
        <Row label="迁移状态" value={save.migration_state} />
        <Row label="当前快照" value={save.current_checkpoint_id} />
        <Row label="检查点数量" value={save.checkpoint_count} />
        <Row
          label="阶段检查点"
          value={save.stage_checkpoint_ids.length ? save.stage_checkpoint_ids.join(" / ") : "暂无"}
        />
        <Row label="回退规则" value={save.rollback_policy} />
      </dl>
    </article>
  );
}

function CluesPage({ projection }: { projection: LedgerViewModel }) {
  return (
    <article className="ledger-page">
      <p className="ledger-page-label">风声与线索</p>
      <h2>只记可见因果，不亮隐藏数值。</h2>
      <div className="clue-access-note">{projection.clue_view.blackmarket_access_summary}</div>
      <div className="clue-ledger">
        {projection.clue_view.known_clues.length ? (
          projection.clue_view.known_clues.map((clue) => (
            <div className={`clue-row is-${clue.tone}`} key={clue.id}>
              <strong>{clue.label}</strong>
              <p>{clue.summary}</p>
            </div>
          ))
        ) : (
          <p className="ledger-muted">暂无已验入账的风声。</p>
        )}
      </div>
    </article>
  );
}

function CausalityPage({ projection }: { projection: LedgerViewModel }) {
  const [latest, ...older] = [...projection.ledger_entries].reverse();

  return (
    <article className="ledger-page">
      <p className="ledger-page-label">因果账</p>
      <h2>最近落账</h2>
      {latest ? (
        <div className="latest-ledger-entry">
          <span>{latest.kind}</span>
          <p>{latest.text}</p>
        </div>
      ) : null}
      <ol className="causality-list">
        {older.map((entry, index) => (
          <li key={`${entry.kind}-${index}`}>
            <span>{entry.kind}</span>
            <p>{entry.text}</p>
          </li>
        ))}
      </ol>
      <div className="performance-line">
        resolve_action {projection.performance.resolve_action_ms}ms / projection{" "}
        {projection.performance.projection_ms}ms / save_load{" "}
        {projection.performance.save_load_ms}ms
      </div>
    </article>
  );
}

function Row({ label, value }: { label: string; value: string | number }) {
  return (
    <>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </>
  );
}

function currentNodeTitle(projection: LedgerViewModel): string {
  return (
    projection.node_view.visible_nodes.find((node) => node.current)?.title ??
    projection.current_node_id
  );
}

function injuryLabel(injury: LedgerViewModel["injury_level"]): string {
  const labels: Record<LedgerViewModel["injury_level"], string> = {
    healthy: "健康",
    light: "轻伤",
    heavy: "重伤",
  };
  return labels[injury];
}

export function toneLabel(tone: ActionChoiceTone): string {
  const labels: Record<ActionChoiceTone, string> = {
    normal: "常态",
    safe: "稳妥",
    risky: "风险",
    danger: "危险",
    blocked: "受阻",
  };
  return labels[tone];
}
