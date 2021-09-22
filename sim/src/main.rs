mod actor;
mod sim;

use actor::action::{Action, Actions};
use actor::active_combos::ActiveCombos;
use actor::apply::{Apply, ApplyCombo, DoDirectDamage, GiveStatusEffect, StartGcd, StartRecast};
use actor::calc::lookup::Job;
use actor::damage::Damage;
use actor::recast_expirations::RecastExpirations;
use actor::rotation::{CheckCombo, Rotation, RotationEntry};
use actor::stat::{SpecialStat, Stat, Stats};
use actor::status_effect::status;
use actor::status_effect::status::{Status, StatusFlag, StatusFlags};
use actor::status_effect::{StatusEffect, StatusEffects};
use actor::{ActorTuple, QueryActor, Target};
use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use sim::SimState;
use std::sync::Arc;

fn setup(mut commands: Commands) {
    let mut actions = Actions::default();
    let mut rotation = Rotation::default();

    let life_surge = actor::Action {
        id: 0,
        name: "Life Surge".into(),
        ogcd: true,
        results: vec![
            Arc::new(GiveStatusEffect {
                status: Status {
                    name: "Life Surge".into(),
                    duration: 10000,
                    flags: StatusFlags::new(&[StatusFlag::ExpireOnDirectDamage]),
                    effects: vec![Arc::new(status::ModifySpecialStat {
                        stat: SpecialStat::CriticalHitPercentOverride,
                        amount: 100,
                    })],
                },
                target_source: true,
            }),
            Arc::new(StartRecast {
                // TODO: maybe id can be inferred
                action_id: 0,
                duration: 45000,
            }),
        ],
        ..Default::default()
    };
    let true_thrust = actor::Action {
        id: 1,
        name: "True Thrust",
        results: vec![
            Arc::new(DoDirectDamage {
                potency: 290,
                ..Default::default()
            }),
            Arc::new(ApplyCombo(1)),
            Arc::new(StartGcd::default()),
        ],
        ..Default::default()
    };
    let vorpal_thrust = actor::Action {
        id: 2,
        name: "Vorpal Thrust",
        results: vec![
            Arc::new(DoDirectDamage {
                potency: 140,
                combo_potency: Some(350),
                combo_action_id: Some(1),
                ..Default::default()
            }),
            Arc::new(StartGcd::default()),
        ],
        ..Default::default()
    };
    rotation.add(RotationEntry::new(&life_surge));
    rotation.add(RotationEntry::new(&vorpal_thrust).with_condition(Arc::new(CheckCombo(1))));
    rotation.add(RotationEntry::new(&true_thrust));
    actions.add(life_surge);
    actions.add(true_thrust);
    actions.add(vorpal_thrust);

    let mut stats = Stats::default();
    stats.set_base(Stat::PhysicalWeaponDamage, 134);
    stats.set_base(Stat::Strength, 5435);
    stats.set_base(Stat::Dexterity, 326);
    stats.set_base(Stat::Vitality, 6258);
    stats.set_base(Stat::Intelligence, 206);
    stats.set_base(Stat::Mind, 339);
    stats.set_base(Stat::CriticalHitRate, 3543);
    stats.set_base(Stat::Determination, 2965);
    stats.set_base(Stat::DirectHitRate, 1620);
    stats.set_base(Stat::Defense, 8740);
    stats.set_base(Stat::MagicDefense, 8740);
    stats.set_base(Stat::AttackPower, 5435);
    stats.set_base(Stat::SkillSpeed, 1012);
    stats.set_base(Stat::AttackMagicPotency, 206);
    stats.set_base(Stat::HealingMagicPotency, 339);
    stats.set_base(Stat::SpellSpeed, 380);
    stats.set_base(Stat::Tenacity, 606);
    stats.set_base(Stat::Piety, 340);

    commands.spawn().insert(SimState::default());
    commands.spawn_bundle((
        Job::DRG,
        actions,
        rotation,
        RecastExpirations::default(),
        Damage::default(),
        StatusEffects::default(),
        stats,
        ActiveCombos::default(),
    ));
    commands.spawn_bundle((
        Target::default(),
        Job::None,
        Actions::default(),
        Rotation::default(),
        RecastExpirations::default(),
        Damage::default(),
        StatusEffects::default(),
        Stats::default(),
        ActiveCombos::default(),
    ));
}

fn tick(mut sim_state_query: Query<&mut SimState>) {
    let mut sim_state = sim_state_query
        .single_mut()
        .expect("There should always be exactly one sim state.");

    let now = sim_state.tick();
    // TODO: Temporary. Sim 15s for now while in dev.
    println!("============ Tick: {}", now);
    if now >= 15000 {
        std::process::exit(0);
    }
}

fn reset_stats(mut query: Query<&mut Stats>) {
    for mut stats in query.iter_mut() {
        stats.reset();
    }
}

fn reset_active_combos(mut query: Query<&mut ActiveCombos>) {
    for mut active_combos in query.iter_mut() {
        active_combos.reset();
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
    let sim = sim_state_query
        .single()
        .expect("There should always be exactly one sim state.");

    let mut bundles = Vec::<StatusEffectApplyBundle>::default();
    for (entity, _, _, _, _, _, status_effects, _, _) in actor_query.iter_mut() {
        for effect in status_effects.iter() {
            bundles.push(StatusEffectApplyBundle {
                status_effect: effect.clone(),
                source: effect.source,
                target: entity,
            });
        }
    }
    for bundle in bundles {
        bundle
            .status_effect
            .apply(sim, &mut actor_query, bundle.source, bundle.target);
    }
}

struct ActionPerformBundle {
    action: Action,
    source_entity: Entity,
    target_entity: Entity,
}
fn perform_actions(
    sim_state_query: Query<&SimState>,
    mut actor_queries: QuerySet<(Query<ActorTuple, With<Target>>, QueryActor)>,
) {
    let sim = sim_state_query
        .single()
        .expect("There should always be exactly one sim state.");
    let sim_time = sim.now();

    let (temp_target_entity, _, _, _, _, _, _, _, _) = actor_queries
        .q0_mut()
        .single_mut()
        .expect("There should always be exactly one sim state.");
    let target_entity = temp_target_entity.clone();

    let mut actor_query = actor_queries.q1_mut();
    let mut perform_bundles = Vec::<ActionPerformBundle>::default();
    for (entity, _, actions, rotation, recast_expirations, _, _, _, active_combos) in
        actor_query.iter_mut()
    {
        match rotation.get_next_action_id(sim_time, &recast_expirations, &active_combos) {
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
        println!(
            ">>>> ACTION [{}s]: {}",
            sim_time as f64 / 1000.0,
            bundle.action.name
        );
        bundle.action.perform(
            sim,
            &mut actor_query,
            bundle.source_entity,
            bundle.target_entity,
        );
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, SystemLabel)]
enum SimLabel {
    Tick,
    Setup,
    Calculate,
    Execute,
}

fn main() {
    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::new(0, 0)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .label(SimLabel::Tick)
                .with_system(tick.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label(SimLabel::Setup)
                .with_system(reset_stats.system())
                .with_system(reset_active_combos.system())
                .with_system(remove_expired_status_effects.system())
                .after(SimLabel::Tick),
        )
        .add_system_set(
            SystemSet::new()
                .label(SimLabel::Calculate)
                .with_system(process_status_effects.system())
                .after(SimLabel::Setup),
        )
        .add_system_set(
            SystemSet::new()
                .label(SimLabel::Execute)
                .with_system(perform_actions.system())
                .after(SimLabel::Calculate),
        )
        .run();
}
