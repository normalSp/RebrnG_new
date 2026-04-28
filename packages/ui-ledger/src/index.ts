export type CommandErrorKind = "validation" | "content" | "save" | "io" | "internal";

export type WindowType = "free" | "anchor";

export type RunMode = "canon_strict" | "sandbox_if";

export type EvidenceLevel =
  | "canon_explicit"
  | "canon_inferred"
  | "gameplay_extrapolated"
  | "sandbox_if";

export type ModePermit = "canon_strict" | "sandbox_if";

export type ActionIntent =
  | "move"
  | "cultivate"
  | "scout"
  | "recover"
  | "trade"
  | "retreat"
  | "confront"
  | "yield"
  | "argue"
  | "delay"
  | "frame"
  | "claim_gu"
  | "refine_gu"
  | "inspect_gu"
  | "wait";

export type InjuryLevel = "healthy" | "light" | "heavy";

export type EncounterType = "extortion" | "public_pressure" | "probe";

export type TalentIntensity = "mild" | "strong_if";

export type SetupIntent = "select_origin" | "toggle_talent";

export interface PerformanceMetrics {
  resolve_action_ms: number;
  projection_ms: number;
  save_load_ms: number;
  bundle_load_ms: number;
}

export interface LedgerEntry {
  kind: string;
  text: string;
}

export interface StatusMarkerView {
  label: string;
  value: string;
  tone: string;
}

export interface BuildLedgerView {
  survival_route: string;
  main_path: string;
  dao_mark_note: string;
  core_gu: string;
  support_gu: string;
  vital_gu: string;
  maintenance_pressure: string;
  gap_summary: string;
}

export interface GuLedgerView {
  owned_count: number;
  moonlight_gu_status: string;
  moonlight_container: string;
  moonlight_condition: string;
  moonlight_feeding: string;
  core_gu_candidate: string;
  vital_gu_status: string;
  entries: string[];
}

export interface FactionRelationshipView {
  family_pressure: string;
  infirmary_debt: string;
  favor_debt: string;
  blackmarket_access: string;
}

export type ActionChoiceGroup =
  | "encounter"
  | "movement"
  | "cultivation"
  | "information"
  | "recovery"
  | "trade"
  | "wait";

export type ActionChoiceTone =
  | "normal"
  | "safe"
  | "risky"
  | "danger"
  | "blocked";

export interface ActionChoiceView {
  id: string;
  label: string;
  intent: ActionIntent;
  target?: string | null;
  enabled: boolean;
  disabled_reason?: string | null;
  cost_hint: string;
  risk_hint: string;
  group: ActionChoiceGroup;
  tone: ActionChoiceTone;
  consequence_hint: string;
}

export interface NodeSummaryView {
  id: string;
  title: string;
  safety: string;
  current: boolean;
}

export interface NodeLedgerView {
  current_node_id: string;
  current_region_id: string;
  visible_nodes: NodeSummaryView[];
}

export interface SaveLedgerView {
  save_version: string;
  rules_version: string;
  content_version: string;
  rng_state: string;
  migration_state: string;
  checkpoint_count: number;
  current_checkpoint_id: string;
  stage_checkpoint_ids: string[];
  rollback_policy: string;
}

export interface NarrativeBoundaryView {
  runtime_ai_enabled: boolean;
  source: string;
  policy: string;
}

export type StageClosureStatus =
  | "in_progress"
  | "foundation_established"
  | "trauma_continuable";

export interface StageClosureView {
  status: StageClosureStatus;
  title: string;
  summary: string;
}

export interface RecentFeedbackView {
  title: string;
  summary: string;
  tone: ActionChoiceTone;
  source_kind: string;
}

export interface ClueLineView {
  id: string;
  label: string;
  summary: string;
  tone: ActionChoiceTone;
}

export interface ClueLedgerView {
  known_clues: ClueLineView[];
  blackmarket_access_summary: string;
}

export interface DeclaredCost {
  ap: number;
  primeval_stones: number;
  exposure_risk: number;
}

export interface ActionCommand {
  actor: "player";
  intent: ActionIntent;
  target?: string | null;
  declared_cost: DeclaredCost;
  context_note?: string | null;
}

