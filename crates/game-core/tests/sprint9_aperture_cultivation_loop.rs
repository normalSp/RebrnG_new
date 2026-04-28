use rebrng_game_core::{
    build_projection_with_content, create_run, resolve_action, starter_content_bundle,
    ActionCommand, ActionIntent, ApertureWallState, AptitudeGrade, CommandErrorKind, ContentBundle,
    DeclaredCost, GameState, MinorRealm, RunMode, SaveEnvelope, STARTER_CONTENT_VERSION,
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

fn act(state: GameState, bundle: &ContentBundle, intent: ActionIntent, target: &str) -> GameState {
    resolve_action(state, command(intent, Some(target)), bundle)
        .unwrap_or_else(|error| panic!("action should resolve: {error:?}"))
        .state
}

fn opened_state() -> (GameState, ContentBundle) {
    let bundle = starter_content_bundle();
    let state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
    (state, bundle)
}

fn moonlight_claimed_state() -> (GameState, ContentBundle) {
    let (state, bundle) = opened_state();
    let state = act(state, &bundle, ActionIntent::ClaimGu, "moonlight_gu");
    (state, bundle)
}

fn moonlight_refined_state() -> (GameState, ContentBundle) {
    let (state, bundle) = moonlight_claimed_state();
    let state = act(state, &bundle, ActionIntent::RefineGu, "moonlight_gu");
    (state, bundle)
}

#[test]
fn opened_aperture_has_recovery_rate_and_wall_progress() {
    let (state, _) = opened_state();
    let aperture = &state.character.mortal_aperture;

    assert!(aperture.opened);
    assert_eq!(aperture.aptitude_grade, AptitudeGrade::C);
    assert_eq!(aperture.primeval_sea_capacity_percent, 44);
    assert_eq!(aperture.primeval_essence_current, 44);
    assert_eq!(aperture.essence_recovery_per_window, 7);
    assert_eq!(aperture.aperture_wall_progress, 0);
    assert_eq!(aperture.minor_realm, MinorRealm::RankOneInitial);
    assert_eq!(
        aperture.aperture_wall_state,
        ApertureWallState::LightMembrane
    );
}

#[test]
fn refining_moonlight_gu_consumes_primeval_essence() {
    let (state, bundle) = moonlight_claimed_state();
    let refined = act(state, &bundle, ActionIntent::RefineGu, "moonlight_gu");

    assert_eq!(
        refined.character.mortal_aperture.primeval_essence_current,
        36
    );
    assert_eq!(refined.resources.primeval_stones, 3);
    assert_eq!(refined.time.ap, 1);
}

#[test]
fn refining_moonlight_gu_requires_enough_primeval_essence() {
    let (mut state, bundle) = moonlight_claimed_state();
    state.character.mortal_aperture.primeval_essence_current = 7;

    let error = resolve_action(
        state,
        command(ActionIntent::RefineGu, Some("moonlight_gu")),
        &bundle,
    )
    .expect_err("refining should require 8 primeval essence");

    assert_eq!(error.kind, CommandErrorKind::Validation);
    assert!(error.message.contains("真元不足"));
}

#[test]
fn moonlight_cultivation_consumes_essence_and_flushes_aperture_wall() {
    let (state, bundle) = moonlight_refined_state();
    let cultivated = act(state, &bundle, ActionIntent::Cultivate, "academy_gate");

    assert_eq!(cultivated.build.moonlight_cultivation_marks, 1);
    assert_eq!(cultivated.resources.primeval_stones, 2);
    assert_eq!(
        cultivated
            .character
            .mortal_aperture
            .primeval_essence_current,
        33
    );
    assert_eq!(
        cultivated.character.mortal_aperture.aperture_wall_progress,
        8
    );
}

#[test]
fn cultivation_is_disabled_when_primeval_essence_is_not_enough() {
    let (mut state, bundle) = moonlight_refined_state();
    state.character.mortal_aperture.primeval_essence_current = 9;

    let projection = build_projection_with_content(&state, &bundle);
    let cultivate = projection
        .action_choices
        .iter()
        .find(|choice| choice.id == "cultivate_moonlight")
        .expect("cultivate action should be projected");
    assert!(!cultivate.enabled);
    assert_eq!(cultivate.disabled_reason.as_deref(), Some("真元不足"));

    let error = resolve_action(
        state,
        command(ActionIntent::Cultivate, Some("academy_gate")),
        &bundle,
    )
    .expect_err("cultivation should require primeval essence");
    assert_eq!(error.kind, CommandErrorKind::Validation);
    assert!(error.message.contains("真元不足"));
}

#[test]
fn recover_essence_consumes_ap_and_caps_at_capacity() {
    let (mut state, bundle) = opened_state();
    state.character.mortal_aperture.primeval_essence_current = 40;

    let recovered = act(state, &bundle, ActionIntent::RecoverEssence, "academy_gate");

    assert_eq!(
        recovered.character.mortal_aperture.primeval_essence_current,
        44
    );
    assert_eq!(recovered.time.ap, 1);
    assert!(recovered
        .ledger
        .last()
        .expect("ledger entry")
        .text
        .contains("调息"));
}

#[test]
fn window_advance_recovers_essence_without_exceeding_capacity() {
    let (mut state, bundle) = opened_state();
    state.character.mortal_aperture.primeval_essence_current = 40;

    let waited = act(state, &bundle, ActionIntent::Wait, "academy_gate");

    assert_eq!(
        waited.character.mortal_aperture.primeval_essence_current,
        44
    );
    assert_eq!(waited.time.window_id, "day1_midday_free");
}

#[test]
fn wall_progress_breaks_to_rank_one_middle_when_it_reaches_one_hundred() {
    let (mut state, bundle) = moonlight_refined_state();
    state.character.mortal_aperture.aperture_wall_progress = 95;
    state.character.mortal_aperture.primeval_essence_current = 44;

    let cultivated = act(state, &bundle, ActionIntent::Cultivate, "academy_gate");

    assert_eq!(
        cultivated.character.mortal_aperture.aperture_wall_progress,
        0
    );
    assert_eq!(
        cultivated.character.mortal_aperture.minor_realm,
        MinorRealm::RankOneMiddle
    );
    assert_eq!(
        cultivated.character.mortal_aperture.aperture_wall_state,
        ApertureWallState::WaterMembrane
    );
    assert!(cultivated
        .ledger
        .last()
        .expect("ledger entry")
        .text
        .contains("一转中阶"));
}

#[test]
fn aperture_cultivation_state_survives_save_round_trip() {
    let (mut state, bundle) = moonlight_refined_state();
    state.character.mortal_aperture.aperture_wall_progress = 95;
    state.character.mortal_aperture.primeval_essence_current = 44;
    let cultivated = act(state, &bundle, ActionIntent::Cultivate, "academy_gate");

    let encoded = serde_json::to_string(&SaveEnvelope::from_state("slot_0", cultivated.clone()))
        .expect("serialize save");
    let decoded: SaveEnvelope = serde_json::from_str(&encoded).expect("deserialize save");
    decoded
        .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
        .expect("validate save");

    assert_eq!(
        decoded.snapshot.character.mortal_aperture,
        cultivated.character.mortal_aperture
    );
    assert_eq!(decoded.snapshot.gu_inventory, cultivated.gu_inventory);
}
