import { useState } from "react";
import type {
  ActionChoiceGroup,
  ActionChoiceTone,
  ActionChoiceView,
  ActionCommand,
  DialogueTimelineView,
  LedgerViewModel,
  SetupCandidateView,
  SetupCommand,
  SetupTalentCandidateView,
  SetupViewModel,
} from "./index";

export interface LedgerShellProps {
  projection: LedgerViewModel | null;
  setupView: SetupViewModel | null;
  status: string;
  onCreateRun: () => Promise<void>;
  onCreatePresetRun: () => Promise<void>;
  onResolveSetupChoice: (command: SetupCommand) => Promise<void>;
  onConfirmSetupRun: () => Promise<void>;
  onResolveAction: (command: ActionCommand) => Promise<void>;
  onWriteSave: () => Promise<void>;
  onLoadSave: () => Promise<void>;
}

type LedgerPage =
  | "overview"
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
  { id: "overview", label: "总览" },
  { id: "map", label: "节点" },
  { id: "resources", label: "物资债务" },
  { id: "build", label: "修行 Build" },
  { id: "relations", label: "关系" },
  { id: "save", label: "存档" },
  { id: "clues", label: "线索" },
  { id: "ledger", label: "因果" },
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
  setupView,
  status,
  onCreateRun,
  onCreatePresetRun,
  onResolveSetupChoice,
  onConfirmSetupRun,
  onResolveAction,
  onWriteSave,
  onLoadSave,
}: LedgerShellProps) {
  const hasRun = projection !== null;
  const hasSetup = setupView !== null;

  return (
    <main className="ledger-shell" aria-label="RebrnG 青茅山人生重开账本">
      <header className="ledger-top">
        <div>
          <p className="ledger-kicker">RebrnG Sprint 3</p>
          <h1>青茅山冷账</h1>
          <p className="ledger-subtitle">
            对话流为主阅读层，账本明细保留规则真相的边界。
          </p>
        </div>
        <div className="ledger-run-controls" aria-label="运行控制">
          <button type="button" onClick={onCreateRun}>
            新开一局
          </button>
          <button type="button" className="button-secondary" onClick={onCreatePresetRun}>
            快速预设开局
          </button>
          <button type="button" disabled={!hasRun} onClick={onWriteSave}>
            写入 slot_0
          </button>
          <button type="button" onClick={onLoadSave}>
            读取 slot_0
          </button>
        </div>
      </header>

      <StatusStrip projection={projection} setupView={setupView} />

      <div className="ledger-system-line" role="status">
        {status}
      </div>

      {hasSetup ? (
        <SetupWorkspace
          setupView={setupView}
          onResolveSetupChoice={onResolveSetupChoice}
          onConfirmSetupRun={onConfirmSetupRun}
        />
      ) : projection ? (
        <RunWorkspace projection={projection} onResolveAction={onResolveAction} />
      ) : (
        <EmptyState onCreateRun={onCreateRun} onCreatePresetRun={onCreatePresetRun} />
      )}
    </main>
  );
}

function StatusStrip({
  projection,
  setupView,
}: {
  projection: LedgerViewModel | null;
  setupView: SetupViewModel | null;
}) {
  if (projection) {
    return (
      <section className="ledger-status-strip" aria-label="当前压力">
        {projection.status_markers.map((marker) => (
          <div className={`ledger-status-item is-${marker.tone}`} key={marker.label}>
            <span>{marker.label}</span>
            <strong>{marker.value}</strong>
          </div>
        ))}
      </section>
    );
  }

  if (setupView) {
    const setupMarkers = [
      { label: "模式", value: setupView.mode },
      { label: "出身", value: setupView.selected_origin_id ?? "未定" },
      { label: "天赋", value: `${setupView.selected_talent_ids.length}/3` },
      { label: "内容版本", value: setupView.content_version },
    ];

    return (
      <section className="ledger-status-strip" aria-label="设置阶段">
        {setupMarkers.map((marker) => (
          <div className="ledger-status-item" key={marker.label}>
            <span>{marker.label}</span>
            <strong>{marker.value}</strong>
          </div>
        ))}
      </section>
    );
  }

  return (
    <section className="ledger-status-strip" aria-label="当前压力">
      <div className="ledger-status-empty">
        尚无 active run。点击“新开一局”进入开窍大典后的设置层。
      </div>
    </section>
  );
}

