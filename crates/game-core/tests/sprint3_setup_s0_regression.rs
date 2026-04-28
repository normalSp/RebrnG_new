use rebrng_game_core::{
    build_projection_with_content, confirm_setup_run, create_setup_run, resolve_action,
    resolve_setup_choice, setup_command, starter_content_bundle, ActionCommand, ActionIntent,
    ContentBundle, DeclaredCost, GameState, InjuryLevel, LedgerViewModel, PipelineStep, RunMode,
    SaveEnvelope, SetupIntent, StageClosureStatus, VitalGuStatus, WindowType,
    STARTER_CONTENT_VERSION,
};

#[derive(Debug)]
struct ActionRecord {
    intent: ActionIntent,
    target: Option<String>,
    resolve_action_ms: u64,
    projection_ms: u64,
}

struct SetupAcceptanceRun {
    state: GameState,
    bundle: ContentBundle,
    actions: Vec<ActionRecord>,
}

impl SetupAcceptanceRun {
    fn canon() -> Self {
        Self::from_setup(
            RunMode::CanonStrict,
            "academy_plain_child",
            ["steady_mind", "quiet_observer", "moonlight_pacing"],
        )
    }

    fn sandbox_if() -> Self {
        Self::from_setup(
            RunMode::SandboxIf,
            "academy_plain_child",
            [
                "reborn_memory_fragment",
                "inheritance_scent",
                "vital_gu_omen",
            ],
        )
    }

