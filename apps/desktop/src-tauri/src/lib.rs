use rebrng_game_core::{
    build_projection_with_content as core_build_projection_with_content,
    confirm_setup_run as core_confirm_setup_run, create_run as core_create_run,
    create_setup_run as core_create_setup_run, resolve_action as core_resolve_action,
    resolve_setup_choice as core_resolve_setup_choice, starter_content_bundle, ActionCommand,
    ActionResponse, CommandError, ContentBundle, ContentManifest, GameState, LedgerViewModel,
    PerformanceMetrics, RunMode, RunSetupState, SaveEnvelope, SaveWriteResult, SetupCommand,
    SetupResponse,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use tauri::Manager;

struct ActiveRunState {
    active_run: Mutex<Option<GameState>>,
    active_setup: Mutex<Option<RunSetupState>>,
    content_bundle: ContentBundle,
}

impl Default for ActiveRunState {
    fn default() -> Self {
        Self {
            active_run: Mutex::new(None),
            active_setup: Mutex::new(None),
            content_bundle: starter_content_bundle(),
        }
    }
}

#[tauri::command]
fn create_run(
    mode: Option<String>,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    create_run_in_runtime(mode, runtime.inner())
}

#[tauri::command]
fn create_setup_run(
    mode: Option<String>,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<SetupResponse, CommandError> {
    create_setup_run_in_runtime(mode, runtime.inner())
}

#[tauri::command]
fn resolve_setup_choice(
    command: SetupCommand,
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<SetupResponse, CommandError> {
    resolve_setup_choice_in_runtime(command, runtime.inner())
}

#[tauri::command]
fn confirm_setup_run(
    runtime: tauri::State<'_, ActiveRunState>,
) -> Result<ActionResponse, CommandError> {
    confirm_setup_run_in_runtime(runtime.inner())
}

fn create_run_in_runtime(
    mode: Option<String>,
    runtime: &ActiveRunState,
) -> Result<ActionResponse, CommandError> {
    let run_mode = parse_mode(mode)?;
    let state = core_create_run(run_mode, runtime.content_bundle.manifest.version.clone());
    let response = response_from_state(
        &state,
        &runtime.content_bundle,
        PerformanceMetrics::default(),
    )?;

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = Some(state);

    let mut active_setup = runtime.active_setup.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_setup mutex poisoned: {error}"),
        )
    })?;
    *active_setup = None;

    Ok(response)
}

fn create_setup_run_in_runtime(
    mode: Option<String>,
    runtime: &ActiveRunState,
) -> Result<SetupResponse, CommandError> {
    let run_mode = parse_mode(mode)?;
    let setup = core_create_setup_run(run_mode, &runtime.content_bundle)?;
    let view = rebrng_game_core::build_setup_view(&setup, &runtime.content_bundle)?;
    let response = SetupResponse {
        setup: setup.clone(),
        view,
    };

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = None;

    let mut active_setup = runtime.active_setup.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_setup mutex poisoned: {error}"),
        )
    })?;
    *active_setup = Some(setup);

    Ok(response)
}

fn resolve_setup_choice_in_runtime(
    command: SetupCommand,
    runtime: &ActiveRunState,
) -> Result<SetupResponse, CommandError> {
    let current = {
        let active_setup = runtime.active_setup.lock().map_err(|error| {
            CommandError::internal(
                "运行态锁定失败",
                format!("active_setup mutex poisoned: {error}"),
            )
        })?;
        active_setup
            .clone()
            .ok_or_else(|| CommandError::validation("当前没有 setup run，请先进入人生重开设置"))?
    };

    let response = core_resolve_setup_choice(current, command, &runtime.content_bundle)?;
    let mut active_setup = runtime.active_setup.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_setup mutex poisoned: {error}"),
        )
    })?;
    *active_setup = Some(response.setup.clone());

    Ok(response)
}

