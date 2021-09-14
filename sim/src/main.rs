mod actor;
mod sim;

use actor::action::{Action, Actions};
use actor::apply::Apply;
use actor::apply::{DoDamage, GiveStatusEffect, StartRecast};
use actor::damage::Damage;
use actor::recast_expirations::RecastExpirations;
use actor::rotation::{Rotation, RotationEntry};
use actor::stat::{Stat, Stats};
use actor::status_effect::{ModifyStat, Status, StatusEffect, StatusEffects};
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
            Arc::new(GiveStatusEffect {
                status: Status {
                    name: "Life Surge".into(),
                    duration: 10000,
                    effects: vec![Arc::new(ModifyStat {
                        stat: Stat::CriticalHit,
                        amount: 100, // TODO: figure out real math
                    })],
                },
                target_source: true,
            }),
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
        StatusEffects::default(),
        Stats::default(),
    ));
    commands.spawn_bundle((
        Target::default(),
        Actor::default(),
        Actions::default(),
        Rotation::default(),
        RecastExpirations::default(),
        Damage::default(),
        StatusEffects::default(),
        Stats::default(),
    ));
}

fn tick(mut sim_state_query: Query<&mut SimState>) {
    let mut sim_state = sim_state_query
        .single_mut()
        .expect("There should always be exactly one sim state.");

    let sim_time = sim_state.tick();
    println!("===== {}ms =====", sim_time);
}

fn reset_stats(mut stats_query: Query<&mut Stats>) {
    for mut stats in stats_query.iter_mut() {
        stats.reset();
    }
}

fn remove_expired_status_effects(
    sim_state_query: Query<&SimState>,
    mut status_effects_query: Query<&mut StatusEffects>,
) {
    let sim_state = sim_state_query
        .single()
        .expect("There should always be exactly one sim state.");
    let sim_time = sim_state.now();
    for mut status_effects in status_effects_query.iter_mut() {
        status_effects.remove_expired(sim_time);
    }
}

#[derive(Debug)]
struct StatusEffectApplyBundle {
    status_effect: StatusEffect,
    source: Entity,
    target: Entity,
}
fn process_status_effects(sim_state_query: Query<&SimState>, mut actor_query: QueryActor) {
    let sim_state = sim_state_query
        .single()
        .expect("There should always be exactly one sim state.");
    let sim_time = sim_state.now();

    let mut bundles = Vec::<StatusEffectApplyBundle>::default();
    for (entity, _, _, _, _, _, status_effects, _) in actor_query.iter_mut() {
        println!("{} status effects", status_effects.len());
        for effect in status_effects.iter() {
            bundles.push(StatusEffectApplyBundle {
                status_effect: effect.clone(),
                source: effect.source,
                target: entity,
            });
        }
    }
    for bundle in bundles {
        println!(
            "{}",
            format!("Apply status_effect: {:?}", bundle.status_effect)
        );
        bundle
            .status_effect
            .apply(sim_time, &mut actor_query, bundle.source, bundle.target);
    }
}

struct ActionPerformBundle {
    action: Action,
    source_entity: Entity,
    target_entity: Entity,
}
fn perform_actions(
    sim_state_query: Query<&SimState>,
    mut actor_queries: QuerySet<(Query<ActorBundle, With<Target>>, QueryActor)>,
) {
    let sim_state = sim_state_query
        .single()
        .expect("There should always be exactly one sim state.");
    let sim_time = sim_state.now();

    let (temp_target_entity, _, _, _, _, _, _, _) = actor_queries
        .q0_mut()
        .single_mut()
        .expect("There should always be exactly one sim state.");
    let target_entity = temp_target_entity.clone();

    let mut actor_query = actor_queries.q1_mut();
    let mut perform_bundles = Vec::<ActionPerformBundle>::default();
    for (entity, _, actions, rotation, recast_expirations, _, _, _) in actor_query.iter_mut() {
        match rotation.get_next_action_id(sim_time, &recast_expirations) {
            Some(action_id) => match actions.get(&action_id) {
                Some(action) => perform_bundles.push(ActionPerformBundle {
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
        .add_system(reset_stats.system())
        .add_system(remove_expired_status_effects.system())
        .add_system(process_status_effects.system())
        .add_system(perform_actions.system())
        .run();
}
