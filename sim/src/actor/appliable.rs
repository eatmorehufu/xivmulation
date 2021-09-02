use super::Actor;
use crate::sim::SimTime;

pub trait Appliable {
    fn apply(&self, source: &Actor, target: &mut Actor);
}

pub struct DoDamage {
    pub potency: u64,
}

impl Appliable for DoDamage {
    fn apply(&self, source: &Actor, target: &mut Actor) {
        // TODO: real damage calculations
        target.receive_damage(self.potency);
    }
}

pub struct StartGcd {
    start_time: SimTime,
}

impl Appliable for StartGcd {
    fn apply(&self, source: &Actor, target: &mut Actor) {
        target.start_gcd(self.start_time);
    }
}

pub struct StartRecast {
    start_time: SimTime,
    action_id: u32,
}

impl Appliable for StartRecast {
    fn apply(&self, source: &Actor, target: &mut Actor) {
        target.start_recast(self.start_time, self.action_id);
    }
}
