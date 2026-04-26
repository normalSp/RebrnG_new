use rebrng_game_core::{
    build_projection as core_build_projection, create_run as core_create_run,
    resolve_action as core_resolve_action, starter_content_manifest, ActionCommand, ActionResponse,
    CommandError, ContentManifest, GameState, LedgerViewModel, PerformanceMetrics, RunMode,
};
use std::sync::Mutex;
use std::time::Instant;

#[derive(Default)]
struct ActiveRunState {
    active_run: Mutex<Option<GameState>>,
}

#[tauri::command]
fn create_run(
    mode: Option<String>,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    let run_mode = parse_mode(mode)?;
    let manifest = starter_content_manifest();
    let state = core_create_run(run_mode, manifest.version);
    let response = response_from_state(state, PerformanceMetrics::default())?;

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = Some(response.state.clone());

    Ok(response)
}

#[tauri::command]
fn resolve_action(
    command: ActionCommand,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;

    let current = active_run
        .clone()
        .ok_or_else(|| CommandError::validation("当前没有 active run，请先新建单局"))?;
    let response = core_resolve_action(current, command)?;
    *active_run = Some(response.state.clone());

    Ok(response)
}

#[tauri::command]
fn build_projection(
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<LedgerViewModel, CommandError> {
    let active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    let state = active_run
        .as_ref()
        .ok_or_else(|| CommandError::validation("当前没有 active run，请先新建单局"))?;

    let started = Instant::now();
    let mut projection = core_build_projection(state);
    projection.performance.projection_ms = started.elapsed().as_millis() as u64;
    Ok(projection)
}

#[tauri::command]
fn get_content_manifest() -> ContentManifest {
    starter_content_manifest()
}

fn parse_mode(mode: Option<String>) -> Result<RunMode, CommandError> {
    match mode.as_deref().unwrap_or("canon_strict") {
        "canon_strict" => Ok(RunMode::CanonStrict),
        "sandbox_if" => Ok(RunMode::SandboxIf),
        other => Err(CommandError::validation(format!(
            "未知模式：{other}，仅支持 canon_strict 或 sandbox_if"
        ))),
    }
}

fn response_from_state(
    state: GameState,
    mut performance: PerformanceMetrics,
) -> Result<ActionResponse, CommandError> {
    let projection_started = Instant::now();
    let mut projection = core_build_projection(&state);
    performance.projection_ms = projection_started.elapsed().as_millis() as u64;
    projection.performance = performance.clone();

    Ok(ActionResponse {
        state,
        projection,
        performance,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ActiveRunState::default())
        .invoke_handler(tauri::generate_handler![
            create_run,
            resolve_action,
            build_projection,
            get_content_manifest
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
