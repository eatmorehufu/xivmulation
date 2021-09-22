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
    pub name: String,
    pub duration: SimTime,
    pub effects: Vec<Arc<dyn Apply + Send + Sync>>,
    pub flags: StatusFlags,
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

#[derive(Default, Clone)]
pub struct StatusFlags(HashSet<StatusFlag>);

impl StatusFlags {
    pub fn new(flags: &[StatusFlag]) -> Self {
        let mut status_flags = StatusFlags::default();
        for flag in flags {
            status_flags.0.insert(*flag);
        }
        status_flags
    }

    delegate! {
        to self.0 {
            pub fn contains(&self, flag: &StatusFlag) -> bool;
        }
    }
}

pub struct ModifyStat {
    pub stat: Stat,
    pub amount: i64,
}

impl Apply for ModifyStat {
    fn apply(&self, _sim: &SimState, query: &mut QueryActor, _source: Entity, target: Entity) {
        if let Ok((_, _, _, _, _, _, _, mut stats, _)) = query.get_mut(target) {
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
        if let Ok((_, _, _, _, _, _, _, mut stats, _)) = query.get_mut(target) {
            stats.set_special(self.stat, self.amount);
        }
    }
}

pub struct SetCombo(pub u32);

impl Apply for SetCombo {
    fn apply(&self, _sim: &SimState, query: &mut QueryActor, source: Entity, _target: Entity) {
        if let Ok((_, _, _, _, _, _, _, _, mut active_combos)) = query.get_mut(source) {
            active_combos.add_action(self.0);
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
        let status = Status {
            flags: StatusFlags::new(&[StatusFlag::ExpireOnDirectDamage]),
            ..Default::default()
        };
        assert_eq!(true, status.has_flag(&StatusFlag::ExpireOnDirectDamage));
    }
}
