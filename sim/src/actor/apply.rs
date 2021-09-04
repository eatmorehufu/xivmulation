use super::Actor;
use super::{Status, StatusEffect};
use crate::sim::{SimState, SimTime};

pub trait Apply {
    fn apply(&self, sim_state: &SimState, target: &mut Actor);
}

pub struct DoDamage {
    pub potency: u64,
}

impl Apply for DoDamage {
    fn apply(&self, sim_state: &SimState, target: &mut Actor) {}
}

pub struct StartGcd {
    start_time: SimTime,
}

impl Apply for StartGcd {
    fn apply(&self, sim_state: &SimState, target: &mut Actor) {}
}

pub struct StartRecast {
    start_time: SimTime,
    action_id: u32,
}

impl Apply for StartRecast {
    fn apply(&self, sim_state: &SimState, target: &mut Actor) {}
}

pub struct GiveStatusEffect {
    start_time: SimTime,
    status: Status,
}

impl Apply for GiveStatusEffect {
    fn apply(&self, sim_state: &SimState, target: &mut Actor) {}
}
