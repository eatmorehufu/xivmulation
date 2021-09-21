use super::calc;
use super::status_effect::status::{SetCombo, Status, StatusFlag, StatusFlags};
use super::status_effect::StatusEffect;
use super::{ActiveCombos, QueryActor};
use crate::sim::{SimState, SimTime};
use bevy_ecs::prelude::Entity;
use std::sync::Arc;

pub trait Apply {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity);
}

#[derive(Default)]
pub struct DoDirectDamage {
    pub potency: i64,
    pub combo_action_id: Option<u32>,
    pub combo_potency: Option<i64>,
    pub attack_type: calc::AttackType,
}

impl DoDirectDamage {
    fn consume_combo(&self, active_combos: &mut ActiveCombos) -> bool {
        if let Some(action_id) = self.combo_action_id {
            if active_combos.has_action(&action_id) {
                active_combos.remove_action(&action_id);
                return true;
            }
        }
        false
    }
}

impl Apply for DoDirectDamage {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity) {
        let calculated_damage;
        if let Ok((_, job, _, _, _, _, mut status_effects, stats, mut active_combos)) =
            query.get_mut(source)
        {
            let mut potency = self.potency;
            if self.consume_combo(&mut active_combos) {
                potency = match self.combo_potency {
                    Some(combo_potency) => combo_potency,
                    None => panic!(
                        "Consumed a combo, but no combo_potency is set. This should not happen."
                    ),
                }
            }
            calculated_damage =
                calc::direct_damage(sim, potency, *job, &stats, self.attack_type, vec![]);
            status_effects.expire_with_flag(StatusFlag::ExpireOnDirectDamage);
        } else {
            panic!("Tried to get stats of a source with no stats.")
        }

        if let Ok((_, _, _, _, _, mut damage, _, _, _)) = query.get_mut(target) {
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
        if let Ok((_, _, _, _, mut recast_expirations, _, _, _, _)) = query.get_mut(source) {
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
        if let Ok((_, _, _, _, _, _, mut status_effects, _, _)) = query.get_mut(receiver) {
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
        if let Ok((_, _, _, _, mut recast_expirations, _, _, _, _)) = query.get_mut(source) {
            // TODO: calculate duration with skill speed
            recast_expirations.set_gcd(sim.now() + self.base_duration);
        }
    }
}

pub struct ApplyCombo(pub u32);

impl Apply for ApplyCombo {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, _target: Entity) {
        let set_combo = Status {
            name: format!("{} Combo", self.0),
            // TODO: figure out how long combos actually last.
            duration: 15000,
            flags: StatusFlags::new(&[StatusFlag::ExpireOnDirectDamage]),
            effects: vec![Arc::new(SetCombo(self.0))],
            ..Default::default()
        };
        if let Ok((_, _, _, _, _, _, mut status_effects, _, _)) = query.get_mut(source) {
            status_effects.add(StatusEffect::new(set_combo, source, sim.now()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn potency() {
        let ddd = DoDirectDamage {
            potency: 100,
            combo_potency: Some(200),
            combo_action_id: Some(1),
            ..Default::default()
        };
        let mut active_combos = ActiveCombos::default();
        active_combos.add_action(2);
        assert_eq!(false, ddd.consume_combo(&mut active_combos));
        active_combos.add_action(1);
        assert_eq!(true, ddd.consume_combo(&mut active_combos));
        assert_eq!(false, ddd.consume_combo(&mut active_combos));
        assert_eq!(true, active_combos.has_action(&2));
        assert_eq!(false, active_combos.has_action(&1));
    }
}
