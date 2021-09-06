use super::QueryActor;
use crate::sim::SimTime;
use bevy_ecs::prelude::Entity;

pub trait Apply {
    fn apply(&self, sim_time: SimTime, query: &mut QueryActor, source: Entity, target: Entity);
}

pub struct DoDamage {
    pub potency: u32,
}

impl Apply for DoDamage {
    fn apply(&self, _sim_time: SimTime, query: &mut QueryActor, _source: Entity, target: Entity) {
        if let Ok((_, _, _, _, _, mut damage)) = query.get_mut(target) {
            damage.add(self.potency);
        } else {
            println!("Tried to do damage to a target that has no Damage component.")
        }
    }
}

pub struct StartRecast {
    pub action_id: u32,
    pub duration: SimTime,
}

impl Apply for StartRecast {
    fn apply(&self, sim_time: SimTime, query: &mut QueryActor, source: Entity, _target: Entity) {
        if let Ok((_, _, _, _, mut recast_expirations, _)) = query.get_mut(source) {
            recast_expirations.set(self.action_id, sim_time + self.duration);
        }
    }
}

pub struct GiveStatusEffect {}

impl Apply for GiveStatusEffect {
    fn apply(&self, _sim_time: SimTime, _query: &mut QueryActor, _source: Entity, _target: Entity) {
        // TODO: implement
        println!("Buff applied");
    }
}
