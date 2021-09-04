pub mod action;
pub mod apply;
pub mod damage;
pub mod recast_expirations;
pub mod rotation;
pub mod stat;
pub mod status_effect;
pub use action::Action;
pub use apply::Apply;
pub use recast_expirations::RecastExpirations;
pub use stat::Stat;
pub use status_effect::{Status, StatusEffect};

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}

// TODO: not everything should be pub, but I'm being lazy right now...
#[derive(Default)]
pub struct Actor {}
