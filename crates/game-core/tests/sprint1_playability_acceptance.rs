use rebrng_game_core::{
    build_projection_with_content, create_run, resolve_action, starter_content_bundle,
    ActionCommand, ActionIntent, ContentBundle, DeclaredCost, GameState, InjuryLevel,
    LedgerViewModel, PipelineStep, RunMode, StageClosureStatus, WindowType,
    STARTER_CONTENT_VERSION,
};

#[derive(Debug)]
struct AcceptanceActionRecord {
    intent: ActionIntent,
    target: Option<String>,
    resolve_action_ms: u64,
    projection_ms: u64,
}

struct AcceptanceRun {
    state: GameState,
    bundle: ContentBundle,
    actions: Vec<AcceptanceActionRecord>,
}

impl AcceptanceRun {
    fn new(mode: RunMode) -> Self {
        Self {
            state: create_run(mode, STARTER_CONTENT_VERSION),
            bundle: starter_content_bundle(),
            actions: Vec::new(),
        }
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn projection(&self) -> LedgerViewModel {
        build_projection_with_content(&self.state, &self.bundle)
    }

    fn visible_action_ids(&self) -> Vec<String> {
        self.projection()
            .action_choices
            .into_iter()
            .map(|choice| choice.id)
            .collect()
    }

    fn action_count(&self, intent: ActionIntent) -> usize {
        self.actions
            .iter()
            .filter(|record| record.intent == intent)
            .count()
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
        self.actions.push(AcceptanceActionRecord {
            intent,
            target: target.map(str::to_string),
            resolve_action_ms: result.response.performance.resolve_action_ms,
            projection_ms: result.response.performance.projection_ms,
        });
        self.state = result.state;
        result.response.projection
    }

    fn wait_until_period(&mut self, desired_period: &str) {
        let mut safety = 0;
        while !self.period_matches(desired_period) {
            assert_eq!(
                self.state.time.window_type,
                WindowType::Free,
                "period {desired_period} was not reached before the anchor"
            );
            self.act(ActionIntent::Wait, None);
            safety += 1;
            assert!(safety <= 8, "wait_until_period loop exceeded safety limit");
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
            assert!(safety <= 16, "wait_until_anchor loop exceeded safety limit");
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

    fn period_matches(&self, desired_period: &str) -> bool {
        if self.projection().current_period == desired_period {
            return true;
        }

        desired_period == "深夜" && self.state.time.window_id.contains("deep_night")
    }
}

#[test]
fn sprint1_moonlight_foundation_path_has_real_ap_and_resource_tradeoff() {
    let mut run = AcceptanceRun::new(RunMode::CanonStrict);

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
    assert!(run.state().build.moonlight_cultivation_marks >= 2);
    assert!(run.state().resources.primeval_stones < 3);
    assert!(run.action_count(ActionIntent::Cultivate) >= 2);
    assert_eq!(run.state().time.window_type, WindowType::Anchor);
}

#[test]
fn sprint1_blackmarket_retreat_path_survives_without_turning_market_into_shop() {
    let mut run = AcceptanceRun::new(RunMode::CanonStrict);
    assert!(run
        .visible_action_ids()
        .iter()
        .all(|id| !id.contains("blackmarket")));

    run.act(ActionIntent::Scout, Some("academy_gate"));
    assert!(run.state().knowledge.blackmarket_route_known);
    run.wait_until_period("深夜");
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
    run.act(ActionIntent::Retreat, Some("blackmarket_extortion"));
    assert!(run.state().encounters.active.is_none());
    assert_eq!(run.state().character.injury.level, InjuryLevel::Healthy);
    assert!(run.state().risk.exposure > exposure_before);
    assert!(run
        .state()
        .encounters
        .resolved_encounter_ids
        .contains(&"blackmarket_extortion".to_string()));

    let current_exposure = run.state().risk.exposure;
    run.wait_until_anchor();
    assert!(run.state().risk.exposure >= current_exposure);
    assert_eq!(run.state().time.window_id, "s0_anchor_pending");
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

#[test]
fn sprint1_infirmary_debt_recovery_path_turns_injury_into_debt_pressure() {
    let mut run = AcceptanceRun::new(RunMode::CanonStrict);

    run.act(ActionIntent::Scout, Some("academy_gate"));
    run.wait_until_period("深夜");
    run.act(ActionIntent::Move, Some("blackmarket_hint"));
    run.act(ActionIntent::Confront, Some("blackmarket_extortion"));
    assert_eq!(run.state().character.injury.level, InjuryLevel::Heavy);

    run.act(ActionIntent::Move, Some("infirmary_lane"));
    let debt_before = run.state().debts_and_credit.pressure();
    run.act(ActionIntent::Recover, Some("infirmary_lane"));

    assert_eq!(run.state().character.injury.level, InjuryLevel::Light);
    assert!(run.state().debts_and_credit.infirmary_debt > 0);
    assert!(run.state().debts_and_credit.favor_debt > 0);
    assert!(run.state().debts_and_credit.pressure() > debt_before);
    assert!(run.projection().debt_pressure > 0);

    run.wait_until_anchor();
    assert_eq!(run.state().time.window_type, WindowType::Anchor);
}

#[test]
fn sprint1_inheritance_temptation_path_withdraws_from_high_risk_if_rumor() {
    let mut run = AcceptanceRun::new(RunMode::SandboxIf);
    let starting_materials = run.state().resources.materials;
    let starting_merit = run.state().resources.merit;

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
    assert_eq!(run.state().time.window_id, "s0_anchor_pending");
}
