use super::stat::{SpecialStat, Stat};
use super::Apply;
use super::QueryActor;
use crate::sim::{SimState, SimTime};
use bevy_ecs::prelude::Entity;
use delegate::delegate;
use std::sync::Arc;

// TODO: Maybe a time ordered heap would be faster. Benchmark when we have more functionality.
#[derive(Default)]
pub struct StatusEffects(Vec<StatusEffect>);

impl StatusEffects {
    delegate! {
        to self.0 {
            #[call(push)]
            pub fn add(&mut self, status_effect: StatusEffect);
            pub fn len(&self) -> usize;
            pub fn iter(&self) -> std::slice::Iter<StatusEffect>;
            pub fn remove(&mut self, index: usize) -> StatusEffect;
        }
    }
    pub fn remove_expired(&mut self, sim_time: SimTime) {
        let mut to_remove = Vec::<usize>::default();
        for (i, effect) in self.0.iter().enumerate() {
            if effect.expiration < sim_time {
                to_remove.push(i);
            }
        }
        for i in to_remove.iter() {
            println!("Removing status effect... ");
            // TODO: This is O(n^2)...
            // could do better with a data structure that supports adding
            // and removing in constant time if this becomes a bottleneck.
            // Should be fine for short effects lists.
            self.remove(*i);
        }
    }
}

#[derive(Clone, Debug)]
// Represents an applied status with an expiration
pub struct StatusEffect {
    // expiration is the simulation timestamp when this status effect should be removed.
    pub expiration: SimTime,
    pub status: Status,
    pub source: Entity,
}

impl StatusEffect {
    pub fn new(status: Status, source: Entity, sim_time: SimTime) -> StatusEffect {
        return StatusEffect {
            expiration: sim_time + status.duration,
            status: status,
            source: source,
        };
    }
}

impl Apply for StatusEffect {
    fn apply(&self, sim: &SimState, query: &mut QueryActor, source: Entity, target: Entity) {
        for effect in &self.status.effects {
            effect.apply(sim, query, source, target);
        }
    }
}

#[derive(Default, Clone)]
pub struct Status {
    pub name: String,
    pub duration: SimTime,
    pub effects: Vec<Arc<dyn Apply + Send + Sync>>,
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Status")
            .field("name", &self.name)
            .field("duration", &self.duration)
            .finish()
    }
}

pub struct ModifyStat {
    pub stat: Stat,
    pub amount: i64,
}

impl Apply for ModifyStat {
    fn apply(&self, _sim: &SimState, query: &mut QueryActor, _source: Entity, target: Entity) {
        if let Ok((_, _, _, _, _, _, _, _, mut stats)) = query.get_mut(target) {
            stats.add(self.stat, self.amount);
        }
    }
}

pub struct ModifySpecialStat {
    pub stat: SpecialStat,
    pub amount: i64,
}

impl Apply for ModifySpecialStat {
    fn apply(&self, _sim: &SimState, query: &mut QueryActor, _source: Entity, target: Entity) {
        if let Ok((_, _, _, _, _, _, _, _, mut stats)) = query.get_mut(target) {
            stats.set_special(self.stat, self.amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_effect_new() {
        let status_effect = StatusEffect::new(Status::default(), Entity::new(1), 10);
        assert_eq!(10, status_effect.expiration);
    }

    #[test]
    fn status_effect_new2() {
        let status_effect = StatusEffect::new(
            Status {
                duration: 1000,
                ..Status::default()
            },
            Entity::new(1),
            10,
        );
        assert_eq!(1010, status_effect.expiration);
    }

    #[test]
    fn receive_status_effect() -> std::result::Result<(), String> {
        let mut effects = StatusEffects::default();
        let status_effect = StatusEffect::new(Status::default(), Entity::new(1), 10);
        assert_eq!(0, effects.len());
        effects.add(status_effect);
        assert_eq!(1, effects.len());
        Ok(())
    }

    #[test]
    fn remove_expired() {
        let mut effects = StatusEffects::default();
        effects.add(StatusEffect::new(Status::default(), Entity::new(1), 10));
        effects.add(StatusEffect::new(Status::default(), Entity::new(1), 12));
        assert_eq!(2, effects.len());
        effects.remove_expired(11);
        assert_eq!(1, effects.len());
    }
}
