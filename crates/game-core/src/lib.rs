use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const DEFAULT_RUN_ID: &str = "sprint-0-active-run";
pub const STARTER_CONTENT_VERSION: &str = "s0.0.1";

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
    pub state: GameState,
    pub projection: LedgerViewModel,
    pub performance: PerformanceMetrics,
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
) -> Result<ActionResponse, CommandError> {
    let started = Instant::now();

    if command.actor != "player" {
        return Err(CommandError::validation("Sprint 0 只允许 player 行动者"));
    }

    if command.declared_cost.ap > state.time.ap {
        return Err(CommandError::validation(format!(
            "AP 不足：需要 {}，当前 {}",
            command.declared_cost.ap, state.time.ap
        )));
    }

    state.time.ap -= command.declared_cost.ap;
    state.resources.primeval_stones -= command.declared_cost.primeval_stones;
    state.resources.exposure += command.declared_cost.exposure_risk;

    let ledger_text = apply_intent(&mut state, &command)?;
    state.ledger.push(LedgerEntry {
        kind: "action".to_string(),
        text: ledger_text,
    });

    let projection_started = Instant::now();
    let mut projection = build_projection(&state);
    let mut performance = PerformanceMetrics {
        resolve_action_ms: started.elapsed().as_millis() as u64,
        projection_ms: projection_started.elapsed().as_millis() as u64,
        save_load_ms: 0,
        bundle_load_ms: 0,
    };
    projection.performance = performance.clone();
    performance.projection_ms = projection.performance.projection_ms;

    Ok(ActionResponse {
        state,
        projection,
        performance,
    })
}

fn apply_intent(state: &mut GameState, command: &ActionCommand) -> Result<String, CommandError> {
    match command.intent {
        ActionIntent::Move => {
            let target = command
                .target
                .clone()
                .ok_or_else(|| CommandError::validation("移动行动缺少 target"))?;
            state.world.current_node_id = target.clone();
            Ok(format!("你压低脚步转向 {target}，账本记下一笔移动代价。"))
        }
        ActionIntent::Cultivate => {
            state.build.survival_route = "月光修行：制度内求稳".to_string();
            Ok("你按下杂念运转真元，空窍的余波让清晨更冷。".to_string())
        }
        ActionIntent::Scout => Ok("你没有急着下注，先听风声、记人脸、看谁在看你。".to_string()),
        ActionIntent::Recover => {
            state.resources.debt_pressure += 1;
            Ok("你换来一口喘息，也把人情债写进了药堂账页。".to_string())
        }
        ActionIntent::Trade => {
            state.resources.exposure += 1;
            Ok("你试探着问价，门路没有白来，风险也没有白涨。".to_string())
        }
        ActionIntent::Retreat => Ok("你没有逞强，撤退本身就是青茅山的生存技。".to_string()),
        ActionIntent::Wait => Ok("你把这个时段耗过去，什么也没拿到，也没有凭空安全。".to_string()),
    }
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

        let response = resolve_action(state, command).expect("move should resolve");

        assert_eq!(response.state.time.ap, 1);
        assert_eq!(response.state.world.current_node_id, "infirmary_lane");
        assert_eq!(response.state.resources.exposure, 1);
        assert!(response.projection.scene_text.contains("移动代价"));
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
}
