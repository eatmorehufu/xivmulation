mod actor;
mod sim;

use actor::action::Actions;
use actor::damage::Damage;
use actor::recast_expirations::RecastExpirations;
use actor::rotation::{Rotation, RotationEntry};
use actor::{Actor, Target};
use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use sim::{SimState, SimTime};

fn setup(mut commands: Commands) {
    // TODO: do I need this any more?
    let actor = Actor::default();
    let mut actions = Actions::default();
    actions.add(actor::Action {
        id: 0,
        name: "True Thrust".into(),
        ..Default::default()
    });
    actions.add(actor::Action {
        id: 1,
        name: "Life Surge".into(),
        recast_duration: 45 * 1000,
        ogcd: true,
        ..Default::default()
    });

    let mut rotation = Rotation::default();
    rotation.add(RotationEntry { action_id: 1 });
    rotation.add(RotationEntry { action_id: 0 });

    commands
        .spawn()
        .insert((actor, rotation, actions, RecastExpirations::default()));
    commands
        .spawn()
        .insert((Target::default(), Actor::default(), Damage::default()));
}

fn tick(
    mut sim_state_query: Query<&mut SimState>,
    mut target_query: Query<(Entity, &mut Actor, &Target)>,
    mut actor_query: Query<(Entity, &mut Actor, &Rotation, &RecastExpirations)>,
) {
    let mut sim_state = sim_state_query
        .single_mut()
        .expect("There should always be exactly one sim state.");
    let (_, mut target_actor, _) = target_query
        .single_mut()
        .expect("There should only be one target (for now).");

    sim_state.tick();
    for (_entity, actor, rotation, recast_expirations) in actor_query.iter_mut() {
        rotation.get_next_action_id(sim_state.milliseconds(), &recast_expirations);
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
