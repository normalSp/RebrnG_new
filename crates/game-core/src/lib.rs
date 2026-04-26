use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Instant;

pub const DEFAULT_RUN_ID: &str = "sprint-0-active-run";
pub const STARTER_CONTENT_VERSION: &str = "s0.0.1";
pub const SAVE_FORMAT_VERSION: &str = "sprint0-save-v1";
pub const RULES_VERSION: &str = "sprint0-rules-v1";

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
    #[serde(default)]
    pub rules_version: String,
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
                rules_version: RULES_VERSION.to_string(),
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

    pub fn validate_for_load(
        &self,
        expected_slot_id: &str,
        expected_content_version: &str,
    ) -> Result<(), CommandError> {
        if self.metadata.slot_id != expected_slot_id {
            return Err(CommandError::save(
                "存档槽位不匹配",
                format!(
                    "expected slot '{}', found '{}'",
                    expected_slot_id, self.metadata.slot_id
                ),
            ));
        }

        if self.metadata.save_version != SAVE_FORMAT_VERSION {
            return Err(CommandError::save(
                "存档格式版本不匹配",
                format!(
                    "expected save_version '{}', found '{}'",
                    SAVE_FORMAT_VERSION, self.metadata.save_version
                ),
            ));
        }

        if self.metadata.rules_version != RULES_VERSION {
            return Err(CommandError::save(
                "规则版本不匹配",
                format!(
                    "expected rules_version '{}', found '{}'",
                    RULES_VERSION, self.metadata.rules_version
                ),
            ));
        }

        if self.metadata.content_version != expected_content_version {
            return Err(CommandError::save(
                "内容包版本不匹配",
                format!(
                    "expected content_version '{}', found '{}'",
                    expected_content_version, self.metadata.content_version
                ),
            ));
        }

        if self.snapshot.content_version != self.metadata.content_version {
            return Err(CommandError::save(
                "存档快照内容版本不一致",
                format!(
                    "metadata content_version '{}', snapshot content_version '{}'",
                    self.metadata.content_version, self.snapshot.content_version
                ),
            ));
        }

        if self.snapshot.mode != self.metadata.mode {
            return Err(CommandError::save(
                "存档快照模式不一致",
                "metadata mode differs from snapshot mode",
            ));
        }

        if self.snapshot.chapter != self.metadata.current_stage {
            return Err(CommandError::save(
                "存档阶段不一致",
                format!(
                    "metadata stage '{}', snapshot chapter '{}'",
                    self.metadata.current_stage, self.snapshot.chapter
                ),
            ));
        }

        if self.ledger != self.snapshot.ledger {
            return Err(CommandError::save(
                "存档账本不一致",
                "ledger copy differs from snapshot ledger",
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveWriteResult {
    pub slot_id: String,
    pub path_hint: String,
    pub save_version: String,
    pub rules_version: String,
    pub content_version: String,
    pub written: bool,
}

impl SaveWriteResult {
    pub fn new(
        slot_id: impl Into<String>,
        path_hint: impl Into<String>,
        content_version: impl Into<String>,
    ) -> Self {
        Self {
            slot_id: slot_id.into(),
            path_hint: path_hint.into(),
            save_version: SAVE_FORMAT_VERSION.to_string(),
            rules_version: RULES_VERSION.to_string(),
            content_version: content_version.into(),
            written: true,
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

    pub fn save(message: impl Into<String>, diagnostics: impl Into<String>) -> Self {
        Self {
            kind: CommandErrorKind::Save,
            message: message.into(),
            diagnostics: Some(diagnostics.into()),
        }
    }

    pub fn io(message: impl Into<String>, diagnostics: impl Into<String>) -> Self {
        Self {
            kind: CommandErrorKind::Io,
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
    pub stage: String,
    pub entry_scene_id: String,
    pub node_count: usize,
    pub action_count: usize,
    pub route_count: usize,
    pub window_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLevel {
    CanonExplicit,
    CanonInferred,
    GameplayExtrapolated,
    SandboxIf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModePermit {
    CanonStrict,
    SandboxIf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentImportance {
    Critical,
    Standard,
    Flavor,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentNode {
    pub id: String,
    pub title: String,
    pub safety: String,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentAction {
    pub id: String,
    pub label: String,
    pub intent: ActionIntent,
    pub target: Option<String>,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
    pub importance: ContentImportance,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentRouteEntry {
    pub id: String,
    pub label: String,
    pub route: String,
    pub entry_action_ids: Vec<String>,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentWindow {
    pub id: String,
    pub day: u8,
    pub period: String,
    pub window_type: WindowType,
    pub default_ap: u8,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentSource {
    pub content_id: String,
    pub version: String,
    pub title: String,
    pub stage: String,
    pub entry_scene_id: String,
    #[serde(default)]
    pub nodes: Vec<ContentNode>,
    #[serde(default)]
    pub actions: Vec<ContentAction>,
    #[serde(default)]
    pub routes: Vec<ContentRouteEntry>,
    #[serde(default)]
    pub windows: Vec<ContentWindow>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentSourceFragment {
    pub content_id: Option<String>,
    pub version: Option<String>,
    pub title: Option<String>,
    pub stage: Option<String>,
    pub entry_scene_id: Option<String>,
    #[serde(default)]
    pub nodes: Vec<ContentNode>,
    #[serde(default)]
    pub actions: Vec<ContentAction>,
    #[serde(default)]
    pub routes: Vec<ContentRouteEntry>,
    #[serde(default)]
    pub windows: Vec<ContentWindow>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBundle {
    pub manifest: ContentManifest,
    pub nodes: Vec<ContentNode>,
    pub actions: Vec<ContentAction>,
    pub routes: Vec<ContentRouteEntry>,
    pub windows: Vec<ContentWindow>,
    pub indexes: ContentIndexes,
    pub diagnostics: ContentBuildDiagnostics,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentIndexes {
    pub node_ids: BTreeMap<String, usize>,
    pub action_ids: BTreeMap<String, usize>,
    pub route_ids: BTreeMap<String, usize>,
    pub window_ids: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBuildDiagnostics {
    pub summary: String,
    pub warnings: Vec<String>,
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

        if source.title.trim().is_empty() {
            return Err(CommandError::content("内容包缺少 title", "title is empty"));
        }

        if source.stage.trim().is_empty() {
            return Err(CommandError::content("内容包缺少 stage", "stage is empty"));
        }

        let node_ids = build_index("node", source.nodes.iter().map(|node| &node.id))?;
        let action_ids = build_index("action", source.actions.iter().map(|action| &action.id))?;
        let route_ids = build_index("route", source.routes.iter().map(|route| &route.id))?;
        let window_ids = build_index("window", source.windows.iter().map(|window| &window.id))?;

        if !node_ids.contains_key(&source.entry_scene_id) {
            return Err(CommandError::content(
                "入口节点不存在",
                format!(
                    "entry_scene_id '{}' not found in nodes",
                    source.entry_scene_id
                ),
            ));
        }

        for node in &source.nodes {
            validate_common_content(
                "node",
                &node.id,
                &node.stage,
                &node.tags,
                &node.evidence,
                &node.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty("node", &node.id, "title", &node.title)?;
            require_non_empty("node", &node.id, "safety", &node.safety)?;
        }

        for action in &source.actions {
            validate_common_content(
                "action",
                &action.id,
                &action.stage,
                &action.tags,
                &action.evidence,
                &action.modes,
                action.importance.clone(),
            )?;
            require_non_empty("action", &action.id, "label", &action.label)?;

            if let Some(target) = &action.target {
                if !node_ids.contains_key(target) {
                    return Err(CommandError::content(
                        "行动目标节点不存在",
                        format!("action '{}' target node '{}' not found", action.id, target),
                    ));
                }
            }
        }

        for route in &source.routes {
            validate_common_content(
                "route",
                &route.id,
                &route.stage,
                &route.tags,
                &route.evidence,
                &route.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty("route", &route.id, "label", &route.label)?;
            require_non_empty("route", &route.id, "route", &route.route)?;

            if route.entry_action_ids.is_empty() {
                return Err(CommandError::content(
                    "路线入口缺少行动引用",
                    format!("route '{}' entry_action_ids is empty", route.id),
                ));
            }

            for action_id in &route.entry_action_ids {
                if !action_ids.contains_key(action_id) {
                    return Err(CommandError::content(
                        "路线入口行动不存在",
                        format!("route '{}' action '{}' not found", route.id, action_id),
                    ));
                }
            }
        }

        for window in &source.windows {
            validate_common_content(
                "window",
                &window.id,
                &window.stage,
                &window.tags,
                &window.evidence,
                &window.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty("window", &window.id, "period", &window.period)?;

            if !(1..=3).contains(&window.default_ap) {
                return Err(CommandError::content(
                    "窗口 AP 超出 Sprint 0 范围",
                    format!("window '{}' default_ap must be 1..=3", window.id),
                ));
            }
        }

        let node_count = source.nodes.len();
        let action_count = source.actions.len();
        let route_count = source.routes.len();
        let window_count = source.windows.len();

        Ok(Self {
            manifest: ContentManifest {
                content_id: source.content_id,
                version: source.version,
                title: source.title,
                stage: source.stage,
                entry_scene_id: source.entry_scene_id,
                node_count,
                action_count,
                route_count,
                window_count,
            },
            nodes: source.nodes,
            actions: source.actions,
            routes: source.routes,
            windows: source.windows,
            indexes: ContentIndexes {
                node_ids,
                action_ids,
                route_ids,
                window_ids,
            },
            diagnostics: ContentBuildDiagnostics {
                summary: format!(
                    "indexed {node_count} nodes, {action_count} actions, {route_count} routes, {window_count} windows"
                ),
                warnings: Vec::new(),
            },
        })
    }
}

impl ContentSource {
    pub fn from_fragments(
        fragments: impl IntoIterator<Item = ContentSourceFragment>,
    ) -> Result<Self, CommandError> {
        let mut content_id = None;
        let mut version = None;
        let mut title = None;
        let mut stage = None;
        let mut entry_scene_id = None;
        let mut nodes = Vec::new();
        let mut actions = Vec::new();
        let mut routes = Vec::new();
        let mut windows = Vec::new();

        for fragment in fragments {
            merge_metadata("content_id", &mut content_id, fragment.content_id)?;
            merge_metadata("version", &mut version, fragment.version)?;
            merge_metadata("title", &mut title, fragment.title)?;
            merge_metadata("stage", &mut stage, fragment.stage)?;
            merge_metadata(
                "entry_scene_id",
                &mut entry_scene_id,
                fragment.entry_scene_id,
            )?;
            nodes.extend(fragment.nodes);
            actions.extend(fragment.actions);
            routes.extend(fragment.routes);
            windows.extend(fragment.windows);
        }

        Ok(Self {
            content_id: content_id.ok_or_else(|| {
                CommandError::content("内容源缺少 content_id", "content_id metadata missing")
            })?,
            version: version.ok_or_else(|| {
                CommandError::content("内容源缺少 version", "version metadata missing")
            })?,
            title: title.ok_or_else(|| {
                CommandError::content("内容源缺少 title", "title metadata missing")
            })?,
            stage: stage.ok_or_else(|| {
                CommandError::content("内容源缺少 stage", "stage metadata missing")
            })?,
            entry_scene_id: entry_scene_id.ok_or_else(|| {
                CommandError::content(
                    "内容源缺少 entry_scene_id",
                    "entry_scene_id metadata missing",
                )
            })?,
            nodes,
            actions,
            routes,
            windows,
        })
    }
}

fn build_index<'a>(
    kind: &str,
    ids: impl IntoIterator<Item = &'a String>,
) -> Result<BTreeMap<String, usize>, CommandError> {
    let mut index = BTreeMap::new();

    for (position, id) in ids.into_iter().enumerate() {
        if id.trim().is_empty() {
            return Err(CommandError::content(
                "内容 id 为空",
                format!("{kind} id is empty"),
            ));
        }

        if index.insert(id.clone(), position).is_some() {
            return Err(CommandError::content(
                "内容 id 重复",
                format!("duplicate {kind} id '{}'", id),
            ));
        }
    }

    Ok(index)
}

fn validate_common_content(
    kind: &str,
    id: &str,
    stage: &str,
    tags: &[String],
    evidence: &EvidenceLevel,
    modes: &[ModePermit],
    importance: ContentImportance,
) -> Result<(), CommandError> {
    require_non_empty(kind, id, "stage", stage)?;

    if tags.is_empty() || tags.iter().any(|tag| tag.trim().is_empty()) {
        return Err(CommandError::content(
            "内容缺少标签",
            format!("{kind} '{id}' tags must be non-empty"),
        ));
    }

    if modes.is_empty() {
        return Err(CommandError::content(
            "内容缺少模式许可",
            format!("{kind} '{id}' modes must be non-empty"),
        ));
    }

    if modes.contains(&ModePermit::CanonStrict)
        && importance == ContentImportance::Critical
        && !matches!(
            evidence,
            EvidenceLevel::CanonExplicit | EvidenceLevel::CanonInferred
        )
    {
        return Err(CommandError::content(
            "canon_strict 关键内容证据不足",
            format!("{kind} '{id}' canon_strict critical content requires canon evidence"),
        ));
    }

    if evidence == &EvidenceLevel::SandboxIf && !modes.contains(&ModePermit::SandboxIf) {
        return Err(CommandError::content(
            "sandbox_if 内容缺少模式许可",
            format!("{kind} '{id}' sandbox_if content requires sandbox_if mode"),
        ));
    }

    Ok(())
}

fn require_non_empty(kind: &str, id: &str, field: &str, value: &str) -> Result<(), CommandError> {
    if value.trim().is_empty() {
        return Err(CommandError::content(
            "内容字段为空",
            format!("{kind} '{id}' field '{field}' is empty"),
        ));
    }

    Ok(())
}

fn merge_metadata(
    field: &str,
    current: &mut Option<String>,
    candidate: Option<String>,
) -> Result<(), CommandError> {
    let Some(candidate) = candidate else {
        return Ok(());
    };

    if candidate.trim().is_empty() {
        return Err(CommandError::content(
            "内容源元数据为空",
            format!("{field} metadata is empty"),
        ));
    }

    if let Some(existing) = current {
        if existing != &candidate {
            return Err(CommandError::content(
                "内容源元数据冲突",
                format!("{field} metadata conflict: '{existing}' vs '{candidate}'"),
            ));
        }
    } else {
        *current = Some(candidate);
    }

    Ok(())
}

pub fn starter_content_manifest() -> ContentManifest {
    ContentManifest {
        content_id: "s0.qingmao.foundation".to_string(),
        version: STARTER_CONTENT_VERSION.to_string(),
        title: "青茅山 Sprint 0 内容骨架".to_string(),
        stage: "s0".to_string(),
        entry_scene_id: "academy_gate".to_string(),
        node_count: 6,
        action_count: 8,
        route_count: 5,
        window_count: 8,
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
            stage: "s0".to_string(),
            entry_scene_id: "missing".to_string(),
            nodes: vec![ContentNode {
                id: "academy_gate".to_string(),
                title: "学堂门前".to_string(),
                safety: "low".to_string(),
                stage: "s0".to_string(),
                tags: vec!["node".to_string()],
                evidence: EvidenceLevel::CanonInferred,
                modes: vec![ModePermit::CanonStrict, ModePermit::SandboxIf],
            }],
            actions: Vec::new(),
            routes: Vec::new(),
            windows: Vec::new(),
        };

        let error = ContentBundle::from_source(source).expect_err("missing entry should fail");
        assert_eq!(error.kind, CommandErrorKind::Content);
    }

    #[test]
    fn content_bundle_builds_indexes_for_s0_sources() {
        let source = valid_content_source();
        let bundle = ContentBundle::from_source(source).expect("valid bundle should build");

        assert_eq!(bundle.manifest.node_count, 1);
        assert_eq!(bundle.manifest.action_count, 1);
        assert_eq!(bundle.manifest.route_count, 1);
        assert_eq!(bundle.manifest.window_count, 1);
        assert_eq!(bundle.indexes.node_ids["academy_gate"], 0);
        assert_eq!(bundle.indexes.action_ids["scout_academy"], 0);
        assert_eq!(bundle.indexes.route_ids["moonlight_entry"], 0);
        assert_eq!(bundle.indexes.window_ids["day1_morning_free"], 0);
        assert!(bundle.diagnostics.summary.contains("indexed 1 nodes"));
        assert!(bundle.diagnostics.warnings.is_empty());
    }

    #[test]
    fn content_bundle_rejects_duplicate_ids() {
        let mut source = valid_content_source();
        source.nodes.push(source.nodes[0].clone());

        let error = ContentBundle::from_source(source).expect_err("duplicate node should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("duplicate node id"));
    }

    #[test]
    fn content_bundle_rejects_action_target_outside_node_index() {
        let mut source = valid_content_source();
        source.actions[0].target = Some("missing_node".to_string());

        let error = ContentBundle::from_source(source).expect_err("missing target should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("target node 'missing_node' not found"));
    }

    #[test]
    fn canon_strict_critical_content_requires_canon_evidence() {
        let mut source = valid_content_source();
        source.actions[0].importance = ContentImportance::Critical;
        source.actions[0].evidence = EvidenceLevel::GameplayExtrapolated;
        source.actions[0].modes = vec![ModePermit::CanonStrict];

        let error =
            ContentBundle::from_source(source).expect_err("weak canon evidence should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("canon_strict critical content requires canon evidence"));
    }

    #[test]
    fn sandbox_if_content_requires_explicit_sandbox_mode() {
        let mut source = valid_content_source();
        source.actions[0].evidence = EvidenceLevel::SandboxIf;
        source.actions[0].modes = vec![ModePermit::CanonStrict];

        let error =
            ContentBundle::from_source(source).expect_err("sandbox_if content should be gated");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("sandbox_if content requires sandbox_if mode"));
    }

    fn valid_content_source() -> ContentSource {
        ContentSource {
            content_id: "s0.qingmao.foundation".to_string(),
            version: STARTER_CONTENT_VERSION.to_string(),
            title: "青茅山 Sprint 0 内容骨架".to_string(),
            stage: "s0".to_string(),
            entry_scene_id: "academy_gate".to_string(),
            nodes: vec![ContentNode {
                id: "academy_gate".to_string(),
                title: "学堂门前".to_string(),
                safety: "low".to_string(),
                stage: "s0".to_string(),
                tags: vec!["node".to_string(), "academy".to_string()],
                evidence: EvidenceLevel::CanonInferred,
                modes: vec![ModePermit::CanonStrict, ModePermit::SandboxIf],
            }],
            actions: vec![ContentAction {
                id: "scout_academy".to_string(),
                label: "观察学堂风声".to_string(),
                intent: ActionIntent::Scout,
                target: Some("academy_gate".to_string()),
                stage: "s0".to_string(),
                tags: vec!["action".to_string(), "scout".to_string()],
                evidence: EvidenceLevel::CanonInferred,
                modes: vec![ModePermit::CanonStrict, ModePermit::SandboxIf],
                importance: ContentImportance::Standard,
            }],
            routes: vec![ContentRouteEntry {
                id: "moonlight_entry".to_string(),
                label: "月光修行入口".to_string(),
                route: "moonlight".to_string(),
                entry_action_ids: vec!["scout_academy".to_string()],
                stage: "s0".to_string(),
                tags: vec!["route".to_string(), "moonlight".to_string()],
                evidence: EvidenceLevel::CanonInferred,
                modes: vec![ModePermit::CanonStrict, ModePermit::SandboxIf],
            }],
            windows: vec![ContentWindow {
                id: "day1_morning_free".to_string(),
                day: 1,
                period: "清晨".to_string(),
                window_type: WindowType::Free,
                default_ap: 2,
                stage: "s0".to_string(),
                tags: vec!["window".to_string(), "opening".to_string()],
                evidence: EvidenceLevel::CanonInferred,
                modes: vec![ModePermit::CanonStrict, ModePermit::SandboxIf],
            }],
        }
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
        assert_eq!(decoded.metadata.rules_version, RULES_VERSION);
        assert_eq!(decoded.metadata.slot_id, "slot_0");
        assert_eq!(decoded.snapshot.time.ap, state.time.ap);
        assert_eq!(decoded.snapshot.ledger, state.ledger);
        assert_eq!(decoded.ledger, state.ledger);
        decoded
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect("fresh envelope should load");
    }

    #[test]
    fn save_envelope_rejects_missing_rules_version() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let mut envelope = SaveEnvelope::from_state("slot_0", state);
        envelope.metadata.rules_version.clear();

        let error = envelope
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("missing rules version should fail");

        assert_eq!(error.kind, CommandErrorKind::Save);
        assert!(error.message.contains("规则版本"));
    }

    #[test]
    fn save_envelope_rejects_slot_content_and_ledger_mismatch() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let slot_error = SaveEnvelope::from_state("slot_0", state.clone())
            .validate_for_load("slot_1", STARTER_CONTENT_VERSION)
            .expect_err("slot mismatch should fail");
        assert_eq!(slot_error.kind, CommandErrorKind::Save);

        let content_error = SaveEnvelope::from_state("slot_0", state.clone())
            .validate_for_load("slot_0", "different-content-version")
            .expect_err("content mismatch should fail");
        assert_eq!(content_error.kind, CommandErrorKind::Save);

        let mut ledger_mismatch = SaveEnvelope::from_state("slot_0", state);
        ledger_mismatch.ledger.push(LedgerEntry {
            kind: "test".to_string(),
            text: "外部篡改的账本".to_string(),
        });
        let ledger_error = ledger_mismatch
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("ledger mismatch should fail");
        assert_eq!(ledger_error.kind, CommandErrorKind::Save);
    }

    #[test]
    fn save_write_result_serializes_minimum_receipt() {
        let result = SaveWriteResult::new(
            "slot_0",
            "saves/sprint0/slot_0.json",
            STARTER_CONTENT_VERSION,
        );
        let json = serde_json::to_value(&result).expect("write result serializes");

        assert_eq!(json["slot_id"], "slot_0");
        assert_eq!(json["save_version"], SAVE_FORMAT_VERSION);
        assert_eq!(json["rules_version"], RULES_VERSION);
        assert_eq!(json["content_version"], STARTER_CONTENT_VERSION);
        assert_eq!(json["written"], true);
    }
}
