mod actor;
mod sim;

use actor::action::{Action, Actions};
use actor::apply::{DoDamage, GiveStatusEffect, StartRecast};
use actor::damage::Damage;
use actor::recast_expirations::RecastExpirations;
use actor::rotation::{Rotation, RotationEntry};
use actor::{Actor, ActorBundle, QueryActor, Target};
use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use sim::SimState;
use std::sync::Arc;

fn setup(mut commands: Commands) {
    let mut actions = Actions::default();
    actions.add(actor::Action {
        id: 0,
        name: "Life Surge".into(),
        ogcd: true,
        results: vec![
            Arc::new(GiveStatusEffect {}),
            Arc::new(StartRecast {
                // TODO: maybe id can be inferred
                action_id: 0,
                duration: 1000,
            }),
        ],
        ..Default::default()
    });
    actions.add(actor::Action {
        id: 1,
        name: "True Thrust".into(),
        results: vec![Arc::new(DoDamage { potency: 1000 })],
        ..Default::default()
    });

    let mut rotation = Rotation::default();
    // NEXT: put life surge on CD and ensure we perform the whole rotation.
    rotation.add(RotationEntry { action_id: 0 });
    rotation.add(RotationEntry { action_id: 1 });

    commands.spawn().insert(SimState::default());
    commands.spawn_bundle((
        Actor::default(),
        actions,
        rotation,
        RecastExpirations::default(),
        Damage::default(),
    ));
    commands.spawn_bundle((
        Target::default(),
        Actor::default(),
        Actions::default(),
        Rotation::default(),
        RecastExpirations::default(),
        Damage::default(),
    ));
}

struct PerformBundle {
    action: Action,
    source_entity: Entity,
    target_entity: Entity,
}

fn tick(
    mut sim_state_query: Query<&mut SimState>,
    mut actor_queries: QuerySet<(Query<ActorBundle, With<Target>>, QueryActor)>,
) {
    let mut sim_state = sim_state_query
        .single_mut()
        .expect("There should always be exactly one sim state.");

    let sim_time = sim_state.tick();

    let (temp_target_entity, _, _, _, _, _) = actor_queries
        .q0_mut()
        .single_mut()
        .expect("There should always be exactly one sim state.");
    let target_entity = temp_target_entity.clone();

    let mut actor_query = actor_queries.q1_mut();
    let mut perform_bundles = Vec::<PerformBundle>::default();
    for (entity, _, actions, rotation, recast_expirations, _) in actor_query.iter_mut() {
        match rotation.get_next_action_id(sim_time, &recast_expirations) {
            Some(action_id) => match actions.get(&action_id) {
                Some(action) => perform_bundles.push(PerformBundle {
                    action: action.clone(),
                    source_entity: entity,
                    target_entity: target_entity,
                }),
                None => (),
            },
            None => (),
        };
    }

    for bundle in perform_bundles {
        bundle.action.perform(
            sim_time,
            &mut actor_query,
            bundle.source_entity,
            bundle.target_entity,
        );
    }
}

fn main() {
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::new(0, 0)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(setup.system())
        .add_system(tick.system())
        .run();
}
