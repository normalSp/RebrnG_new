export type CommandErrorKind = "validation" | "content" | "save" | "io" | "internal";

export type WindowType = "free" | "anchor";

export type ActionIntent =
  | "move"
  | "cultivate"
  | "scout"
  | "recover"
  | "trade"
  | "retreat"
  | "confront"
  | "wait";

export type InjuryLevel = "healthy" | "light" | "heavy";

export type EncounterType = "extortion";

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

export interface LedgerViewModel {
  scene_text: string;
  current_day: number;
  current_period: string;
  window_id: string;
  window_type: WindowType;
  available_ap: number;
  current_node_id: string;
  primeval_stones: number;
  materials: number;
  merit: number;
  exposure: number;
  debt_pressure: number;
  build_summary: string;
  injury_level: InjuryLevel;
  active_encounter_id?: string | null;
  active_encounter_type?: EncounterType | null;
  active_encounter_known_risk?: string | null;
  active_encounter_decisions: ActionIntent[];
  ledger_entries: LedgerEntry[];
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
  written: boolean;
}

export interface CommandError {
  kind: CommandErrorKind;
  message: string;
  diagnostics?: string | null;
}

export { LedgerShell } from "./LedgerShell";
