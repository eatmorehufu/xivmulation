pub mod action;
pub mod apply;
pub mod calc;
pub mod damage;
pub mod recast_expirations;
pub mod rotation;
pub mod stat;
pub mod status_effect;
pub use action::{Action, Actions};
pub mod active_combos;
use active_combos::ActiveCombos;
use apply::Apply;
use bevy_ecs::prelude::{Entity, Query};
use calc::lookup::Job;
use damage::Damage;
use recast_expirations::RecastExpirations;
use rotation::Rotation;
use stat::Stats;
use status_effect::StatusEffects;

pub type ActorTuple = (
    Entity,
    &'static Job,
    &'static Actions,
    &'static Rotation,
    &'static mut RecastExpirations,
    &'static mut Damage,
    &'static mut StatusEffects,
    &'static mut Stats,
    &'static mut ActiveCombos,
);

pub type QueryActor<'a> = Query<'a, ActorTuple>;

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}
