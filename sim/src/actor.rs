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
use calc::lookup::Job;
use damage::Damage;
use delegate::delegate;
use recast_expirations::RecastExpirations;
use rotation::Rotation;
use stat::Stats;
use status_effect::StatusEffects;
use std::collections::HashSet;

pub type ActorBundle = (
    Entity,
    &'static Actor,
    &'static Job,
    &'static Actions,
    &'static Rotation,
    &'static mut RecastExpirations,
    &'static mut Damage,
    &'static mut StatusEffects,
    &'static mut Stats,
    &'static mut ActiveCombos,
);

pub type QueryActor<'a> = Query<'a, ActorBundle>;

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}

// TODO: not everything should be pub, but I'm being lazy right now...
#[derive(Default)]
pub struct Actor {}

#[derive(Default)]
pub struct ActiveCombos(HashSet<u32>);

impl ActiveCombos {
    delegate! {
        to self.0 {
            #[call(contains)]
            pub fn has_action(&self, action_id: &u32) -> bool;
            #[call(insert)]
            pub fn add_action(&mut self, action_id: u32);
            #[call(remove)]
            pub fn remove_action(&mut self, action_id: &u32);
            #[call(clear)]
            pub fn reset(&mut self);
        }
    }
}
