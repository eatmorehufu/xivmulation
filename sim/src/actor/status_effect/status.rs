use super::super::stat::{SpecialStat, Stat};
use super::Apply;
use super::QueryActor;
use crate::sim::{SimState, SimTime};
use bevy_ecs::prelude::Entity;
use delegate::delegate;
use std::collections::HashSet;
use std::sync::Arc;

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
            pub fn has_flag(&self, flag: &StatusFlag) -> bool;
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
    fn has_flag() {
        let status = Status::default();
        assert_eq!(false, status.has_flag(&StatusFlag::ExpireOnDirectDamage));
    }
    #[test]
    fn has_flag2() {
        let mut flags = HashSet::<StatusFlag>::new();
        flags.insert(StatusFlag::ExpireOnDirectDamage);
        let status = Status {
            flags: flags,
            ..Default::default()
        };
        assert_eq!(true, status.has_flag(&StatusFlag::ExpireOnDirectDamage));
    }
}
