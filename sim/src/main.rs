mod actor;
mod sim;

use actor::{Actor, Appliable, Target};
use bevy_app::{App, ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy_ecs::prelude::*;
use bevy_utils::Duration;
use sim::SimTime;

const TICKS_PER_SECOND: SimTime = 20;
const MS_PER_TICK: SimTime = 1000 / TICKS_PER_SECOND;

struct Sim {
    milliseconds: SimTime,
}

impl Sim {
    pub fn tick(&mut self) {
        self.milliseconds += MS_PER_TICK;
    }

    pub fn milliseconds(&self) -> SimTime {
        self.milliseconds
    }
}

fn setup(mut commands: Commands) {
    let mut actor = Actor::default();
    actor.actions.insert(
        0,
        actor::Action {
            id: 0,
            name: "True Thrust".into(),
            ..Default::default()
        },
    );
    actor.actions.insert(
        1,
        actor::Action {
            id: 1,
            name: "Life Surge".into(),
            recast_duration: 45 * 1000,
            ogcd: true,
            ..Default::default()
        },
    );
    actor.rotation.push(actor::RotationEntry { action_id: 1 });
    actor.rotation.push(actor::RotationEntry { action_id: 0 });
    commands.spawn().insert(actor).id();
    commands
        .spawn()
        .insert((Actor::default(), Target::default()));
}

fn tick(
    mut sim_query: Query<&mut Sim>,
    mut target_query: Query<(Entity, &mut Actor, &Target)>,
    mut actor_query: Query<(Entity, &mut Actor)>,
) {
    let mut sim = sim_query
        .single_mut()
        .expect("There should always be exactly one simulation.");
    let (_, mut target_actor, _) = target_query
        .single_mut()
        .expect("There should only be one target (for now).");

    sim.tick();
    for (_entity, actor) in actor_query.iter_mut() {
        match actor.get_next_action(sim.milliseconds()) {
            Some(action) => {
                action.apply(&actor, &mut target_actor);
            }
            None => println!("zzz"),
        }
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
