use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const DEFAULT_RUN_ID: &str = "sprint-0-active-run";
pub const STARTER_CONTENT_VERSION: &str = "s0.0.1";
pub const SAVE_FORMAT_VERSION: &str = "sprint0-save-v1";

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RunMode {
    #[default]
    CanonStrict,
    SandboxIf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WindowType {
    Free,
    Anchor,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionIntent {
    Move,
    Cultivate,
    Scout,
    Recover,
    Trade,
    Retreat,
    Wait,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PipelineStep {
    AvailabilityCheck,
    CostReservation,
    SubsystemResolution,
    AnchorRecalculation,
    EffectCommit,
    LedgerAppend,
    ProjectionRefresh,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeState {
    pub chapter_day: u8,
    pub period: String,
    pub window_type: WindowType,
    pub ap: u8,
    pub next_anchor_pressure: String,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            chapter_day: 1,
            period: "清晨".to_string(),
            window_type: WindowType::Free,
            ap: 2,
            next_anchor_pressure: "学堂点卯将近".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorldSpaceState {
    pub current_node_id: String,
    pub current_region_id: String,
}

impl Default for WorldSpaceState {
    fn default() -> Self {
        Self {
            current_node_id: "academy_gate".to_string(),
            current_region_id: "qingmao_core".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceState {
    pub primeval_stones: i32,
    pub debt_pressure: i32,
    pub exposure: i32,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self {
            primeval_stones: 3,
            debt_pressure: 0,
            exposure: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildState {
    pub survival_route: String,
    pub main_path: Option<String>,
    pub dao_mark_note: Option<String>,
    pub maintenance_pressure: String,
}

impl Default for BuildState {
    fn default() -> Self {
        Self {
            survival_route: "未定：仍在学堂秩序缝隙里求活".to_string(),
            main_path: None,
            dao_mark_note: None,
            maintenance_pressure: "暂无蛊虫喂养压力".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerEntry {
    pub kind: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameState {
    pub run_id: String,
    pub mode: RunMode,
    pub chapter: String,
    pub content_version: String,
    pub time: TimeState,
    pub world: WorldSpaceState,
    pub resources: ResourceState,
    pub build: BuildState,
    pub ledger: Vec<LedgerEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveMetadata {
    pub slot_id: String,
    pub save_version: String,
    pub mode: RunMode,
    pub current_stage: String,
    pub content_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveCheckpoint {
    pub checkpoint_id: String,
    pub chapter: String,
    pub period: String,
    pub node_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveEnvelope {
    pub metadata: SaveMetadata,
    pub snapshot: GameState,
    pub ledger: Vec<LedgerEntry>,
    pub checkpoints: Vec<SaveCheckpoint>,
    pub rng_state: String,
    pub migration_state: String,
}

impl SaveEnvelope {
    pub fn from_state(slot_id: impl Into<String>, state: GameState) -> Self {
        let checkpoint = SaveCheckpoint {
            checkpoint_id: "sprint_0_current".to_string(),
            chapter: state.chapter.clone(),
            period: state.time.period.clone(),
            node_id: state.world.current_node_id.clone(),
        };

        Self {
            metadata: SaveMetadata {
                slot_id: slot_id.into(),
                save_version: SAVE_FORMAT_VERSION.to_string(),
                mode: state.mode.clone(),
                current_stage: state.chapter.clone(),
                content_version: state.content_version.clone(),
            },
            ledger: state.ledger.clone(),
            snapshot: state,
            checkpoints: vec![checkpoint],
            rng_state: "sprint_0_deterministic_seed".to_string(),
            migration_state: "none".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeclaredCost {
    pub ap: u8,
    pub primeval_stones: i32,
    pub exposure_risk: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionCommand {
    pub actor: String,
    pub intent: ActionIntent,
    pub target: Option<String>,
    #[serde(default)]
    pub declared_cost: DeclaredCost,
    #[serde(default)]
    pub context_note: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerformanceMetrics {
    pub resolve_action_ms: u64,
    pub projection_ms: u64,
    pub save_load_ms: u64,
    pub bundle_load_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerViewModel {
    pub scene_text: String,
    pub current_period: String,
    pub window_type: WindowType,
    pub available_ap: u8,
    pub current_node_id: String,
    pub exposure: i32,
    pub debt_pressure: i32,
    pub build_summary: String,
    pub ledger_entries: Vec<LedgerEntry>,
    pub performance: PerformanceMetrics,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionResponse {
    pub projection: LedgerViewModel,
    pub performance: PerformanceMetrics,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionResult {
    pub state: GameState,
    pub response: ActionResponse,
    pub pipeline_trace: Vec<PipelineStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommandErrorKind {
    Validation,
    Content,
    Save,
    Io,
    Internal,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandError {
    pub kind: CommandErrorKind,
    pub message: String,
    pub diagnostics: Option<String>,
}

impl CommandError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            kind: CommandErrorKind::Validation,
            message: message.into(),
            diagnostics: None,
        }
    }

    pub fn content(message: impl Into<String>, diagnostics: impl Into<String>) -> Self {
        Self {
            kind: CommandErrorKind::Content,
            message: message.into(),
            diagnostics: Some(diagnostics.into()),
        }
    }

    pub fn internal(message: impl Into<String>, diagnostics: impl Into<String>) -> Self {
        Self {
            kind: CommandErrorKind::Internal,
            message: message.into(),
            diagnostics: Some(diagnostics.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentManifest {
    pub content_id: String,
    pub version: String,
    pub title: String,
    pub entry_scene_id: String,
    pub node_count: usize,
    pub action_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentNode {
    pub id: String,
    pub title: String,
    pub safety: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentAction {
    pub id: String,
    pub label: String,
    pub intent: ActionIntent,
    pub target: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentSource {
    pub content_id: String,
    pub version: String,
    pub title: String,
    pub entry_scene_id: String,
    #[serde(default)]
    pub nodes: Vec<ContentNode>,
    #[serde(default)]
    pub actions: Vec<ContentAction>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBundle {
    pub manifest: ContentManifest,
    pub nodes: Vec<ContentNode>,
    pub actions: Vec<ContentAction>,
}

impl ContentBundle {
    pub fn from_source(source: ContentSource) -> Result<Self, CommandError> {
        if source.content_id.trim().is_empty() {
            return Err(CommandError::content(
                "内容包缺少 content_id",
                "content_id is empty",
            ));
        }

        if source.version.trim().is_empty() {
            return Err(CommandError::content(
                "内容包缺少 version",
                "version is empty",
            ));
        }

        if !source
            .nodes
            .iter()
            .any(|node| node.id == source.entry_scene_id)
        {
            return Err(CommandError::content(
                "入口节点不存在",
                format!(
                    "entry_scene_id '{}' not found in nodes",
                    source.entry_scene_id
                ),
            ));
        }

        Ok(Self {
            manifest: ContentManifest {
                content_id: source.content_id,
                version: source.version,
                title: source.title,
                entry_scene_id: source.entry_scene_id,
                node_count: source.nodes.len(),
                action_count: source.actions.len(),
            },
            nodes: source.nodes,
            actions: source.actions,
        })
    }
}

pub fn starter_content_manifest() -> ContentManifest {
    ContentManifest {
        content_id: "s0.qingmao.foundation".to_string(),
        version: STARTER_CONTENT_VERSION.to_string(),
        title: "青茅山 Sprint 0 内容骨架".to_string(),
        entry_scene_id: "academy_gate".to_string(),
        node_count: 3,
        action_count: 4,
    }
}

pub fn create_run(mode: RunMode, content_version: impl Into<String>) -> GameState {
    GameState {
        run_id: DEFAULT_RUN_ID.to_string(),
        mode,
        chapter: "s0_qingmao_foundation".to_string(),
        content_version: content_version.into(),
        time: TimeState::default(),
        world: WorldSpaceState::default(),
        resources: ResourceState::default(),
        build: BuildState::default(),
        ledger: vec![LedgerEntry {
            kind: "scene".to_string(),
            text: "你站在学堂门前，清晨的山雾压着木梁，点卯声还没有响。".to_string(),
        }],
    }
}

pub fn build_projection(state: &GameState) -> LedgerViewModel {
    LedgerViewModel {
        scene_text: state
            .ledger
            .last()
            .map(|entry| entry.text.clone())
            .unwrap_or_else(|| "账本空白，局势尚未落笔。".to_string()),
        current_period: state.time.period.clone(),
        window_type: state.time.window_type.clone(),
        available_ap: state.time.ap,
        current_node_id: state.world.current_node_id.clone(),
        exposure: state.resources.exposure,
        debt_pressure: state.resources.debt_pressure,
        build_summary: state.build.survival_route.clone(),
        ledger_entries: state.ledger.clone(),
        performance: PerformanceMetrics::default(),
    }
}

pub fn resolve_action(
    mut state: GameState,
    command: ActionCommand,
) -> Result<ActionResult, CommandError> {
    let started = Instant::now();
    let mut pipeline_trace = Vec::with_capacity(7);

    availability_check(&state, &command)?;
    pipeline_trace.push(PipelineStep::AvailabilityCheck);

    let reserved_cost = cost_reservation(&state, &command)?;
    pipeline_trace.push(PipelineStep::CostReservation);

    let outcome = subsystem_resolution(&state, &command)?;
    pipeline_trace.push(PipelineStep::SubsystemResolution);

    anchor_recalculation(&mut state, &outcome);
    pipeline_trace.push(PipelineStep::AnchorRecalculation);

    effect_commit(&mut state, reserved_cost, &outcome);
    pipeline_trace.push(PipelineStep::EffectCommit);

    ledger_append(&mut state, &outcome);
    pipeline_trace.push(PipelineStep::LedgerAppend);

    let projection_started = Instant::now();
    let mut projection = build_projection(&state);
    pipeline_trace.push(PipelineStep::ProjectionRefresh);
    let performance = PerformanceMetrics {
        resolve_action_ms: started.elapsed().as_millis() as u64,
        projection_ms: projection_started.elapsed().as_millis() as u64,
        save_load_ms: 0,
        bundle_load_ms: 0,
    };
    projection.performance = performance.clone();

    let response = ActionResponse {
        projection,
        performance,
    };

    Ok(ActionResult {
        state,
        response,
        pipeline_trace,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReservedCost {
    ap: u8,
    primeval_stones: i32,
    exposure_risk: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SubsystemOutcome {
    ledger_kind: String,
    ledger_text: String,
    target_node_id: Option<String>,
    survival_route: Option<String>,
    debt_delta: i32,
    exposure_delta: i32,
}

fn availability_check(state: &GameState, command: &ActionCommand) -> Result<(), CommandError> {
    if command.actor != "player" {
        return Err(CommandError::validation("Sprint 0 只允许 player 行动者"));
    }

    match command.intent {
        ActionIntent::Move => {
            if command
                .target
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty()
            {
                return Err(CommandError::validation("移动行动缺少 target"));
            }
        }
        ActionIntent::Cultivate
        | ActionIntent::Scout
        | ActionIntent::Recover
        | ActionIntent::Trade
        | ActionIntent::Retreat
        | ActionIntent::Wait => {}
    }

    if state.time.window_type == WindowType::Anchor && command.intent != ActionIntent::Wait {
        return Err(CommandError::validation(
            "锚点窗口暂不接受自由行动，请先处理剧情压力",
        ));
    }

    Ok(())
}

fn cost_reservation(
    state: &GameState,
    command: &ActionCommand,
) -> Result<ReservedCost, CommandError> {
    if command.declared_cost.ap > state.time.ap {
        return Err(CommandError::validation(format!(
            "AP 不足：需要 {}，当前 {}",
            command.declared_cost.ap, state.time.ap
        )));
    }

    if command.declared_cost.primeval_stones < 0 {
        return Err(CommandError::validation("行动成本不能反向增加元石"));
    }

    if command.declared_cost.primeval_stones > state.resources.primeval_stones {
        return Err(CommandError::validation(format!(
            "元石不足：需要 {}，当前 {}",
            command.declared_cost.primeval_stones, state.resources.primeval_stones
        )));
    }

    if command.declared_cost.exposure_risk < 0 {
        return Err(CommandError::validation("暴露风险成本不能为负"));
    }

    Ok(ReservedCost {
        ap: command.declared_cost.ap,
        primeval_stones: command.declared_cost.primeval_stones,
        exposure_risk: command.declared_cost.exposure_risk,
    })
}

fn subsystem_resolution(
    state: &GameState,
    command: &ActionCommand,
) -> Result<SubsystemOutcome, CommandError> {
    match command.intent {
        ActionIntent::Move => {
            let target = command
                .target
                .clone()
                .ok_or_else(|| CommandError::validation("移动行动缺少 target"))?;
            Ok(SubsystemOutcome {
                ledger_kind: "action".to_string(),
                ledger_text: format!(
                    "你压低脚步从 {} 转向 {target}，账本记下一笔移动代价。",
                    state.world.current_node_id
                ),
                target_node_id: Some(target),
                survival_route: None,
                debt_delta: 0,
                exposure_delta: 0,
            })
        }
        ActionIntent::Cultivate => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你按下杂念运转真元，空窍的余波让清晨更冷。".to_string(),
            target_node_id: None,
            survival_route: Some("月光修行：制度内求稳".to_string()),
            debt_delta: 0,
            exposure_delta: 0,
        }),
        ActionIntent::Scout => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你没有急着下注，先听风声、记人脸、看谁在看你。".to_string(),
            target_node_id: None,
            survival_route: None,
            debt_delta: 0,
            exposure_delta: 0,
        }),
        ActionIntent::Recover => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你换来一口喘息，也把人情债写进了药堂账页。".to_string(),
            target_node_id: None,
            survival_route: None,
            debt_delta: 1,
            exposure_delta: 0,
        }),
        ActionIntent::Trade => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你试探着问价，门路没有白来，风险也没有白涨。".to_string(),
            target_node_id: None,
            survival_route: None,
            debt_delta: 0,
            exposure_delta: 1,
        }),
        ActionIntent::Retreat => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你没有逞强，撤退本身就是青茅山的生存技。".to_string(),
            target_node_id: None,
            survival_route: None,
            debt_delta: 0,
            exposure_delta: 0,
        }),
        ActionIntent::Wait => Ok(SubsystemOutcome {
            ledger_kind: "action".to_string(),
            ledger_text: "你把这个时段耗过去，什么也没拿到，也没有凭空安全。".to_string(),
            target_node_id: None,
            survival_route: None,
            debt_delta: 0,
            exposure_delta: 0,
        }),
    }
}

fn anchor_recalculation(_state: &mut GameState, _outcome: &SubsystemOutcome) {
    // Sprint 0 keeps hidden anchor variables out of the public response; the hook is fixed here
    // so later systems do not bypass the unified action pipeline.
}

fn effect_commit(state: &mut GameState, reserved_cost: ReservedCost, outcome: &SubsystemOutcome) {
    state.time.ap -= reserved_cost.ap;
    state.resources.primeval_stones -= reserved_cost.primeval_stones;
    state.resources.exposure += reserved_cost.exposure_risk + outcome.exposure_delta;
    state.resources.debt_pressure += outcome.debt_delta;

    if let Some(target_node_id) = &outcome.target_node_id {
        state.world.current_node_id = target_node_id.clone();
    }

    if let Some(survival_route) = &outcome.survival_route {
        state.build.survival_route = survival_route.clone();
    }
}

fn ledger_append(state: &mut GameState, outcome: &SubsystemOutcome) {
    state.ledger.push(LedgerEntry {
        kind: outcome.ledger_kind.clone(),
        text: outcome.ledger_text.clone(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_run_projects_initial_ledger() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection(&state);

        assert_eq!(state.time.ap, 2);
        assert_eq!(projection.current_node_id, "academy_gate");
        assert!(projection.scene_text.contains("学堂门前"));
    }

    #[test]
    fn resolve_action_rejects_ap_shortage() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let command = ActionCommand {
            actor: "player".to_string(),
            intent: ActionIntent::Scout,
            target: None,
            declared_cost: DeclaredCost {
                ap: 3,
                primeval_stones: 0,
                exposure_risk: 0,
            },
            context_note: None,
        };

        let error = resolve_action(state, command).expect_err("AP gate should fail");
        assert_eq!(error.kind, CommandErrorKind::Validation);
    }

    #[test]
    fn move_action_updates_node_and_records_ledger() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let command = ActionCommand {
            actor: "player".to_string(),
            intent: ActionIntent::Move,
            target: Some("infirmary_lane".to_string()),
            declared_cost: DeclaredCost {
                ap: 1,
                primeval_stones: 0,
                exposure_risk: 1,
            },
            context_note: None,
        };

        let result = resolve_action(state, command).expect("move should resolve");

        assert_eq!(result.state.time.ap, 1);
        assert_eq!(result.state.world.current_node_id, "infirmary_lane");
        assert_eq!(result.state.resources.exposure, 1);
        assert!(result.response.projection.scene_text.contains("移动代价"));
    }

    #[test]
    fn content_bundle_requires_entry_node() {
        let source = ContentSource {
            content_id: "s0.test".to_string(),
            version: "0.1.0".to_string(),
            title: "test".to_string(),
            entry_scene_id: "missing".to_string(),
            nodes: vec![ContentNode {
                id: "academy_gate".to_string(),
                title: "学堂门前".to_string(),
                safety: "low".to_string(),
            }],
            actions: Vec::new(),
        };

        let error = ContentBundle::from_source(source).expect_err("missing entry should fail");
        assert_eq!(error.kind, CommandErrorKind::Content);
    }

    #[test]
    fn action_response_serializes_projection_without_full_game_state() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let command = ActionCommand {
            actor: "player".to_string(),
            intent: ActionIntent::Scout,
            target: None,
            declared_cost: DeclaredCost {
                ap: 1,
                primeval_stones: 0,
                exposure_risk: 0,
            },
            context_note: None,
        };

        let result = resolve_action(state, command).expect("action should resolve");
        let response_json = serde_json::to_value(&result.response).expect("response serializes");

        assert!(response_json.get("projection").is_some());
        assert!(response_json.get("performance").is_some());
        assert!(response_json.get("state").is_none());
        assert!(response_json.get("pipeline_trace").is_none());
    }

    #[test]
    fn resolve_action_records_explicit_pipeline_trace() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let command = ActionCommand {
            actor: "player".to_string(),
            intent: ActionIntent::Scout,
            target: None,
            declared_cost: DeclaredCost {
                ap: 1,
                primeval_stones: 0,
                exposure_risk: 0,
            },
            context_note: None,
        };

        let result = resolve_action(state, command).expect("action should resolve");

        assert_eq!(
            result.pipeline_trace,
            vec![
                PipelineStep::AvailabilityCheck,
                PipelineStep::CostReservation,
                PipelineStep::SubsystemResolution,
                PipelineStep::AnchorRecalculation,
                PipelineStep::EffectCommit,
                PipelineStep::LedgerAppend,
                PipelineStep::ProjectionRefresh,
            ]
        );
    }

    #[test]
    fn save_envelope_round_trips_snapshot_and_ledger() {
        let state = create_run(RunMode::SandboxIf, STARTER_CONTENT_VERSION);
        let envelope = SaveEnvelope::from_state("slot_0", state.clone());
        let encoded = serde_json::to_string(&envelope).expect("save envelope serializes");
        let decoded: SaveEnvelope =
            serde_json::from_str(&encoded).expect("save envelope deserializes");

        assert_eq!(decoded.metadata.save_version, SAVE_FORMAT_VERSION);
        assert_eq!(decoded.metadata.slot_id, "slot_0");
        assert_eq!(decoded.snapshot.time.ap, state.time.ap);
        assert_eq!(decoded.snapshot.ledger, state.ledger);
        assert_eq!(decoded.ledger, state.ledger);
    }
}
