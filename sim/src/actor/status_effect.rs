use super::stat::{SpecialStat, Stat};
use super::Apply;
use super::QueryActor;
use crate::sim::{SimState, SimTime};
use bevy_ecs::prelude::Entity;
use delegate::delegate;
use std::collections::HashSet;
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
        }
    }
    pub fn remove_expired(&mut self, sim_time: SimTime) {
        self.0.retain(|effect| !effect.is_expired(sim_time));
    }

    pub fn expire_with_flag(&mut self, flag: StatusFlag) {
        for effect in self.0.iter_mut() {
            if effect.has_flag(&flag) {
                effect.expire();
            }
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
    // expired can be set to true to force an effect to expire without the expiration time passing.
    pub force_expired: bool,
}

impl StatusEffect {
    pub fn new(status: Status, source: Entity, sim_time: SimTime) -> StatusEffect {
        return StatusEffect {
            expiration: sim_time + status.duration,
            status: status,
            source: source,
            force_expired: false,
        };
    }

    pub fn is_expired(&self, sim_time: SimTime) -> bool {
        self.force_expired || sim_time >= self.expiration
    }

    pub fn expire(&mut self) {
        self.force_expired = true;
    }

    delegate! {
        to self.status {
            pub fn has_flag(&self, flag: &StatusFlag) -> bool;
        }
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
    pub name: &'static str,
    pub duration: SimTime,
    pub effects: Vec<Arc<dyn Apply + Send + Sync>>,
    pub flags: HashSet<StatusFlag>,
}

impl Status {
    delegate! {
        to self.flags {
            #[call(contains)]
            fn has_flag(&self, flag: &StatusFlag) -> bool;
        }
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Status")
            .field("name", &self.name)
            .field("duration", &self.duration)
            .finish()
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum StatusFlag {
    ExpireOnDirectDamage,
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
        let should_expire = Status {
            name: "Should Expire",
            ..Default::default()
        };
        let should_not_expire = Status {
            name: "Should Not Expire",
            ..Default::default()
        };
        effects.add(StatusEffect::new(should_expire.clone(), Entity::new(1), 10));
        effects.add(StatusEffect::new(
            should_not_expire.clone(),
            Entity::new(2),
            12,
        ));
        effects.add(StatusEffect::new(should_expire.clone(), Entity::new(1), 10));
        effects.add(StatusEffect::new(
            should_not_expire.clone(),
            Entity::new(2),
            12,
        ));
        effects.add(StatusEffect::new(should_expire.clone(), Entity::new(1), 10));
        assert_eq!(5, effects.len());
        effects.remove_expired(11);
        assert_eq!(2, effects.len());

        for effect in effects.iter() {
            assert_eq!(should_not_expire.name, effect.status.name);
        }
    }

    #[test]
    fn expire_with_flag() {
        let mut effects = StatusEffects::default();
        let mut flags = HashSet::<StatusFlag>::new();
        flags.insert(StatusFlag::ExpireOnDirectDamage);
        let should_expire = Status {
            name: "Should Expire",
            flags: flags,
            ..Default::default()
        };
        let should_not_expire = Status {
            name: "Should Not Expire",
            ..Default::default()
        };
        effects.add(StatusEffect::new(should_expire.clone(), Entity::new(1), 12));
        effects.add(StatusEffect::new(
            should_not_expire.clone(),
            Entity::new(1),
            12,
        ));

        effects.remove_expired(11);
        assert_eq!(2, effects.len());
        effects.expire_with_flag(StatusFlag::ExpireOnDirectDamage);
        effects.remove_expired(11);
        assert_eq!(1, effects.len());

        for effect in effects.iter() {
            assert_eq!(should_not_expire.name, effect.status.name);
        }
    }

    #[test]
    fn is_expired() {
        let effect = StatusEffect::new(Status::default(), Entity::new(1), 10);
        assert_eq!(false, effect.is_expired(9));
        assert_eq!(true, effect.is_expired(10));
        assert_eq!(true, effect.is_expired(11));
    }

    #[test]
    fn expire() {
        let mut effect = StatusEffect::new(Status::default(), Entity::new(1), 10);
        assert_eq!(false, effect.is_expired(9));
        effect.expire();
        assert_eq!(true, effect.is_expired(9));
    }

    #[test]
    fn has_flag() {
        let mut flags = HashSet::<StatusFlag>::new();
        flags.insert(StatusFlag::ExpireOnDirectDamage);
        let effect = StatusEffect::new(
            Status {
                flags: flags,
                ..Default::default()
            },
            Entity::new(1),
            10,
        );
        assert_eq!(true, effect.has_flag(&StatusFlag::ExpireOnDirectDamage));
    }

    #[test]
    fn has_flag2() {
        let effect = StatusEffect::new(
            Status {
                flags: HashSet::<StatusFlag>::new(),
                ..Default::default()
            },
            Entity::new(1),
            10,
        );
        assert_eq!(false, effect.has_flag(&StatusFlag::ExpireOnDirectDamage));
    }
}