function SetupWorkspace({
  setupView,
  onResolveSetupChoice,
  onConfirmSetupRun,
}: {
  setupView: SetupViewModel;
  onResolveSetupChoice: (command: SetupCommand) => Promise<void>;
  onConfirmSetupRun: () => Promise<void>;
}) {
  return (
    <div className="dialogue-workspace is-setup">
      <section className="dialogue-main">
        <DialoguePanel dialogue={setupView.dialogue} />
        <SetupChoiceDock
          setupView={setupView}
          onResolveSetupChoice={onResolveSetupChoice}
          onConfirmSetupRun={onConfirmSetupRun}
        />
      </section>
      <aside className="ledger-detail-panel" aria-label="重开设置账页">
        <SetupLedgerView setupView={setupView} />
      </aside>
    </div>
  );
}

function RunWorkspace({
  projection,
  onResolveAction,
}: {
  projection: LedgerViewModel;
  onResolveAction: (command: ActionCommand) => Promise<void>;
}) {
  return (
    <div className="dialogue-workspace">
      <section className="dialogue-main">
        <DialoguePanel dialogue={projection.dialogue} />
        <ActionGroups choices={projection.action_choices} onResolveAction={onResolveAction} />
      </section>
      <LedgerDetailPanel projection={projection} />
    </div>
  );
}

function DialoguePanel({ dialogue }: { dialogue: DialogueTimelineView }) {
  return (
    <article className={`dialogue-panel is-${dialogue.tone}`} aria-label="对话流正文">
      <p className="ledger-page-label">对话流</p>
      <h2>{dialogue.stage_title}</h2>
      <div className="dialogue-paragraphs">
        {dialogue.paragraphs.map((paragraph, index) => (
          <p key={`${index}-${paragraph.slice(0, 12)}`}>{paragraph}</p>
        ))}
      </div>

      <div className="dialogue-result-grid">
        <ResultChip label="上一选择" value={dialogue.previous_choice_title ?? "尚未落子"} />
        <ResultChip label="结果摘要" value={dialogue.previous_result_summary ?? "等待第一笔因果"} />
        <ResultChip label="最新落账" value={dialogue.latest_ledger_delta ?? "暂无新增账目"} />
      </div>

      <div className="dialogue-boundary">
        <strong>{dialogue.mode_gate_hint}</strong>
        <span>{dialogue.source_summary}</span>
      </div>
    </article>
  );
}

