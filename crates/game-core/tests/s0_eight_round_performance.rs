use rebrng_game_core::{
    create_run, resolve_action, starter_content_bundle, ActionCommand, ActionIntent, DeclaredCost,
    InjuryLevel, PipelineStep, RunMode, SaveEnvelope, StageClosureStatus, WindowType,
    STARTER_CONTENT_VERSION,
};
use std::time::Instant;

#[derive(Debug)]
struct ActionSample {
    intent: ActionIntent,
    target: Option<String>,
    resolve_action_ms: u64,
    projection_ms: u64,
}

#[test]
fn s0_eight_round_script_reaches_anchor_with_performance_report() {
    let bundle = starter_content_bundle();
    let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);
    let mut samples = Vec::new();

    for (intent, target) in [
        (ActionIntent::Scout, Some("academy_gate")),
        (ActionIntent::Cultivate, Some("academy_gate")),
        (ActionIntent::Move, Some("merit_notice")),
        (ActionIntent::Scout, Some("merit_notice")),
        (ActionIntent::Wait, None),
        (ActionIntent::Move, Some("academy_gate")),
        (ActionIntent::Wait, None),
        (ActionIntent::Move, Some("blackmarket_hint")),
        (ActionIntent::Retreat, Some("blackmarket_extortion")),
        (ActionIntent::Move, Some("infirmary_lane")),
        (ActionIntent::Recover, Some("infirmary_lane")),
        (ActionIntent::Move, Some("academy_gate")),
        (ActionIntent::Cultivate, Some("academy_gate")),
        (ActionIntent::Wait, None),
        (ActionIntent::Move, Some("moonlight_corner")),
        (ActionIntent::Wait, None),
        (ActionIntent::Wait, None),
    ] {
        let command = command(intent.clone(), target);
        let result = resolve_action(state, command, &bundle).unwrap_or_else(|error| {
            panic!("action {intent:?} {target:?} should resolve: {error:?}")
        });
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
        assert!(
            result.response.performance.resolve_action_ms < 300,
            "resolve_action for {intent:?} {target:?} took {:?}",
            result.response.performance
        );
        assert!(
            result.response.performance.resolve_action_ms
                + result.response.performance.projection_ms
                < 1000,
            "next interactive budget for {intent:?} {target:?} took {:?}",
            result.response.performance
        );
        samples.push(ActionSample {
            intent,
            target: target.map(str::to_string),
            resolve_action_ms: result.response.performance.resolve_action_ms,
            projection_ms: result.response.performance.projection_ms,
        });
        state = result.state;
    }

    assert_eq!(state.time.free_rounds_elapsed, 8);
    assert_eq!(state.time.window_type, WindowType::Anchor);
    assert_eq!(state.time.window_id, "s0_anchor_pending");
    assert_eq!(state.time.ap, 0);
    assert_eq!(state.world.current_node_id, "moonlight_corner");
    assert!(state.encounters.active.is_none());
    assert_eq!(state.build.moonlight_cultivation_marks, 2);
    assert!(state.knowledge.blackmarket_route_known);
    assert!(state.resources.merit >= 1);
    assert!(state.risk.exposure > 0);
    assert!(state.debts_and_credit.pressure() > 0);

    let envelope = SaveEnvelope::from_state("slot_0", state.clone());
    let save_started = Instant::now();
    let encoded = serde_json::to_string(&envelope).expect("save envelope serializes");
    let decoded: SaveEnvelope = serde_json::from_str(&encoded).expect("save envelope deserializes");
    decoded
        .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
        .expect("eight-round save should validate");
    let save_load_ms = save_started.elapsed().as_millis() as u64;
    assert!(save_load_ms < 300, "save/load budget took {save_load_ms}ms");
    assert_eq!(decoded.snapshot.time.window_id, state.time.window_id);
    assert_eq!(
        decoded.snapshot.build.moonlight_cultivation_marks,
        state.build.moonlight_cultivation_marks
    );
    assert_eq!(decoded.ledger.len(), state.ledger.len());

    let projection = decoded
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.checkpoint_id == "sprint_0_current")
        .expect("current checkpoint is present");
    assert_eq!(projection.window_id, "s0_anchor_pending");

    let action_report = samples
        .iter()
        .map(|sample| format!("{:?}:{:?}", sample.intent, sample.target))
        .collect::<Vec<_>>()
        .join(" -> ");
    let total_resolve: u64 = samples.iter().map(|sample| sample.resolve_action_ms).sum();
    let total_projection: u64 = samples.iter().map(|sample| sample.projection_ms).sum();
    println!(
        "S0 Phase 10 performance report: actions={}, total_resolve_ms={}, total_projection_ms={}, save_load_ms={}, action_path={}, samples={samples:?}",
        samples.len(),
        total_resolve,
        total_projection,
        save_load_ms,
        action_report
    );
}

#[test]
fn stage_closure_marks_foundation_when_two_moonlight_cultivations_reach_anchor() {
    let bundle = starter_content_bundle();
    let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

    state = resolve_action(
        state,
        command(ActionIntent::Cultivate, Some("academy_gate")),
        &bundle,
    )
    .expect("first cultivation should resolve")
    .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance through midday")
        .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance through evening")
        .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance through deep night")
        .state;
    state = resolve_action(
        state,
        command(ActionIntent::Cultivate, Some("academy_gate")),
        &bundle,
    )
    .expect("second cultivation should resolve")
    .state;
    while state.time.window_type == WindowType::Free {
        state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
            .expect("wait should continue to the anchor")
            .state;
    }

    let projection = rebrng_game_core::build_projection_with_content(&state, &bundle);
    assert_eq!(
        projection.stage_closure.status,
        StageClosureStatus::FoundationEstablished
    );
    assert_eq!(projection.stage_closure.title, "站稳一转根基");
}

#[test]
fn stage_closure_keeps_trauma_continuable_when_heavy_injury_reaches_anchor() {
    let bundle = starter_content_bundle();
    let mut state = create_run(RunMode::CanonStrict, STARTER_CONTENT_VERSION);

    state = resolve_action(
        state,
        command(ActionIntent::Scout, Some("academy_gate")),
        &bundle,
    )
    .expect("scouting unlocks blackmarket")
    .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance to midday")
        .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance to evening")
        .state;
    state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
        .expect("advance to deep night")
        .state;
    state = resolve_action(
        state,
        command(ActionIntent::Move, Some("blackmarket_hint")),
        &bundle,
    )
    .expect("deep-night blackmarket movement should trigger extortion")
    .state;
    state = resolve_action(
        state,
        command(ActionIntent::Confront, Some("blackmarket_extortion")),
        &bundle,
    )
    .expect("confront should be trauma-continuable")
    .state;

    assert_eq!(state.character.injury.level, InjuryLevel::Heavy);
    assert!(state.encounters.active.is_none());
    assert_eq!(state.time.window_id, "day2_morning_free");
    assert_eq!(state.time.ap, 1, "heavy injury compresses next window AP");

    while state.time.window_type == WindowType::Free {
        state = resolve_action(state, command(ActionIntent::Wait, None), &bundle)
            .expect("wait should continue to the anchor")
            .state;
    }

    let projection = rebrng_game_core::build_projection_with_content(&state, &bundle);
    assert_eq!(
        projection.stage_closure.status,
        StageClosureStatus::TraumaContinuable
    );
    assert!(projection.stage_closure.summary.contains("重创可续"));
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
