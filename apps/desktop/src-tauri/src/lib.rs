use rebrng_game_core::{
    build_projection as core_build_projection, create_run as core_create_run,
    resolve_action as core_resolve_action, starter_content_bundle, ActionCommand, ActionResponse,
    CommandError, ContentBundle, ContentManifest, GameState, LedgerViewModel, PerformanceMetrics,
    RunMode, SaveEnvelope, SaveWriteResult,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use tauri::Manager;

struct ActiveRunState {
    active_run: Mutex<Option<GameState>>,
    content_bundle: ContentBundle,
}

impl Default for ActiveRunState {
    fn default() -> Self {
        Self {
            active_run: Mutex::new(None),
            content_bundle: starter_content_bundle(),
        }
    }
}

#[tauri::command]
fn create_run(
    mode: Option<String>,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    let run_mode = parse_mode(mode)?;
    let state = core_create_run(run_mode, runtime.content_bundle.manifest.version.clone());
    let response = response_from_state(&state, PerformanceMetrics::default())?;

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = Some(state);

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
    let result = core_resolve_action(current, command, &runtime.content_bundle)?;
    *active_run = Some(result.state);

    Ok(result.response)
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
fn get_content_manifest(runtime: tauri::State<'_, ActiveRunState>) -> ContentManifest {
    runtime.content_bundle.manifest.clone()
}

#[tauri::command]
fn write_save(
    slot_id: String,
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<SaveWriteResult, CommandError> {
    let state = {
        let active_run = runtime.active_run.lock().map_err(|error| {
            CommandError::internal(
                "运行态锁定失败",
                format!("active_run mutex poisoned: {error}"),
            )
        })?;
        active_run
            .as_ref()
            .cloned()
            .ok_or_else(|| CommandError::validation("当前没有 active run，请先新建单局"))?
    };

    let root = save_root_from_app(&app_handle)?;
    write_save_to_root(&root, &slot_id, &state)
}

#[tauri::command]
fn load_save(
    slot_id: String,
    app_handle: tauri::AppHandle,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    let started = Instant::now();
    let root = save_root_from_app(&app_handle)?;
    let envelope = load_save_from_root(&root, &slot_id, &runtime.content_bundle.manifest.version)?;
    let state = envelope.snapshot;
    let performance = PerformanceMetrics {
        save_load_ms: started.elapsed().as_millis() as u64,
        ..PerformanceMetrics::default()
    };
    let response = response_from_state(&state, performance)?;

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = Some(state);

    Ok(response)
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
    state: &GameState,
    mut performance: PerformanceMetrics,
) -> Result<ActionResponse, CommandError> {
    let projection_started = Instant::now();
    let mut projection = core_build_projection(state);
    performance.projection_ms = projection_started.elapsed().as_millis() as u64;
    projection.performance = performance.clone();

    Ok(ActionResponse {
        projection,
        performance,
    })
}

fn save_root_from_app(app_handle: &tauri::AppHandle) -> Result<PathBuf, CommandError> {
    app_handle.path().app_data_dir().map_err(|error| {
        CommandError::io(
            "无法定位应用数据目录",
            format!("app_data_dir resolution failed: {error}"),
        )
    })
}

fn validate_slot_id(slot_id: &str) -> Result<String, CommandError> {
    if slot_id.is_empty() || slot_id.len() > 64 {
        return Err(CommandError::validation("存档槽位必须是 1 到 64 个字符"));
    }

    if !slot_id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
    {
        return Err(CommandError::validation(
            "存档槽位只允许 ASCII 字母、数字、_ 和 -",
        ));
    }

    Ok(slot_id.to_string())
}

fn save_path_for_root(root: &Path, slot_id: &str) -> Result<PathBuf, CommandError> {
    let safe_slot_id = validate_slot_id(slot_id)?;
    Ok(root
        .join("saves")
        .join("sprint0")
        .join(format!("{safe_slot_id}.json")))
}

fn write_save_to_root(
    root: &Path,
    slot_id: &str,
    state: &GameState,
) -> Result<SaveWriteResult, CommandError> {
    let path = save_path_for_root(root, slot_id)?;
    let parent = path.parent().ok_or_else(|| {
        CommandError::io(
            "无法计算存档目录",
            format!("save path has no parent: {}", path.display()),
        )
    })?;
    fs::create_dir_all(parent).map_err(|error| {
        CommandError::io("创建存档目录失败", format!("{}: {error}", parent.display()))
    })?;

    let envelope = SaveEnvelope::from_state(slot_id, state.clone());
    envelope.validate_for_load(slot_id, &state.content_version)?;
    let json = serde_json::to_string_pretty(&envelope).map_err(|error| {
        CommandError::save("序列化存档失败", format!("serde_json error: {error}"))
    })?;

    let temp_path = path.with_extension("json.tmp");
    fs::write(&temp_path, json).map_err(|error| {
        CommandError::io(
            "写入临时存档失败",
            format!("{}: {error}", temp_path.display()),
        )
    })?;

    if path.exists() {
        fs::remove_file(&path).map_err(|error| {
            CommandError::io("替换旧存档失败", format!("{}: {error}", path.display()))
        })?;
    }

    fs::rename(&temp_path, &path).map_err(|error| {
        CommandError::io(
            "提交存档文件失败",
            format!("{} -> {}: {error}", temp_path.display(), path.display()),
        )
    })?;

    Ok(SaveWriteResult::new(
        slot_id,
        save_path_hint(slot_id)?,
        state.content_version.clone(),
    ))
}

fn load_save_from_root(
    root: &Path,
    slot_id: &str,
    expected_content_version: &str,
) -> Result<SaveEnvelope, CommandError> {
    let path = save_path_for_root(root, slot_id)?;
    let json = fs::read_to_string(&path).map_err(|error| {
        CommandError::io("读取存档失败", format!("{}: {error}", path.display()))
    })?;
    let envelope: SaveEnvelope = serde_json::from_str(&json).map_err(|error| {
        CommandError::save("解析存档失败", format!("serde_json error: {error}"))
    })?;
    envelope.validate_for_load(slot_id, expected_content_version)?;
    Ok(envelope)
}

fn save_path_hint(slot_id: &str) -> Result<String, CommandError> {
    let safe_slot_id = validate_slot_id(slot_id)?;
    Ok(format!("saves/sprint0/{safe_slot_id}.json"))
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
            get_content_manifest,
            write_save,
            load_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rebrng_game_core::STARTER_CONTENT_VERSION;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn save_slot_id_allows_only_safe_ascii_names() {
        assert_eq!(validate_slot_id("slot_0").expect("valid slot"), "slot_0");
        assert_eq!(
            validate_slot_id("abc-XYZ_123").expect("valid mixed slot"),
            "abc-XYZ_123"
        );

        assert!(validate_slot_id("").is_err());
        assert!(validate_slot_id("../slot_0").is_err());
        assert!(validate_slot_id("slot 0").is_err());
        assert!(validate_slot_id(&"a".repeat(65)).is_err());
    }

    #[test]
    fn save_path_stays_inside_sprint0_directory() {
        let root = unique_temp_dir();
        let path = save_path_for_root(&root, "slot_0").expect("save path");

        assert!(path.ends_with("saves/sprint0/slot_0.json"));
        assert!(path.starts_with(&root));
    }

    #[test]
    fn save_envelope_file_round_trips_state() {
        let root = unique_temp_dir();
        let state = core_create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

        let result = write_save_to_root(&root, "slot_0", &state).expect("write save");
        let loaded =
            load_save_from_root(&root, "slot_0", STARTER_CONTENT_VERSION).expect("load save");

        assert!(result.written);
        assert_eq!(loaded.snapshot, state);
        assert_eq!(loaded.metadata.slot_id, "slot_0");

        fs::remove_dir_all(root).expect("cleanup temp save root");
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        std::env::temp_dir().join(format!("rebrng-desktop-save-test-{nanos}"))
    }
}
