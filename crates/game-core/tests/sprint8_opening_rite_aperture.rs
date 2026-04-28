use rebrng_game_core::{
    build_projection_with_content, confirm_setup_run, create_run, create_setup_run, resolve_action,
    resolve_setup_choice, setup_command, starter_content_bundle, ActionCommand, ActionIntent,
    ApertureWallState, AptitudeGrade, CommandErrorKind, ContentBundle, DeclaredCost, GameState,
    MinorRealm, PrimevalEssenceQuality, RunMode, SaveEnvelope, SetupIntent, WindowType,
    STARTER_CONTENT_VERSION,
};

fn command(intent: ActionIntent, target: Option<&str>) -> ActionCommand {
    ActionCommand {
        actor: "player".to_string(),
        intent,
        target: target.map(str::to_string),
        declared_cost: DeclaredCost::default(),
        context_note: None,
    }
}

fn confirmed_setup_state() -> (GameState, ContentBundle) {
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

    (
        confirm_setup_run(setup, &bundle).expect("confirm setup"),
        bundle,
    )
}

fn resolve(
    state: GameState,
    bundle: &ContentBundle,
    intent: ActionIntent,
    target: &str,
) -> GameState {
    resolve_action(state, command(intent, Some(target)), bundle)
        .unwrap_or_else(|error| panic!("opening action should resolve: {error:?}"))
        .state
}

#[test]
fn setup_confirmation_enters_opening_rite_anchor_before_free_window() {
    let (state, bundle) = confirmed_setup_state();

    assert_eq!(state.time.window_type, WindowType::Anchor);
    assert_eq!(state.time.window_id, "s0_opening_rite_anchor");
    assert_eq!(state.time.ap, 0);
    assert_eq!(state.world.current_node_id, "opening_rite_cave");
    assert!(!state.character.aperture_opened);
    assert!(!state.character.mortal_aperture.opened);

    let projection = build_projection_with_content(&state, &bundle);
    assert!(projection.dialogue.stage_title.contains("开窍大典"));
    assert!(projection.aperture_view.summary.contains("尚未开窍"));
    assert!(projection
        .action_choices
        .iter()
        .any(|choice| choice.intent == ActionIntent::EnterOpeningCave));
    assert!(projection
        .action_choices
        .iter()
        .all(|choice| choice.intent != ActionIntent::ClaimGu));
}

#[test]
fn opening_rite_sequence_creates_mortal_aperture_and_enters_s0_loop() {
    let (state, bundle) = confirmed_setup_state();

    let state = resolve(
        state,
        &bundle,
        ActionIntent::EnterOpeningCave,
        "opening_rite_cave",
    );
    assert!(!state.character.mortal_aperture.opened);

    let state = resolve(
        state,
        &bundle,
        ActionIntent::CrossOpeningRiver,
        "opening_rite_river",
    );
    assert!(!state.character.mortal_aperture.opened);

    let state = resolve(state, &bundle, ActionIntent::ReceiveHopeGu, "hope_gu");
    assert_eq!(state.time.window_type, WindowType::Free);
    assert_eq!(state.time.window_id, "day1_morning_free");
    assert_eq!(state.time.ap, 2);
    assert_eq!(state.world.current_node_id, "academy_gate");
    assert!(state.character.aperture_opened);

    let aperture = &state.character.mortal_aperture;
    assert!(aperture.opened);
    assert_eq!(aperture.aptitude_grade, AptitudeGrade::B);
    assert!((30..=39).contains(&aperture.opening_steps));
    assert_eq!(aperture.primeval_sea_capacity_percent, 66);
    assert_eq!(aperture.primeval_essence_current, 66);
    assert_eq!(
        aperture.primeval_essence_quality,
        PrimevalEssenceQuality::Bronze
    );
    assert_eq!(
        aperture.aperture_wall_state,
        ApertureWallState::LightMembrane
    );
    assert_eq!(aperture.minor_realm, MinorRealm::RankOneInitial);
    assert!(aperture.recovery_profile.contains("乙等"));

    let projection = build_projection_with_content(&state, &bundle);
    assert!(projection.aperture_view.summary.contains("乙等"));
    assert!(projection.aperture_view.primeval_sea.contains("66%"));
    assert!(projection.dialogue.latest_ledger_delta.is_some());
}

#[test]
fn opening_rite_blocks_gu_actions_until_aperture_is_opened() {
    let (state, bundle) = confirmed_setup_state();

    let claim_error = resolve_action(
        state.clone(),
        command(ActionIntent::ClaimGu, Some("moonlight_gu")),
        &bundle,
    )
    .expect_err("claiming gu before opening should be blocked");
    assert_eq!(claim_error.kind, CommandErrorKind::Validation);
    assert!(claim_error.message.contains("开窍大典尚未完成"));

    let cultivate_error = resolve_action(
        state,
        command(ActionIntent::Cultivate, Some("academy_gate")),
        &bundle,
    )
    .expect_err("cultivation before opening should be blocked");
    assert_eq!(cultivate_error.kind, CommandErrorKind::Validation);
    assert!(cultivate_error.message.contains("开窍大典尚未完成"));
}

#[test]
fn opened_aperture_profile_survives_save_round_trip() {
    let (state, bundle) = confirmed_setup_state();
    let state = resolve(
        state,
        &bundle,
        ActionIntent::EnterOpeningCave,
        "opening_rite_cave",
    );
    let state = resolve(
        state,
        &bundle,
        ActionIntent::CrossOpeningRiver,
        "opening_rite_river",
    );
    let state = resolve(state, &bundle, ActionIntent::ReceiveHopeGu, "hope_gu");

    let encoded = serde_json::to_string(&SaveEnvelope::from_state("slot_0", state.clone()))
        .expect("serialize save");
    let decoded: SaveEnvelope = serde_json::from_str(&encoded).expect("deserialize save");
    decoded
        .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
        .expect("validate save");

    assert_eq!(
        decoded.snapshot.character.mortal_aperture,
        state.character.mortal_aperture
    );
}

#[test]
fn quick_preset_create_run_still_starts_at_s0_free_window() {
    let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

    assert_eq!(state.time.window_type, WindowType::Free);
    assert_eq!(state.time.window_id, "day1_morning_free");
    assert!(state.character.aperture_opened);
    assert!(state.character.mortal_aperture.opened);
}