fn confirm_setup_run_in_runtime(runtime: &ActiveRunState) -> Result<ActionResponse, CommandError> {
    let setup = {
        let active_setup = runtime.active_setup.lock().map_err(|error| {
            CommandError::internal(
                "运行态锁定失败",
                format!("active_setup mutex poisoned: {error}"),
            )
        })?;
        active_setup
            .clone()
            .ok_or_else(|| CommandError::validation("当前没有 setup run，请先进入人生重开设置"))?
    };

    let state = core_confirm_setup_run(setup, &runtime.content_bundle)?;
    let response = response_from_state(
        &state,
        &runtime.content_bundle,
        PerformanceMetrics::default(),
    )?;

    let mut active_run = runtime.active_run.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_run mutex poisoned: {error}"),
        )
    })?;
    *active_run = Some(state);

    let mut active_setup = runtime.active_setup.lock().map_err(|error| {
        CommandError::internal(
            "运行态锁定失败",
            format!("active_setup mutex poisoned: {error}"),
        )
    })?;
    *active_setup = None;

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
    let mut projection = core_build_projection_with_content(state, &runtime.content_bundle);
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
    let response = response_from_state(&state, &runtime.content_bundle, performance)?;

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
    content_bundle: &ContentBundle,
    mut performance: PerformanceMetrics,
) -> Result<ActionResponse, CommandError> {
    let projection_started = Instant::now();
    let mut projection = core_build_projection_with_content(state, content_bundle);
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

    let stage_checkpoint_ids = envelope
        .checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.kind == rebrng_game_core::SaveCheckpointKind::StageBoundary)
        .map(|checkpoint| checkpoint.checkpoint_id.clone())
        .collect::<Vec<_>>();
    let current_checkpoint_id = envelope
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.kind == rebrng_game_core::SaveCheckpointKind::CurrentSnapshot)
        .map(|checkpoint| checkpoint.checkpoint_id.clone())
        .unwrap_or_else(|| "sprint_0_current".to_string());

    Ok(SaveWriteResult::new(
        slot_id,
        save_path_hint(slot_id)?,
        state.content_version.clone(),
        stage_checkpoint_ids,
        current_checkpoint_id,
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
            create_setup_run,
            resolve_setup_choice,
            confirm_setup_run,
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
    use rebrng_game_core::{CommandErrorKind, STARTER_CONTENT_VERSION};
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
        assert_eq!(result.current_checkpoint_id, "sprint_0_current");
        assert_eq!(
            result.stage_checkpoint_ids,
            vec!["s0_qingmao_foundation_stage".to_string()]
        );
        assert_eq!(result.checkpoint_count, 2);
        assert_eq!(loaded.snapshot, state);
        assert_eq!(loaded.metadata.slot_id, "slot_0");

        fs::remove_dir_all(root).expect("cleanup temp save root");
    }

    #[test]
    fn setup_commands_require_active_setup_before_choice_or_confirm() {
        let runtime = ActiveRunState::default();

        let choice_error = resolve_setup_choice_in_runtime(
            rebrng_game_core::setup_command(
                rebrng_game_core::SetupIntent::SelectOrigin,
                "academy_plain_child",
            ),
            &runtime,
        )
        .expect_err("choice without setup should fail");
        assert_eq!(choice_error.kind, CommandErrorKind::Validation);
        assert!(choice_error.message.contains("当前没有 setup run"));

        let confirm_error =
            confirm_setup_run_in_runtime(&runtime).expect_err("confirm without setup should fail");
        assert_eq!(confirm_error.kind, CommandErrorKind::Validation);
        assert!(confirm_error.message.contains("当前没有 setup run"));
    }

    #[test]
    fn setup_command_flow_confirms_active_run_and_clears_setup() {
        let runtime = ActiveRunState::default();

        let created = create_setup_run_in_runtime(Some("canon_strict".to_string()), &runtime)
            .expect("create setup");
        assert!(!created.view.confirm_enabled);
        assert!(runtime
            .active_run
            .lock()
            .expect("active run lock")
            .is_none());

        resolve_setup_choice_in_runtime(
            rebrng_game_core::setup_command(
                rebrng_game_core::SetupIntent::SelectOrigin,
                "academy_plain_child",
            ),
            &runtime,
        )
        .expect("select origin");
        for talent_id in ["steady_mind", "quiet_observer", "moonlight_pacing"] {
            resolve_setup_choice_in_runtime(
                rebrng_game_core::setup_command(
                    rebrng_game_core::SetupIntent::ToggleTalent,
                    talent_id,
                ),
                &runtime,
            )
            .expect("select talent");
        }

        let response = confirm_setup_run_in_runtime(&runtime).expect("confirm setup");
        assert!(response.projection.scene_text.contains("开窍大典"));
        assert!(runtime
            .active_setup
            .lock()
            .expect("active setup lock")
            .is_none());
        let active_run = runtime
            .active_run
            .lock()
            .expect("active run lock")
            .clone()
            .expect("active run");
        assert!(active_run.setup_summary.is_some());
        assert_eq!(active_run.resources.primeval_stones, 3);
    }

    #[test]
    fn default_create_run_clears_setup_flow() {
        let runtime = ActiveRunState::default();
        create_setup_run_in_runtime(Some("canon_strict".to_string()), &runtime)
            .expect("create setup");
        assert!(runtime
            .active_setup
            .lock()
            .expect("active setup lock")
            .is_some());

        let response = create_run_in_runtime(Some("canon_strict".to_string()), &runtime)
            .expect("create default run");
        assert_eq!(response.projection.current_node_id, "academy_gate");
        assert!(runtime
            .active_setup
            .lock()
            .expect("active setup lock")
            .is_none());
        let active_run = runtime
            .active_run
            .lock()
            .expect("active run lock")
            .clone()
            .expect("active run");
        assert!(active_run.setup_summary.is_none());
    }

    fn unique_temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        std::env::temp_dir().join(format!("rebrng-desktop-save-test-{nanos}"))
    }
}
