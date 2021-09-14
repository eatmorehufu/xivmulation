// TODO: don't pub use any more. Let others import the mod qualified path
pub mod action;
pub mod apply;
pub mod calc;
pub mod damage;
pub mod recast_expirations;
pub mod rotation;
pub mod stat;
pub mod status_effect;
pub use action::{Action, Actions};
use apply::Apply;
use bevy_ecs::prelude::{Entity, Query};
use damage::Damage;
use recast_expirations::RecastExpirations;
use rotation::Rotation;
use stat::Stats;
use status_effect::StatusEffects;

pub type ActorBundle = (
    Entity,
    &'static Actor,
    &'static Actions,
    &'static Rotation,
    &'static mut RecastExpirations,
    &'static mut Damage,
    &'static mut StatusEffects,
    &'static mut Stats,
);

pub type QueryActor<'a> = Query<'a, ActorBundle>;

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}

// TODO: not everything should be pub, but I'm being lazy right now...
#[derive(Default)]
pub struct Actor {}