function ResultChip({ label, value }: { label: string; value: string }) {
  return (
    <div className="result-chip">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function SetupChoiceDock({
  setupView,
  onResolveSetupChoice,
  onConfirmSetupRun,
}: {
  setupView: SetupViewModel;
  onResolveSetupChoice: (command: SetupCommand) => Promise<void>;
  onConfirmSetupRun: () => Promise<void>;
}) {
  return (
    <section className="choice-dock" aria-label="人生重开设置选项">
      <div className="choice-dock-heading">
        <span>开局落账</span>
        <small>先定出身，再选三项天赋</small>
      </div>

      <SetupCandidateList
        title="出身"
        candidates={setupView.origin_candidates}
        onSelect={(candidate) =>
          onResolveSetupChoice({
            intent: "select_origin",
            target_id: candidate.id,
          })
        }
      />

      <SetupTalentList
        candidates={setupView.talent_candidates}
        onToggle={(candidate) =>
          onResolveSetupChoice({
            intent: "toggle_talent",
            target_id: candidate.id,
          })
        }
      />

      <div className="confirm-panel">
        {setupView.confirm_blockers.length ? (
          <ul>
            {setupView.confirm_blockers.map((blocker) => (
              <li key={blocker}>{blocker}</li>
            ))}
          </ul>
        ) : (
          <p>确认条件已满足。开窍大典结果会写入 S0 青茅山账本。</p>
        )}
        <button
          type="button"
          className="confirm-button"
          disabled={!setupView.confirm_enabled}
          onClick={onConfirmSetupRun}
        >
          确认开窍大典，进入青茅山
        </button>
      </div>
    </section>
  );
}

function SetupCandidateList({
  title,
  candidates,
  onSelect,
}: {
  title: string;
  candidates: SetupCandidateView[];
  onSelect: (candidate: SetupCandidateView) => Promise<void>;
}) {
  return (
    <section className="setup-list">
      <h3>{title}</h3>
      <div className="setup-card-list">
        {candidates.map((candidate) => (
          <button
            type="button"
            className={candidate.selected ? "setup-card is-selected" : "setup-card"}
            disabled={!candidate.enabled && !candidate.selected}
            key={candidate.id}
            onClick={() => onSelect(candidate)}
          >
            <span>{candidate.title}</span>
            <p>{candidate.summary}</p>
            <small>{candidate.disabled_reason ?? candidate.evidence}</small>
          </button>
        ))}
      </div>
    </section>
  );
}

function SetupTalentList({
  candidates,
  onToggle,
}: {
  candidates: SetupTalentCandidateView[];
  onToggle: (candidate: SetupTalentCandidateView) => Promise<void>;
}) {
  return (
    <section className="setup-list">
      <h3>天赋</h3>
      <div className="setup-card-list">
        {candidates.map((candidate) => (
          <button
            type="button"
            className={candidate.selected ? "setup-card is-selected" : "setup-card"}
            disabled={!candidate.enabled && !candidate.selected}
            key={candidate.id}
            onClick={() => onToggle(candidate)}
          >
            <span>{candidate.title}</span>
            <p>{candidate.summary}</p>
            <em>{candidate.pressure_note}</em>
            <small>
              {candidate.disabled_reason ??
                `${candidate.intensity} / ${candidate.route_tags.join("、")}`}
            </small>
          </button>
        ))}
      </div>
    </section>
  );
}

function SetupLedgerView({ setupView }: { setupView: SetupViewModel }) {
  const resource = setupView.resource_preview;

  return (
    <div className="setup-ledger-view">
      <p className="ledger-page-label">设置账页</p>
      <h2>{setupView.opening_rite_title}</h2>
      <p>{setupView.opening_rite_summary}</p>

      <dl className="ledger-rows compact">
        {setupView.attributes.map((attribute) => (
          <Row
            key={attribute.id}
            label={attribute.label}
            value={`${attribute.value} / ${attribute.min}-${attribute.max}`}
          />
        ))}
      </dl>

      <dl className="ledger-rows compact">
        <Row label="元石" value={resource.primeval_stones} />
        <Row label="材料" value={resource.materials} />
        <Row label="功绩" value={resource.merit} />
        <Row
          label="债务"
          value={
            resource.infirmary_debt +
            resource.favor_debt +
            resource.organization_debt
          }
        />
        <Row label="关注暴露" value={resource.exposure} />
      </dl>
    </div>
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
    <section className="choice-dock" aria-label="可选行动">
      <div className="choice-dock-heading">
        <span>行动选择</span>
        <small>所有代价由 Rust 规则结算</small>
      </div>
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
    </section>
  );
}

function LedgerDetailPanel({ projection }: { projection: LedgerViewModel }) {
  const [activePage, setActivePage] = useState<LedgerPage>("overview");

  return (
    <aside className="ledger-detail-panel" aria-label="账本明细侧栏">
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
    </aside>
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
    case "overview":
    default:
      return <OverviewPage projection={projection} />;
  }
}

function OverviewPage({ projection }: { projection: LedgerViewModel }) {
  return (
    <article className="ledger-page">
      <p className="ledger-page-label">账本总览</p>
      <h2>{currentNodeTitle(projection)}</h2>
      <dl className="ledger-rows compact">
        <Row label="时段" value={`${projection.current_period} / ${projection.window_id}`} />
        <Row label="AP" value={projection.available_ap} />
        <Row label="元石" value={projection.primeval_stones} />
        <Row label="债务压力" value={projection.debt_pressure} />
        <Row label="暴露" value={projection.exposure} />
        <Row label="伤势" value={injuryLabel(projection.injury_level)} />
      </dl>
      <div className={`danger-note is-${projection.stage_closure.status}`}>
        <strong>{projection.stage_closure.title}</strong>
        <span>{projection.stage_closure.summary}</span>
      </div>
    </article>
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

function EmptyState({
  onCreateRun,
  onCreatePresetRun,
}: {
  onCreateRun: () => Promise<void>;
  onCreatePresetRun: () => Promise<void>;
}) {
  return (
    <section className="ledger-empty-state">
      <p className="ledger-page-label">尚未开局</p>
      <h2>先过开窍大典，再进青茅山。</h2>
      <p>
        新开一局会进入人生重开设置层；快速预设开局保留给自动测试和快速走查。
      </p>
      <div className="empty-actions">
        <button type="button" onClick={onCreateRun}>
          新开一局
        </button>
        <button type="button" className="button-secondary" onClick={onCreatePresetRun}>
          快速预设开局
        </button>
      </div>
    </section>
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