    fn from_setup(mode: RunMode, origin_id: &str, talent_ids: [&str; 3]) -> Self {
        let bundle = starter_content_bundle();
        let mut setup = create_setup_run(mode, &bundle).expect("create setup run");
        setup = resolve_setup_choice(
            setup,
            setup_command(SetupIntent::SelectOrigin, origin_id),
            &bundle,
        )
        .expect("select origin")
        .setup;

        for talent_id in talent_ids {
            setup = resolve_setup_choice(
                setup,
                setup_command(SetupIntent::ToggleTalent, talent_id),
                &bundle,
            )
            .expect("select setup talent")
            .setup;
        }

        let state = confirm_setup_run(setup, &bundle).expect("confirm setup run");
        assert!(state.character.aperture_opened);
        assert!(state.setup_summary.is_some());
        assert_eq!(state.time.window_type, WindowType::Free);
        assert_eq!(state.world.current_node_id, "academy_gate");

        let projection = build_projection_with_content(&state, &bundle);
        assert!(!projection.dialogue.paragraphs.is_empty());
        assert!(projection.dialogue.stage_title.contains("青茅山"));
        assert!(projection.scene_text.contains("开窍大典"));

        Self {
            state,
            bundle,
            actions: Vec::new(),
        }
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn projection(&self) -> LedgerViewModel {
        build_projection_with_content(&self.state, &self.bundle)
    }

    fn act(&mut self, intent: ActionIntent, target: Option<&str>) -> LedgerViewModel {
        let command = command(intent.clone(), target);
        let result =
            resolve_action(self.state.clone(), command, &self.bundle).unwrap_or_else(|error| {
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
        assert_eq!(
            result.response.projection.dialogue.previous_result_summary,
            result
                .response
                .projection
                .recent_feedback
                .as_ref()
                .map(|feedback| feedback.summary.clone())
        );
        assert!(!result.response.projection.dialogue.paragraphs.is_empty());

        self.actions.push(ActionRecord {
            intent,
            target: target.map(str::to_string),
            resolve_action_ms: result.response.performance.resolve_action_ms,
            projection_ms: result.response.performance.projection_ms,
        });
        self.state = result.state;
        result.response.projection
    }

    fn wait_until_deep_night(&mut self) {
        let mut safety = 0;
        while !self.state.time.window_id.contains("deep_night") {
            assert_eq!(self.state.time.window_type, WindowType::Free);
            self.act(ActionIntent::Wait, None);
            safety += 1;
            assert!(safety <= 8, "deep night was not reached before anchor");
        }
    }

    fn wait_until_anchor(&mut self) {
        let mut safety = 0;
        while self.state.time.window_type == WindowType::Free {
            assert!(
                self.state.encounters.active.is_none(),
                "active encounter must be resolved before waiting to anchor"
            );
            self.act(ActionIntent::Wait, None);
            safety += 1;
            assert!(safety <= 16, "wait_until_anchor exceeded safety limit");
        }

        assert_eq!(self.state.time.free_rounds_elapsed, 8);
        assert_eq!(self.state.time.window_type, WindowType::Anchor);
        assert_eq!(self.state.time.window_id, "s0_anchor_pending");
        assert!(!self.actions.is_empty());
        let action_path = self
            .actions
            .iter()
            .map(|record| format!("{:?}:{:?}", record.intent, record.target))
            .collect::<Vec<_>>()
            .join(" -> ");
        assert!(!action_path.is_empty());
        for record in &self.actions {
            assert!(
                record.resolve_action_ms < 300,
                "recorded resolve_action budget failed for {record:?}"
            );
            assert!(
                record.resolve_action_ms + record.projection_ms < 1000,
                "recorded interactive budget failed for {record:?}"
            );
        }
    }
}

#[test]
fn sprint3_setup_moonlight_foundation_reaches_anchor() {
    let mut run = SetupAcceptanceRun::canon();

    run.act(ActionIntent::Scout, Some("academy_gate"));
    run.act(ActionIntent::Cultivate, Some("academy_gate"));
    run.act(ActionIntent::Move, Some("moonlight_corner"));
    run.act(ActionIntent::Cultivate, Some("moonlight_corner"));
    run.act(ActionIntent::Yield, Some("academy_public_pressure"));
    run.wait_until_anchor();

    let projection = run.projection();
    assert_eq!(
        projection.stage_closure.status,
        StageClosureStatus::FoundationEstablished
    );
    assert!(run.state().setup_summary.is_some());
    assert!(run.state().build.moonlight_cultivation_marks >= 2);
    assert!(projection.dialogue.latest_ledger_delta.is_some());
    assert!(!projection.dialogue.available_actions_summary.is_empty());
}

#[test]
fn sprint3_setup_blackmarket_retreat_keeps_core_loop_intact() {
    let mut run = SetupAcceptanceRun::canon();
    assert!(run
        .projection()
        .action_choices
        .iter()
        .all(|choice| !choice.id.contains("blackmarket")));

    run.act(ActionIntent::Scout, Some("academy_gate"));
    assert!(run.state().knowledge.blackmarket_route_known);
    run.wait_until_deep_night();
    run.act(ActionIntent::Move, Some("blackmarket_hint"));
    assert_eq!(
        run.state()
            .encounters
            .active
            .as_ref()
            .map(|active| active.encounter_id.as_str()),
        Some("blackmarket_extortion")
    );

    let exposure_before = run.state().risk.exposure;
    let projection = run.act(ActionIntent::Retreat, Some("blackmarket_extortion"));
    assert!(run.state().encounters.active.is_none());
    assert_eq!(run.state().character.injury.level, InjuryLevel::Healthy);
    assert!(run.state().risk.exposure > exposure_before);
    assert!(projection.dialogue.previous_result_summary.is_some());
    assert!(run
        .state()
        .encounters
        .resolved_encounter_ids
        .contains(&"blackmarket_extortion".to_string()));

    run.wait_until_anchor();
}

#[test]
fn sprint3_setup_sandbox_inheritance_temptation_withdraws_without_stable_reward() {
    let mut run = SetupAcceptanceRun::sandbox_if();
    let starting_materials = run.state().resources.materials;
    let starting_merit = run.state().resources.merit;
    let summary = run
        .state()
        .setup_summary
        .as_ref()
        .expect("setup summary exists");
    assert!(summary
        .talent_ids
        .contains(&"inheritance_scent".to_string()));
    assert_eq!(
        run.state().build.vital_gu.status,
        VitalGuStatus::NotEstablished
    );

    run.act(ActionIntent::Move, Some("inheritance_rumor"));
    run.act(ActionIntent::Scout, Some("inheritance_rumor"));
    assert!(run
        .state()
        .knowledge
        .known_clues
        .contains(&"rumor_inheritance_bamboo".to_string()));
    assert!(run.state().risk.exposure > 0);

    run.act(ActionIntent::Move, Some("academy_gate"));
    run.wait_until_anchor();

    assert_eq!(run.state().resources.materials, starting_materials);
    assert_eq!(run.state().resources.merit, starting_merit);
    assert_ne!(
        run.projection().stage_closure.status,
        StageClosureStatus::FoundationEstablished
    );
}

#[test]
fn sprint3_setup_save_envelope_preserves_setup_and_dialogue_rebuilds() {
    let mut run = SetupAcceptanceRun::canon();
    run.act(ActionIntent::Scout, Some("academy_gate"));

    let envelope = SaveEnvelope::from_state("slot_0", run.state().clone());
    let encoded = serde_json::to_string(&envelope).expect("save envelope serializes");
    let decoded: SaveEnvelope = serde_json::from_str(&encoded).expect("save envelope deserializes");
    decoded
        .validate_for_load("slot_0", STARTER_CONTENT_VERSION)
        .expect("setup save should validate");

    assert!(decoded.snapshot.setup_summary.is_some());
    assert_eq!(decoded.snapshot.time.window_id, run.state().time.window_id);
    assert_eq!(decoded.snapshot.ledger.len(), run.state().ledger.len());
    assert!(decoded
        .checkpoints
        .iter()
        .any(|checkpoint| checkpoint.checkpoint_id == "sprint_0_current"));

    let projection = build_projection_with_content(&decoded.snapshot, &run.bundle);
    assert!(!projection.dialogue.paragraphs.is_empty());
    assert!(projection.dialogue.previous_result_summary.is_some());
    assert_eq!(
        projection.current_node_id,
        run.state().world.current_node_id
    );
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
