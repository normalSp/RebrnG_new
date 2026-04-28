use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

pub const DEFAULT_RUN_ID: &str = "sprint-0-active-run";
pub const STARTER_CONTENT_VERSION: &str = "s0.2.0";
pub const SAVE_FORMAT_VERSION: &str = "sprint0-save-v2";
pub const RULES_VERSION: &str = "sprint3-rules-v1";
pub const DEFAULT_RNG_STATE: &str = "sprint_0_deterministic_seed";
pub const DEFAULT_MIGRATION_STATE: &str = "none";

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
    Confront,
    Yield,
    Argue,
    Delay,
    Frame,
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
    pub window_id: String,
    pub window_index: usize,
    pub free_rounds_elapsed: u8,
    pub chapter_day: u8,
    pub period: String,
    pub window_type: WindowType,
    pub ap: u8,
    pub next_anchor_pressure: String,
}

impl Default for TimeState {
    fn default() -> Self {
        Self {
            window_id: "day1_morning_free".to_string(),
            window_index: 0,
            free_rounds_elapsed: 0,
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
    pub materials: i32,
    pub merit: i32,
}

impl Default for ResourceState {
    fn default() -> Self {
        Self {
            primeval_stones: 3,
            materials: 0,
            merit: 0,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DebtAndCreditState {
    pub infirmary_debt: i32,
    pub favor_debt: i32,
    pub organization_debt: i32,
    pub trading_credit: i32,
}

impl DebtAndCreditState {
    pub fn pressure(&self) -> i32 {
        self.infirmary_debt + self.favor_debt + self.organization_debt
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RiskState {
    pub exposure: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InjuryLevel {
    #[default]
    Healthy,
    Light,
    Heavy,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct InjuryState {
    pub level: InjuryLevel,
    pub ap_penalty_pending: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterState {
    pub aperture_opened: bool,
    pub injury: InjuryState,
}

impl CharacterState {
    fn opened_aperture() -> Self {
        Self {
            aperture_opened: true,
            injury: InjuryState::default(),
        }
    }
}

impl Default for CharacterState {
    fn default() -> Self {
        Self::opened_aperture()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeState {
    pub blackmarket_route_known: bool,
    pub known_clues: Vec<String>,
}

impl KnowledgeState {
    fn record_clue(&mut self, clue_id: &str) {
        if !self.known_clues.iter().any(|clue| clue == clue_id) {
            self.known_clues.push(clue_id.to_string());
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GuSlotKind {
    Core,
    Support,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuSlotState {
    pub slot: GuSlotKind,
    pub display_name: String,
    pub instance_id: Option<String>,
    pub role_note: String,
}

impl GuSlotState {
    fn core(display_name: &str, role_note: &str) -> Self {
        Self {
            slot: GuSlotKind::Core,
            display_name: display_name.to_string(),
            instance_id: None,
            role_note: role_note.to_string(),
        }
    }

    fn support(display_name: &str, role_note: &str) -> Self {
        Self {
            slot: GuSlotKind::Support,
            display_name: display_name.to_string(),
            instance_id: None,
            role_note: role_note.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VitalGuStatus {
    #[default]
    NotEstablished,
    Established,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VitalGuState {
    pub status: VitalGuStatus,
    pub instance_id: Option<String>,
    pub binding_scope: String,
    pub binding_risk: String,
}

impl Default for VitalGuState {
    fn default() -> Self {
        Self {
            status: VitalGuStatus::NotEstablished,
            instance_id: None,
            binding_scope: "未绑定".to_string(),
            binding_risk: "未暴露".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildState {
    pub survival_route: String,
    pub main_path: Option<String>,
    pub dao_mark_note: Option<String>,
    pub moonlight_cultivation_marks: u8,
    pub core_gu: GuSlotState,
    pub support_gu: GuSlotState,
    pub vital_gu: VitalGuState,
    pub maintenance_pressure: String,
    pub gap_summary: String,
}

impl Default for BuildState {
    fn default() -> Self {
        Self {
            survival_route: "未定：仍在学堂秩序缝隙里求活".to_string(),
            main_path: None,
            dao_mark_note: None,
            moonlight_cultivation_marks: 0,
            core_gu: GuSlotState::core("月光蛊线索未稳", "当前路线核心候选"),
            support_gu: GuSlotState::support("暂无", "尚无辅助蛊承托"),
            vital_gu: VitalGuState::default(),
            gap_summary: "缺口：缺稳定资源、缺支撑蛊、缺安全情报".to_string(),
            maintenance_pressure: "暂无蛊虫喂养压力".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EncounterType {
    Extortion,
    PublicPressure,
    Probe,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActiveEncounter {
    pub encounter_id: String,
    pub encounter_type: EncounterType,
    pub known_risk: String,
    pub decision_intents: Vec<ActionIntent>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncounterState {
    pub active: Option<ActiveEncounter>,
    pub resolved_encounter_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerEntry {
    pub kind: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupSummary {
    pub origin_id: String,
    pub origin_title: String,
    pub talent_ids: Vec<String>,
    pub talent_titles: Vec<String>,
    pub attributes: BTreeMap<String, i32>,
    pub opening_rite_outcome_id: String,
    pub opening_rite_outcome_title: String,
    pub resource_package_ids: Vec<String>,
    pub attention_delta: i32,
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
    pub debts_and_credit: DebtAndCreditState,
    pub risk: RiskState,
    pub character: CharacterState,
    pub knowledge: KnowledgeState,
    pub encounters: EncounterState,
    pub build: BuildState,
    pub ledger: Vec<LedgerEntry>,
    #[serde(default)]
    pub setup_summary: Option<SetupSummary>,
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
#[serde(rename_all = "snake_case")]
pub enum SaveCheckpointKind {
    StageBoundary,
    CurrentSnapshot,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SaveRestorePolicy {
    StageCheckpoint,
    CurrentSnapshot,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveCheckpoint {
    pub checkpoint_id: String,
    pub kind: SaveCheckpointKind,
    pub restore_policy: SaveRestorePolicy,
    pub chapter: String,
    pub period: String,
    pub window_id: String,
    pub window_index: usize,
    pub free_rounds_elapsed: u8,
    pub node_id: String,
    pub ledger_len: usize,
    pub rules_version: String,
    pub content_version: String,
    pub summary: String,
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
        let checkpoints = save_checkpoints_for_state(&state);

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
            checkpoints,
            rng_state: DEFAULT_RNG_STATE.to_string(),
            migration_state: DEFAULT_MIGRATION_STATE.to_string(),
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

        if self.metadata.content_version != expected_content_version
            || self.snapshot.content_version != expected_content_version
        {
            return Err(CommandError::save(
                "内容版本不匹配",
                format!(
                    "expected content_version '{}', metadata '{}', snapshot '{}'",
                    expected_content_version,
                    self.metadata.content_version,
                    self.snapshot.content_version
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

        if self.rng_state.trim().is_empty() {
            return Err(CommandError::save(
                "存档 RNG 状态缺失",
                "rng_state must not be empty",
            ));
        }

        if self.migration_state.trim().is_empty() {
            return Err(CommandError::save(
                "存档迁移状态缺失",
                "migration_state must not be empty",
            ));
        }

        self.validate_checkpoints(expected_content_version)?;

        Ok(())
    }

    fn validate_checkpoints(&self, expected_content_version: &str) -> Result<(), CommandError> {
        if self.checkpoints.is_empty() {
            return Err(CommandError::save(
                "存档检查点缺失",
                "SaveEnvelope.checkpoints must contain at least one checkpoint",
            ));
        }

        let mut seen = BTreeSet::new();
        for checkpoint in &self.checkpoints {
            if checkpoint.checkpoint_id.trim().is_empty() {
                return Err(CommandError::save(
                    "存档检查点 ID 缺失",
                    "checkpoint_id must not be empty",
                ));
            }
            if !seen.insert(checkpoint.checkpoint_id.clone()) {
                return Err(CommandError::save(
                    "存档检查点重复",
                    format!("duplicate checkpoint_id '{}'", checkpoint.checkpoint_id),
                ));
            }
            if checkpoint.rules_version != RULES_VERSION {
                return Err(CommandError::save(
                    "检查点规则版本不一致",
                    format!(
                        "checkpoint '{}' expected rules_version '{}', found '{}'",
                        checkpoint.checkpoint_id, RULES_VERSION, checkpoint.rules_version
                    ),
                ));
            }
            if checkpoint.content_version != expected_content_version {
                return Err(CommandError::save(
                    "检查点内容版本不一致",
                    format!(
                        "checkpoint '{}' expected content_version '{}', found '{}'",
                        checkpoint.checkpoint_id,
                        expected_content_version,
                        checkpoint.content_version
                    ),
                ));
            }
            if checkpoint.chapter != self.snapshot.chapter {
                return Err(CommandError::save(
                    "检查点阶段不一致",
                    format!(
                        "checkpoint '{}' chapter '{}', snapshot chapter '{}'",
                        checkpoint.checkpoint_id, checkpoint.chapter, self.snapshot.chapter
                    ),
                ));
            }
            if checkpoint.ledger_len > self.snapshot.ledger.len() {
                return Err(CommandError::save(
                    "检查点账本长度越界",
                    format!(
                        "checkpoint '{}' ledger_len {}, snapshot ledger len {}",
                        checkpoint.checkpoint_id,
                        checkpoint.ledger_len,
                        self.snapshot.ledger.len()
                    ),
                ));
            }
        }

        let current = self
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.kind == SaveCheckpointKind::CurrentSnapshot)
            .ok_or_else(|| {
                CommandError::save(
                    "当前快照检查点缺失",
                    "expected one current_snapshot checkpoint",
                )
            })?;

        if current.restore_policy != SaveRestorePolicy::CurrentSnapshot {
            return Err(CommandError::save(
                "当前快照恢复策略不一致",
                format!(
                    "checkpoint '{}' restore policy must be current_snapshot",
                    current.checkpoint_id
                ),
            ));
        }

        if current.node_id != self.snapshot.world.current_node_id
            || current.window_id != self.snapshot.time.window_id
            || current.window_index != self.snapshot.time.window_index
            || current.free_rounds_elapsed != self.snapshot.time.free_rounds_elapsed
            || current.period != self.snapshot.time.period
            || current.ledger_len != self.snapshot.ledger.len()
        {
            return Err(CommandError::save(
                "当前快照检查点与规则状态不一致",
                format!(
                    "checkpoint '{}' does not match snapshot node/window/ledger boundary",
                    current.checkpoint_id
                ),
            ));
        }

        let has_stage_boundary = self.checkpoints.iter().any(|checkpoint| {
            checkpoint.kind == SaveCheckpointKind::StageBoundary
                && checkpoint.restore_policy == SaveRestorePolicy::StageCheckpoint
        });
        if !has_stage_boundary {
            return Err(CommandError::save(
                "阶段检查点缺失",
                "expected at least one stage_boundary checkpoint",
            ));
        }

        Ok(())
    }
}

fn save_checkpoints_for_state(state: &GameState) -> Vec<SaveCheckpoint> {
    vec![
        save_checkpoint_from_state(
            format!("{}_stage", state.chapter),
            SaveCheckpointKind::StageBoundary,
            SaveRestorePolicy::StageCheckpoint,
            "阶段检查点：只代表当前阶段边界，不提供每个选择回退。",
            state,
        ),
        save_checkpoint_from_state(
            "sprint_0_current",
            SaveCheckpointKind::CurrentSnapshot,
            SaveRestorePolicy::CurrentSnapshot,
            "当前快照：用于读回 active run 的完整规则状态。",
            state,
        ),
    ]
}

fn save_checkpoint_from_state(
    checkpoint_id: impl Into<String>,
    kind: SaveCheckpointKind,
    restore_policy: SaveRestorePolicy,
    summary: impl Into<String>,
    state: &GameState,
) -> SaveCheckpoint {
    SaveCheckpoint {
        checkpoint_id: checkpoint_id.into(),
        kind,
        restore_policy,
        chapter: state.chapter.clone(),
        period: state.time.period.clone(),
        window_id: state.time.window_id.clone(),
        window_index: state.time.window_index,
        free_rounds_elapsed: state.time.free_rounds_elapsed,
        node_id: state.world.current_node_id.clone(),
        ledger_len: state.ledger.len(),
        rules_version: RULES_VERSION.to_string(),
        content_version: state.content_version.clone(),
        summary: summary.into(),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveWriteResult {
    pub slot_id: String,
    pub path_hint: String,
    pub save_version: String,
    pub rules_version: String,
    pub content_version: String,
    pub checkpoint_count: usize,
    pub current_checkpoint_id: String,
    pub stage_checkpoint_ids: Vec<String>,
    pub written: bool,
}

impl SaveWriteResult {
    pub fn new(
        slot_id: impl Into<String>,
        path_hint: impl Into<String>,
        content_version: impl Into<String>,
        stage_checkpoint_ids: Vec<String>,
        current_checkpoint_id: impl Into<String>,
    ) -> Self {
        let checkpoint_count = stage_checkpoint_ids.len() + 1;
        Self {
            slot_id: slot_id.into(),
            path_hint: path_hint.into(),
            save_version: SAVE_FORMAT_VERSION.to_string(),
            rules_version: RULES_VERSION.to_string(),
            content_version: content_version.into(),
            checkpoint_count,
            current_checkpoint_id: current_checkpoint_id.into(),
            stage_checkpoint_ids,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunSetupState {
    pub run_id: String,
    pub mode: RunMode,
    pub content_version: String,
    pub selected_origin_id: Option<String>,
    pub selected_talent_ids: Vec<String>,
    pub attribute_values: BTreeMap<String, i32>,
    pub opening_rite_outcome_id: String,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntent {
    SelectOrigin,
    ToggleTalent,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupCommand {
    pub intent: SetupIntent,
    pub target_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupCandidateView {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub selected: bool,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupTalentCandidateView {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub intensity: TalentIntensity,
    pub selected: bool,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub pressure_note: String,
    pub route_tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupAttributeView {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupResourcePreview {
    pub primeval_stones: i32,
    pub materials: i32,
    pub merit: i32,
    pub infirmary_debt: i32,
    pub favor_debt: i32,
    pub organization_debt: i32,
    pub trading_credit: i32,
    pub exposure: i32,
    pub resource_package_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupViewModel {
    pub mode: RunMode,
    pub content_version: String,
    pub origin_candidates: Vec<SetupCandidateView>,
    pub talent_candidates: Vec<SetupTalentCandidateView>,
    pub attributes: Vec<SetupAttributeView>,
    pub resource_preview: SetupResourcePreview,
    pub selected_origin_id: Option<String>,
    pub selected_talent_ids: Vec<String>,
    pub opening_rite_outcome_id: String,
    pub opening_rite_title: String,
    pub opening_rite_summary: String,
    pub confirm_enabled: bool,
    pub confirm_blockers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetupResponse {
    pub setup: RunSetupState,
    pub view: SetupViewModel,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerformanceMetrics {
    pub resolve_action_ms: u64,
    pub projection_ms: u64,
    pub save_load_ms: u64,
    pub bundle_load_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusMarkerView {
    pub label: String,
    pub value: String,
    pub tone: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildLedgerView {
    pub survival_route: String,
    pub main_path: String,
    pub dao_mark_note: String,
    pub core_gu: String,
    pub support_gu: String,
    pub vital_gu: String,
    pub maintenance_pressure: String,
    pub gap_summary: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FactionRelationshipView {
    pub family_pressure: String,
    pub infirmary_debt: String,
    pub favor_debt: String,
    pub blackmarket_access: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionChoiceGroup {
    Encounter,
    Movement,
    Cultivation,
    Information,
    Recovery,
    Trade,
    Wait,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActionChoiceTone {
    Normal,
    Safe,
    Risky,
    Danger,
    Blocked,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionChoiceView {
    pub id: String,
    pub label: String,
    pub intent: ActionIntent,
    pub target: Option<String>,
    pub enabled: bool,
    pub disabled_reason: Option<String>,
    pub cost_hint: String,
    pub risk_hint: String,
    pub group: ActionChoiceGroup,
    pub tone: ActionChoiceTone,
    pub consequence_hint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeSummaryView {
    pub id: String,
    pub title: String,
    pub safety: String,
    pub current: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NodeLedgerView {
    pub current_node_id: String,
    pub current_region_id: String,
    pub visible_nodes: Vec<NodeSummaryView>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveLedgerView {
    pub save_version: String,
    pub rules_version: String,
    pub content_version: String,
    pub rng_state: String,
    pub migration_state: String,
    pub checkpoint_count: usize,
    pub current_checkpoint_id: String,
    pub stage_checkpoint_ids: Vec<String>,
    pub rollback_policy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NarrativeBoundaryView {
    pub runtime_ai_enabled: bool,
    pub source: String,
    pub policy: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StageClosureStatus {
    InProgress,
    FoundationEstablished,
    TraumaContinuable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StageClosureView {
    pub status: StageClosureStatus,
    pub title: String,
    pub summary: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecentFeedbackView {
    pub title: String,
    pub summary: String,
    pub tone: ActionChoiceTone,
    pub source_kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClueLineView {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub tone: ActionChoiceTone,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClueLedgerView {
    pub known_clues: Vec<ClueLineView>,
    pub blackmarket_access_summary: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LedgerViewModel {
    pub scene_text: String,
    pub current_day: u8,
    pub current_period: String,
    pub window_id: String,
    pub window_type: WindowType,
    pub available_ap: u8,
    pub next_anchor_pressure: String,
    pub current_node_id: String,
    pub primeval_stones: i32,
    pub materials: i32,
    pub merit: i32,
    pub exposure: i32,
    pub debt_pressure: i32,
    pub build_summary: String,
    pub status_markers: Vec<StatusMarkerView>,
    pub build_view: BuildLedgerView,
    pub relationship_view: FactionRelationshipView,
    pub save_view: SaveLedgerView,
    pub action_choices: Vec<ActionChoiceView>,
    pub node_view: NodeLedgerView,
    pub injury_level: InjuryLevel,
    pub active_encounter_id: Option<String>,
    pub active_encounter_type: Option<EncounterType>,
    pub active_encounter_known_risk: Option<String>,
    pub active_encounter_decisions: Vec<ActionIntent>,
    pub ledger_entries: Vec<LedgerEntry>,
    pub recent_feedback: Option<RecentFeedbackView>,
    pub clue_view: ClueLedgerView,
    pub narrative_boundary: NarrativeBoundaryView,
    pub stage_closure: StageClosureView,
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
    pub movement_count: usize,
    pub encounter_count: usize,
    pub narrative_count: usize,
    pub origin_count: usize,
    pub talent_count: usize,
    pub attribute_profile_count: usize,
    pub opening_rite_outcome_count: usize,
    pub initial_resource_package_count: usize,
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
pub struct ContentMovementEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub ap_cost: u8,
    pub arrival_ap_penalty: u8,
    pub exposure_delta: i32,
    #[serde(default)]
    pub required_period: Option<String>,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentEncounterTemplate {
    pub id: String,
    pub encounter_type: EncounterType,
    pub trigger_node_id: String,
    pub known_risk: String,
    #[serde(default)]
    pub min_exposure: Option<i32>,
    #[serde(default)]
    pub min_moonlight_marks: Option<u8>,
    #[serde(default)]
    pub required_clue_ids: Vec<String>,
    pub decisions: Vec<ContentEncounterDecision>,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentEncounterDecision {
    pub intent: ActionIntent,
    pub ap_cost: u8,
    pub primeval_stones_cost: i32,
    pub exposure_delta: i32,
    #[serde(default)]
    pub injury_level: Option<InjuryLevel>,
    #[serde(default)]
    pub injury_ap_penalty_pending: bool,
    #[serde(default)]
    pub target_node_id: Option<String>,
    pub survival_route: String,
    pub narrative_id: String,
    #[serde(default)]
    pub clue_ids: Vec<String>,
    #[serde(default)]
    pub mitigating_clue_id: Option<String>,
    #[serde(default)]
    pub mitigated_exposure_delta: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentNarrativeTemplate {
    pub id: String,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentOriginSpec {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub resource_package_id: String,
    #[serde(default)]
    pub attribute_modifiers: BTreeMap<String, i32>,
    pub attention_delta: i32,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TalentIntensity {
    Mild,
    StrongIf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentTalentSpec {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub intensity: TalentIntensity,
    #[serde(default)]
    pub attribute_modifiers: BTreeMap<String, i32>,
    #[serde(default)]
    pub route_tags: Vec<String>,
    pub pressure_note: String,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentAttributeProfile {
    pub id: String,
    pub title: String,
    pub attributes: Vec<ContentAttributeDefinition>,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentAttributeDefinition {
    pub id: String,
    pub label: String,
    pub summary: String,
    pub min: i32,
    pub max: i32,
    pub base: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentOpeningRiteOutcome {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub aperture_opened: bool,
    pub displayed_grade: String,
    pub attention_delta: i32,
    pub resource_package_id: String,
    pub stage: String,
    pub tags: Vec<String>,
    pub evidence: EvidenceLevel,
    pub modes: Vec<ModePermit>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentInitialResourcePackage {
    pub id: String,
    pub title: String,
    pub primeval_stones: i32,
    pub materials: i32,
    pub merit: i32,
    pub infirmary_debt: i32,
    pub favor_debt: i32,
    pub organization_debt: i32,
    pub trading_credit: i32,
    pub exposure: i32,
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
    #[serde(default)]
    pub movements: Vec<ContentMovementEdge>,
    #[serde(default)]
    pub encounters: Vec<ContentEncounterTemplate>,
    #[serde(default)]
    pub narratives: Vec<ContentNarrativeTemplate>,
    #[serde(default)]
    pub origins: Vec<ContentOriginSpec>,
    #[serde(default)]
    pub talents: Vec<ContentTalentSpec>,
    #[serde(default)]
    pub attribute_profiles: Vec<ContentAttributeProfile>,
    #[serde(default)]
    pub opening_rite_outcomes: Vec<ContentOpeningRiteOutcome>,
    #[serde(default)]
    pub initial_resource_packages: Vec<ContentInitialResourcePackage>,
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
    #[serde(default)]
    pub movements: Vec<ContentMovementEdge>,
    #[serde(default)]
    pub encounters: Vec<ContentEncounterTemplate>,
    #[serde(default)]
    pub narratives: Vec<ContentNarrativeTemplate>,
    #[serde(default)]
    pub origins: Vec<ContentOriginSpec>,
    #[serde(default)]
    pub talents: Vec<ContentTalentSpec>,
    #[serde(default)]
    pub attribute_profiles: Vec<ContentAttributeProfile>,
    #[serde(default)]
    pub opening_rite_outcomes: Vec<ContentOpeningRiteOutcome>,
    #[serde(default)]
    pub initial_resource_packages: Vec<ContentInitialResourcePackage>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBundle {
    pub manifest: ContentManifest,
    pub nodes: Vec<ContentNode>,
    pub actions: Vec<ContentAction>,
    pub routes: Vec<ContentRouteEntry>,
    pub windows: Vec<ContentWindow>,
    pub movements: Vec<ContentMovementEdge>,
    pub encounters: Vec<ContentEncounterTemplate>,
    pub narratives: Vec<ContentNarrativeTemplate>,
    pub origins: Vec<ContentOriginSpec>,
    pub talents: Vec<ContentTalentSpec>,
    pub attribute_profiles: Vec<ContentAttributeProfile>,
    pub opening_rite_outcomes: Vec<ContentOpeningRiteOutcome>,
    pub initial_resource_packages: Vec<ContentInitialResourcePackage>,
    pub indexes: ContentIndexes,
    pub diagnostics: ContentBuildDiagnostics,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentIndexes {
    pub node_ids: BTreeMap<String, usize>,
    pub action_ids: BTreeMap<String, usize>,
    pub route_ids: BTreeMap<String, usize>,
    pub window_ids: BTreeMap<String, usize>,
    pub movement_ids: BTreeMap<String, usize>,
    pub encounter_ids: BTreeMap<String, usize>,
    pub narrative_ids: BTreeMap<String, usize>,
    pub origin_ids: BTreeMap<String, usize>,
    pub talent_ids: BTreeMap<String, usize>,
    pub attribute_profile_ids: BTreeMap<String, usize>,
    pub opening_rite_outcome_ids: BTreeMap<String, usize>,
    pub initial_resource_package_ids: BTreeMap<String, usize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContentBuildDiagnostics {
    pub summary: String,
    pub warnings: Vec<String>,
}

impl ContentBundle {
    pub fn from_source(source: ContentSource) -> Result<Self, CommandError> {
        require_non_empty("bundle", "root", "content_id", &source.content_id)?;
        require_non_empty("bundle", "root", "version", &source.version)?;
        require_non_empty("bundle", "root", "title", &source.title)?;
        require_non_empty("bundle", "root", "stage", &source.stage)?;

        let node_ids = build_index("node", source.nodes.iter().map(|node| &node.id))?;
        let action_ids = build_index("action", source.actions.iter().map(|action| &action.id))?;
        let route_ids = build_index("route", source.routes.iter().map(|route| &route.id))?;
        let window_ids = build_index("window", source.windows.iter().map(|window| &window.id))?;
        let movement_ids = build_index(
            "movement",
            source.movements.iter().map(|movement| &movement.id),
        )?;
        let encounter_ids = build_index(
            "encounter",
            source.encounters.iter().map(|encounter| &encounter.id),
        )?;
        let narrative_ids = build_index(
            "narrative",
            source.narratives.iter().map(|narrative| &narrative.id),
        )?;
        let origin_ids = build_index("origin", source.origins.iter().map(|origin| &origin.id))?;
        let talent_ids = build_index("talent", source.talents.iter().map(|talent| &talent.id))?;
        let attribute_profile_ids = build_index(
            "attribute_profile",
            source.attribute_profiles.iter().map(|profile| &profile.id),
        )?;
        let opening_rite_outcome_ids = build_index(
            "opening_rite_outcome",
            source
                .opening_rite_outcomes
                .iter()
                .map(|outcome| &outcome.id),
        )?;
        let initial_resource_package_ids = build_index(
            "initial_resource_package",
            source
                .initial_resource_packages
                .iter()
                .map(|package| &package.id),
        )?;

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
                let target_is_encounter_decision = is_encounter_decision_intent(&action.intent)
                    && encounter_ids.contains_key(target);
                if !node_ids.contains_key(target) && !target_is_encounter_decision {
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

        for movement in &source.movements {
            validate_common_content(
                "movement",
                &movement.id,
                &movement.stage,
                &movement.tags,
                &movement.evidence,
                &movement.modes,
                ContentImportance::Standard,
            )?;
            if !node_ids.contains_key(&movement.from) {
                return Err(CommandError::content(
                    "移动边起点不存在",
                    format!(
                        "movement '{}' from node '{}' not found",
                        movement.id, movement.from
                    ),
                ));
            }
            if !node_ids.contains_key(&movement.to) {
                return Err(CommandError::content(
                    "移动边终点不存在",
                    format!(
                        "movement '{}' to node '{}' not found",
                        movement.id, movement.to
                    ),
                ));
            }
            if movement.ap_cost > 3 {
                return Err(CommandError::content(
                    "移动 AP 成本超出 Sprint 0 范围",
                    format!("movement '{}' ap_cost must be 0..=3", movement.id),
                ));
            }
            if movement.arrival_ap_penalty > 3 {
                return Err(CommandError::content(
                    "移动到达 AP 压缩超出 Sprint 0 范围",
                    format!(
                        "movement '{}' arrival_ap_penalty must be 0..=3",
                        movement.id
                    ),
                ));
            }
            if movement.exposure_delta < 0 {
                return Err(CommandError::content(
                    "移动暴露变化不能为负",
                    format!("movement '{}' exposure_delta must be >= 0", movement.id),
                ));
            }
        }

        for encounter in &source.encounters {
            validate_common_content(
                "encounter",
                &encounter.id,
                &encounter.stage,
                &encounter.tags,
                &encounter.evidence,
                &encounter.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty(
                "encounter",
                &encounter.id,
                "known_risk",
                &encounter.known_risk,
            )?;
            if !node_ids.contains_key(&encounter.trigger_node_id) {
                return Err(CommandError::content(
                    "encounter trigger node not found",
                    format!(
                        "encounter '{}' trigger node '{}' not found",
                        encounter.id, encounter.trigger_node_id
                    ),
                ));
            }
            if encounter.min_exposure.is_some_and(|value| value < 0) {
                return Err(CommandError::content(
                    "encounter trigger threshold cannot be negative",
                    format!("encounter '{}' min_exposure must be >= 0", encounter.id),
                ));
            }
            if encounter.decisions.is_empty() {
                return Err(CommandError::content(
                    "encounter decisions cannot be empty",
                    format!("encounter '{}' decisions must not be empty", encounter.id),
                ));
            }
            let mut decision_intents = Vec::new();
            for decision in &encounter.decisions {
                if !is_encounter_decision_intent(&decision.intent) {
                    return Err(CommandError::content(
                        "encounter decision intent is invalid",
                        format!(
                            "encounter '{}' decision '{:?}' is not an encounter decision",
                            encounter.id, decision.intent
                        ),
                    ));
                }
                if decision_intents.contains(&decision.intent) {
                    return Err(CommandError::content(
                        "encounter decision intent duplicated",
                        format!(
                            "encounter '{}' decision '{:?}' appears more than once",
                            encounter.id, decision.intent
                        ),
                    ));
                }
                decision_intents.push(decision.intent.clone());
                if decision.ap_cost > 3
                    || decision.primeval_stones_cost < 0
                    || decision.exposure_delta < 0
                    || decision
                        .mitigated_exposure_delta
                        .is_some_and(|value| value < 0)
                {
                    return Err(CommandError::content(
                        "encounter decision costs cannot be invalid",
                        format!(
                            "encounter '{}' decision costs are out of range",
                            encounter.id
                        ),
                    ));
                }
                if let Some(target_node_id) = &decision.target_node_id {
                    if !node_ids.contains_key(target_node_id) {
                        return Err(CommandError::content(
                            "encounter decision target node not found",
                            format!(
                                "encounter '{}' decision target node '{}' not found",
                                encounter.id, target_node_id
                            ),
                        ));
                    }
                }
                require_non_empty(
                    "encounter decision",
                    &encounter.id,
                    "survival_route",
                    &decision.survival_route,
                )?;
            }
        }

        for narrative in &source.narratives {
            validate_common_content(
                "narrative",
                &narrative.id,
                &narrative.stage,
                &narrative.tags,
                &narrative.evidence,
                &narrative.modes,
                ContentImportance::Flavor,
            )?;
            require_non_empty("narrative", &narrative.id, "text", &narrative.text)?;
        }

        if source.origins.len() < 3 {
            return Err(CommandError::content(
                "出身候选不足",
                "Sprint 3 setup content requires at least 3 origins",
            ));
        }
        for origin in &source.origins {
            validate_common_content(
                "origin",
                &origin.id,
                &origin.stage,
                &origin.tags,
                &origin.evidence,
                &origin.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty("origin", &origin.id, "title", &origin.title)?;
            require_non_empty("origin", &origin.id, "summary", &origin.summary)?;
            if !initial_resource_package_ids.contains_key(&origin.resource_package_id) {
                return Err(CommandError::content(
                    "出身资源包不存在",
                    format!(
                        "origin '{}' resource package '{}' not found",
                        origin.id, origin.resource_package_id
                    ),
                ));
            }
        }

        if source.talents.len() < 10 {
            return Err(CommandError::content(
                "天赋候选不足",
                "Sprint 3 setup content requires at least 10 talents",
            ));
        }
        for talent in &source.talents {
            if talent.intensity == TalentIntensity::StrongIf
                && talent.modes.contains(&ModePermit::CanonStrict)
            {
                return Err(CommandError::content(
                    "强 IF 天赋不能进入严谨模式",
                    format!(
                        "talent '{}' is strong_if but allows canon_strict",
                        talent.id
                    ),
                ));
            }
            validate_common_content(
                "talent",
                &talent.id,
                &talent.stage,
                &talent.tags,
                &talent.evidence,
                &talent.modes,
                if talent.intensity == TalentIntensity::StrongIf {
                    ContentImportance::Critical
                } else {
                    ContentImportance::Standard
                },
            )?;
            require_non_empty("talent", &talent.id, "title", &talent.title)?;
            require_non_empty("talent", &talent.id, "summary", &talent.summary)?;
            require_non_empty("talent", &talent.id, "pressure_note", &talent.pressure_note)?;
            if talent.intensity == TalentIntensity::StrongIf
                && talent.modes.contains(&ModePermit::CanonStrict)
            {
                return Err(CommandError::content(
                    "强 IF 天赋不能进入严谨模式",
                    format!(
                        "talent '{}' is strong_if but allows canon_strict",
                        talent.id
                    ),
                ));
            }
        }

        if source.attribute_profiles.is_empty() {
            return Err(CommandError::content(
                "属性面板缺失",
                "Sprint 3 setup content requires an attribute profile",
            ));
        }
        for profile in &source.attribute_profiles {
            validate_common_content(
                "attribute_profile",
                &profile.id,
                &profile.stage,
                &profile.tags,
                &profile.evidence,
                &profile.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty("attribute_profile", &profile.id, "title", &profile.title)?;
            validate_attribute_profile(profile)?;
        }
        let attribute_ids = source
            .attribute_profiles
            .iter()
            .flat_map(|profile| profile.attributes.iter().map(|attribute| &attribute.id))
            .cloned()
            .collect::<BTreeSet<_>>();
        for origin in &source.origins {
            validate_attribute_modifiers(
                "origin",
                &origin.id,
                &origin.attribute_modifiers,
                &attribute_ids,
            )?;
        }
        for talent in &source.talents {
            validate_attribute_modifiers(
                "talent",
                &talent.id,
                &talent.attribute_modifiers,
                &attribute_ids,
            )?;
        }

        if source.opening_rite_outcomes.is_empty() {
            return Err(CommandError::content(
                "开窍大典结果缺失",
                "Sprint 3 setup content requires an opening rite outcome",
            ));
        }
        for outcome in &source.opening_rite_outcomes {
            validate_common_content(
                "opening_rite_outcome",
                &outcome.id,
                &outcome.stage,
                &outcome.tags,
                &outcome.evidence,
                &outcome.modes,
                ContentImportance::Critical,
            )?;
            require_non_empty("opening_rite_outcome", &outcome.id, "title", &outcome.title)?;
            require_non_empty(
                "opening_rite_outcome",
                &outcome.id,
                "summary",
                &outcome.summary,
            )?;
            require_non_empty(
                "opening_rite_outcome",
                &outcome.id,
                "displayed_grade",
                &outcome.displayed_grade,
            )?;
            if !outcome.aperture_opened {
                return Err(CommandError::content(
                    "开窍大典结果必须打开空窍",
                    format!("opening outcome '{}' has aperture_opened=false", outcome.id),
                ));
            }
            if !initial_resource_package_ids.contains_key(&outcome.resource_package_id) {
                return Err(CommandError::content(
                    "开窍大典资源包不存在",
                    format!(
                        "opening outcome '{}' resource package '{}' not found",
                        outcome.id, outcome.resource_package_id
                    ),
                ));
            }
        }

        if source.initial_resource_packages.is_empty() {
            return Err(CommandError::content(
                "初始资源包缺失",
                "Sprint 3 setup content requires an initial resource package",
            ));
        }
        for package in &source.initial_resource_packages {
            validate_common_content(
                "initial_resource_package",
                &package.id,
                &package.stage,
                &package.tags,
                &package.evidence,
                &package.modes,
                ContentImportance::Standard,
            )?;
            require_non_empty(
                "initial_resource_package",
                &package.id,
                "title",
                &package.title,
            )?;
            if package.primeval_stones < 0
                || package.materials < 0
                || package.merit < 0
                || package.infirmary_debt < 0
                || package.favor_debt < 0
                || package.organization_debt < 0
                || package.trading_credit < 0
                || package.exposure < 0
            {
                return Err(CommandError::content(
                    "初始资源包数值不能为负",
                    format!(
                        "initial resource package '{}' has negative values",
                        package.id
                    ),
                ));
            }
        }

        let node_count = source.nodes.len();
        let action_count = source.actions.len();
        let route_count = source.routes.len();
        let window_count = source.windows.len();
        let movement_count = source.movements.len();
        let encounter_count = source.encounters.len();
        let narrative_count = source.narratives.len();
        let origin_count = source.origins.len();
        let talent_count = source.talents.len();
        let attribute_profile_count = source.attribute_profiles.len();
        let opening_rite_outcome_count = source.opening_rite_outcomes.len();
        let initial_resource_package_count = source.initial_resource_packages.len();

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
                movement_count,
                encounter_count,
                narrative_count,
                origin_count,
                talent_count,
                attribute_profile_count,
                opening_rite_outcome_count,
                initial_resource_package_count,
            },
            nodes: source.nodes,
            actions: source.actions,
            routes: source.routes,
            windows: source.windows,
            movements: source.movements,
            encounters: source.encounters,
            narratives: source.narratives,
            origins: source.origins,
            talents: source.talents,
            attribute_profiles: source.attribute_profiles,
            opening_rite_outcomes: source.opening_rite_outcomes,
            initial_resource_packages: source.initial_resource_packages,
            indexes: ContentIndexes {
                node_ids,
                action_ids,
                route_ids,
                window_ids,
                movement_ids,
                encounter_ids,
                narrative_ids,
                origin_ids,
                talent_ids,
                attribute_profile_ids,
                opening_rite_outcome_ids,
                initial_resource_package_ids,
            },
            diagnostics: ContentBuildDiagnostics {
                summary: format!(
                    "indexed {node_count} nodes, {action_count} actions, {route_count} routes, {window_count} windows, {movement_count} movements, {encounter_count} encounters, {narrative_count} narratives, {origin_count} origins, {talent_count} talents, {attribute_profile_count} attribute profiles, {opening_rite_outcome_count} opening rite outcomes, {initial_resource_package_count} initial resource packages"
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
        let mut movements = Vec::new();
        let mut encounters = Vec::new();
        let mut narratives = Vec::new();
        let mut origins = Vec::new();
        let mut talents = Vec::new();
        let mut attribute_profiles = Vec::new();
        let mut opening_rite_outcomes = Vec::new();
        let mut initial_resource_packages = Vec::new();

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
            movements.extend(fragment.movements);
            encounters.extend(fragment.encounters);
            narratives.extend(fragment.narratives);
            origins.extend(fragment.origins);
            talents.extend(fragment.talents);
            attribute_profiles.extend(fragment.attribute_profiles);
            opening_rite_outcomes.extend(fragment.opening_rite_outcomes);
            initial_resource_packages.extend(fragment.initial_resource_packages);
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
            movements,
            encounters,
            narratives,
            origins,
            talents,
            attribute_profiles,
            opening_rite_outcomes,
            initial_resource_packages,
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

fn validate_attribute_profile(profile: &ContentAttributeProfile) -> Result<(), CommandError> {
    if profile.attributes.len() != 4 {
        return Err(CommandError::content(
            "属性面板必须包含四项属性",
            format!(
                "attribute profile '{}' must contain exactly 4 attributes",
                profile.id
            ),
        ));
    }

    let mut seen = BTreeSet::new();
    for attribute in &profile.attributes {
        require_non_empty("attribute_definition", &profile.id, "id", &attribute.id)?;
        require_non_empty(
            "attribute_definition",
            &attribute.id,
            "label",
            &attribute.label,
        )?;
        require_non_empty(
            "attribute_definition",
            &attribute.id,
            "summary",
            &attribute.summary,
        )?;
        if !seen.insert(attribute.id.clone()) {
            return Err(CommandError::content(
                "属性 id 重复",
                format!(
                    "attribute profile '{}' has duplicate attribute '{}'",
                    profile.id, attribute.id
                ),
            ));
        }
        if attribute.min > attribute.max
            || attribute.base < attribute.min
            || attribute.base > attribute.max
        {
            return Err(CommandError::content(
                "属性范围非法",
                format!(
                    "attribute profile '{}' attribute '{}' has invalid min/base/max",
                    profile.id, attribute.id
                ),
            ));
        }
    }

    for required in ["aptitude", "wit", "bone", "luck"] {
        if !seen.contains(required) {
            return Err(CommandError::content(
                "属性面板缺少核心属性",
                format!(
                    "attribute profile '{}' missing required attribute '{}'",
                    profile.id, required
                ),
            ));
        }
    }

    Ok(())
}

fn validate_attribute_modifiers(
    kind: &str,
    id: &str,
    modifiers: &BTreeMap<String, i32>,
    attribute_ids: &BTreeSet<String>,
) -> Result<(), CommandError> {
    for attribute_id in modifiers.keys() {
        if !attribute_ids.contains(attribute_id) {
            return Err(CommandError::content(
                "属性修正指向未知属性",
                format!("{kind} '{id}' modifier attribute '{attribute_id}' not found"),
            ));
        }
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
    starter_content_bundle().manifest
}

pub fn starter_content_bundle() -> ContentBundle {
    ContentBundle::from_source(starter_content_source())
        .expect("starter S0 content bundle must validate")
}

pub fn starter_content_source() -> ContentSource {
    ContentSource {
        content_id: "s0.qingmao.foundation".to_string(),
        version: STARTER_CONTENT_VERSION.to_string(),
        title: "青茅山 S0 首发内容骨架".to_string(),
        stage: "s0".to_string(),
        entry_scene_id: "academy_gate".to_string(),
        nodes: starter_nodes(),
        actions: starter_actions(),
        routes: starter_routes(),
        windows: starter_windows(),
        movements: starter_movements(),
        encounters: starter_encounters(),
        narratives: starter_narratives(),
        origins: starter_origins(),
        talents: starter_talents(),
        attribute_profiles: starter_attribute_profiles(),
        opening_rite_outcomes: starter_opening_rite_outcomes(),
        initial_resource_packages: starter_initial_resource_packages(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn modifiers(values: &[(&str, i32)]) -> BTreeMap<String, i32> {
    values
        .iter()
        .map(|(key, value)| ((*key).to_string(), *value))
        .collect()
}

fn all_modes() -> Vec<ModePermit> {
    vec![ModePermit::CanonStrict, ModePermit::SandboxIf]
}

fn sandbox_only() -> Vec<ModePermit> {
    vec![ModePermit::SandboxIf]
}

fn starter_initial_resource_packages() -> Vec<ContentInitialResourcePackage> {
    vec![
        initial_resource_package(
            "s0_opening_plain_supplies",
            "开窍后普通配给",
            3,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "resource-package", "opening"],
        ),
        initial_resource_package(
            "s0_branch_child_supplies",
            "旁支子弟薄底",
            2,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "resource-package", "branch"],
        ),
        initial_resource_package(
            "s0_infirmary_tie_supplies",
            "药堂人情牵连",
            2,
            1,
            0,
            1,
            1,
            0,
            0,
            0,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "resource-package", "infirmary"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn initial_resource_package(
    id: &str,
    title: &str,
    primeval_stones: i32,
    materials: i32,
    merit: i32,
    infirmary_debt: i32,
    favor_debt: i32,
    organization_debt: i32,
    trading_credit: i32,
    exposure: i32,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentInitialResourcePackage {
    ContentInitialResourcePackage {
        id: id.to_string(),
        title: title.to_string(),
        primeval_stones,
        materials,
        merit,
        infirmary_debt,
        favor_debt,
        organization_debt,
        trading_credit,
        exposure,
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
    }
}

fn starter_origins() -> Vec<ContentOriginSpec> {
    vec![
        origin(
            "qingmao_branch_child",
            "古月旁支子弟",
            "有族姓庇护，也有薄弱家底和旁支债影；适合作为严谨模式的普通压迫开局。",
            "s0_branch_child_supplies",
            &[("wit", 1)],
            0,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "origin", "qingmao", "branch"],
        ),
        origin(
            "academy_plain_child",
            "学堂普通子弟",
            "从开窍大典后直接被纳入学堂秩序，资源普通，压力透明，没有额外机缘。",
            "s0_opening_plain_supplies",
            &[("aptitude", 1)],
            1,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "origin", "qingmao", "academy"],
        ),
        origin(
            "infirmary_debt_tie",
            "药堂人情牵连",
            "身边早有一条药堂人情线，恢复门路更清楚，但债务从开局就压在账上。",
            "s0_infirmary_tie_supplies",
            &[("bone", 1)],
            0,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["setup", "origin", "qingmao", "infirmary"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn origin(
    id: &str,
    title: &str,
    summary: &str,
    resource_package_id: &str,
    attribute_modifiers: &[(&str, i32)],
    attention_delta: i32,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentOriginSpec {
    ContentOriginSpec {
        id: id.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        resource_package_id: resource_package_id.to_string(),
        attribute_modifiers: modifiers(attribute_modifiers),
        attention_delta,
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
    }
}

fn starter_talents() -> Vec<ContentTalentSpec> {
    vec![
        talent(
            "steady_mind",
            "心性沉稳",
            "面对羞辱和债务时更不容易乱走一步。",
            TalentIntensity::Mild,
            &[("wit", 1)],
            &["survival", "academy"],
            "只提供心理韧性，不生成额外机缘。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "quiet_observer",
            "静观其变",
            "更容易从风声、眼色和账本变化里看出门路。",
            TalentIntensity::Mild,
            &[("wit", 1), ("luck", 1)],
            &["information", "rumor"],
            "只能帮助读局，不能直接改变原著硬事实。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "frugal_hands",
            "手头俭省",
            "元石花出去前会多掂量一次。",
            TalentIntensity::Mild,
            &[("luck", 1)],
            &["economy", "debt"],
            "节省倾向不等于凭空获得资源。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "bitter_bone",
            "苦骨耐压",
            "受伤、欠债、被盯上时，仍能维持一点行动余地。",
            TalentIntensity::Mild,
            &[("bone", 1)],
            &["injury", "pressure"],
            "只保留为后续抗压钩子，不免除重创可续代价。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "academy_ear",
            "学堂耳目",
            "更早注意到学堂公开压力和功绩审计的风向。",
            TalentIntensity::Mild,
            &[("wit", 1)],
            &["academy", "merit"],
            "获得的是风声入口，不是导师庇护。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "moonlight_pacing",
            "月光步调",
            "对月光修行节奏更敏感，但仍受空窍、元石和窗口限制。",
            TalentIntensity::Mild,
            &[("aptitude", 1)],
            &["moonlight", "cultivation"],
            "不能绕过开窍、资源和学堂制度压力。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "debt_memory",
            "欠账记性",
            "对人情和债务更警醒，适合药堂和黑市边缘求活。",
            TalentIntensity::Mild,
            &[("wit", 1)],
            &["debt", "infirmary"],
            "只能提醒追索风险，不洗掉债。",
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        talent(
            "reborn_memory_fragment",
            "残缺重生记忆",
            "脑中有少量不稳定的未来碎片，会推高锚点偏移压力。",
            TalentIntensity::StrongIf,
            &[("wit", 2), ("luck", 1)],
            &["if", "timeline"],
            "只能在 sandbox_if 作为高风险 IF 使用，不进入严谨模式。",
            EvidenceLevel::SandboxIf,
            sandbox_only(),
        ),
        talent(
            "inheritance_scent",
            "传承嗅觉",
            "对半真半假的传承线索更敏感，也更容易被危险线索牵走。",
            TalentIntensity::StrongIf,
            &[("luck", 2)],
            &["if", "inheritance"],
            "不能稳定抢原著机缘，必须受模式和锚点门禁限制。",
            EvidenceLevel::SandboxIf,
            sandbox_only(),
        ),
        talent(
            "vital_gu_omen",
            "本命蛊异兆",
            "空窍深处有一丝不明牵动，只作为本命蛊保留位的 IF 预兆。",
            TalentIntensity::StrongIf,
            &[("aptitude", 1), ("luck", 1)],
            &["if", "vital-gu"],
            "不等于获得本命蛊，也不能在 S0 直接绑定。",
            EvidenceLevel::SandboxIf,
            sandbox_only(),
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn talent(
    id: &str,
    title: &str,
    summary: &str,
    intensity: TalentIntensity,
    attribute_modifiers: &[(&str, i32)],
    route_tags: &[&str],
    pressure_note: &str,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
) -> ContentTalentSpec {
    ContentTalentSpec {
        id: id.to_string(),
        title: title.to_string(),
        summary: summary.to_string(),
        intensity,
        attribute_modifiers: modifiers(attribute_modifiers),
        route_tags: strings(route_tags),
        pressure_note: pressure_note.to_string(),
        stage: "s0".to_string(),
        tags: strings(&["setup", "talent"]),
        evidence,
        modes,
    }
}

fn starter_attribute_profiles() -> Vec<ContentAttributeProfile> {
    vec![ContentAttributeProfile {
        id: "s0_default_attributes".to_string(),
        title: "开窍前基础四项".to_string(),
        attributes: vec![
            attribute(
                "aptitude",
                "资质",
                "空窍潜力与修行承载的粗略表现。",
                1,
                10,
                5,
            ),
            attribute("wit", "心智", "读局、忍耐、识别风险的能力。", 1, 10, 5),
            attribute("bone", "体魄", "受伤后维持行动与恢复的基础。", 1, 10, 5),
            attribute("luck", "运势", "撞见机会或避开麻烦的波动余地。", 1, 10, 5),
        ],
        stage: "s0".to_string(),
        tags: strings(&["setup", "attributes"]),
        evidence: EvidenceLevel::CanonInferred,
        modes: all_modes(),
    }]
}

fn attribute(
    id: &str,
    label: &str,
    summary: &str,
    min: i32,
    max: i32,
    base: i32,
) -> ContentAttributeDefinition {
    ContentAttributeDefinition {
        id: id.to_string(),
        label: label.to_string(),
        summary: summary.to_string(),
        min,
        max,
        base,
    }
}

fn starter_opening_rite_outcomes() -> Vec<ContentOpeningRiteOutcome> {
    vec![ContentOpeningRiteOutcome {
        id: "s0_opening_rite_established".to_string(),
        title: "开窍大典已过".to_string(),
        summary: "空窍已经开启，族中学堂秩序随之压下；自由窗口从根基未稳之后开始。".to_string(),
        aperture_opened: true,
        displayed_grade: "普通可训".to_string(),
        attention_delta: 1,
        resource_package_id: "s0_opening_plain_supplies".to_string(),
        stage: "s0".to_string(),
        tags: strings(&["setup", "opening-rite"]),
        evidence: EvidenceLevel::CanonInferred,
        modes: all_modes(),
    }]
}

fn starter_nodes() -> Vec<ContentNode> {
    vec![
        node(
            "academy_gate",
            "学堂门前",
            "low",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "academy"],
        ),
        node(
            "moonlight_corner",
            "月光修行角",
            "low",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "moonlight"],
        ),
        node(
            "merit_notice",
            "功绩告示旁",
            "watched",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "merit"],
        ),
        node(
            "infirmary_lane",
            "药堂侧巷",
            "debt",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "infirmary"],
        ),
        node(
            "branch_lodging",
            "旁支落脚点",
            "low",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "branch", "lodging"],
        ),
        node(
            "clan_alley_rumor",
            "山寨巷道风声点",
            "watched",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["node", "clan", "alley", "rumor"],
        ),
        node(
            "blackmarket_hint",
            "黑市暗口",
            "hidden-risk",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["node", "blackmarket", "hidden"],
        ),
        node(
            "inheritance_rumor",
            "传承残线",
            "high-risk-if",
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["node", "inheritance", "sandbox-if"],
        ),
    ]
}

fn node(
    id: &str,
    title: &str,
    safety: &str,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentNode {
    ContentNode {
        id: id.to_string(),
        title: title.to_string(),
        safety: safety.to_string(),
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
    }
}

fn starter_actions() -> Vec<ContentAction> {
    vec![
        action(
            "scout_academy",
            "观察学堂风声",
            ActionIntent::Scout,
            Some("academy_gate"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout"],
        ),
        action(
            "cultivate_moonlight",
            "月光修行",
            ActionIntent::Cultivate,
            Some("academy_gate"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "cultivate", "moonlight"],
        ),
        action(
            "cultivate_moonlight_corner",
            "借月光角修行",
            ActionIntent::Cultivate,
            Some("moonlight_corner"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "cultivate", "moonlight"],
        ),
        action(
            "move_moonlight_corner",
            "挪到月光修行角",
            ActionIntent::Move,
            Some("moonlight_corner"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "move"],
        ),
        action(
            "observe_moonlight_pressure",
            "观察月光角压力",
            ActionIntent::Scout,
            Some("moonlight_corner"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "moonlight"],
        ),
        action(
            "move_merit_notice",
            "靠近功绩告示",
            ActionIntent::Move,
            Some("merit_notice"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "move", "merit"],
        ),
        action(
            "check_merit_notice",
            "查功绩告示",
            ActionIntent::Scout,
            Some("merit_notice"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "merit"],
        ),
        action(
            "audit_merit_notice",
            "核对功绩审计",
            ActionIntent::Scout,
            Some("merit_notice"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "merit", "audit"],
        ),
        action(
            "move_infirmary_lane",
            "去药堂侧巷",
            ActionIntent::Move,
            Some("infirmary_lane"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "move", "infirmary"],
        ),
        action(
            "seek_treatment_debt",
            "赊一口恢复",
            ActionIntent::Recover,
            Some("infirmary_lane"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "recover", "debt"],
        ),
        action(
            "ask_infirmary_price",
            "打听药堂价码",
            ActionIntent::Scout,
            Some("infirmary_lane"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "infirmary", "debt"],
        ),
        action(
            "move_branch_lodging",
            "回旁支落脚点",
            ActionIntent::Move,
            Some("branch_lodging"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "move", "branch"],
        ),
        action(
            "listen_branch_lodging_debt",
            "听旁支债声",
            ActionIntent::Scout,
            Some("branch_lodging"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "branch", "debt"],
        ),
        action(
            "move_clan_alley_rumor",
            "绕到山寨巷道",
            ActionIntent::Move,
            Some("clan_alley_rumor"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "move", "alley", "rumor"],
        ),
        action(
            "listen_clan_alley_rumor",
            "听巷道风声",
            ActionIntent::Scout,
            Some("clan_alley_rumor"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "scout", "alley", "rumor"],
        ),
        action(
            "move_blackmarket_hint",
            "摸向黑市暗口",
            ActionIntent::Move,
            Some("blackmarket_hint"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "move", "blackmarket"],
        ),
        action(
            "probe_blackmarket_hint",
            "黑市换料",
            ActionIntent::Trade,
            Some("blackmarket_hint"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "trade", "blackmarket"],
        ),
        action(
            "retreat_blackmarket_extortion",
            "退避勒索",
            ActionIntent::Retreat,
            Some("blackmarket_extortion"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "retreat"],
        ),
        action(
            "confront_blackmarket_extortion",
            "硬顶勒索",
            ActionIntent::Confront,
            Some("blackmarket_extortion"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "confront"],
        ),
        action(
            "yield_academy_public_pressure",
            "忍让学堂压力",
            ActionIntent::Yield,
            Some("academy_public_pressure"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "encounter", "academy", "yield"],
        ),
        action(
            "argue_academy_public_pressure",
            "争辩学堂压力",
            ActionIntent::Argue,
            Some("academy_public_pressure"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "encounter", "academy", "argue"],
        ),
        action(
            "confront_academy_public_pressure",
            "硬撑学堂压力",
            ActionIntent::Confront,
            Some("academy_public_pressure"),
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["action", "encounter", "academy", "confront"],
        ),
        action(
            "retreat_alley_probe",
            "退走巷道试探",
            ActionIntent::Retreat,
            Some("alley_probe"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "alley", "retreat"],
        ),
        action(
            "delay_alley_probe",
            "拖延巷道试探",
            ActionIntent::Delay,
            Some("alley_probe"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "alley", "delay"],
        ),
        action(
            "frame_alley_probe",
            "嫁祸巷道试探",
            ActionIntent::Frame,
            Some("alley_probe"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "alley", "frame"],
        ),
        action(
            "confront_alley_probe",
            "硬顶巷道试探",
            ActionIntent::Confront,
            Some("alley_probe"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["action", "encounter", "alley", "confront"],
        ),
        action(
            "chase_inheritance_rumor",
            "追传承残线",
            ActionIntent::Move,
            Some("inheritance_rumor"),
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["action", "move", "inheritance"],
        ),
        action(
            "verify_inheritance_rumor",
            "查验传承残线",
            ActionIntent::Scout,
            Some("inheritance_rumor"),
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["action", "scout", "inheritance", "rumor"],
        ),
    ]
}

fn action(
    id: &str,
    label: &str,
    intent: ActionIntent,
    target: Option<&str>,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentAction {
    ContentAction {
        id: id.to_string(),
        label: label.to_string(),
        intent,
        target: target.map(str::to_string),
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
        importance: ContentImportance::Standard,
    }
}

fn starter_routes() -> Vec<ContentRouteEntry> {
    vec![
        route(
            "moonlight_entry",
            "月光路线入口",
            "moonlight",
            &[
                "cultivate_moonlight",
                "move_moonlight_corner",
                "cultivate_moonlight_corner",
                "observe_moonlight_pressure",
            ],
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        route(
            "merit_entry",
            "功绩路线入口",
            "merit",
            &[
                "move_merit_notice",
                "check_merit_notice",
                "audit_merit_notice",
            ],
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        route(
            "infirmary_entry",
            "药堂半主路线入口",
            "infirmary",
            &[
                "move_infirmary_lane",
                "seek_treatment_debt",
                "ask_infirmary_price",
            ],
            EvidenceLevel::CanonInferred,
            all_modes(),
        ),
        route(
            "blackmarket_entry",
            "黑市路线入口",
            "blackmarket",
            &[
                "move_clan_alley_rumor",
                "listen_clan_alley_rumor",
                "move_blackmarket_hint",
                "probe_blackmarket_hint",
            ],
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
        ),
        route(
            "inheritance_entry",
            "传承路线入口",
            "inheritance",
            &["chase_inheritance_rumor", "verify_inheritance_rumor"],
            EvidenceLevel::SandboxIf,
            sandbox_only(),
        ),
    ]
}

fn route(
    id: &str,
    label: &str,
    route: &str,
    entry_action_ids: &[&str],
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
) -> ContentRouteEntry {
    ContentRouteEntry {
        id: id.to_string(),
        label: label.to_string(),
        route: route.to_string(),
        entry_action_ids: strings(entry_action_ids),
        stage: "s0".to_string(),
        tags: strings(&["route", route]),
        evidence,
        modes,
    }
}

fn starter_windows() -> Vec<ContentWindow> {
    vec![
        window("day1_morning_free", 1, "清晨", 2),
        window("day1_midday_free", 1, "日中", 2),
        window("day1_evening_free", 1, "傍晚", 2),
        window("day1_deep_night_free", 1, "深夜", 1),
        window("day2_morning_free", 2, "清晨", 2),
        window("day2_midday_free", 2, "日中", 2),
        window("day2_evening_free", 2, "傍晚", 2),
        window("day2_deep_night_free", 2, "深夜", 1),
    ]
}

fn window(id: &str, day: u8, period: &str, default_ap: u8) -> ContentWindow {
    ContentWindow {
        id: id.to_string(),
        day,
        period: period.to_string(),
        window_type: WindowType::Free,
        default_ap,
        stage: "s0".to_string(),
        tags: strings(&["window", "free"]),
        evidence: EvidenceLevel::CanonInferred,
        modes: all_modes(),
    }
}

fn starter_movements() -> Vec<ContentMovementEdge> {
    vec![
        movement(
            "academy_to_moonlight",
            "academy_gate",
            "moonlight_corner",
            0,
            0,
            0,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "near"],
        ),
        movement(
            "moonlight_to_academy",
            "moonlight_corner",
            "academy_gate",
            0,
            0,
            0,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "near"],
        ),
        movement(
            "academy_to_merit_notice",
            "academy_gate",
            "merit_notice",
            0,
            0,
            1,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "watched"],
        ),
        movement(
            "merit_notice_to_academy",
            "merit_notice",
            "academy_gate",
            0,
            0,
            0,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "watched"],
        ),
        movement(
            "academy_to_infirmary_lane",
            "academy_gate",
            "infirmary_lane",
            0,
            1,
            1,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "infirmary"],
        ),
        movement(
            "infirmary_lane_to_academy",
            "infirmary_lane",
            "academy_gate",
            0,
            1,
            1,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "infirmary"],
        ),
        movement(
            "academy_to_branch_lodging",
            "academy_gate",
            "branch_lodging",
            0,
            0,
            0,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "branch", "near"],
        ),
        movement(
            "branch_lodging_to_academy",
            "branch_lodging",
            "academy_gate",
            0,
            0,
            0,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "branch", "near"],
        ),
        movement(
            "academy_to_clan_alley_rumor",
            "academy_gate",
            "clan_alley_rumor",
            0,
            0,
            1,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "alley", "rumor"],
        ),
        movement(
            "clan_alley_rumor_to_academy",
            "clan_alley_rumor",
            "academy_gate",
            0,
            0,
            1,
            None,
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["movement", "alley", "rumor"],
        ),
        movement(
            "clan_alley_rumor_to_blackmarket_hint",
            "clan_alley_rumor",
            "blackmarket_hint",
            0,
            0,
            2,
            Some("深夜"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["movement", "blackmarket", "hidden"],
        ),
        movement(
            "blackmarket_hint_to_clan_alley_rumor",
            "blackmarket_hint",
            "clan_alley_rumor",
            0,
            0,
            1,
            Some("深夜"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["movement", "blackmarket", "hidden"],
        ),
        movement(
            "academy_to_blackmarket_hint",
            "academy_gate",
            "blackmarket_hint",
            0,
            0,
            2,
            Some("深夜"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["movement", "blackmarket", "hidden"],
        ),
        movement(
            "blackmarket_hint_to_academy",
            "blackmarket_hint",
            "academy_gate",
            0,
            0,
            1,
            Some("深夜"),
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["movement", "blackmarket", "hidden"],
        ),
        movement(
            "academy_to_inheritance_rumor",
            "academy_gate",
            "inheritance_rumor",
            1,
            0,
            3,
            None,
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["movement", "inheritance", "sandbox-if"],
        ),
        movement(
            "inheritance_rumor_to_academy",
            "inheritance_rumor",
            "academy_gate",
            1,
            0,
            2,
            None,
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["movement", "inheritance", "sandbox-if"],
        ),
        movement(
            "clan_alley_rumor_to_inheritance_rumor",
            "clan_alley_rumor",
            "inheritance_rumor",
            1,
            0,
            3,
            None,
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["movement", "inheritance", "sandbox-if"],
        ),
        movement(
            "inheritance_rumor_to_clan_alley_rumor",
            "inheritance_rumor",
            "clan_alley_rumor",
            1,
            0,
            2,
            None,
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["movement", "inheritance", "sandbox-if"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn movement(
    id: &str,
    from: &str,
    to: &str,
    ap_cost: u8,
    arrival_ap_penalty: u8,
    exposure_delta: i32,
    required_period: Option<&str>,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentMovementEdge {
    ContentMovementEdge {
        id: id.to_string(),
        from: from.to_string(),
        to: to.to_string(),
        ap_cost,
        arrival_ap_penalty,
        exposure_delta,
        required_period: required_period.map(str::to_string),
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
    }
}

fn starter_encounters() -> Vec<ContentEncounterTemplate> {
    vec![
        ContentEncounterTemplate {
            id: "blackmarket_extortion".to_string(),
            encounter_type: EncounterType::Extortion,
            trigger_node_id: "blackmarket_hint".to_string(),
            known_risk: "边路被盯上：对方要元石，硬顶会受创。".to_string(),
            min_exposure: None,
            min_moonlight_marks: None,
            required_clue_ids: vec![],
            decisions: vec![
                encounter_decision(
                    ActionIntent::Retreat,
                    1,
                    0,
                    1,
                    None,
                    false,
                    Some("academy_gate"),
                    "黑市退避：保命先于脸面",
                    "s0.encounter.blackmarket_extortion.retreat",
                    &[],
                    None,
                    None,
                ),
                encounter_decision(
                    ActionIntent::Confront,
                    1,
                    1,
                    2,
                    Some(InjuryLevel::Heavy),
                    true,
                    Some("academy_gate"),
                    "黑市硬顶：带伤续命",
                    "s0.encounter.blackmarket_extortion.confront",
                    &[],
                    None,
                    None,
                ),
            ],
            stage: "s0".to_string(),
            tags: strings(&["encounter", "blackmarket", "extortion"]),
            evidence: EvidenceLevel::GameplayExtrapolated,
            modes: all_modes(),
        },
        ContentEncounterTemplate {
            id: "academy_public_pressure".to_string(),
            encounter_type: EncounterType::PublicPressure,
            trigger_node_id: "moonlight_corner".to_string(),
            known_risk: "学堂目光压来：修行越显眼，公开羞辱和对练压力越近。".to_string(),
            min_exposure: None,
            min_moonlight_marks: Some(2),
            required_clue_ids: vec![],
            decisions: vec![
                encounter_decision(
                    ActionIntent::Yield,
                    1,
                    0,
                    1,
                    None,
                    false,
                    None,
                    "学堂忍让：保身压脸面",
                    "s0.encounter.academy_public_pressure.yield",
                    &[],
                    Some("rumor_academy_pressure"),
                    Some(0),
                ),
                encounter_decision(
                    ActionIntent::Argue,
                    1,
                    0,
                    2,
                    None,
                    false,
                    None,
                    "学堂争辩：被记下一笔",
                    "s0.encounter.academy_public_pressure.argue",
                    &[],
                    Some("rumor_academy_pressure"),
                    Some(1),
                ),
                encounter_decision(
                    ActionIntent::Confront,
                    1,
                    0,
                    2,
                    Some(InjuryLevel::Light),
                    true,
                    None,
                    "学堂硬撑：带伤保脸面",
                    "s0.encounter.academy_public_pressure.confront",
                    &[],
                    None,
                    None,
                ),
            ],
            stage: "s0".to_string(),
            tags: strings(&["encounter", "academy", "public_pressure"]),
            evidence: EvidenceLevel::CanonInferred,
            modes: all_modes(),
        },
        ContentEncounterTemplate {
            id: "alley_probe".to_string(),
            encounter_type: EncounterType::Probe,
            trigger_node_id: "clan_alley_rumor".to_string(),
            known_risk: "巷道有人试探尾巴：暴露越高，越容易被借机堵住。".to_string(),
            min_exposure: Some(2),
            min_moonlight_marks: None,
            required_clue_ids: vec![],
            decisions: vec![
                encounter_decision(
                    ActionIntent::Retreat,
                    1,
                    0,
                    1,
                    None,
                    false,
                    Some("academy_gate"),
                    "巷道退走：断尾保身",
                    "s0.encounter.alley_probe.retreat",
                    &[],
                    None,
                    None,
                ),
                encounter_decision(
                    ActionIntent::Delay,
                    1,
                    0,
                    1,
                    None,
                    false,
                    None,
                    "巷道拖延：用时间换缝隙",
                    "s0.encounter.alley_probe.delay",
                    &[],
                    Some("rumor_alley_probe"),
                    Some(0),
                ),
                encounter_decision(
                    ActionIntent::Frame,
                    1,
                    0,
                    2,
                    None,
                    false,
                    None,
                    "巷道嫁祸：把尾巴甩给旁人",
                    "s0.encounter.alley_probe.frame",
                    &["rumor_blackmarket_tail"],
                    None,
                    None,
                ),
                encounter_decision(
                    ActionIntent::Confront,
                    1,
                    1,
                    3,
                    Some(InjuryLevel::Light),
                    true,
                    Some("academy_gate"),
                    "巷道硬顶：带伤脱身",
                    "s0.encounter.alley_probe.confront",
                    &[],
                    None,
                    None,
                ),
            ],
            stage: "s0".to_string(),
            tags: strings(&["encounter", "alley", "probe"]),
            evidence: EvidenceLevel::GameplayExtrapolated,
            modes: all_modes(),
        },
    ]
}

#[allow(clippy::too_many_arguments)]
fn encounter_decision(
    intent: ActionIntent,
    ap_cost: u8,
    primeval_stones_cost: i32,
    exposure_delta: i32,
    injury_level: Option<InjuryLevel>,
    injury_ap_penalty_pending: bool,
    target_node_id: Option<&str>,
    survival_route: &str,
    narrative_id: &str,
    clue_ids: &[&str],
    mitigating_clue_id: Option<&str>,
    mitigated_exposure_delta: Option<i32>,
) -> ContentEncounterDecision {
    ContentEncounterDecision {
        intent,
        ap_cost,
        primeval_stones_cost,
        exposure_delta,
        injury_level,
        injury_ap_penalty_pending,
        target_node_id: target_node_id.map(str::to_string),
        survival_route: survival_route.to_string(),
        narrative_id: narrative_id.to_string(),
        clue_ids: strings(clue_ids),
        mitigating_clue_id: mitigating_clue_id.map(str::to_string),
        mitigated_exposure_delta,
    }
}

fn starter_narratives() -> Vec<ContentNarrativeTemplate> {
    vec![
        content_narrative(
            "s0.scene.opening.academy_gate",
            "你站在学堂门前，清晨的山雾压着木檐，点卯声还没有响。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "opening", "academy"],
        ),
        content_narrative(
            "s0.scene.node.branch_lodging",
            "旁支落脚点狭窄安静，能藏住一点喘息，也藏不住欠账的影子。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scene", "branch"],
        ),
        content_narrative(
            "s0.scene.node.clan_alley_rumor",
            "山寨巷道里脚步杂乱，风声混着避让的眼神，能听见门路，也会留下痕迹。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scene", "alley", "rumor"],
        ),
        content_narrative(
            "s0.movement.default",
            "你换了一个位置，账本只记下路径、时段与暴露，不替你粉饰动机。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "movement"],
        ),
        content_narrative(
            "s0.action.cultivate.moonlight",
            "你按下杂念运转真元，月光修行痕迹更深。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "cultivate", "moonlight"],
        ),
        content_narrative(
            "s0.action.cultivate.moonlight_corner",
            "你借月光角避开几道视线，真元运转更稳，账上仍记下一枚元石的缺口。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "cultivate", "moonlight"],
        ),
        content_narrative(
            "s0.action.scout.default",
            "你没有急着下注，先听风声、记人脸。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout"],
        ),
        content_narrative(
            "s0.action.scout.academy_gate",
            "你在学堂门前听见几句低声风声，暗口二字被记进线索页。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "academy", "blackmarket"],
        ),
        content_narrative(
            "s0.action.scout.merit_notice",
            "你在功绩告示旁核对机会，记下一点可用功绩。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "merit"],
        ),
        content_narrative(
            "s0.action.scout.moonlight_corner",
            "你在月光角看清几处站位，学堂里的比较压力比明面规矩更锋利。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "moonlight"],
        ),
        content_narrative(
            "s0.action.scout.merit_audit",
            "功绩告示旁不只写机会，也写着谁会查账、谁会记仇。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "merit", "audit"],
        ),
        content_narrative(
            "s0.action.scout.infirmary_lane",
            "药堂侧巷的价码不只算元石，还算人情和下一次被追索的时机。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "infirmary", "debt"],
        ),
        content_narrative(
            "s0.action.scout.branch_lodging",
            "旁支落脚点能挡一时风雨，也把欠账和亲疏写得更清楚。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "branch", "debt"],
        ),
        content_narrative(
            "s0.action.scout.clan_alley_rumor",
            "巷道里有人提到暗口，又立刻噤声；门路有了，暴露也跟着有了轮廓。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "scout", "alley", "blackmarket"],
        ),
        content_narrative(
            "s0.action.scout.inheritance_rumor",
            "传承残线半真半假，能记进账本，但不能当作稳妥出路。",
            EvidenceLevel::SandboxIf,
            sandbox_only(),
            &["narrative", "scout", "inheritance", "rumor"],
        ),
        content_narrative(
            "s0.action.recover.default",
            "你换来一口喘息，也把债写进药堂账页。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "recover", "infirmary"],
        ),
        content_narrative(
            "s0.action.recover.heavy_to_light",
            "药堂处理重伤，伤势降为轻伤，债仍跟着你。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "recover", "infirmary", "injury"],
        ),
        content_narrative(
            "s0.action.recover.light_to_healthy",
            "药堂清掉轻伤，又在债账上添了一笔。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "recover", "infirmary", "injury"],
        ),
        content_narrative(
            "s0.action.trade.blackmarket_hint",
            "你在暗口换来材料，门路和风险一起上涨。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "trade", "blackmarket"],
        ),
        content_narrative(
            "s0.encounter.blackmarket_extortion.trigger",
            "黑市边路有人拦住去路，勒索的风险已经明牌。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "blackmarket", "extortion"],
        ),
        content_narrative(
            "s0.encounter.blackmarket_extortion.retreat",
            "你选择跑路，丢一点脸面和掩护，保住筋骨。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "retreat"],
        ),
        content_narrative(
            "s0.encounter.blackmarket_extortion.confront",
            "你硬顶勒索，代价落在元石和伤势上。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "confront"],
        ),
        content_narrative(
            "s0.encounter.academy_public_pressure.trigger",
            "月光修行的痕迹一深，学堂里的目光就压了过来。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "encounter", "academy", "public_pressure"],
        ),
        content_narrative(
            "s0.encounter.academy_public_pressure.yield",
            "你忍下一句刺耳话，脸面被踩了一脚，但局面没有失控。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "encounter", "academy", "yield"],
        ),
        content_narrative(
            "s0.encounter.academy_public_pressure.argue",
            "你回了一句，声音不高，却足够让旁人记住这笔账。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "encounter", "academy", "argue"],
        ),
        content_narrative(
            "s0.encounter.academy_public_pressure.confront",
            "你硬撑住对练和羞辱，伤不重，但下一段窗口被压得更窄。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "encounter", "academy", "confront"],
        ),
        content_narrative(
            "s0.encounter.alley_probe.trigger",
            "巷道里的脚步慢了半拍，有人像是在试探你身后的尾巴。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "alley", "probe"],
        ),
        content_narrative(
            "s0.encounter.alley_probe.retreat",
            "你退得很快，断尾保身，账上只多一笔暴露。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "alley", "retreat"],
        ),
        content_narrative(
            "s0.encounter.alley_probe.delay",
            "你拖住话头，让对方摸不准你的底，时间替你撬开一条缝。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "alley", "delay"],
        ),
        content_narrative(
            "s0.encounter.alley_probe.frame",
            "你把尾巴甩向旁人，眼前脱身，后账却不会就此消失。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "alley", "frame"],
        ),
        content_narrative(
            "s0.encounter.alley_probe.confront",
            "你硬顶巷道试探，带伤脱身，也把自己写进更多眼睛里。",
            EvidenceLevel::GameplayExtrapolated,
            all_modes(),
            &["narrative", "encounter", "alley", "confront"],
        ),
        content_narrative(
            "s0.action.wait.default",
            "你把这个时段耗过去，未用 AP 不会结转。",
            EvidenceLevel::CanonInferred,
            all_modes(),
            &["narrative", "wait"],
        ),
    ]
}

fn content_narrative(
    id: &str,
    text: &str,
    evidence: EvidenceLevel,
    modes: Vec<ModePermit>,
    tags: &[&str],
) -> ContentNarrativeTemplate {
    ContentNarrativeTemplate {
        id: id.to_string(),
        stage: "s0".to_string(),
        tags: strings(tags),
        evidence,
        modes,
        text: text.to_string(),
    }
}

pub fn create_setup_run(
    mode: RunMode,
    content_bundle: &ContentBundle,
) -> Result<RunSetupState, CommandError> {
    let profile = setup_attribute_profile(content_bundle)?;
    let opening = setup_opening_rite(content_bundle)?;

    Ok(RunSetupState {
        run_id: DEFAULT_RUN_ID.to_string(),
        mode,
        content_version: content_bundle.manifest.version.clone(),
        selected_origin_id: None,
        selected_talent_ids: Vec::new(),
        attribute_values: base_attribute_values(profile),
        opening_rite_outcome_id: opening.id.clone(),
        completed: false,
    })
}

pub fn setup_command(intent: SetupIntent, target_id: impl Into<String>) -> SetupCommand {
    SetupCommand {
        intent,
        target_id: target_id.into(),
    }
}

pub fn resolve_setup_choice(
    mut setup: RunSetupState,
    command: SetupCommand,
    content_bundle: &ContentBundle,
) -> Result<SetupResponse, CommandError> {
    if setup.completed {
        return Err(CommandError::validation("设置已完成，不能继续修改"));
    }

    match command.intent {
        SetupIntent::SelectOrigin => {
            let origin = setup_origin_by_id(content_bundle, &command.target_id)?;
            require_mode(&setup.mode, &origin.modes, "origin", &origin.id)?;
            setup.selected_origin_id = Some(origin.id.clone());
        }
        SetupIntent::ToggleTalent => {
            let talent = setup_talent_by_id(content_bundle, &command.target_id)?;
            require_setup_talent_allowed(&setup.mode, talent)?;

            if let Some(position) = setup
                .selected_talent_ids
                .iter()
                .position(|selected| selected == &talent.id)
            {
                setup.selected_talent_ids.remove(position);
            } else {
                if setup.selected_talent_ids.len() >= 3 {
                    return Err(CommandError::validation("最多选择 3 个天赋"));
                }
                setup.selected_talent_ids.push(talent.id.clone());
            }
        }
    }

    setup.attribute_values = calculate_setup_attributes(&setup, content_bundle)?;
    let view = build_setup_view(&setup, content_bundle)?;
    Ok(SetupResponse { setup, view })
}

pub fn build_setup_view(
    setup: &RunSetupState,
    content_bundle: &ContentBundle,
) -> Result<SetupViewModel, CommandError> {
    let opening = setup_opening_rite_by_id(content_bundle, &setup.opening_rite_outcome_id)?;
    let attributes = setup_attribute_profile(content_bundle)?
        .attributes
        .iter()
        .map(|attribute| SetupAttributeView {
            id: attribute.id.clone(),
            label: attribute.label.clone(),
            summary: attribute.summary.clone(),
            value: *setup
                .attribute_values
                .get(&attribute.id)
                .unwrap_or(&attribute.base),
            min: attribute.min,
            max: attribute.max,
        })
        .collect::<Vec<_>>();

    let origin_candidates = content_bundle
        .origins
        .iter()
        .filter(|origin| setup_mode_permitted(&setup.mode, &origin.modes))
        .map(|origin| SetupCandidateView {
            id: origin.id.clone(),
            title: origin.title.clone(),
            summary: origin.summary.clone(),
            selected: setup.selected_origin_id.as_deref() == Some(origin.id.as_str()),
            enabled: true,
            disabled_reason: None,
            evidence: origin.evidence.clone(),
            modes: origin.modes.clone(),
        })
        .collect::<Vec<_>>();

    let talent_candidates = content_bundle
        .talents
        .iter()
        .filter(|talent| setup_talent_visible(&setup.mode, talent))
        .map(|talent| {
            let selected = setup.selected_talent_ids.iter().any(|id| id == &talent.id);
            let enabled = selected || setup.selected_talent_ids.len() < 3;
            SetupTalentCandidateView {
                id: talent.id.clone(),
                title: talent.title.clone(),
                summary: talent.summary.clone(),
                intensity: talent.intensity.clone(),
                selected,
                enabled,
                disabled_reason: if enabled {
                    None
                } else {
                    Some("最多选择 3 个天赋".to_string())
                },
                pressure_note: talent.pressure_note.clone(),
                route_tags: talent.route_tags.clone(),
                evidence: talent.evidence.clone(),
                modes: talent.modes.clone(),
            }
        })
        .collect::<Vec<_>>();

    let resource_preview = calculate_setup_resource_preview(setup, content_bundle)?;
    let confirm_blockers = setup_confirm_blockers(setup);
    let confirm_enabled = confirm_blockers.is_empty();

    Ok(SetupViewModel {
        mode: setup.mode.clone(),
        content_version: setup.content_version.clone(),
        origin_candidates,
        talent_candidates,
        attributes,
        resource_preview,
        selected_origin_id: setup.selected_origin_id.clone(),
        selected_talent_ids: setup.selected_talent_ids.clone(),
        opening_rite_outcome_id: opening.id.clone(),
        opening_rite_title: opening.title.clone(),
        opening_rite_summary: opening.summary.clone(),
        confirm_enabled,
        confirm_blockers,
    })
}

pub fn confirm_setup_run(
    mut setup: RunSetupState,
    content_bundle: &ContentBundle,
) -> Result<GameState, CommandError> {
    let blockers = setup_confirm_blockers(&setup);
    if !blockers.is_empty() {
        return Err(CommandError::validation(blockers.join("；")));
    }

    setup.attribute_values = calculate_setup_attributes(&setup, content_bundle)?;
    setup.completed = true;

    let origin = setup_origin_by_id(
        content_bundle,
        setup
            .selected_origin_id
            .as_deref()
            .ok_or_else(|| CommandError::validation("必须选择 1 个出身"))?,
    )?;
    let opening = setup_opening_rite_by_id(content_bundle, &setup.opening_rite_outcome_id)?;
    let resource_preview = calculate_setup_resource_preview(&setup, content_bundle)?;
    let talent_titles = setup
        .selected_talent_ids
        .iter()
        .map(|id| setup_talent_by_id(content_bundle, id).map(|talent| talent.title.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = create_run(setup.mode.clone(), setup.content_version.clone());
    state.resources = ResourceState {
        primeval_stones: resource_preview.primeval_stones,
        materials: resource_preview.materials,
        merit: resource_preview.merit,
    };
    state.debts_and_credit = DebtAndCreditState {
        infirmary_debt: resource_preview.infirmary_debt,
        favor_debt: resource_preview.favor_debt,
        organization_debt: resource_preview.organization_debt,
        trading_credit: resource_preview.trading_credit,
    };
    state.risk.exposure =
        resource_preview.exposure + origin.attention_delta + opening.attention_delta;
    state.character.aperture_opened = opening.aperture_opened;
    state.setup_summary = Some(SetupSummary {
        origin_id: origin.id.clone(),
        origin_title: origin.title.clone(),
        talent_ids: setup.selected_talent_ids.clone(),
        talent_titles,
        attributes: setup.attribute_values.clone(),
        opening_rite_outcome_id: opening.id.clone(),
        opening_rite_outcome_title: opening.title.clone(),
        resource_package_ids: resource_preview.resource_package_ids.clone(),
        attention_delta: origin.attention_delta + opening.attention_delta,
    });

    let talent_text = state
        .setup_summary
        .as_ref()
        .map(|summary| summary.talent_titles.join("、"))
        .unwrap_or_else(|| "无".to_string());
    state.ledger.insert(
        0,
        LedgerEntry {
            kind: "setup".to_string(),
            text: format!(
                "开窍大典落定：你以「{}」身份入账，天赋为「{}」，结果为「{}」。空窍已开，但资源、人情和关注度仍按账本结算。",
                origin.title, talent_text, opening.title
            ),
        },
    );

    Ok(state)
}

fn setup_confirm_blockers(setup: &RunSetupState) -> Vec<String> {
    let mut blockers = Vec::new();
    if setup.selected_origin_id.is_none() {
        blockers.push("需要选择 1 个出身".to_string());
    }
    if setup.selected_talent_ids.len() != 3 {
        blockers.push(format!(
            "需要选择 3 个天赋，当前已选 {} 个",
            setup.selected_talent_ids.len()
        ));
    }
    blockers
}

fn setup_attribute_profile(
    content_bundle: &ContentBundle,
) -> Result<&ContentAttributeProfile, CommandError> {
    content_bundle
        .attribute_profiles
        .first()
        .ok_or_else(|| CommandError::content("缺少设置层属性面板", "attribute_profiles is empty"))
}

fn setup_opening_rite(
    content_bundle: &ContentBundle,
) -> Result<&ContentOpeningRiteOutcome, CommandError> {
    content_bundle
        .opening_rite_outcomes
        .first()
        .ok_or_else(|| CommandError::content("缺少开窍大典结果", "opening_rite_outcomes is empty"))
}

fn setup_opening_rite_by_id<'a>(
    content_bundle: &'a ContentBundle,
    id: &str,
) -> Result<&'a ContentOpeningRiteOutcome, CommandError> {
    content_bundle
        .opening_rite_outcomes
        .iter()
        .find(|outcome| outcome.id == id)
        .ok_or_else(|| CommandError::validation(format!("未知开窍大典结果：{id}")))
}

fn setup_origin_by_id<'a>(
    content_bundle: &'a ContentBundle,
    id: &str,
) -> Result<&'a ContentOriginSpec, CommandError> {
    content_bundle
        .origins
        .iter()
        .find(|origin| origin.id == id)
        .ok_or_else(|| CommandError::validation(format!("未知出身：{id}")))
}

fn setup_talent_by_id<'a>(
    content_bundle: &'a ContentBundle,
    id: &str,
) -> Result<&'a ContentTalentSpec, CommandError> {
    content_bundle
        .talents
        .iter()
        .find(|talent| talent.id == id)
        .ok_or_else(|| CommandError::validation(format!("未知天赋：{id}")))
}

fn setup_resource_package_by_id<'a>(
    content_bundle: &'a ContentBundle,
    id: &str,
) -> Result<&'a ContentInitialResourcePackage, CommandError> {
    content_bundle
        .initial_resource_packages
        .iter()
        .find(|package| package.id == id)
        .ok_or_else(|| CommandError::validation(format!("未知初始资源包：{id}")))
}

fn base_attribute_values(profile: &ContentAttributeProfile) -> BTreeMap<String, i32> {
    profile
        .attributes
        .iter()
        .map(|attribute| (attribute.id.clone(), attribute.base))
        .collect()
}

fn calculate_setup_attributes(
    setup: &RunSetupState,
    content_bundle: &ContentBundle,
) -> Result<BTreeMap<String, i32>, CommandError> {
    let profile = setup_attribute_profile(content_bundle)?;
    let mut values = base_attribute_values(profile);

    if let Some(origin_id) = setup.selected_origin_id.as_deref() {
        let origin = setup_origin_by_id(content_bundle, origin_id)?;
        apply_attribute_modifiers(&mut values, &origin.attribute_modifiers);
    }

    for talent_id in &setup.selected_talent_ids {
        let talent = setup_talent_by_id(content_bundle, talent_id)?;
        require_setup_talent_allowed(&setup.mode, talent)?;
        apply_attribute_modifiers(&mut values, &talent.attribute_modifiers);
    }

    for attribute in &profile.attributes {
        if let Some(value) = values.get_mut(&attribute.id) {
            *value = (*value).clamp(attribute.min, attribute.max);
        }
    }

    Ok(values)
}

fn apply_attribute_modifiers(
    values: &mut BTreeMap<String, i32>,
    modifiers: &BTreeMap<String, i32>,
) {
    for (attribute_id, delta) in modifiers {
        *values.entry(attribute_id.clone()).or_insert(0) += delta;
    }
}

fn calculate_setup_resource_preview(
    setup: &RunSetupState,
    content_bundle: &ContentBundle,
) -> Result<SetupResourcePreview, CommandError> {
    let mut preview = SetupResourcePreview::default();
    let mut package_ids = Vec::<String>::new();

    if let Some(origin_id) = setup.selected_origin_id.as_deref() {
        package_ids.push(
            setup_origin_by_id(content_bundle, origin_id)?
                .resource_package_id
                .clone(),
        );
    }
    package_ids.push(
        setup_opening_rite_by_id(content_bundle, &setup.opening_rite_outcome_id)?
            .resource_package_id
            .clone(),
    );

    for package_id in package_ids {
        if preview
            .resource_package_ids
            .iter()
            .any(|id| id == &package_id)
        {
            continue;
        }
        let package = setup_resource_package_by_id(content_bundle, &package_id)?;
        preview.primeval_stones += package.primeval_stones;
        preview.materials += package.materials;
        preview.merit += package.merit;
        preview.infirmary_debt += package.infirmary_debt;
        preview.favor_debt += package.favor_debt;
        preview.organization_debt += package.organization_debt;
        preview.trading_credit += package.trading_credit;
        preview.exposure += package.exposure;
        preview.resource_package_ids.push(package_id);
    }

    Ok(preview)
}

fn setup_talent_visible(mode: &RunMode, talent: &ContentTalentSpec) -> bool {
    setup_mode_permitted(mode, &talent.modes)
        && (*mode != RunMode::CanonStrict || talent.intensity == TalentIntensity::Mild)
}

fn require_setup_talent_allowed(
    mode: &RunMode,
    talent: &ContentTalentSpec,
) -> Result<(), CommandError> {
    require_mode(mode, &talent.modes, "talent", &talent.id)?;
    if *mode == RunMode::CanonStrict && talent.intensity != TalentIntensity::Mild {
        return Err(CommandError::validation(format!(
            "严谨模式不能选择强 IF 天赋：{}",
            talent.title
        )));
    }
    Ok(())
}

fn setup_mode_permitted(mode: &RunMode, modes: &[ModePermit]) -> bool {
    match mode {
        RunMode::CanonStrict => modes.contains(&ModePermit::CanonStrict),
        RunMode::SandboxIf => modes.contains(&ModePermit::SandboxIf),
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
        debts_and_credit: DebtAndCreditState::default(),
        risk: RiskState::default(),
        character: CharacterState::opened_aperture(),
        knowledge: KnowledgeState::default(),
        encounters: EncounterState::default(),
        build: BuildState::default(),
        ledger: vec![LedgerEntry {
            kind: "scene".to_string(),
            text: "开窍大典的余声刚落，你站在学堂门前，清晨山雾压着木檐；空窍已开，真正的家族秩序才刚开始记账。".to_string(),
        }],
        setup_summary: None,
    }
}

pub fn build_projection(state: &GameState) -> LedgerViewModel {
    let content_bundle = starter_content_bundle();
    build_projection_from_content(state, &content_bundle)
}

pub fn build_projection_with_content(
    state: &GameState,
    content_bundle: &ContentBundle,
) -> LedgerViewModel {
    build_projection_from_content(state, content_bundle)
}

fn build_projection_from_content(
    state: &GameState,
    content_bundle: &ContentBundle,
) -> LedgerViewModel {
    let active_encounter = state.encounters.active.as_ref();

    LedgerViewModel {
        scene_text: state
            .ledger
            .last()
            .map(clean_ledger_text)
            .unwrap_or_else(|| "账本空白，局势尚未落笔。".to_string()),
        current_day: state.time.chapter_day,
        current_period: display_period(&state.time.period),
        window_id: state.time.window_id.clone(),
        window_type: state.time.window_type.clone(),
        available_ap: state.time.ap,
        next_anchor_pressure: clean_anchor_pressure(&state.time.next_anchor_pressure),
        current_node_id: state.world.current_node_id.clone(),
        primeval_stones: state.resources.primeval_stones,
        materials: state.resources.materials,
        merit: state.resources.merit,
        exposure: state.risk.exposure,
        debt_pressure: state.debts_and_credit.pressure(),
        build_summary: state.build.survival_route.clone(),
        status_markers: status_markers(state, active_encounter, content_bundle),
        build_view: build_view(state),
        relationship_view: relationship_view(state),
        save_view: save_view(state),
        action_choices: projected_action_choices(state, content_bundle),
        node_view: node_view(state, content_bundle),
        injury_level: state.character.injury.level.clone(),
        active_encounter_id: active_encounter.map(|encounter| encounter.encounter_id.clone()),
        active_encounter_type: active_encounter.map(|encounter| encounter.encounter_type.clone()),
        active_encounter_known_risk: active_encounter.map(|encounter| encounter.known_risk.clone()),
        active_encounter_decisions: active_encounter
            .map(|encounter| encounter.decision_intents.clone())
            .unwrap_or_default(),
        ledger_entries: state
            .ledger
            .iter()
            .map(|entry| LedgerEntry {
                kind: entry.kind.clone(),
                text: clean_ledger_text(entry),
            })
            .collect(),
        recent_feedback: recent_feedback_view(state),
        clue_view: clue_ledger_view(state),
        narrative_boundary: narrative_boundary_view(),
        stage_closure: stage_closure_view(state),
        performance: PerformanceMetrics::default(),
    }
}

fn clean_ledger_text(entry: &LedgerEntry) -> String {
    entry.text.clone()
}

fn recent_feedback_view(state: &GameState) -> Option<RecentFeedbackView> {
    state.ledger.last().map(|entry| RecentFeedbackView {
        title: "最近落账".to_string(),
        summary: clean_ledger_text(entry),
        tone: feedback_tone(&entry.kind),
        source_kind: entry.kind.clone(),
    })
}

fn feedback_tone(kind: &str) -> ActionChoiceTone {
    if kind.contains("encounter") || kind.contains("injury") || kind.contains("confront") {
        ActionChoiceTone::Danger
    } else if kind.contains("recover") || kind.contains("trade") || kind.contains("debt") {
        ActionChoiceTone::Risky
    } else if kind.contains("scout") || kind.contains("clue") || kind.contains("test") {
        ActionChoiceTone::Safe
    } else {
        ActionChoiceTone::Normal
    }
}

fn clue_ledger_view(state: &GameState) -> ClueLedgerView {
    ClueLedgerView {
        known_clues: state
            .knowledge
            .known_clues
            .iter()
            .map(|clue_id| clue_line_view(clue_id))
            .collect(),
        blackmarket_access_summary: if state.knowledge.blackmarket_route_known {
            "黑市门路：已记下暗口风声；仍受时段、AP 与暴露约束。".to_string()
        } else {
            "黑市门路：未解锁；未知门路不会显示成可选行动。".to_string()
        },
    }
}

fn clue_line_view(clue_id: &str) -> ClueLineView {
    let (label, summary, tone) = match clue_id {
        "rumor_blackmarket_tail" => (
            "黑市尾巴",
            "有人绕过学堂秩序谈换料，但门路本身会引来尾巴。",
            ActionChoiceTone::Risky,
        ),
        "rumor_academy_pressure" => (
            "学堂压力",
            "月光修行的比较会把你推到众人眼前，提前知道可减轻冲突代价。",
            ActionChoiceTone::Risky,
        ),
        "rumor_merit_audit" => (
            "功绩审计",
            "功绩账不是白拿的资源，稳健积累也会留下审计痕迹。",
            ActionChoiceTone::Normal,
        ),
        "rumor_infirmary_debt" => (
            "药堂债价",
            "恢复可以救命，但药堂债和人情债会追到后续窗口。",
            ActionChoiceTone::Risky,
        ),
        "rumor_family_debt" => (
            "旁支债声",
            "旁支落脚点不等于安全屋，家族秩序会把债声记在你身上。",
            ActionChoiceTone::Risky,
        ),
        "rumor_inheritance_bamboo" => (
            "传承竹影",
            "传承残线半真半假，诱惑大，风险也不会替你兜底。",
            ActionChoiceTone::Danger,
        ),
        "rumor_alley_probe" => (
            "巷道试探",
            "巷道里有人试探来路；提前侦查能降低拖延的暴露代价。",
            ActionChoiceTone::Risky,
        ),
        _ => (
            "未整理风声",
            "账本只确认你听到过这条风声，细节仍待验真。",
            ActionChoiceTone::Normal,
        ),
    };

    ClueLineView {
        id: clue_id.to_string(),
        label: label.to_string(),
        summary: summary.to_string(),
        tone,
    }
}

fn display_period(period: &str) -> String {
    period.to_string()
}

fn period_matches(required: &str, current: &str) -> bool {
    display_period(required) == display_period(current)
}

fn clean_anchor_pressure(value: &str) -> String {
    value.to_string()
}

fn narrative_boundary_view() -> NarrativeBoundaryView {
    NarrativeBoundaryView {
        runtime_ai_enabled: false,
        source: "内容包 + 因果账本 + Rust 本地兜底".to_string(),
        policy: "resolve_action、Tauri command 与 UI 点击链路不等待远程 AI".to_string(),
    }
}

fn stage_closure_view(state: &GameState) -> StageClosureView {
    if state.time.window_type != WindowType::Anchor {
        return StageClosureView {
            status: StageClosureStatus::InProgress,
            title: "尚未收口".to_string(),
            summary: format!(
                "自由窗口已过 {}/8，阶段锚点尚未落下。",
                state.time.free_rounds_elapsed
            ),
        };
    }

    if state.character.injury.level == InjuryLevel::Heavy {
        return StageClosureView {
            status: StageClosureStatus::TraumaContinuable,
            title: "重创可续".to_string(),
            summary: "重创可续：你带着重伤抵达阶段锚点，局面没有清账，只是还能续命。".to_string(),
        };
    }

    if state.build.moonlight_cultivation_marks >= 2 {
        return StageClosureView {
            status: StageClosureStatus::FoundationEstablished,
            title: "站稳一转根基".to_string(),
            summary: "月光修行留下足够痕迹，资源、债务和风险仍在，但一转根基已经站住。".to_string(),
        };
    }

    StageClosureView {
        status: StageClosureStatus::InProgress,
        title: "锚点待判".to_string(),
        summary: "自由窗口已经耗尽，但当前根基不足以记为阶段成功。".to_string(),
    }
}

fn save_view(state: &GameState) -> SaveLedgerView {
    let checkpoints = save_checkpoints_for_state(state);
    let current_checkpoint_id = checkpoints
        .iter()
        .find(|checkpoint| checkpoint.kind == SaveCheckpointKind::CurrentSnapshot)
        .map(|checkpoint| checkpoint.checkpoint_id.clone())
        .unwrap_or_else(|| "sprint_0_current".to_string());
    let stage_checkpoint_ids = checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.kind == SaveCheckpointKind::StageBoundary)
        .map(|checkpoint| checkpoint.checkpoint_id.clone())
        .collect::<Vec<_>>();

    SaveLedgerView {
        save_version: SAVE_FORMAT_VERSION.to_string(),
        rules_version: RULES_VERSION.to_string(),
        content_version: state.content_version.clone(),
        rng_state: DEFAULT_RNG_STATE.to_string(),
        migration_state: DEFAULT_MIGRATION_STATE.to_string(),
        checkpoint_count: checkpoints.len(),
        current_checkpoint_id,
        stage_checkpoint_ids,
        rollback_policy: "阶段检查点只保留阶段边界与当前快照，不提供每个选择无限回退。".to_string(),
    }
}

fn status_markers(
    state: &GameState,
    active_encounter: Option<&ActiveEncounter>,
    content_bundle: &ContentBundle,
) -> Vec<StatusMarkerView> {
    vec![
        marker("时段", &display_period(&state.time.period), "normal"),
        marker("窗口", &format!("{:?}", state.time.window_type), "normal"),
        marker(
            "AP",
            &state.time.ap.to_string(),
            if state.time.ap == 0 {
                "danger"
            } else {
                "normal"
            },
        ),
        marker(
            "地点",
            &node_title_by_id(&state.world.current_node_id, content_bundle),
            "normal",
        ),
        marker(
            "空窍",
            if state.character.aperture_opened {
                "已开窍"
            } else {
                "未开窍"
            },
            if state.character.aperture_opened {
                "normal"
            } else {
                "danger"
            },
        ),
        marker(
            "暴露",
            &state.risk.exposure.to_string(),
            pressure_tone(state.risk.exposure),
        ),
        marker(
            "债务",
            &state.debts_and_credit.pressure().to_string(),
            pressure_tone(state.debts_and_credit.pressure()),
        ),
        marker(
            "伤势",
            injury_label(&state.character.injury.level),
            injury_tone(&state.character.injury.level),
        ),
        marker(
            "遭遇",
            active_encounter
                .map(|encounter| encounter.encounter_id.as_str())
                .unwrap_or("无"),
            if active_encounter.is_some() {
                "danger"
            } else {
                "normal"
            },
        ),
    ]
}

fn marker(label: &str, value: &str, tone: &str) -> StatusMarkerView {
    StatusMarkerView {
        label: label.to_string(),
        value: value.to_string(),
        tone: tone.to_string(),
    }
}

fn build_view(state: &GameState) -> BuildLedgerView {
    BuildLedgerView {
        survival_route: state.build.survival_route.clone(),
        main_path: state
            .build
            .main_path
            .clone()
            .unwrap_or_else(|| "主修流派：未定".to_string()),
        dao_mark_note: state
            .build
            .dao_mark_note
            .clone()
            .unwrap_or_else(|| "道痕：凡人期保留位".to_string()),
        core_gu: gu_slot_display("核心蛊", &state.build.core_gu),
        support_gu: gu_slot_display("辅助蛊", &state.build.support_gu),
        vital_gu: vital_gu_display(&state.build.vital_gu),
        maintenance_pressure: state.build.maintenance_pressure.clone(),
        gap_summary: state.build.gap_summary.clone(),
    }
}

fn gu_slot_display(label: &str, slot: &GuSlotState) -> String {
    format!("{label}：{}", slot.display_name)
}

fn vital_gu_display(vital: &VitalGuState) -> String {
    match vital.status {
        VitalGuStatus::NotEstablished => "本命蛊：未建立".to_string(),
        VitalGuStatus::Established => vital
            .instance_id
            .as_ref()
            .map(|id| format!("本命蛊：已绑定 {id}"))
            .unwrap_or_else(|| "本命蛊：已建立，实例未登记".to_string()),
    }
}

fn relationship_view(state: &GameState) -> FactionRelationshipView {
    FactionRelationshipView {
        family_pressure: format!("家族秩序：{}", family_pressure_label(state.risk.exposure)),
        infirmary_debt: format!("药堂债：{}", state.debts_and_credit.infirmary_debt),
        favor_debt: format!("人情债：{}", state.debts_and_credit.favor_debt),
        blackmarket_access: if state.knowledge.blackmarket_route_known {
            "黑市门路：已听到暗口风声".to_string()
        } else {
            "黑市门路：未解锁".to_string()
        },
    }
}

fn family_pressure_label(exposure: i32) -> &'static str {
    if exposure >= 5 {
        "高压盯防"
    } else if exposure >= 2 {
        "有人留意"
    } else {
        "低压监视"
    }
}

fn projected_action_choices(
    state: &GameState,
    content_bundle: &ContentBundle,
) -> Vec<ActionChoiceView> {
    let mut choices = content_bundle
        .actions
        .iter()
        .filter(|action| mode_permitted(&state.mode, &action.modes))
        .filter(|action| action_is_projectable(state, action))
        .map(|action| {
            let command = ActionCommand {
                actor: "player".to_string(),
                intent: action.intent.clone(),
                target: action.target.clone(),
                declared_cost: DeclaredCost::default(),
                context_note: None,
            };
            action_choice_from_command(
                &action.id,
                clean_action_label(action),
                command,
                state,
                content_bundle,
            )
        })
        .collect::<Vec<_>>();

    let wait = ActionCommand {
        actor: "player".to_string(),
        intent: ActionIntent::Wait,
        target: None,
        declared_cost: DeclaredCost::default(),
        context_note: None,
    };
    choices.push(action_choice_from_command(
        "wait_current_window",
        "等过当前时段".to_string(),
        wait,
        state,
        content_bundle,
    ));

    choices
}

fn action_choice_from_command(
    id: &str,
    label: String,
    command: ActionCommand,
    state: &GameState,
    content_bundle: &ContentBundle,
) -> ActionChoiceView {
    let check = availability_check(state, &command, content_bundle)
        .and_then(|_| cost_reservation(state, &command, content_bundle).map(|_| ()));
    let (enabled, disabled_reason) = match check {
        Ok(()) => (true, None),
        Err(error) => (false, Some(display_disabled_reason(&error))),
    };
    let group = action_choice_group(&command.intent);
    let tone = action_choice_tone(&command, enabled);
    let consequence_hint = consequence_hint(id, &command.intent, command.target.as_deref());

    ActionChoiceView {
        id: id.to_string(),
        label,
        intent: command.intent.clone(),
        target: command.target,
        enabled,
        disabled_reason,
        cost_hint: cost_hint(&command.intent),
        risk_hint: risk_hint(&command.intent),
        group,
        tone,
        consequence_hint,
    }
}

fn node_view(state: &GameState, content_bundle: &ContentBundle) -> NodeLedgerView {
    NodeLedgerView {
        current_node_id: state.world.current_node_id.clone(),
        current_region_id: state.world.current_region_id.clone(),
        visible_nodes: content_bundle
            .nodes
            .iter()
            .filter(|node| mode_permitted(&state.mode, &node.modes))
            .filter(|node| {
                !is_blackmarket_tagged(&node.tags) || state.knowledge.blackmarket_route_known
            })
            .map(|node| NodeSummaryView {
                id: node.id.clone(),
                title: clean_node_title(node),
                safety: clean_safety(&node.safety),
                current: node.id == state.world.current_node_id,
            })
            .collect(),
    }
}

fn action_is_projectable(state: &GameState, action: &ContentAction) -> bool {
    if is_encounter_decision_intent(&action.intent) {
        return state
            .encounters
            .active
            .as_ref()
            .is_some_and(|active| action.target.as_deref() == Some(active.encounter_id.as_str()));
    }

    if is_blackmarket_tagged(&action.tags) && !state.knowledge.blackmarket_route_known {
        return false;
    }

    if action.intent == ActionIntent::Move {
        if action
            .target
            .as_deref()
            .is_some_and(|target| target == state.world.current_node_id)
        {
            return false;
        }
        return true;
    }

    if action_requires_current_node(&action.intent) {
        return action
            .target
            .as_deref()
            .is_some_and(|target| target == state.world.current_node_id);
    }

    true
}

fn is_encounter_decision_intent(intent: &ActionIntent) -> bool {
    matches!(
        intent,
        ActionIntent::Retreat
            | ActionIntent::Confront
            | ActionIntent::Yield
            | ActionIntent::Argue
            | ActionIntent::Delay
            | ActionIntent::Frame
    )
}

fn is_blackmarket_tagged(tags: &[String]) -> bool {
    tags.iter().any(|tag| tag == "blackmarket")
}

fn mode_permitted(mode: &RunMode, modes: &[ModePermit]) -> bool {
    match mode {
        RunMode::CanonStrict => modes.contains(&ModePermit::CanonStrict),
        RunMode::SandboxIf => modes.contains(&ModePermit::SandboxIf),
    }
}

fn action_choice_group(intent: &ActionIntent) -> ActionChoiceGroup {
    match intent {
        ActionIntent::Move => ActionChoiceGroup::Movement,
        ActionIntent::Cultivate => ActionChoiceGroup::Cultivation,
        ActionIntent::Scout => ActionChoiceGroup::Information,
        ActionIntent::Recover => ActionChoiceGroup::Recovery,
        ActionIntent::Trade => ActionChoiceGroup::Trade,
        ActionIntent::Retreat
        | ActionIntent::Confront
        | ActionIntent::Yield
        | ActionIntent::Argue
        | ActionIntent::Delay
        | ActionIntent::Frame => ActionChoiceGroup::Encounter,
        ActionIntent::Wait => ActionChoiceGroup::Wait,
    }
}

fn action_choice_tone(command: &ActionCommand, enabled: bool) -> ActionChoiceTone {
    if !enabled {
        return ActionChoiceTone::Blocked;
    }

    match command.intent {
        ActionIntent::Scout | ActionIntent::Retreat => ActionChoiceTone::Safe,
        ActionIntent::Move => match command.target.as_deref() {
            Some("blackmarket_hint") | Some("inheritance_rumor") => ActionChoiceTone::Risky,
            _ => ActionChoiceTone::Normal,
        },
        ActionIntent::Recover
        | ActionIntent::Trade
        | ActionIntent::Yield
        | ActionIntent::Argue
        | ActionIntent::Delay => ActionChoiceTone::Risky,
        ActionIntent::Confront | ActionIntent::Frame => ActionChoiceTone::Danger,
        ActionIntent::Cultivate | ActionIntent::Wait => ActionChoiceTone::Normal,
    }
}

fn consequence_hint(id: &str, intent: &ActionIntent, target: Option<&str>) -> String {
    match intent {
        ActionIntent::Move => match target {
            Some("moonlight_corner") => "换到月光修行角，可能引出学堂压力".to_string(),
            Some("merit_notice") => "转向功绩告示，获得制度内线索".to_string(),
            Some("infirmary_lane") => "靠近药堂，恢复机会伴随债务".to_string(),
            Some("branch_lodging") => "回到旁支落脚点，听见家族债声".to_string(),
            Some("clan_alley_rumor") => "进入山寨巷道，风声更杂也更危险".to_string(),
            Some("blackmarket_hint") => "摸向黑市暗口，暴露和遭遇风险上升".to_string(),
            Some("inheritance_rumor") => "追索传承残线，高风险且只在 IF 路线成立".to_string(),
            _ => "更换节点，路径风险按 Rust 规则结算".to_string(),
        },
        ActionIntent::Cultivate => "推进月光修行痕迹，消耗元石与窗口".to_string(),
        ActionIntent::Scout => match target {
            Some("academy_gate") => "记录学堂风声与黑市尾巴线索".to_string(),
            Some("moonlight_corner") => "记录月光角压力线索".to_string(),
            Some("merit_notice") => "记录功绩审计线索".to_string(),
            Some("infirmary_lane") => "记录药堂债价线索".to_string(),
            Some("branch_lodging") => "记录旁支家族债声".to_string(),
            Some("clan_alley_rumor") => "记录巷道试探与黑市尾巴线索".to_string(),
            Some("inheritance_rumor") => "记录传承残线的半真半假痕迹".to_string(),
            _ => "换取局部真实情报".to_string(),
        },
        ActionIntent::Recover => "减轻伤势，同时增加药堂债与人情债".to_string(),
        ActionIntent::Trade => "换取材料，暴露上升".to_string(),
        ActionIntent::Retreat => "保命优先，少量暴露后脱离遭遇".to_string(),
        ActionIntent::Confront => {
            if id.contains("academy") {
                "硬撑压力，可能换来轻伤与更高暴露".to_string()
            } else {
                "硬顶风险，可能进入重创可续".to_string()
            }
        }
        ActionIntent::Yield => "忍让压低冲突，保留行动余地".to_string(),
        ActionIntent::Argue => "争辩换取余地，暴露会上升".to_string(),
        ActionIntent::Delay => "拖延争取缝隙，消耗窗口".to_string(),
        ActionIntent::Frame => "嫁祸脱身，短期解围但后患更深".to_string(),
        ActionIntent::Wait => "放弃剩余安排，推进到下一窗口".to_string(),
    }
}

fn clean_action_label(action: &ContentAction) -> String {
    if !action.id.is_empty() {
        return match action.id.as_str() {
            "scout_academy" => "观察学堂风声",
            "cultivate_moonlight" => "月光修行",
            "cultivate_moonlight_corner" => "借月光角修行",
            "move_moonlight_corner" => "去月光角",
            "observe_moonlight_pressure" => "观察月光角压力",
            "move_merit_notice" => "去功绩告示",
            "check_merit_notice" => "查功绩告示",
            "audit_merit_notice" => "核对功绩审计",
            "move_infirmary_lane" => "去药堂侧巷",
            "seek_treatment_debt" => "药堂恢复",
            "ask_infirmary_price" => "打听药堂价码",
            "move_branch_lodging" => "回旁支落脚点",
            "listen_branch_lodging_debt" => "听旁支债声",
            "move_clan_alley_rumor" => "绕到山寨巷道",
            "listen_clan_alley_rumor" => "听巷道风声",
            "move_blackmarket_hint" => "摸黑市暗口",
            "probe_blackmarket_hint" => "黑市换料",
            "retreat_blackmarket_extortion" => "跑路",
            "confront_blackmarket_extortion" => "硬顶",
            "yield_academy_public_pressure" => "忍让",
            "argue_academy_public_pressure" => "争辩",
            "confront_academy_public_pressure" => "硬撑",
            "retreat_alley_probe" => "退走",
            "delay_alley_probe" => "拖延",
            "frame_alley_probe" => "嫁祸",
            "confront_alley_probe" => "硬顶",
            "chase_inheritance_rumor" => "追传承残线",
            "verify_inheritance_rumor" => "查验传承残线",
            _ => action.label.as_str(),
        }
        .to_string();
    }

    match action.id.as_str() {
        "scout_academy" => "观察学堂风声",
        "cultivate_moonlight" => "月光修行",
        "cultivate_moonlight_corner" => "借月光角修行",
        "move_moonlight_corner" => "去月光角",
        "observe_moonlight_pressure" => "观察月光角压力",
        "move_merit_notice" => "去功绩告示",
        "check_merit_notice" => "查功绩告示",
        "audit_merit_notice" => "核对功绩审计",
        "move_infirmary_lane" => "去药堂侧巷",
        "seek_treatment_debt" => "药堂恢复",
        "ask_infirmary_price" => "打听药堂价码",
        "move_branch_lodging" => "回旁支落脚点",
        "listen_branch_lodging_debt" => "听旁支债声",
        "move_clan_alley_rumor" => "绕到山寨巷道",
        "listen_clan_alley_rumor" => "听巷道风声",
        "move_blackmarket_hint" => "摸黑市暗口",
        "probe_blackmarket_hint" => "黑市换料",
        "retreat_blackmarket_extortion" => "跑路",
        "confront_blackmarket_extortion" => "硬顶",
        "yield_academy_public_pressure" => "忍让",
        "argue_academy_public_pressure" => "争辩",
        "confront_academy_public_pressure" => "硬撑",
        "retreat_alley_probe" => "退走",
        "delay_alley_probe" => "拖延",
        "frame_alley_probe" => "嫁祸",
        "confront_alley_probe" => "硬顶",
        "chase_inheritance_rumor" => "追传承残线",
        "verify_inheritance_rumor" => "查验传承残线",
        _ => action.label.as_str(),
    }
    .to_string()
}

fn clean_node_title(node: &ContentNode) -> String {
    if !node.id.is_empty() {
        return match node.id.as_str() {
            "academy_gate" => "学堂门前",
            "moonlight_corner" => "月光修行角",
            "merit_notice" => "功绩告示处",
            "infirmary_lane" => "药堂侧巷",
            "branch_lodging" => "旁支落脚点",
            "clan_alley_rumor" => "山寨巷道风声点",
            "blackmarket_hint" => "黑市暗口",
            "inheritance_rumor" => "传承残线",
            _ => node.title.as_str(),
        }
        .to_string();
    }

    match node.id.as_str() {
        "academy_gate" => "学堂门前",
        "moonlight_corner" => "月光修行角",
        "merit_notice" => "功绩告示旁",
        "infirmary_lane" => "药堂侧巷",
        "branch_lodging" => "旁支落脚点",
        "clan_alley_rumor" => "山寨巷道风声点",
        "blackmarket_hint" => "黑市暗口",
        "inheritance_rumor" => "传承残线",
        _ => node.title.as_str(),
    }
    .to_string()
}

fn node_title_by_id(node_id: &str, content_bundle: &ContentBundle) -> String {
    content_bundle
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .map(clean_node_title)
        .unwrap_or_else(|| node_id.to_string())
}

fn clean_safety(safety: &str) -> String {
    if !safety.is_empty() {
        return match safety {
            "low" => "低",
            "medium" => "中",
            "high" => "高",
            other => other,
        }
        .to_string();
    }

    match safety {
        "low" => "低",
        "medium" => "中",
        "high" => "高",
        other => other,
    }
    .to_string()
}

fn display_disabled_reason(error: &CommandError) -> String {
    if !error.message.is_empty() {
        return if error.message.contains("active encounter") {
            "遭遇未处理，普通行动暂不可用".to_string()
        } else if error.message.contains("aperture is not opened") {
            "空窍未开，不能修行".to_string()
        } else if error.message.contains("requires period") {
            "时段不合，当前不可达".to_string()
        } else if error.message.contains("primeval stones not enough") {
            "元石不足".to_string()
        } else if error.message.contains("AP not enough") {
            "AP 不足，当前窗口已被压尽".to_string()
        } else if error.message.contains("recover requires") {
            "需要先到药堂侧巷".to_string()
        } else if error.message.contains("blackmarket trade requires") {
            "需要在深夜抵达黑市暗口".to_string()
        } else if error.message.contains("requires an active encounter") {
            "当前没有可处理的遭遇".to_string()
        } else {
            error.message.clone()
        };
    }

    if error.message.contains("active encounter") {
        "遭遇未处理，普通行动暂不可用".to_string()
    } else if error.message.contains("aperture is not opened") {
        "空窍未开，不能修行".to_string()
    } else if error.message.contains("requires period") {
        "时段不合，当前不可达".to_string()
    } else if error.message.contains("primeval stones not enough") {
        "元石不足".to_string()
    } else if error.message.contains("AP not enough") {
        "AP 不足".to_string()
    } else if error.message.contains("recover requires") {
        "需要先到药堂侧巷".to_string()
    } else if error.message.contains("blackmarket trade requires") {
        "需要在深夜抵达黑市暗口".to_string()
    } else if error.message.contains("requires an active encounter") {
        "当前没有可处理的遭遇".to_string()
    } else {
        error.message.clone()
    }
}

fn cost_hint(intent: &ActionIntent) -> String {
    if matches!(
        intent,
        ActionIntent::Move
            | ActionIntent::Cultivate
            | ActionIntent::Scout
            | ActionIntent::Recover
            | ActionIntent::Trade
            | ActionIntent::Retreat
            | ActionIntent::Confront
            | ActionIntent::Yield
            | ActionIntent::Argue
            | ActionIntent::Delay
            | ActionIntent::Frame
            | ActionIntent::Wait
    ) {
        return match intent {
            ActionIntent::Move => "按路径结算",
            ActionIntent::Cultivate => "1 AP / 1 元石",
            ActionIntent::Scout => "1 AP",
            ActionIntent::Recover => "1-2 AP / 药堂债",
            ActionIntent::Trade => "1 AP / 1 元石",
            ActionIntent::Retreat => "1 AP",
            ActionIntent::Confront => "1 AP / 1 元石",
            ActionIntent::Yield => "1 AP",
            ActionIntent::Argue => "1 AP",
            ActionIntent::Delay => "1 AP",
            ActionIntent::Frame => "1 AP",
            ActionIntent::Wait => "吃掉当前窗口",
        }
        .to_string();
    }

    match intent {
        ActionIntent::Move => "按路径结算",
        ActionIntent::Cultivate => "1 AP / 1 元石",
        ActionIntent::Scout => "1 AP",
        ActionIntent::Recover => "1-2 AP / 药堂债",
        ActionIntent::Trade => "1 AP / 1 元石",
        ActionIntent::Retreat => "1 AP",
        ActionIntent::Confront => "1 AP / 1 元石",
        ActionIntent::Yield => "1 AP",
        ActionIntent::Argue => "1 AP",
        ActionIntent::Delay => "1 AP",
        ActionIntent::Frame => "1 AP",
        ActionIntent::Wait => "吃掉当前窗口",
    }
    .to_string()
}

fn risk_hint(intent: &ActionIntent) -> String {
    if matches!(
        intent,
        ActionIntent::Move
            | ActionIntent::Cultivate
            | ActionIntent::Scout
            | ActionIntent::Recover
            | ActionIntent::Trade
            | ActionIntent::Retreat
            | ActionIntent::Confront
            | ActionIntent::Yield
            | ActionIntent::Argue
            | ActionIntent::Delay
            | ActionIntent::Frame
            | ActionIntent::Wait
    ) {
        return match intent {
            ActionIntent::Move => "移动风险随路径变化",
            ActionIntent::Cultivate => "资源压力",
            ActionIntent::Scout => "低风险，换取情报",
            ActionIntent::Recover => "债务与人情",
            ActionIntent::Trade => "暴露上升",
            ActionIntent::Retreat => "少量暴露，保命优先",
            ActionIntent::Confront => "重创可续，高暴露",
            ActionIntent::Yield => "压低冲突，脸面受损",
            ActionIntent::Argue => "暴露上升，换取余地",
            ActionIntent::Delay => "拖出缝隙，消耗窗口",
            ActionIntent::Frame => "嫁祸脱身，后患更深",
            ActionIntent::Wait => "错过窗口",
        }
        .to_string();
    }

    match intent {
        ActionIntent::Move => "移动风险随路径变化",
        ActionIntent::Cultivate => "资源压力",
        ActionIntent::Scout => "低风险，换取情报",
        ActionIntent::Recover => "债务与人情",
        ActionIntent::Trade => "暴露上升",
        ActionIntent::Retreat => "少量暴露，保命优先",
        ActionIntent::Confront => "重创可续，高暴露",
        ActionIntent::Yield => "压低冲突，脸面受损",
        ActionIntent::Argue => "暴露上升，换取余地",
        ActionIntent::Delay => "拖出缝隙，消耗窗口",
        ActionIntent::Frame => "嫁祸脱身，后患更深",
        ActionIntent::Wait => "错过窗口",
    }
    .to_string()
}

fn injury_label(level: &InjuryLevel) -> &'static str {
    match level {
        InjuryLevel::Healthy => "健康",
        InjuryLevel::Light => "轻伤",
        InjuryLevel::Heavy => "重伤",
    }
}

fn injury_tone(level: &InjuryLevel) -> &'static str {
    match level {
        InjuryLevel::Healthy => "normal",
        InjuryLevel::Light => "warn",
        InjuryLevel::Heavy => "danger",
    }
}

fn pressure_tone(value: i32) -> &'static str {
    if value >= 4 {
        "danger"
    } else if value > 0 {
        "warn"
    } else {
        "normal"
    }
}

pub fn resolve_action(
    mut state: GameState,
    command: ActionCommand,
    content_bundle: &ContentBundle,
) -> Result<ActionResult, CommandError> {
    let started = Instant::now();
    let mut pipeline_trace = Vec::with_capacity(7);

    availability_check(&state, &command, content_bundle)?;
    pipeline_trace.push(PipelineStep::AvailabilityCheck);

    let reserved_cost = cost_reservation(&state, &command, content_bundle)?;
    pipeline_trace.push(PipelineStep::CostReservation);

    let outcome = subsystem_resolution(&state, &command, content_bundle)?;
    pipeline_trace.push(PipelineStep::SubsystemResolution);

    anchor_recalculation(&mut state, &outcome);
    pipeline_trace.push(PipelineStep::AnchorRecalculation);

    effect_commit(&mut state, reserved_cost, &outcome, content_bundle);
    pipeline_trace.push(PipelineStep::EffectCommit);

    ledger_append(&mut state, &outcome, content_bundle);
    pipeline_trace.push(PipelineStep::LedgerAppend);

    let projection_started = Instant::now();
    let mut projection = build_projection_with_content(&state, content_bundle);
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
    consume_window: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SubsystemOutcome {
    ledger_kind: String,
    ledger_text: String,
    narrative_id: Option<String>,
    target_node_id: Option<String>,
    survival_route: Option<String>,
    materials_delta: i32,
    merit_delta: i32,
    infirmary_debt_delta: i32,
    favor_debt_delta: i32,
    organization_debt_delta: i32,
    trading_credit_delta: i32,
    exposure_delta: i32,
    moonlight_cultivation_delta: u8,
    arrival_ap_penalty: u8,
    trigger_encounter: Option<ActiveEncounter>,
    clear_active_encounter: bool,
    injury_level: Option<InjuryLevel>,
    injury_ap_penalty_pending: Option<bool>,
    reveal_blackmarket_route: bool,
    clue_ids: Vec<String>,
}

impl SubsystemOutcome {
    fn new(kind: &str, text: impl Into<String>) -> Self {
        Self {
            ledger_kind: kind.to_string(),
            ledger_text: text.into(),
            narrative_id: None,
            target_node_id: None,
            survival_route: None,
            materials_delta: 0,
            merit_delta: 0,
            infirmary_debt_delta: 0,
            favor_debt_delta: 0,
            organization_debt_delta: 0,
            trading_credit_delta: 0,
            exposure_delta: 0,
            moonlight_cultivation_delta: 0,
            arrival_ap_penalty: 0,
            trigger_encounter: None,
            clear_active_encounter: false,
            injury_level: None,
            injury_ap_penalty_pending: None,
            reveal_blackmarket_route: false,
            clue_ids: Vec::new(),
        }
    }

    fn with_narrative_id(mut self, narrative_id: &str) -> Self {
        self.narrative_id = Some(narrative_id.to_string());
        self
    }

    fn remember_clue(&mut self, clue_id: &str) {
        if !self.clue_ids.iter().any(|clue| clue == clue_id) {
            self.clue_ids.push(clue_id.to_string());
        }
    }
}

fn availability_check(
    state: &GameState,
    command: &ActionCommand,
    content_bundle: &ContentBundle,
) -> Result<(), CommandError> {
    if command.actor != "player" {
        return Err(CommandError::validation(
            "Sprint 0 only accepts player actions",
        ));
    }

    if state.time.window_type == WindowType::Anchor && command.intent != ActionIntent::Wait {
        return Err(CommandError::validation(
            "anchor window is pending; free actions are closed",
        ));
    }

    if command.intent == ActionIntent::Cultivate && !state.character.aperture_opened {
        return Err(CommandError::validation(
            "aperture is not opened; cultivation unavailable",
        ));
    }

    if let Some(active) = &state.encounters.active {
        if !is_encounter_decision_intent(&command.intent) {
            return Err(CommandError::validation(
                "active encounter must be resolved before ordinary actions",
            ));
        }

        let target = command
            .target
            .as_deref()
            .ok_or_else(|| CommandError::validation("encounter decision target is required"))?;
        if target != active.encounter_id {
            return Err(CommandError::validation(format!(
                "encounter decision target '{}' does not match active encounter '{}'",
                target, active.encounter_id
            )));
        }

        let encounter = encounter_by_id(&active.encounter_id, content_bundle)?;
        require_mode(&state.mode, &encounter.modes, "encounter", &encounter.id)?;
        let action = action_by_intent_target(command.intent.clone(), Some(target), content_bundle)?;
        require_mode(&state.mode, &action.modes, "action", &action.id)?;
        active_encounter_decision(state, content_bundle, &command.intent)?;
        return Ok(());
    }

    if is_encounter_decision_intent(&command.intent) {
        return Err(CommandError::validation(
            "encounter decision requires an active encounter",
        ));
    }

    if command.intent == ActionIntent::Wait {
        return Ok(());
    }

    let target = target_or_current(state, command)?;
    if target == "blackmarket_hint" && !state.knowledge.blackmarket_route_known {
        return Err(CommandError::validation("黑市门路未明"));
    }

    if command.intent == ActionIntent::Move {
        let edge = movement_edge(state, &target, content_bundle)?;
        require_mode(&state.mode, &edge.modes, "movement", &edge.id)?;
        if let Some(required_period) = &edge.required_period {
            if !period_matches(required_period, &state.time.period) {
                return Err(CommandError::validation(format!(
                    "movement '{}' requires period '{}'",
                    edge.id, required_period
                )));
            }
        }
        let node = node_by_id(&target, content_bundle)?;
        require_mode(&state.mode, &node.modes, "node", &node.id)?;
        return Ok(());
    }

    let action = action_by_intent_target(command.intent.clone(), Some(&target), content_bundle)?;
    require_mode(&state.mode, &action.modes, "action", &action.id)?;
    let node = node_by_id(&target, content_bundle)?;
    require_mode(&state.mode, &node.modes, "node", &node.id)?;

    if action_requires_current_node(&command.intent) && target != state.world.current_node_id {
        return Err(CommandError::validation(format!(
            "action target '{}' requires current node '{}'",
            target, state.world.current_node_id
        )));
    }

    match command.intent {
        ActionIntent::Recover if state.world.current_node_id != "infirmary_lane" => {
            Err(CommandError::validation("recover requires infirmary_lane"))
        }
        ActionIntent::Trade
            if state.world.current_node_id != "blackmarket_hint" || state.time.period != "深夜" =>
        {
            Err(CommandError::validation(
                "blackmarket trade requires blackmarket_hint during 深夜",
            ))
        }
        _ => Ok(()),
    }
}

fn action_requires_current_node(intent: &ActionIntent) -> bool {
    matches!(
        intent,
        ActionIntent::Cultivate | ActionIntent::Scout | ActionIntent::Recover | ActionIntent::Trade
    )
}

fn cost_reservation(
    state: &GameState,
    command: &ActionCommand,
    content_bundle: &ContentBundle,
) -> Result<ReservedCost, CommandError> {
    if command.declared_cost.primeval_stones < 0 || command.declared_cost.exposure_risk < 0 {
        return Err(CommandError::validation(
            "declared_cost cannot contain negative values",
        ));
    }

    let (ap, primeval_stones, consume_window) = match command.intent {
        ActionIntent::Move => {
            let target = target_or_current(state, command)?;
            let edge = movement_edge(state, &target, content_bundle)?;
            (edge.ap_cost, 0, false)
        }
        ActionIntent::Cultivate => (1, 1, false),
        ActionIntent::Scout => (1, 0, false),
        ActionIntent::Recover => (recover_ap_cost(&state.character.injury.level), 0, false),
        ActionIntent::Trade => (1, 1, false),
        ActionIntent::Retreat
        | ActionIntent::Confront
        | ActionIntent::Yield
        | ActionIntent::Argue
        | ActionIntent::Delay
        | ActionIntent::Frame => {
            let decision = active_encounter_decision(state, content_bundle, &command.intent)?;
            (decision.ap_cost, decision.primeval_stones_cost, false)
        }
        ActionIntent::Wait => (state.time.ap, 0, true),
    };

    if ap > state.time.ap {
        return Err(CommandError::validation(format!(
            "AP not enough: need {ap}, current {}",
            state.time.ap
        )));
    }

    if primeval_stones > state.resources.primeval_stones {
        return Err(CommandError::validation(format!(
            "primeval stones not enough: need {primeval_stones}, current {}",
            state.resources.primeval_stones
        )));
    }

    Ok(ReservedCost {
        ap,
        primeval_stones,
        consume_window,
    })
}

fn recover_ap_cost(injury_level: &InjuryLevel) -> u8 {
    match injury_level {
        InjuryLevel::Healthy => 1,
        InjuryLevel::Light | InjuryLevel::Heavy => 2,
    }
}

fn subsystem_resolution(
    state: &GameState,
    command: &ActionCommand,
    content_bundle: &ContentBundle,
) -> Result<SubsystemOutcome, CommandError> {
    match command.intent {
        ActionIntent::Move => {
            let target = target_or_current(state, command)?;
            let edge = movement_edge(state, &target, content_bundle)?;
            let mut outcome = SubsystemOutcome::new(
                "movement",
                format!(
                    "你从 {} 转向 {}，账本记下移动代价。",
                    state.world.current_node_id, target
                ),
            )
            .with_narrative_id("s0.movement.default");
            outcome.target_node_id = Some(target);
            outcome.exposure_delta = edge.exposure_delta;
            outcome.arrival_ap_penalty = edge.arrival_ap_penalty;
            if let Some(encounter) = eligible_encounter_trigger(
                state,
                outcome.target_node_id.as_deref(),
                state.risk.exposure + outcome.exposure_delta,
                state.build.moonlight_cultivation_marks,
                content_bundle,
            ) {
                require_mode(&state.mode, &encounter.modes, "encounter", &encounter.id)?;
                outcome.trigger_encounter = Some(ActiveEncounter {
                    encounter_id: encounter.id.clone(),
                    encounter_type: encounter.encounter_type.clone(),
                    known_risk: encounter.known_risk.clone(),
                    decision_intents: encounter
                        .decisions
                        .iter()
                        .map(|decision| decision.intent.clone())
                        .collect(),
                });
                outcome.ledger_kind = "encounter".to_string();
                outcome.ledger_text = format!(
                    "黑市边路有人拦住去路，勒索的风险已经明牌：{}",
                    encounter.known_risk
                );
                outcome.narrative_id = Some(format!("s0.encounter.{}.trigger", encounter.id));
            }
            Ok(outcome)
        }
        ActionIntent::Cultivate => {
            let mut outcome =
                SubsystemOutcome::new("action", "你按下杂念运转真元，月光修行痕迹更深。")
                    .with_narrative_id("s0.action.cultivate.moonlight");
            outcome.survival_route = Some("月光修行：制度内求稳".to_string());
            outcome.moonlight_cultivation_delta = 1;
            if let Some(encounter) = eligible_encounter_trigger(
                state,
                Some(state.world.current_node_id.as_str()),
                state.risk.exposure,
                state
                    .build
                    .moonlight_cultivation_marks
                    .saturating_add(outcome.moonlight_cultivation_delta),
                content_bundle,
            ) {
                require_mode(&state.mode, &encounter.modes, "encounter", &encounter.id)?;
                apply_encounter_trigger(&mut outcome, encounter);
            }
            Ok(outcome)
        }
        ActionIntent::Scout => {
            let mut outcome = SubsystemOutcome::new("action", "你没有急着下注，先听风声、记人脸。")
                .with_narrative_id("s0.action.scout.default");
            let target = target_or_current(state, command)?;
            if target == "academy_gate" {
                outcome.reveal_blackmarket_route = true;
                outcome.remember_clue("rumor_blackmarket_tail");
                outcome.ledger_text =
                    "你在学堂门前听见几句低声风声，暗口二字被记进线索页。".to_string();
                outcome.narrative_id = Some("s0.action.scout.academy_gate".to_string());
            } else if target == "moonlight_corner" {
                outcome.remember_clue("rumor_academy_pressure");
                outcome.ledger_text =
                    "你在月光角看清几处站位，学堂里的比较压力比明面规矩更锋利。".to_string();
                outcome.narrative_id = Some("s0.action.scout.moonlight_corner".to_string());
            } else if target == "merit_notice" {
                outcome.merit_delta = 1;
                outcome.remember_clue("rumor_merit_audit");
                outcome.ledger_text = "你在功绩告示旁核对机会，记下一点可用功绩。".to_string();
                outcome.narrative_id = Some("s0.action.scout.merit_notice".to_string());
            } else if target == "infirmary_lane" {
                outcome.remember_clue("rumor_infirmary_debt");
                outcome.ledger_text =
                    "药堂侧巷的价码不只算元石，还算人情和下一次被追索的时机。".to_string();
                outcome.narrative_id = Some("s0.action.scout.infirmary_lane".to_string());
            } else if target == "branch_lodging" {
                outcome.remember_clue("rumor_family_debt");
                outcome.ledger_text =
                    "旁支落脚点能挡一时风雨，也把欠账和亲疏写得更清楚。".to_string();
                outcome.narrative_id = Some("s0.action.scout.branch_lodging".to_string());
            } else if target == "clan_alley_rumor" {
                outcome.reveal_blackmarket_route = true;
                outcome.remember_clue("rumor_blackmarket_tail");
                outcome.remember_clue("rumor_alley_probe");
                outcome.ledger_text =
                    "巷道里有人提到暗口，又立刻噤声；门路有了，暴露也跟着有了轮廓。".to_string();
                outcome.narrative_id = Some("s0.action.scout.clan_alley_rumor".to_string());
            } else if target == "inheritance_rumor" {
                outcome.remember_clue("rumor_inheritance_bamboo");
                outcome.ledger_text =
                    "传承残线半真半假，能记进账本，但不能当作稳妥出路。".to_string();
                outcome.narrative_id = Some("s0.action.scout.inheritance_rumor".to_string());
            }
            Ok(outcome)
        }
        ActionIntent::Recover => {
            let mut outcome =
                SubsystemOutcome::new("action", "你换来一口喘息，也把债写进药堂账页。")
                    .with_narrative_id("s0.action.recover.default");
            outcome.infirmary_debt_delta = 1;
            outcome.favor_debt_delta = 1;
            match state.character.injury.level {
                InjuryLevel::Heavy => {
                    outcome.injury_level = Some(InjuryLevel::Light);
                    outcome.injury_ap_penalty_pending = Some(false);
                    outcome.ledger_text = "药堂处理重伤，伤势降为轻伤，债仍跟着你。".to_string();
                    outcome.narrative_id = Some("s0.action.recover.heavy_to_light".to_string());
                }
                InjuryLevel::Light => {
                    outcome.injury_level = Some(InjuryLevel::Healthy);
                    outcome.injury_ap_penalty_pending = Some(false);
                    outcome.ledger_text = "药堂清掉轻伤，又在债账上添了一笔。".to_string();
                    outcome.narrative_id = Some("s0.action.recover.light_to_healthy".to_string());
                }
                InjuryLevel::Healthy => {}
            }
            Ok(outcome)
        }
        ActionIntent::Trade => {
            let mut outcome =
                SubsystemOutcome::new("action", "你在暗口换来材料，门路和风险一起上涨。")
                    .with_narrative_id("s0.action.trade.blackmarket_hint");
            outcome.materials_delta = 1;
            outcome.exposure_delta = 2;
            Ok(outcome)
        }
        ActionIntent::Retreat => {
            let decision = active_encounter_decision(state, content_bundle, &command.intent)?;
            let mut outcome =
                SubsystemOutcome::new("encounter", "你选择跑路，丢一点脸面和掩护，保住筋骨。")
                    .with_narrative_id(&decision.narrative_id);
            outcome.exposure_delta =
                mitigated_exposure_delta(state, decision).unwrap_or(decision.exposure_delta);
            outcome.clear_active_encounter = true;
            outcome.target_node_id = decision.target_node_id.clone();
            outcome.survival_route = Some(decision.survival_route.clone());
            Ok(outcome)
        }
        ActionIntent::Confront => {
            let decision = active_encounter_decision(state, content_bundle, &command.intent)?;
            let mut outcome =
                SubsystemOutcome::new("encounter", "你硬顶勒索，代价落在元石和伤势上。")
                    .with_narrative_id(&decision.narrative_id);
            outcome.exposure_delta =
                mitigated_exposure_delta(state, decision).unwrap_or(decision.exposure_delta);
            outcome.clear_active_encounter = true;
            outcome.target_node_id = decision.target_node_id.clone();
            outcome.injury_level = decision.injury_level.clone();
            outcome.injury_ap_penalty_pending = Some(decision.injury_ap_penalty_pending);
            outcome.survival_route = Some(decision.survival_route.clone());
            Ok(outcome)
        }
        ActionIntent::Yield | ActionIntent::Argue | ActionIntent::Delay | ActionIntent::Frame => {
            let decision = active_encounter_decision(state, content_bundle, &command.intent)?;
            let mut outcome =
                SubsystemOutcome::new("encounter", "你在遭遇里作出决断，代价和余波被写进账本。")
                    .with_narrative_id(&decision.narrative_id);
            outcome.exposure_delta =
                mitigated_exposure_delta(state, decision).unwrap_or(decision.exposure_delta);
            outcome.clear_active_encounter = true;
            outcome.target_node_id = decision.target_node_id.clone();
            outcome.injury_level = decision.injury_level.clone();
            if decision.injury_ap_penalty_pending {
                outcome.injury_ap_penalty_pending = Some(true);
            }
            outcome.survival_route = Some(decision.survival_route.clone());
            for clue_id in &decision.clue_ids {
                outcome.remember_clue(clue_id);
            }
            Ok(outcome)
        }
        ActionIntent::Wait => Ok(SubsystemOutcome::new(
            "action",
            "你把这个时段耗过去，未用 AP 不会结转。",
        )
        .with_narrative_id("s0.action.wait.default")),
    }
}

fn anchor_recalculation(_state: &mut GameState, _outcome: &SubsystemOutcome) {
    // Hidden anchor variables stay behind this hook so later systems do not bypass the pipeline.
}

fn effect_commit(
    state: &mut GameState,
    reserved_cost: ReservedCost,
    outcome: &SubsystemOutcome,
    content_bundle: &ContentBundle,
) {
    state.time.ap = state.time.ap.saturating_sub(reserved_cost.ap);
    state.resources.primeval_stones -= reserved_cost.primeval_stones;
    state.resources.materials += outcome.materials_delta;
    state.resources.merit += outcome.merit_delta;
    state.debts_and_credit.infirmary_debt += outcome.infirmary_debt_delta;
    state.debts_and_credit.favor_debt += outcome.favor_debt_delta;
    state.debts_and_credit.organization_debt += outcome.organization_debt_delta;
    state.debts_and_credit.trading_credit += outcome.trading_credit_delta;
    state.risk.exposure += outcome.exposure_delta;
    state.build.moonlight_cultivation_marks = state
        .build
        .moonlight_cultivation_marks
        .saturating_add(outcome.moonlight_cultivation_delta);

    if let Some(target_node_id) = &outcome.target_node_id {
        state.world.current_node_id = target_node_id.clone();
    }

    state.time.ap = state.time.ap.saturating_sub(outcome.arrival_ap_penalty);

    if outcome.clear_active_encounter {
        if let Some(active) = &state.encounters.active {
            if !state
                .encounters
                .resolved_encounter_ids
                .contains(&active.encounter_id)
            {
                state
                    .encounters
                    .resolved_encounter_ids
                    .push(active.encounter_id.clone());
            }
        }
        state.encounters.active = None;
    }

    if let Some(active_encounter) = &outcome.trigger_encounter {
        state.encounters.active = Some(active_encounter.clone());
    }

    if let Some(injury_level) = &outcome.injury_level {
        state.character.injury.level = injury_level.clone();
    }

    if let Some(pending) = outcome.injury_ap_penalty_pending {
        state.character.injury.ap_penalty_pending = pending;
    }

    if let Some(survival_route) = &outcome.survival_route {
        state.build.survival_route = survival_route.clone();
    }

    if outcome.reveal_blackmarket_route {
        state.knowledge.blackmarket_route_known = true;
    }

    for clue_id in &outcome.clue_ids {
        state.knowledge.record_clue(clue_id);
    }

    if reserved_cost.consume_window || state.time.ap == 0 {
        advance_window(state, content_bundle);
    }
}

fn advance_window(state: &mut GameState, content_bundle: &ContentBundle) {
    if state.time.window_type == WindowType::Free {
        state.time.free_rounds_elapsed = state.time.free_rounds_elapsed.saturating_add(1);
    }

    let next_index = state.time.window_index + 1;
    if let Some(next_window) = content_bundle.windows.get(next_index) {
        state.time.window_id = next_window.id.clone();
        state.time.window_index = next_index;
        state.time.chapter_day = next_window.day;
        state.time.period = next_window.period.clone();
        state.time.window_type = next_window.window_type.clone();
        state.time.ap = next_window.default_ap;
        if state.character.injury.ap_penalty_pending
            && state.time.window_type == WindowType::Free
            && state.time.ap > 0
        {
            state.time.ap = state.time.ap.saturating_sub(1);
            state.character.injury.ap_penalty_pending = false;
        }
        state.time.next_anchor_pressure = "下一处制度压力正在靠近".to_string();
    } else {
        state.time.window_id = "s0_anchor_pending".to_string();
        state.time.window_index = next_index;
        state.time.window_type = WindowType::Anchor;
        state.time.ap = 0;
        state.time.next_anchor_pressure = "首个阶段锚点临近".to_string();
    }
}

fn ledger_append(
    state: &mut GameState,
    outcome: &SubsystemOutcome,
    content_bundle: &ContentBundle,
) {
    state.ledger.push(LedgerEntry {
        kind: outcome.ledger_kind.clone(),
        text: render_local_narrative(content_bundle, outcome),
    });
}

fn render_local_narrative(content_bundle: &ContentBundle, outcome: &SubsystemOutcome) -> String {
    outcome
        .narrative_id
        .as_deref()
        .and_then(|narrative_id| narrative_by_id(narrative_id, content_bundle))
        .map(|narrative| narrative.text.clone())
        .filter(|text| !text.trim().is_empty())
        .unwrap_or_else(|| outcome.ledger_text.clone())
}

fn target_or_current(state: &GameState, command: &ActionCommand) -> Result<String, CommandError> {
    if command.intent == ActionIntent::Move {
        return command
            .target
            .clone()
            .filter(|target| !target.trim().is_empty())
            .ok_or_else(|| CommandError::validation("move target is required"));
    }

    Ok(command
        .target
        .clone()
        .filter(|target| !target.trim().is_empty())
        .unwrap_or_else(|| state.world.current_node_id.clone()))
}

fn movement_edge<'a>(
    state: &GameState,
    target: &str,
    content_bundle: &'a ContentBundle,
) -> Result<&'a ContentMovementEdge, CommandError> {
    content_bundle
        .movements
        .iter()
        .find(|movement| movement.from == state.world.current_node_id && movement.to == target)
        .ok_or_else(|| {
            CommandError::validation(format!(
                "no movement edge from '{}' to '{}'",
                state.world.current_node_id, target
            ))
        })
}

fn node_by_id<'a>(
    node_id: &str,
    content_bundle: &'a ContentBundle,
) -> Result<&'a ContentNode, CommandError> {
    let index = content_bundle
        .indexes
        .node_ids
        .get(node_id)
        .ok_or_else(|| CommandError::validation(format!("node '{node_id}' is not in bundle")))?;
    Ok(&content_bundle.nodes[*index])
}

fn encounter_by_id<'a>(
    encounter_id: &str,
    content_bundle: &'a ContentBundle,
) -> Result<&'a ContentEncounterTemplate, CommandError> {
    let index = content_bundle
        .indexes
        .encounter_ids
        .get(encounter_id)
        .ok_or_else(|| {
            CommandError::validation(format!("encounter '{encounter_id}' is not in bundle"))
        })?;
    Ok(&content_bundle.encounters[*index])
}

fn narrative_by_id<'a>(
    narrative_id: &str,
    content_bundle: &'a ContentBundle,
) -> Option<&'a ContentNarrativeTemplate> {
    content_bundle
        .indexes
        .narrative_ids
        .get(narrative_id)
        .map(|index| &content_bundle.narratives[*index])
}

fn active_encounter_decision<'a>(
    state: &GameState,
    content_bundle: &'a ContentBundle,
    intent: &ActionIntent,
) -> Result<&'a ContentEncounterDecision, CommandError> {
    let active =
        state.encounters.active.as_ref().ok_or_else(|| {
            CommandError::validation("encounter decision requires active encounter")
        })?;
    let encounter = encounter_by_id(&active.encounter_id, content_bundle)?;
    encounter
        .decisions
        .iter()
        .find(|decision| &decision.intent == intent)
        .ok_or_else(|| {
            CommandError::validation(format!(
                "encounter '{}' does not allow decision '{intent:?}'",
                active.encounter_id
            ))
        })
}

fn mitigated_exposure_delta(state: &GameState, decision: &ContentEncounterDecision) -> Option<i32> {
    let clue_id = decision.mitigating_clue_id.as_ref()?;
    state
        .knowledge
        .known_clues
        .iter()
        .any(|known| known == clue_id)
        .then_some(
            decision
                .mitigated_exposure_delta
                .unwrap_or(decision.exposure_delta),
        )
}

fn apply_encounter_trigger(outcome: &mut SubsystemOutcome, encounter: &ContentEncounterTemplate) {
    outcome.trigger_encounter = Some(ActiveEncounter {
        encounter_id: encounter.id.clone(),
        encounter_type: encounter.encounter_type.clone(),
        known_risk: encounter.known_risk.clone(),
        decision_intents: encounter
            .decisions
            .iter()
            .map(|decision| decision.intent.clone())
            .collect(),
    });
    outcome.ledger_kind = "encounter".to_string();
    outcome.ledger_text = format!("遭遇压身：{}", encounter.known_risk);
    outcome.narrative_id = Some(format!("s0.encounter.{}.trigger", encounter.id));
}

fn eligible_encounter_trigger<'a>(
    state: &GameState,
    node_id: Option<&str>,
    prospective_exposure: i32,
    prospective_moonlight_marks: u8,
    content_bundle: &'a ContentBundle,
) -> Option<&'a ContentEncounterTemplate> {
    let node_id = node_id?;
    content_bundle.encounters.iter().find(|encounter| {
        encounter.trigger_node_id == node_id
            && !state
                .encounters
                .resolved_encounter_ids
                .contains(&encounter.id)
            && encounter
                .min_exposure
                .is_none_or(|min| prospective_exposure >= min)
            && encounter
                .min_moonlight_marks
                .is_none_or(|min| prospective_moonlight_marks >= min)
            && encounter.required_clue_ids.iter().all(|required| {
                state
                    .knowledge
                    .known_clues
                    .iter()
                    .any(|known| known == required)
            })
            && mode_permitted(&state.mode, &encounter.modes)
    })
}

fn action_by_intent_target<'a>(
    intent: ActionIntent,
    target: Option<&str>,
    content_bundle: &'a ContentBundle,
) -> Result<&'a ContentAction, CommandError> {
    content_bundle
        .actions
        .iter()
        .find(|action| action.intent == intent && action.target.as_deref() == target)
        .ok_or_else(|| {
            CommandError::validation(format!(
                "no content action for intent '{:?}' and target '{:?}'",
                intent, target
            ))
        })
}

fn require_mode(
    mode: &RunMode,
    modes: &[ModePermit],
    kind: &str,
    id: &str,
) -> Result<(), CommandError> {
    let permitted = match mode {
        RunMode::CanonStrict => modes.contains(&ModePermit::CanonStrict),
        RunMode::SandboxIf => modes.contains(&ModePermit::SandboxIf),
    };

    if permitted {
        Ok(())
    } else {
        Err(CommandError::validation(format!(
            "{kind} '{id}' is not permitted in mode '{:?}'",
            mode
        )))
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
        assert_eq!(state.time.window_id, "day1_morning_free");
        assert_eq!(state.resources.primeval_stones, 3);
        assert_eq!(projection.current_node_id, "academy_gate");
        assert!(state.character.aperture_opened);
        assert!(projection.scene_text.contains("开窍大典"));
        assert!(projection.scene_text.contains("学堂门前"));
        assert!(projection
            .status_markers
            .iter()
            .any(|marker| marker.label == "空窍" && marker.value == "已开窍"));
        assert!(projection
            .status_markers
            .iter()
            .any(|marker| marker.label == "地点" && marker.value == "学堂门前"));
    }

    #[test]
    fn unopened_aperture_blocks_cultivation_without_hiding_the_reason() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.character.aperture_opened = false;

        let projection = build_projection_with_content(&state, &bundle);
        let cultivate = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "cultivate_moonlight")
            .expect("cultivation action should remain visible with a clear gate");

        assert!(!cultivate.enabled);
        assert_eq!(
            cultivate.disabled_reason.as_deref(),
            Some("空窍未开，不能修行")
        );

        let error = resolve_action(
            state,
            command(ActionIntent::Cultivate, Some("academy_gate")),
            &bundle,
        )
        .expect_err("unopened aperture must reject cultivation");
        assert_eq!(error.kind, CommandErrorKind::Validation);
    }

    #[test]
    fn projection_separates_vital_gu_from_core_and_support_gu() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection(&state);

        assert_eq!(state.build.vital_gu.status, VitalGuStatus::NotEstablished);
        assert_eq!(state.build.vital_gu.instance_id, None);
        assert_eq!(state.build.vital_gu.binding_scope, "未绑定");
        assert_eq!(state.build.vital_gu.binding_risk, "未暴露");
        assert_eq!(projection.build_view.core_gu, "核心蛊：月光蛊线索未稳");
        assert_eq!(projection.build_view.support_gu, "辅助蛊：暂无");
        assert_eq!(projection.build_view.vital_gu, "本命蛊：未建立");
        assert_ne!(
            projection.build_view.vital_gu,
            projection.build_view.core_gu
        );
        assert_ne!(
            projection.build_view.vital_gu,
            projection.build_view.support_gu
        );
        assert!(projection
            .status_markers
            .iter()
            .any(|marker| marker.label == "AP" && marker.value == "2"));
    }

    #[test]
    fn blackmarket_route_is_hidden_until_knowledge_unlock() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection_with_content(&state, &bundle);

        assert!(!state.knowledge.blackmarket_route_known);
        assert!(projection
            .action_choices
            .iter()
            .all(|choice| !choice.id.contains("blackmarket")));
        assert!(projection
            .node_view
            .visible_nodes
            .iter()
            .all(|node| node.id != "blackmarket_hint"));

        let mut night = state.clone();
        set_deep_night(&mut night);
        let hidden_error = resolve_action(
            night,
            command(ActionIntent::Move, Some("blackmarket_hint")),
            &bundle,
        )
        .expect_err("blackmarket route should not be directly usable before knowledge unlock");
        assert_eq!(hidden_error.kind, CommandErrorKind::Validation);
        assert!(hidden_error.message.contains("黑市门路未明"));

        let scouted = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("academy scouting should unlock a blackmarket hint");
        assert!(scouted.state.knowledge.blackmarket_route_known);

        let unlocked_projection = build_projection_with_content(&scouted.state, &bundle);
        assert!(unlocked_projection
            .action_choices
            .iter()
            .any(|choice| choice.id == "move_blackmarket_hint"));
        assert!(unlocked_projection
            .node_view
            .visible_nodes
            .iter()
            .any(|node| node.id == "blackmarket_hint"));
    }

    #[test]
    fn relationship_projection_tracks_s0_pressure_and_blackmarket_access() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection_with_content(&state, &bundle);

        assert_eq!(
            projection.relationship_view.family_pressure,
            "家族秩序：低压监视"
        );
        assert_eq!(projection.relationship_view.infirmary_debt, "药堂债：0");
        assert_eq!(projection.relationship_view.favor_debt, "人情债：0");
        assert_eq!(
            projection.relationship_view.blackmarket_access,
            "黑市门路：未解锁"
        );

        let unlocked = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("academy scouting should refresh relationship projection");

        assert_eq!(
            unlocked
                .response
                .projection
                .relationship_view
                .blackmarket_access,
            "黑市门路：已听到暗口风声"
        );
    }

    #[test]
    fn sprint1_starter_bundle_contains_route_content_outline() {
        let bundle = starter_content_bundle();

        for node_id in [
            "academy_gate",
            "moonlight_corner",
            "merit_notice",
            "infirmary_lane",
            "blackmarket_hint",
            "inheritance_rumor",
            "branch_lodging",
            "clan_alley_rumor",
        ] {
            assert!(
                bundle.indexes.node_ids.contains_key(node_id),
                "missing S0 node {node_id}"
            );
        }

        for action_id in [
            "cultivate_moonlight_corner",
            "observe_moonlight_pressure",
            "audit_merit_notice",
            "ask_infirmary_price",
            "move_branch_lodging",
            "listen_branch_lodging_debt",
            "move_clan_alley_rumor",
            "listen_clan_alley_rumor",
            "verify_inheritance_rumor",
        ] {
            assert!(
                bundle.indexes.action_ids.contains_key(action_id),
                "missing S0 action {action_id}"
            );
        }

        assert_eq!(bundle.nodes.len(), 8);
        assert!(
            bundle.narratives.len() >= 23,
            "Sprint 1 Phase 2 should carry local narrative coverage for new route content"
        );
        for route in &bundle.routes {
            assert!(
                route.entry_action_ids.len() >= 2,
                "route '{}' should expose at least two entry content points",
                route.id
            );
        }
    }

    #[test]
    fn sprint1_projection_localizes_non_move_actions_and_keeps_blackmarket_hidden() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection_with_content(&state, &bundle);

        assert!(projection
            .action_choices
            .iter()
            .any(|choice| choice.id == "move_branch_lodging"));
        assert!(projection
            .action_choices
            .iter()
            .all(|choice| choice.id != "listen_branch_lodging_debt"));
        assert!(projection
            .action_choices
            .iter()
            .all(|choice| !choice.id.contains("blackmarket")));

        let moved = resolve_action(
            state,
            command(ActionIntent::Move, Some("branch_lodging")),
            &bundle,
        )
        .expect("branch lodging movement should resolve");
        let localized = build_projection_with_content(&moved.state, &bundle);

        let lodging_scout = localized
            .action_choices
            .iter()
            .find(|choice| choice.id == "listen_branch_lodging_debt")
            .expect("branch lodging should expose its local scout action");
        assert!(lodging_scout.enabled);
        assert!(localized
            .action_choices
            .iter()
            .all(|choice| choice.id != "check_merit_notice"));
    }

    #[test]
    fn sprint1_scout_records_unique_route_clues() {
        let bundle = starter_content_bundle();
        let moved = resolve_action(
            create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION),
            command(ActionIntent::Move, Some("branch_lodging")),
            &bundle,
        )
        .expect("branch lodging movement should resolve");

        let scouted = resolve_action(
            moved.state,
            command(ActionIntent::Scout, Some("branch_lodging")),
            &bundle,
        )
        .expect("branch lodging scout should record a clue");
        assert!(scouted
            .state
            .knowledge
            .known_clues
            .contains(&"rumor_family_debt".to_string()));

        let repeated = resolve_action(
            scouted.state,
            command(ActionIntent::Scout, Some("branch_lodging")),
            &bundle,
        )
        .expect("repeated branch lodging scout should not duplicate clue ids");
        let family_debt_count = repeated
            .state
            .knowledge
            .known_clues
            .iter()
            .filter(|clue| clue.as_str() == "rumor_family_debt")
            .count();
        assert_eq!(family_debt_count, 1);
    }

    #[test]
    fn sprint1_phase3_bundle_contains_three_encounter_variants_and_decisions() {
        let bundle = starter_content_bundle();

        for (encounter_id, encounter_type, decision_intents) in [
            (
                "blackmarket_extortion",
                EncounterType::Extortion,
                vec![ActionIntent::Retreat, ActionIntent::Confront],
            ),
            (
                "academy_public_pressure",
                EncounterType::PublicPressure,
                vec![
                    ActionIntent::Yield,
                    ActionIntent::Argue,
                    ActionIntent::Confront,
                ],
            ),
            (
                "alley_probe",
                EncounterType::Probe,
                vec![
                    ActionIntent::Retreat,
                    ActionIntent::Delay,
                    ActionIntent::Frame,
                    ActionIntent::Confront,
                ],
            ),
        ] {
            let encounter = encounter_by_id(encounter_id, &bundle)
                .unwrap_or_else(|_| panic!("missing encounter {encounter_id}"));
            assert_eq!(encounter.encounter_type, encounter_type);
            let actual_intents = encounter
                .decisions
                .iter()
                .map(|decision| decision.intent.clone())
                .collect::<Vec<_>>();
            assert_eq!(actual_intents, decision_intents);
            for decision in &encounter.decisions {
                assert!(
                    bundle
                        .indexes
                        .narrative_ids
                        .contains_key(&decision.narrative_id),
                    "decision {:?} for {encounter_id} points at missing narrative {}",
                    decision.intent,
                    decision.narrative_id
                );
            }
        }
    }

    #[test]
    fn academy_pressure_clue_mitigates_public_pressure_decision_cost() {
        let bundle = starter_content_bundle();

        let unprepared = state_at_academy_pressure_without_clue(&bundle);
        let unprepared_yield = resolve_action(
            unprepared,
            command(ActionIntent::Yield, Some("academy_public_pressure")),
            &bundle,
        )
        .expect("yield should resolve without the pressure clue");

        let prepared = state_at_academy_pressure_with_clue(&bundle);
        let prepared_yield = resolve_action(
            prepared,
            command(ActionIntent::Yield, Some("academy_public_pressure")),
            &bundle,
        )
        .expect("yield should resolve with the pressure clue");

        assert!(
            prepared_yield.state.risk.exposure < unprepared_yield.state.risk.exposure,
            "rumor_academy_pressure should reduce the visible exposure cost of yielding"
        );
        assert!(prepared_yield
            .state
            .encounters
            .resolved_encounter_ids
            .contains(&"academy_public_pressure".to_string()));
    }

    #[test]
    fn alley_probe_scout_records_probe_clue_and_later_triggers_probe() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.world.current_node_id = "clan_alley_rumor".to_string();

        let scouted = resolve_action(
            state,
            command(ActionIntent::Scout, Some("clan_alley_rumor")),
            &bundle,
        )
        .expect("alley scout should resolve");
        assert!(scouted
            .state
            .knowledge
            .known_clues
            .contains(&"rumor_alley_probe".to_string()));
        assert!(
            scouted.state.encounters.active.is_none(),
            "scouting should provide warning before the probe is sprung"
        );

        let mut exposed = scouted.state;
        exposed.risk.exposure = 2;
        let returned = resolve_action(
            exposed,
            command(ActionIntent::Move, Some("academy_gate")),
            &bundle,
        )
        .expect("returning to academy should resolve");
        let probed = resolve_action(
            returned.state,
            command(ActionIntent::Move, Some("clan_alley_rumor")),
            &bundle,
        )
        .expect("returning to the alley at high exposure should trigger probe");

        let active = probed
            .state
            .encounters
            .active
            .expect("alley probe should become active");
        assert_eq!(active.encounter_id, "alley_probe");
        assert_eq!(active.encounter_type, EncounterType::Probe);
        assert!(active.decision_intents.contains(&ActionIntent::Delay));
        assert!(active.decision_intents.contains(&ActionIntent::Frame));
    }

    #[test]
    fn resolved_encounter_does_not_retrigger_on_same_node() {
        let bundle = starter_content_bundle();
        let probed = state_at_alley_probe(&bundle);
        let delayed = resolve_action(
            probed,
            command(ActionIntent::Delay, Some("alley_probe")),
            &bundle,
        )
        .expect("delay should resolve alley probe");
        assert!(delayed.state.encounters.active.is_none());
        assert!(delayed
            .state
            .encounters
            .resolved_encounter_ids
            .contains(&"alley_probe".to_string()));

        let returned = resolve_action(
            delayed.state,
            command(ActionIntent::Move, Some("academy_gate")),
            &bundle,
        )
        .expect("can leave alley after resolved probe");
        let revisited = resolve_action(
            returned.state,
            command(ActionIntent::Move, Some("clan_alley_rumor")),
            &bundle,
        )
        .expect("can revisit alley after resolved probe");

        assert!(
            revisited.state.encounters.active.is_none(),
            "resolved alley probe should not immediately repeat"
        );
    }

    #[test]
    fn save_envelope_preserves_resolved_encounter_ids() {
        let bundle = starter_content_bundle();
        let probed = state_at_alley_probe(&bundle);
        let framed = resolve_action(
            probed,
            command(ActionIntent::Frame, Some("alley_probe")),
            &bundle,
        )
        .expect("frame should resolve alley probe");

        let encoded =
            serde_json::to_string(&SaveEnvelope::from_state("slot_0", framed.state.clone()))
                .expect("save envelope serializes");
        let decoded: SaveEnvelope =
            serde_json::from_str(&encoded).expect("save envelope deserializes");
        decoded
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect("phase 3 save should load");
        assert_eq!(
            decoded.snapshot.encounters.resolved_encounter_ids,
            framed.state.encounters.resolved_encounter_ids
        );
    }

    #[test]
    fn encounter_ledger_projection_does_not_leak_english_fallback_text() {
        let bundle = starter_content_bundle();
        let encountered = state_at_blackmarket_extortion(&bundle);
        let projection = build_projection_with_content(&encountered, &bundle);

        assert_no_user_visible_english(&projection.ledger_entries);

        let retreated = resolve_action(
            encountered.clone(),
            command(ActionIntent::Retreat, Some("blackmarket_extortion")),
            &bundle,
        )
        .expect("retreat should resolve with Chinese ledger text");
        assert_no_user_visible_english(&retreated.response.projection.ledger_entries);

        let confronted = resolve_action(
            encountered,
            command(ActionIntent::Confront, Some("blackmarket_extortion")),
            &bundle,
        )
        .expect("confront should resolve with Chinese ledger text");
        assert_no_user_visible_english(&confronted.response.projection.ledger_entries);
    }

    #[test]
    fn projection_action_choices_are_built_from_rust_rules() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection_with_content(&state, &bundle);

        let scout = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "scout_academy")
            .expect("scout action should be projected");
        assert_eq!(scout.intent, ActionIntent::Scout);
        assert_eq!(scout.target.as_deref(), Some("academy_gate"));
        assert!(scout.enabled);
        assert_eq!(scout.group, ActionChoiceGroup::Information);
        assert_eq!(scout.tone, ActionChoiceTone::Safe);
        assert!(scout.consequence_hint.contains("线索"));

        let wait = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "wait_current_window")
            .expect("wait action should be projected");
        assert_eq!(wait.intent, ActionIntent::Wait);
        assert!(wait.enabled);
        assert_eq!(wait.group, ActionChoiceGroup::Wait);
        assert_eq!(wait.tone, ActionChoiceTone::Normal);
        assert!(wait.consequence_hint.contains("窗口"));
    }

    #[test]
    fn movement_feedback_projection_updates_current_node_and_hides_current_destination() {
        let bundle = starter_content_bundle();
        let moved = resolve_action(
            create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION),
            command(ActionIntent::Move, Some("moonlight_corner")),
            &bundle,
        )
        .expect("movement should resolve");

        let projection = moved.response.projection;
        assert_eq!(projection.current_node_id, "moonlight_corner");
        assert!(projection
            .status_markers
            .iter()
            .any(|marker| marker.label == "地点" && marker.value == "月光修行角"));
        assert!(projection
            .node_view
            .visible_nodes
            .iter()
            .any(|node| node.current && node.title == "月光修行角"));
        assert!(projection.recent_feedback.is_some());
        assert!(projection
            .action_choices
            .iter()
            .all(|choice| choice.id != "move_moonlight_corner"));
    }

    #[test]
    fn active_encounter_projection_surfaces_decisions_and_disables_wait() {
        let bundle = starter_content_bundle();
        let state = state_at_blackmarket_extortion(&bundle);
        let projection = build_projection_with_content(&state, &bundle);

        let retreat = projection
            .action_choices
            .iter()
            .find(|choice| choice.intent == ActionIntent::Retreat)
            .expect("retreat should be projected during an encounter");
        assert!(retreat.enabled);
        assert_eq!(retreat.target.as_deref(), Some("blackmarket_extortion"));
        assert_eq!(retreat.group, ActionChoiceGroup::Encounter);
        assert_eq!(retreat.tone, ActionChoiceTone::Safe);
        assert!(retreat.consequence_hint.contains("保命"));

        assert!(projection
            .action_choices
            .iter()
            .all(|choice| choice.id != "scout_academy"));

        let wait = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "wait_current_window")
            .expect("wait remains visible but disabled during an encounter");
        assert!(!wait.enabled);
        assert!(wait
            .disabled_reason
            .as_deref()
            .unwrap_or_default()
            .contains("遭遇"));
        assert_eq!(wait.group, ActionChoiceGroup::Wait);
        assert_eq!(wait.tone, ActionChoiceTone::Blocked);
    }

    #[test]
    fn phase4_projection_groups_tones_and_consequence_hints_cover_core_actions() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.knowledge.blackmarket_route_known = true;
        state.time.period = "清晨".to_string();
        state.time.ap = 0;

        let projection = build_projection_with_content(&state, &bundle);

        let cultivate = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "cultivate_moonlight")
            .expect("cultivation action should be visible at academy gate");
        assert_eq!(cultivate.group, ActionChoiceGroup::Cultivation);
        assert_eq!(cultivate.tone, ActionChoiceTone::Blocked);
        assert_eq!(
            cultivate.disabled_reason.as_deref(),
            Some("AP 不足，当前窗口已被压尽")
        );
        assert!(cultivate.consequence_hint.contains("月光"));

        let blackmarket = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "move_blackmarket_hint")
            .expect("known blackmarket route should be visible");
        assert_eq!(blackmarket.group, ActionChoiceGroup::Movement);
        assert_eq!(blackmarket.tone, ActionChoiceTone::Blocked);
        assert_eq!(
            blackmarket.disabled_reason.as_deref(),
            Some("时段不合，当前不可达")
        );
        assert!(blackmarket.consequence_hint.contains("黑市"));

        let recover = projection
            .action_choices
            .iter()
            .find(|choice| choice.id == "move_infirmary_lane")
            .expect("infirmary movement should be visible");
        assert_eq!(recover.group, ActionChoiceGroup::Movement);
        assert!(recover.consequence_hint.contains("药堂"));
    }

    #[test]
    fn phase4_projection_exposes_recent_feedback_and_clue_ledger() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.knowledge.blackmarket_route_known = true;
        state.knowledge.record_clue("rumor_blackmarket_tail");
        state.knowledge.record_clue("rumor_academy_pressure");
        state.knowledge.record_clue("rumor_infirmary_debt");
        state.knowledge.record_clue("rumor_alley_probe");
        state.ledger.push(LedgerEntry {
            kind: "test_result".to_string(),
            text: "你记下一笔新风声。".to_string(),
        });

        let projection = build_projection_with_content(&state, &bundle);
        let feedback = projection
            .recent_feedback
            .as_ref()
            .expect("latest ledger entry should produce recent feedback");

        assert_eq!(feedback.title, "最近落账");
        assert_eq!(feedback.summary, "你记下一笔新风声。");
        assert_eq!(feedback.tone, ActionChoiceTone::Safe);
        assert_eq!(feedback.source_kind, "test_result");
        assert!(projection
            .clue_view
            .blackmarket_access_summary
            .contains("已记下"));

        let clue_labels: Vec<_> = projection
            .clue_view
            .known_clues
            .iter()
            .map(|clue| clue.label.as_str())
            .collect();
        assert!(clue_labels.contains(&"黑市尾巴"));
        assert!(clue_labels.contains(&"学堂压力"));
        assert!(clue_labels.contains(&"药堂债价"));
        assert!(clue_labels.contains(&"巷道试探"));
        assert!(projection
            .clue_view
            .known_clues
            .iter()
            .all(|clue| !clue.summary.contains("rumor_")));
    }

    #[test]
    fn user_visible_sources_have_no_mojibake_markers() {
        let bundle = starter_content_bundle();
        let mut canon_state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        canon_state.knowledge.blackmarket_route_known = true;
        canon_state.knowledge.record_clue("rumor_blackmarket_tail");
        canon_state.knowledge.record_clue("rumor_academy_pressure");
        canon_state.ledger.push(LedgerEntry {
            kind: "test_result".to_string(),
            text: "你把新风声记进因果账。".to_string(),
        });

        let mut sandbox_state = create_run(RunMode::SandboxIf, STARTER_CONTENT_VERSION);
        sandbox_state.knowledge.blackmarket_route_known = true;
        sandbox_state
            .knowledge
            .record_clue("rumor_inheritance_bamboo");

        let payloads = [
            (
                "starter content bundle",
                serde_json::to_string(&bundle).expect("starter bundle serializes"),
            ),
            (
                "canon projection",
                serde_json::to_string(&build_projection_with_content(&canon_state, &bundle))
                    .expect("canon projection serializes"),
            ),
            (
                "sandbox projection",
                serde_json::to_string(&build_projection_with_content(&sandbox_state, &bundle))
                    .expect("sandbox projection serializes"),
            ),
        ];

        for (context, payload) in payloads {
            assert_no_mojibake_markers(context, &payload);
        }
    }

    #[test]
    fn resolve_action_rejects_ap_shortage() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.time.ap = 0;

        let error = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect_err("AP gate should fail");

        assert_eq!(error.kind, CommandErrorKind::Validation);
    }

    #[test]
    fn move_action_updates_node_and_records_ledger() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Move, Some("infirmary_lane")),
            &bundle,
        )
        .expect("move should resolve");

        assert_eq!(result.state.time.ap, 1);
        assert_eq!(result.state.world.current_node_id, "infirmary_lane");
        assert_eq!(result.state.risk.exposure, 1);
        assert!(result
            .response
            .projection
            .scene_text
            .contains("路径、时段与暴露"));
    }

    #[test]
    fn content_bundle_requires_entry_node() {
        let mut source = valid_content_source();
        source.entry_scene_id = "missing".to_string();

        let error = ContentBundle::from_source(source).expect_err("missing entry should fail");
        assert_eq!(error.kind, CommandErrorKind::Content);
    }

    #[test]
    fn content_bundle_builds_indexes_for_s0_sources() {
        let source = valid_content_source();
        let bundle = ContentBundle::from_source(source).expect("valid bundle should build");

        assert_eq!(bundle.manifest.node_count, 2);
        assert_eq!(bundle.manifest.action_count, 1);
        assert_eq!(bundle.manifest.route_count, 1);
        assert_eq!(bundle.manifest.window_count, 1);
        assert_eq!(bundle.manifest.movement_count, 1);
        assert_eq!(bundle.manifest.encounter_count, 0);
        assert_eq!(bundle.manifest.narrative_count, 1);
        assert_eq!(bundle.manifest.origin_count, 3);
        assert_eq!(bundle.manifest.talent_count, 10);
        assert_eq!(bundle.manifest.attribute_profile_count, 1);
        assert_eq!(bundle.manifest.opening_rite_outcome_count, 1);
        assert_eq!(bundle.manifest.initial_resource_package_count, 3);
        assert_eq!(bundle.indexes.node_ids["academy_gate"], 0);
        assert_eq!(bundle.indexes.action_ids["scout_academy"], 0);
        assert_eq!(bundle.indexes.route_ids["moonlight_entry"], 0);
        assert_eq!(bundle.indexes.window_ids["day1_morning_free"], 0);
        assert_eq!(bundle.indexes.movement_ids["academy_to_moonlight"], 0);
        assert_eq!(bundle.indexes.narrative_ids["s0.action.scout.default"], 0);
        assert!(bundle
            .indexes
            .origin_ids
            .contains_key("academy_plain_child"));
        assert!(bundle.indexes.talent_ids.contains_key("steady_mind"));
        assert!(bundle
            .indexes
            .attribute_profile_ids
            .contains_key("s0_default_attributes"));
        assert!(bundle
            .indexes
            .opening_rite_outcome_ids
            .contains_key("s0_opening_rite_established"));
        assert!(bundle
            .indexes
            .initial_resource_package_ids
            .contains_key("s0_opening_plain_supplies"));
        assert!(bundle.diagnostics.summary.contains("indexed 2 nodes"));
        assert!(bundle.diagnostics.summary.contains("1 narratives"));
        assert!(bundle.diagnostics.summary.contains("10 talents"));
        assert!(bundle.diagnostics.warnings.is_empty());
    }

    #[test]
    fn sprint3_starter_bundle_contains_setup_content_contracts() {
        let bundle = starter_content_bundle();

        assert_eq!(bundle.manifest.version, STARTER_CONTENT_VERSION);
        assert!(bundle.origins.len() >= 3);
        assert_eq!(bundle.talents.len(), 10);
        assert_eq!(bundle.attribute_profiles[0].attributes.len(), 4);
        assert!(bundle
            .opening_rite_outcomes
            .iter()
            .any(|outcome| outcome.id == "s0_opening_rite_established" && outcome.aperture_opened));
        assert!(bundle
            .initial_resource_packages
            .iter()
            .any(|package| package.id == "s0_opening_plain_supplies"));
    }

    #[test]
    fn sprint3_canon_strict_excludes_strong_if_talents() {
        let bundle = starter_content_bundle();
        let strong_if = bundle
            .talents
            .iter()
            .filter(|talent| talent.intensity == TalentIntensity::StrongIf)
            .collect::<Vec<_>>();

        assert!(!strong_if.is_empty());
        assert!(strong_if.iter().all(|talent| {
            talent.evidence == EvidenceLevel::SandboxIf
                && talent.modes == vec![ModePermit::SandboxIf]
                && !talent.modes.contains(&ModePermit::CanonStrict)
        }));
    }

    #[test]
    fn sprint3_rejects_strong_if_talent_in_canon_strict() {
        let mut source = valid_content_source();
        let talent = source
            .talents
            .iter_mut()
            .find(|talent| talent.id == "reborn_memory_fragment")
            .expect("test source includes IF talent");
        talent.modes = all_modes();

        let error = ContentBundle::from_source(source).expect_err("strong IF talent must be gated");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("strong_if but allows canon_strict"));
    }

    #[test]
    fn sprint3_rejects_opening_outcome_missing_resource_package() {
        let mut source = valid_content_source();
        source.opening_rite_outcomes[0].resource_package_id = "missing_package".to_string();

        let error =
            ContentBundle::from_source(source).expect_err("missing setup package should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("resource package 'missing_package' not found"));
    }

    #[test]
    fn sprint3_create_setup_run_filters_strong_if_talents_in_canon_strict() {
        let bundle = starter_content_bundle();
        let setup = create_setup_run(RunMode::CanonStrict, &bundle).expect("create setup run");
        let view = build_setup_view(&setup, &bundle).expect("build setup view");

        assert_eq!(setup.mode, RunMode::CanonStrict);
        assert_eq!(setup.content_version, STARTER_CONTENT_VERSION);
        assert_eq!(view.talent_candidates.len(), 7);
        assert!(view
            .talent_candidates
            .iter()
            .all(|talent| talent.intensity == TalentIntensity::Mild));
        assert!(view
            .talent_candidates
            .iter()
            .all(|talent| talent.id != "reborn_memory_fragment"));
        assert!(!view.confirm_enabled);
        assert!(view
            .confirm_blockers
            .iter()
            .any(|blocker| blocker.contains("选择 1 个出身")));
    }

    #[test]
    fn sprint3_setup_talent_selection_requires_exactly_three_and_toggles() {
        let bundle = starter_content_bundle();
        let mut setup = create_setup_run(RunMode::CanonStrict, &bundle).expect("create setup run");

        setup = resolve_setup_choice(
            setup,
            setup_command(SetupIntent::SelectOrigin, "academy_plain_child"),
            &bundle,
        )
        .expect("select origin")
        .setup;

        for talent_id in ["steady_mind", "quiet_observer", "frugal_hands"] {
            setup = resolve_setup_choice(
                setup,
                setup_command(SetupIntent::ToggleTalent, talent_id),
                &bundle,
            )
            .expect("toggle talent")
            .setup;
        }

        let fourth = resolve_setup_choice(
            setup.clone(),
            setup_command(SetupIntent::ToggleTalent, "bitter_bone"),
            &bundle,
        )
        .expect_err("fourth talent must be rejected");
        assert_eq!(fourth.kind, CommandErrorKind::Validation);
        assert!(fourth.message.contains("最多选择 3 个天赋"));

        let toggled = resolve_setup_choice(
            setup,
            setup_command(SetupIntent::ToggleTalent, "frugal_hands"),
            &bundle,
        )
        .expect("toggle selected talent off");
        assert_eq!(
            toggled.setup.selected_talent_ids,
            vec!["steady_mind".to_string(), "quiet_observer".to_string()]
        );
        assert!(!toggled.view.confirm_enabled);
    }

    #[test]
    fn sprint3_sandbox_if_can_select_strong_if_without_granting_hard_rewards() {
        let bundle = starter_content_bundle();
        let mut setup = create_setup_run(RunMode::SandboxIf, &bundle).expect("create setup run");
        let view = build_setup_view(&setup, &bundle).expect("build setup view");
        assert!(view
            .talent_candidates
            .iter()
            .any(|talent| talent.id == "reborn_memory_fragment"));
        assert!(view
            .talent_candidates
            .iter()
            .any(|talent| talent.id == "vital_gu_omen"));

        setup = resolve_setup_choice(
            setup,
            setup_command(SetupIntent::SelectOrigin, "qingmao_branch_child"),
            &bundle,
        )
        .expect("select origin")
        .setup;
        for talent_id in [
            "reborn_memory_fragment",
            "inheritance_scent",
            "vital_gu_omen",
        ] {
            setup = resolve_setup_choice(
                setup,
                setup_command(SetupIntent::ToggleTalent, talent_id),
                &bundle,
            )
            .expect("select sandbox talent")
            .setup;
        }

        let state = confirm_setup_run(setup, &bundle).expect("confirm setup run");
        let summary = state.setup_summary.expect("setup summary");
        assert_eq!(summary.talent_ids.len(), 3);
        assert!(summary
            .talent_ids
            .contains(&"reborn_memory_fragment".to_string()));
        assert_eq!(state.build.vital_gu.status, VitalGuStatus::NotEstablished);
        assert_eq!(state.build.vital_gu.instance_id, None);
        assert!(state
            .ledger
            .iter()
            .all(|entry| !entry.text.contains("完整传承")));
        assert!(state
            .ledger
            .iter()
            .all(|entry| !entry.text.contains("方源")));
    }

    #[test]
    fn sprint3_confirm_setup_generates_s0_state_with_summary_and_deduped_resources() {
        let bundle = starter_content_bundle();
        let mut setup = create_setup_run(RunMode::CanonStrict, &bundle).expect("create setup run");

        setup = resolve_setup_choice(
            setup,
            setup_command(SetupIntent::SelectOrigin, "academy_plain_child"),
            &bundle,
        )
        .expect("select origin")
        .setup;
        for talent_id in ["steady_mind", "quiet_observer", "moonlight_pacing"] {
            setup = resolve_setup_choice(
                setup,
                setup_command(SetupIntent::ToggleTalent, talent_id),
                &bundle,
            )
            .expect("select talent")
            .setup;
        }

        let state = confirm_setup_run(setup, &bundle).expect("confirm setup");
        let summary = state.setup_summary.as_ref().expect("setup summary");

        assert!(state.character.aperture_opened);
        assert_eq!(state.resources.primeval_stones, 3);
        assert_eq!(state.risk.exposure, 2);
        assert_eq!(summary.origin_id, "academy_plain_child");
        assert_eq!(
            summary.resource_package_ids,
            vec!["s0_opening_plain_supplies".to_string()]
        );
        assert_eq!(summary.attributes.get("aptitude"), Some(&7));
        assert_eq!(summary.attributes.get("wit"), Some(&7));
        assert_eq!(summary.attributes.get("luck"), Some(&6));
        assert!(state
            .ledger
            .iter()
            .any(|entry| entry.text.contains("开窍大典")));
        assert!(state
            .ledger
            .iter()
            .any(|entry| entry.text.contains("学堂普通子弟")));
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

    #[test]
    fn content_bundle_rejects_movement_endpoint_outside_node_index() {
        let mut source = valid_content_source();
        source.movements[0].to = "missing_node".to_string();

        let error = ContentBundle::from_source(source).expect_err("bad movement should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("to node 'missing_node' not found"));
    }

    #[test]
    fn content_bundle_rejects_duplicate_narrative_ids() {
        let mut source = valid_content_source();
        source.narratives.push(source.narratives[0].clone());

        let error =
            ContentBundle::from_source(source).expect_err("duplicate narrative should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("duplicate narrative id"));
    }

    #[test]
    fn content_bundle_rejects_empty_narrative_text() {
        let mut source = valid_content_source();
        source.narratives[0].text = "   ".to_string();

        let error = ContentBundle::from_source(source).expect_err("empty narrative should fail");

        assert_eq!(error.kind, CommandErrorKind::Content);
        assert!(error
            .diagnostics
            .unwrap_or_default()
            .contains("field 'text' is empty"));
    }

    fn valid_content_source() -> ContentSource {
        ContentSource {
            content_id: "s0.qingmao.foundation".to_string(),
            version: STARTER_CONTENT_VERSION.to_string(),
            title: "青茅山 Sprint 0 内容骨架".to_string(),
            stage: "s0".to_string(),
            entry_scene_id: "academy_gate".to_string(),
            nodes: vec![
                node(
                    "academy_gate",
                    "学堂门前",
                    "low",
                    EvidenceLevel::CanonInferred,
                    all_modes(),
                    &["node", "academy"],
                ),
                node(
                    "moonlight_corner",
                    "月光修行角",
                    "low",
                    EvidenceLevel::CanonInferred,
                    all_modes(),
                    &["node", "moonlight"],
                ),
            ],
            actions: vec![action(
                "scout_academy",
                "观察学堂风声",
                ActionIntent::Scout,
                Some("academy_gate"),
                EvidenceLevel::CanonInferred,
                all_modes(),
                &["action", "scout"],
            )],
            routes: vec![route(
                "moonlight_entry",
                "月光修行入口",
                "moonlight",
                &["scout_academy"],
                EvidenceLevel::CanonInferred,
                all_modes(),
            )],
            windows: vec![window("day1_morning_free", 1, "清晨", 2)],
            movements: vec![movement(
                "academy_to_moonlight",
                "academy_gate",
                "moonlight_corner",
                0,
                0,
                0,
                None,
                EvidenceLevel::CanonInferred,
                all_modes(),
                &["movement", "near"],
            )],
            encounters: Vec::new(),
            narratives: vec![narrative(
                "s0.action.scout.default",
                "你从内容包里听见一段经策展的本地风声。",
                EvidenceLevel::CanonInferred,
                all_modes(),
                &["narrative", "scout"],
            )],
            origins: starter_origins(),
            talents: starter_talents(),
            attribute_profiles: starter_attribute_profiles(),
            opening_rite_outcomes: starter_opening_rite_outcomes(),
            initial_resource_packages: starter_initial_resource_packages(),
        }
    }

    #[test]
    fn action_response_serializes_projection_without_full_game_state() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("action should resolve");
        let response_json = serde_json::to_value(&result.response).expect("response serializes");

        assert!(response_json.get("projection").is_some());
        assert!(response_json.get("performance").is_some());
        assert!(response_json.get("state").is_none());
        assert!(response_json.get("pipeline_trace").is_none());
        assert!(response_json
            .pointer("/projection/narrative_boundary/runtime_ai_enabled")
            .is_some());
    }

    #[test]
    fn local_narrative_template_overrides_rust_fallback_text() {
        let mut source = starter_content_source();
        let narrative = source
            .narratives
            .iter_mut()
            .find(|item| item.id == "s0.action.scout.academy_gate")
            .expect("starter source should include academy scout narrative");
        narrative.text = "内容包覆写：你把暗口风声压进账页。".to_string();
        let bundle = ContentBundle::from_source(source).expect("source should remain valid");
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("scout should resolve through local narrative renderer");

        assert_eq!(
            result.state.ledger.last().map(|entry| entry.text.as_str()),
            Some("内容包覆写：你把暗口风声压进账页。")
        );
        assert_eq!(
            result.response.projection.scene_text,
            "内容包覆写：你把暗口风声压进账页。"
        );
    }

    #[test]
    fn missing_local_narrative_uses_rust_fallback_without_blocking() {
        let mut source = starter_content_source();
        source.narratives.clear();
        let bundle = ContentBundle::from_source(source).expect("empty narratives are allowed");
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("missing narrative templates should not block local resolution");

        assert!(result
            .state
            .ledger
            .last()
            .expect("ledger entry")
            .text
            .contains("暗口二字"));
    }

    #[test]
    fn resolve_action_has_no_runtime_ai_dependency() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let result = resolve_action(
            state,
            command(ActionIntent::Cultivate, Some("academy_gate")),
            &bundle,
        )
        .expect("local rules and local narrative should not require AI config");

        assert!(
            !result
                .response
                .projection
                .narrative_boundary
                .runtime_ai_enabled
        );
        assert!(result
            .response
            .projection
            .narrative_boundary
            .source
            .contains("内容包"));
    }

    #[test]
    fn resolve_action_records_explicit_pipeline_trace() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Scout, Some("academy_gate")),
            &bundle,
        )
        .expect("action should resolve");

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
    fn save_envelope_records_stage_and_current_checkpoint_boundaries() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let envelope = SaveEnvelope::from_state("slot_0", state.clone());

        assert_eq!(envelope.checkpoints.len(), 2);
        assert_eq!(envelope.metadata.save_version, "sprint0-save-v2");

        let stage = envelope
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.kind == SaveCheckpointKind::StageBoundary)
            .expect("stage checkpoint should exist");
        assert_eq!(stage.checkpoint_id, "s0_qingmao_foundation_stage");
        assert_eq!(stage.restore_policy, SaveRestorePolicy::StageCheckpoint);
        assert_eq!(stage.rules_version, RULES_VERSION);
        assert_eq!(stage.content_version, STARTER_CONTENT_VERSION);
        assert_eq!(stage.ledger_len, state.ledger.len());

        let current = envelope
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.kind == SaveCheckpointKind::CurrentSnapshot)
            .expect("current checkpoint should exist");
        assert_eq!(current.checkpoint_id, "sprint_0_current");
        assert_eq!(current.restore_policy, SaveRestorePolicy::CurrentSnapshot);
        assert_eq!(current.window_id, state.time.window_id);
        assert_eq!(current.window_index, state.time.window_index);
        assert_eq!(current.free_rounds_elapsed, state.time.free_rounds_elapsed);
        assert_eq!(current.node_id, state.world.current_node_id);
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

        let mut mode_mismatch = SaveEnvelope::from_state("slot_0", state.clone());
        mode_mismatch.metadata.mode = RunMode::SandboxIf;
        let mode_error = mode_mismatch
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("mode mismatch should fail");
        assert_eq!(mode_error.kind, CommandErrorKind::Save);

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
    fn save_envelope_rejects_checkpoint_boundary_mismatch() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let mut missing = SaveEnvelope::from_state("slot_0", state.clone());
        missing.checkpoints.clear();
        let missing_error = missing
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("missing checkpoint should fail");
        assert_eq!(missing_error.kind, CommandErrorKind::Save);
        assert!(missing_error.message.contains("检查点"));

        let mut duplicate = SaveEnvelope::from_state("slot_0", state.clone());
        duplicate.checkpoints.push(duplicate.checkpoints[0].clone());
        let duplicate_error = duplicate
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("duplicate checkpoint should fail");
        assert_eq!(duplicate_error.kind, CommandErrorKind::Save);

        let mut current_mismatch = SaveEnvelope::from_state("slot_0", state.clone());
        current_mismatch
            .checkpoints
            .iter_mut()
            .find(|checkpoint| checkpoint.kind == SaveCheckpointKind::CurrentSnapshot)
            .expect("current checkpoint")
            .node_id = "forged_node".to_string();
        let current_error = current_mismatch
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("current checkpoint mismatch should fail");
        assert_eq!(current_error.kind, CommandErrorKind::Save);

        let mut version_mismatch = SaveEnvelope::from_state("slot_0", state);
        version_mismatch.checkpoints[0].content_version = "wrong-content".to_string();
        let version_error = version_mismatch
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect_err("checkpoint version mismatch should fail");
        assert_eq!(version_error.kind, CommandErrorKind::Save);
    }

    #[test]
    fn save_write_result_serializes_minimum_receipt() {
        let result = SaveWriteResult::new(
            "slot_0",
            "saves/sprint0/slot_0.json",
            STARTER_CONTENT_VERSION,
            vec!["s0_qingmao_foundation_stage".to_string()],
            "sprint_0_current",
        );
        let json = serde_json::to_value(&result).expect("write result serializes");

        assert_eq!(json["slot_id"], "slot_0");
        assert_eq!(json["save_version"], SAVE_FORMAT_VERSION);
        assert_eq!(json["rules_version"], RULES_VERSION);
        assert_eq!(json["content_version"], STARTER_CONTENT_VERSION);
        assert_eq!(json["checkpoint_count"], 2);
        assert_eq!(json["current_checkpoint_id"], "sprint_0_current");
        assert_eq!(
            json["stage_checkpoint_ids"][0],
            "s0_qingmao_foundation_stage"
        );
        assert_eq!(json["written"], true);
    }

    #[test]
    fn projection_exposes_save_checkpoint_boundary() {
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let projection = build_projection(&state);

        assert_eq!(projection.save_view.save_version, SAVE_FORMAT_VERSION);
        assert_eq!(projection.save_view.rules_version, RULES_VERSION);
        assert_eq!(
            projection.save_view.content_version,
            STARTER_CONTENT_VERSION
        );
        assert_eq!(
            projection.save_view.current_checkpoint_id,
            "sprint_0_current"
        );
        assert_eq!(projection.save_view.stage_checkpoint_ids.len(), 1);
        assert!(projection.save_view.rollback_policy.contains("阶段检查点"));
    }

    #[test]
    fn wait_advances_window_without_carrying_unused_ap() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
            .expect("wait should consume the current window");

        assert_eq!(result.state.time.window_id, "day1_midday_free");
        assert_eq!(result.state.time.window_index, 1);
        assert_eq!(result.state.time.chapter_day, 1);
        assert_eq!(result.state.time.period, "日中");
        assert_eq!(result.state.time.ap, 2);
        assert_eq!(result.state.time.free_rounds_elapsed, 1);
    }

    #[test]
    fn movement_uses_edge_costs_instead_of_fixed_ap_tax() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Move, Some("moonlight_corner")),
            &bundle,
        )
        .expect("near movement should resolve");

        assert_eq!(result.state.world.current_node_id, "moonlight_corner");
        assert_eq!(
            result.state.time.ap, 2,
            "near movement should not cost fixed AP"
        );
        assert_eq!(result.state.risk.exposure, 0);
    }

    #[test]
    fn infirmary_movement_compresses_arrival_ap_and_adds_exposure() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = resolve_action(
            state,
            command(ActionIntent::Move, Some("infirmary_lane")),
            &bundle,
        )
        .expect("infirmary movement should resolve");

        assert_eq!(result.state.world.current_node_id, "infirmary_lane");
        assert_eq!(result.state.time.ap, 1);
        assert_eq!(result.state.risk.exposure, 1);
    }

    #[test]
    fn hidden_and_if_nodes_obey_period_and_mode_gates() {
        let bundle = starter_content_bundle();
        let morning = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let blackmarket_error = resolve_action(
            morning,
            command(ActionIntent::Move, Some("blackmarket_hint")),
            &bundle,
        )
        .expect_err("blackmarket should require deep night");
        assert_eq!(blackmarket_error.kind, CommandErrorKind::Validation);

        let mut canon_night = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        set_deep_night(&mut canon_night);
        let inheritance_error = resolve_action(
            canon_night,
            command(ActionIntent::Move, Some("inheritance_rumor")),
            &bundle,
        )
        .expect_err("canon strict should reject sandbox-only inheritance node");
        assert_eq!(inheritance_error.kind, CommandErrorKind::Validation);

        let mut sandbox_night = create_run(RunMode::SandboxIf, STARTER_CONTENT_VERSION);
        set_deep_night(&mut sandbox_night);
        let inheritance = resolve_action(
            sandbox_night,
            command(ActionIntent::Move, Some("inheritance_rumor")),
            &bundle,
        )
        .expect("sandbox_if can chase the inheritance rumor at high risk");
        assert_eq!(inheritance.state.world.current_node_id, "inheritance_rumor");
        assert!(inheritance.state.risk.exposure >= 3);
    }

    #[test]
    fn action_costs_are_computed_by_rust_not_declared_cost() {
        let bundle = starter_content_bundle();
        let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        let mut cultivate = command(ActionIntent::Cultivate, Some("academy_gate"));
        cultivate.declared_cost = DeclaredCost {
            ap: 0,
            primeval_stones: 0,
            exposure_risk: 0,
        };

        let result = resolve_action(state, cultivate, &bundle).expect("cultivation should resolve");

        assert_eq!(result.state.time.ap, 1);
        assert_eq!(result.state.resources.primeval_stones, 2);
        assert!(result.state.build.survival_route.contains("月光"));

        let mut broke = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        broke.resources.primeval_stones = 0;
        let error = resolve_action(
            broke,
            command(ActionIntent::Cultivate, Some("academy_gate")),
            &bundle,
        )
        .expect_err("cultivation should require primeval stones");
        assert_eq!(error.kind, CommandErrorKind::Validation);
    }

    #[test]
    fn s0_economy_debt_and_blackmarket_rules_apply() {
        let bundle = starter_content_bundle();
        let at_merit = resolve_action(
            create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION),
            command(ActionIntent::Move, Some("merit_notice")),
            &bundle,
        )
        .expect("merit notice movement should resolve");
        let merit = resolve_action(
            at_merit.state,
            command(ActionIntent::Scout, Some("merit_notice")),
            &bundle,
        )
        .expect("merit notice scout should resolve");
        assert_eq!(merit.state.resources.merit, 1);

        let mut wounded = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        wounded.world.current_node_id = "infirmary_lane".to_string();
        let recovered = resolve_action(
            wounded,
            command(ActionIntent::Recover, Some("infirmary_lane")),
            &bundle,
        )
        .expect("infirmary recovery should resolve");
        assert_eq!(recovered.state.debts_and_credit.infirmary_debt, 1);
        assert_eq!(recovered.state.debts_and_credit.favor_debt, 1);

        let mut trader = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        trader.knowledge.blackmarket_route_known = true;
        trader.world.current_node_id = "blackmarket_hint".to_string();
        set_deep_night(&mut trader);
        let traded = resolve_action(
            trader,
            command(ActionIntent::Trade, Some("blackmarket_hint")),
            &bundle,
        )
        .expect("deep-night blackmarket trade should resolve");
        assert_eq!(traded.state.resources.primeval_stones, 2);
        assert_eq!(traded.state.resources.materials, 1);
        assert!(traded.state.risk.exposure >= 2);
    }

    #[test]
    fn blackmarket_deep_night_movement_triggers_extortion_without_resetting_ap() {
        let bundle = starter_content_bundle();
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.knowledge.blackmarket_route_known = true;
        set_deep_night(&mut state);

        let result = resolve_action(
            state,
            command(ActionIntent::Move, Some("blackmarket_hint")),
            &bundle,
        )
        .expect("deep-night blackmarket movement should trigger encounter");

        let active = result
            .state
            .encounters
            .active
            .as_ref()
            .expect("extortion encounter should be active");
        assert_eq!(active.encounter_id, "blackmarket_extortion");
        assert_eq!(active.encounter_type, EncounterType::Extortion);
        assert_eq!(result.state.time.ap, 1, "encounter must not reset AP");
        assert_eq!(result.state.world.current_node_id, "blackmarket_hint");
        assert_eq!(
            result.response.projection.active_encounter_id.as_deref(),
            Some("blackmarket_extortion")
        );
    }

    #[test]
    fn active_encounter_blocks_ordinary_actions_until_decided() {
        let bundle = starter_content_bundle();
        let state = state_at_blackmarket_extortion(&bundle);

        let error = resolve_action(
            state,
            command(ActionIntent::Scout, Some("blackmarket_hint")),
            &bundle,
        )
        .expect_err("ordinary actions should not bypass an active encounter");

        assert_eq!(error.kind, CommandErrorKind::Validation);
    }

    #[test]
    fn retreat_is_better_than_confronting_the_blackmarket_extortion() {
        let bundle = starter_content_bundle();
        let encountered = state_at_blackmarket_extortion(&bundle);

        let retreated = resolve_action(
            encountered.clone(),
            command(ActionIntent::Retreat, Some("blackmarket_extortion")),
            &bundle,
        )
        .expect("retreat should resolve");
        assert!(retreated.state.encounters.active.is_none());
        assert_eq!(retreated.state.character.injury.level, InjuryLevel::Healthy);
        assert_eq!(retreated.state.resources.primeval_stones, 3);
        assert_eq!(retreated.state.world.current_node_id, "academy_gate");
        assert_eq!(retreated.state.time.window_id, "day2_morning_free");
        assert_eq!(retreated.state.time.ap, 2);
        assert!(retreated.state.build.survival_route.contains("退避"));

        let confronted = resolve_action(
            encountered,
            command(ActionIntent::Confront, Some("blackmarket_extortion")),
            &bundle,
        )
        .expect("confront should resolve as trauma-continuable failure");
        assert!(confronted.state.encounters.active.is_none());
        assert_eq!(confronted.state.character.injury.level, InjuryLevel::Heavy);
        assert_eq!(confronted.state.resources.primeval_stones, 2);
        assert_eq!(confronted.state.world.current_node_id, "academy_gate");
        assert_eq!(confronted.state.time.window_id, "day2_morning_free");
        assert_eq!(
            confronted.state.time.ap, 1,
            "heavy injury should compress the next free window"
        );
        assert!(confronted.state.risk.exposure > retreated.state.risk.exposure);
    }

    #[test]
    fn infirmary_recovery_reduces_heavy_and_light_injury() {
        let bundle = starter_content_bundle();
        let mut wounded = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        wounded.world.current_node_id = "infirmary_lane".to_string();
        wounded.character.injury.level = InjuryLevel::Heavy;
        wounded.character.injury.ap_penalty_pending = false;

        let light = resolve_action(
            wounded,
            command(ActionIntent::Recover, Some("infirmary_lane")),
            &bundle,
        )
        .expect("infirmary should reduce heavy injury");
        assert_eq!(light.state.character.injury.level, InjuryLevel::Light);
        assert_eq!(light.state.debts_and_credit.infirmary_debt, 1);
        assert_eq!(light.state.debts_and_credit.favor_debt, 1);

        let healthy = resolve_action(
            light.state,
            command(ActionIntent::Recover, Some("infirmary_lane")),
            &bundle,
        )
        .expect("infirmary should clear light injury");
        assert_eq!(healthy.state.character.injury.level, InjuryLevel::Healthy);
    }

    #[test]
    fn save_envelope_preserves_phase_five_state_boundaries() {
        let bundle = starter_content_bundle();
        let state = resolve_action(
            create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION),
            command(ActionIntent::Move, Some("infirmary_lane")),
            &bundle,
        )
        .expect("movement should resolve")
        .state;

        let encoded = serde_json::to_string(&SaveEnvelope::from_state("slot_0", state.clone()))
            .expect("save envelope serializes");
        let decoded: SaveEnvelope =
            serde_json::from_str(&encoded).expect("save envelope deserializes");

        decoded
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect("phase five save should load");
        assert_eq!(decoded.snapshot.time.window_id, state.time.window_id);
        assert_eq!(decoded.snapshot.time.window_index, state.time.window_index);
        assert_eq!(decoded.snapshot.resources, state.resources);
        assert_eq!(decoded.snapshot.debts_and_credit, state.debts_and_credit);
        assert_eq!(decoded.snapshot.risk, state.risk);
    }

    #[test]
    fn save_envelope_preserves_active_encounter_and_injury_state() {
        let bundle = starter_content_bundle();
        let mut state = state_at_blackmarket_extortion(&bundle);
        state.character.injury.level = InjuryLevel::Heavy;
        state.character.injury.ap_penalty_pending = true;

        let encoded = serde_json::to_string(&SaveEnvelope::from_state("slot_0", state.clone()))
            .expect("save envelope serializes");
        let decoded: SaveEnvelope =
            serde_json::from_str(&encoded).expect("save envelope deserializes");

        decoded
            .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
            .expect("phase six save should load");
        assert_eq!(decoded.snapshot.encounters, state.encounters);
        assert_eq!(decoded.snapshot.character, state.character);
    }

    fn state_at_blackmarket_extortion(bundle: &ContentBundle) -> GameState {
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.knowledge.blackmarket_route_known = true;
        set_deep_night(&mut state);
        resolve_action(
            state,
            command(ActionIntent::Move, Some("blackmarket_hint")),
            bundle,
        )
        .expect("blackmarket movement should trigger extortion")
        .state
    }

    fn state_at_academy_pressure_without_clue(bundle: &ContentBundle) -> GameState {
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.world.current_node_id = "moonlight_corner".to_string();
        state.build.moonlight_cultivation_marks = 1;
        resolve_action(
            state,
            command(ActionIntent::Cultivate, Some("moonlight_corner")),
            bundle,
        )
        .expect("cultivation should trigger academy pressure")
        .state
    }

    fn state_at_academy_pressure_with_clue(bundle: &ContentBundle) -> GameState {
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.world.current_node_id = "moonlight_corner".to_string();
        state.knowledge.record_clue("rumor_academy_pressure");
        state.build.moonlight_cultivation_marks = 1;
        resolve_action(
            state,
            command(ActionIntent::Cultivate, Some("moonlight_corner")),
            bundle,
        )
        .expect("cultivation should trigger academy pressure")
        .state
    }

    fn state_at_alley_probe(bundle: &ContentBundle) -> GameState {
        let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
        state.risk.exposure = 2;
        resolve_action(
            state,
            command(ActionIntent::Move, Some("clan_alley_rumor")),
            bundle,
        )
        .expect("high exposure alley movement should trigger probe")
        .state
    }

    fn command(intent: ActionIntent, target: Option<&str>) -> ActionCommand {
        ActionCommand {
            actor: "player".to_string(),
            intent,
            target: target.map(str::to_string),
            declared_cost: DeclaredCost::default(),
            context_note: None,
        }
    }

    fn set_deep_night(state: &mut GameState) {
        state.time.window_id = "day1_deep_night_free".to_string();
        state.time.window_index = 3;
        state.time.period = "深夜".to_string();
        state.time.ap = 1;
    }

    fn assert_no_user_visible_english(entries: &[LedgerEntry]) {
        let forbidden_fragments = [
            format!("{} {}", "A", "blackmarket"),
            format!("{} {}", "You", "retreat"),
            format!("{} {}", "You", "harden"),
            format!("{} {}", "Infirmary", "recovery"),
        ];
        for entry in entries {
            for fragment in &forbidden_fragments {
                assert!(!entry.text.contains(fragment), "{}", entry.text);
            }
        }
    }

    fn assert_no_mojibake_markers(context: &str, text: &str) {
        let forbidden_fragments = [
            ("U+FFFD replacement character", "\u{fffd}"),
            ("known mojibake marker U+95C8", "\u{95c8}"),
            ("known mojibake marker U+947A", "\u{947a}"),
            ("known mojibake marker U+7490", "\u{7490}"),
            ("known mojibake marker U+9365", "\u{9365}"),
            ("known mojibake marker U+94D4", "\u{94d4}"),
            ("known mojibake marker U+947E", "\u{947e}"),
            ("known mojibake marker U+5A13", "\u{5a13}"),
            ("known mojibake marker U+699B", "\u{699b}"),
            ("known mojibake marker U+7EC9", "\u{7ec9}"),
            ("known mojibake marker U+947D", "\u{947d}"),
            ("legacy mojibake compatibility marker U+535E", "\u{535e}"),
        ];

        for (name, fragment) in forbidden_fragments {
            assert!(!text.contains(fragment), "{context} contains {name}");
        }
    }

    fn narrative(
        id: &str,
        text: &str,
        evidence: EvidenceLevel,
        modes: Vec<ModePermit>,
        tags: &[&str],
    ) -> ContentNarrativeTemplate {
        ContentNarrativeTemplate {
            id: id.to_string(),
            stage: "s0".to_string(),
            tags: strings(tags),
            evidence,
            modes,
            text: text.to_string(),
        }
    }
}