export interface SetupCommand {
  intent: SetupIntent;
  target_id: string;
}

export interface RunSetupState {
  run_id: string;
  mode: RunMode;
  content_version: string;
  selected_origin_id?: string | null;
  selected_talent_ids: string[];
  attribute_values: Record<string, number>;
  opening_rite_outcome_id: string;
  completed: boolean;
}

export interface SetupCandidateView {
  id: string;
  title: string;
  summary: string;
  selected: boolean;
  enabled: boolean;
  disabled_reason?: string | null;
  evidence: EvidenceLevel;
  modes: ModePermit[];
}

export interface SetupTalentCandidateView {
  id: string;
  title: string;
  summary: string;
  intensity: TalentIntensity;
  selected: boolean;
  enabled: boolean;
  disabled_reason?: string | null;
  pressure_note: string;
  route_tags: string[];
  evidence: EvidenceLevel;
  modes: ModePermit[];
}

export interface SetupAttributeView {
  id: string;
  label: string;
  summary: string;
  value: number;
  min: number;
  max: number;
}

export interface SetupResourcePreview {
  primeval_stones: number;
  materials: number;
  merit: number;
  infirmary_debt: number;
  favor_debt: number;
  organization_debt: number;
  trading_credit: number;
  exposure: number;
  resource_package_ids: string[];
}

export interface DialogueTimelineView {
  stage_title: string;
  paragraphs: string[];
  previous_choice_title?: string | null;
  previous_result_summary?: string | null;
  available_actions_summary: string[];
  latest_ledger_delta?: string | null;
  mode_gate_hint: string;
  source_summary: string;
  tone: ActionChoiceTone;
}

export interface SetupViewModel {
  mode: RunMode;
  content_version: string;
  origin_candidates: SetupCandidateView[];
  talent_candidates: SetupTalentCandidateView[];
  attributes: SetupAttributeView[];
  resource_preview: SetupResourcePreview;
  selected_origin_id?: string | null;
  selected_talent_ids: string[];
  opening_rite_outcome_id: string;
  opening_rite_title: string;
  opening_rite_summary: string;
  confirm_enabled: boolean;
  confirm_blockers: string[];
  dialogue: DialogueTimelineView;
}

export interface SetupResponse {
  setup: RunSetupState;
  view: SetupViewModel;
}

export interface LedgerViewModel {
  scene_text: string;
  dialogue: DialogueTimelineView;
  current_day: number;
  current_period: string;
  window_id: string;
  window_type: WindowType;
  available_ap: number;
  next_anchor_pressure: string;
  current_node_id: string;
  primeval_stones: number;
  materials: number;
  merit: number;
  exposure: number;
  debt_pressure: number;
  build_summary: string;
  status_markers: StatusMarkerView[];
  build_view: BuildLedgerView;
  gu_view: GuLedgerView;
  relationship_view: FactionRelationshipView;
  save_view: SaveLedgerView;
  action_choices: ActionChoiceView[];
  node_view: NodeLedgerView;
  injury_level: InjuryLevel;
  active_encounter_id?: string | null;
  active_encounter_type?: EncounterType | null;
  active_encounter_known_risk?: string | null;
  active_encounter_decisions: ActionIntent[];
  ledger_entries: LedgerEntry[];
  recent_feedback?: RecentFeedbackView | null;
  clue_view: ClueLedgerView;
  narrative_boundary: NarrativeBoundaryView;
  stage_closure: StageClosureView;
  performance: PerformanceMetrics;
}

export interface ActionResponse {
  projection: LedgerViewModel;
  performance: PerformanceMetrics;
}

export interface SaveWriteResult {
  slot_id: string;
  path_hint: string;
  save_version: string;
  rules_version: string;
  content_version: string;
  checkpoint_count: number;
  current_checkpoint_id: string;
  stage_checkpoint_ids: string[];
  written: boolean;
}

export interface CommandError {
  kind: CommandErrorKind;
  message: string;
  diagnostics?: string | null;
}

export { LedgerShell } from "./LedgerShell";
