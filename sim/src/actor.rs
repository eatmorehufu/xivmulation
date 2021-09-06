// TODO: don't pub use any more. Let others import the mod qualified path
pub mod action;
pub mod apply;
pub mod damage;
pub mod recast_expirations;
pub mod rotation;
pub mod stat;
pub mod status_effect;
pub use action::{Action, Actions};
pub use apply::Apply;
use bevy_ecs::prelude::{Entity, Query};
pub use damage::Damage;
pub use recast_expirations::RecastExpirations;
pub use rotation::Rotation;
pub use stat::Stat;
pub use status_effect::{Status, StatusEffect};

pub type ActorBundle = (
    Entity,
    &'static Actor,
    &'static Actions,
    &'static Rotation,
    &'static mut RecastExpirations,
    &'static mut Damage,
);

pub type QueryActor<'a> = Query<'a, ActorBundle>;

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}

// TODO: not everything should be pub, but I'm being lazy right now...
#[derive(Default)]
pub struct Actor {}
