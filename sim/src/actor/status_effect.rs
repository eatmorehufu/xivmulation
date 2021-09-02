use crate::sim::SimTime;

#[derive(Default)]
pub struct StatusEffect {
    // expiration is the simulation timestamp when this status effect should be removed.
    expiration: SimTime,
}
