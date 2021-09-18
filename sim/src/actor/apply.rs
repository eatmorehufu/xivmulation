use super::calc;
use super::status_effect::status::{Status, StatusFlag};
use super::status_effect::StatusEffect;
use super::QueryActor;
use crate::sim::{SimState, SimTime};
use bevy_ecs::prelude::Entity;

pub trait Apply {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity);
}

pub struct DoDirectDamage {
    pub potency: i64,
    pub attack_type: calc::AttackType,
}

impl Apply for DoDirectDamage {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity) {
        let calculated_damage;
        if let Ok((_, _, job, _, _, _, _, mut status_effects, stats)) = query.get_mut(source) {
            calculated_damage =
                calc::direct_damage(sim, self.potency, *job, &stats, self.attack_type, vec![]);
            status_effects.expire_with_flag(StatusFlag::ExpireOnDirectDamage);
        } else {
            panic!("Tried to get stats of a source with no stats.")
        }

        if let Ok((_, _, _, _, _, _, mut damage, _, _)) = query.get_mut(target) {
            damage.add(calculated_damage);
        } else {
            panic!("Tried to do damage to a target that has no Damage component.")
        }
    }
}

pub struct StartRecast {
    pub action_id: u32,
    pub duration: SimTime,
}

impl Apply for StartRecast {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, _target: Entity) {
        if let Ok((_, _, _, _, _, mut recast_expirations, _, _, _)) = query.get_mut(source) {
            recast_expirations.set(self.action_id, sim.now() + self.duration);
        }
    }
}

pub struct GiveStatusEffect {
    pub status: Status,
    pub target_source: bool,
}

impl Apply for GiveStatusEffect {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity) {
        let receiver = if self.target_source { source } else { target };
        if let Ok((_, _, _, _, _, _, _, mut status_effects, _)) = query.get_mut(receiver) {
            status_effects.add(StatusEffect::new(self.status.clone(), source, sim.now()));
        }
    }
}

pub struct StartGcd {
    base_duration: SimTime,
}

impl StartGcd {
    pub fn new(duration: SimTime) -> Self {
        StartGcd {
            base_duration: duration,
        }
    }
}

impl Default for StartGcd {
    fn default() -> Self {
        StartGcd::new(2500)
    }
}

impl Apply for StartGcd {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, _target: Entity) {
        if let Ok((_, _, _, _, _, mut recast_expirations, _, _, _)) = query.get_mut(source) {
            // TODO: calculate duration with skill speed
            recast_expirations.set_gcd(sim.now() + self.base_duration);
        }
    }
}
