use super::recast_expirations::RecastExpirations;
use super::Action;
use crate::actor::ActiveCombos;
use crate::sim::SimTime;
use std::sync::Arc;

pub trait Check {
    fn check(&self, active_combos: &ActiveCombos) -> bool;
}

pub struct CheckCombo(pub u32);

impl Check for CheckCombo {
    fn check(&self, active_combos: &ActiveCombos) -> bool {
        active_combos.has_action(&self.0)
    }
}

#[derive(Default, Clone)]
pub struct RotationEntry {
    pub action_id: u32,
    pub ogcd: bool,
    pub conditions: Vec<Arc<dyn Check + Send + Sync>>,
}

impl RotationEntry {
    pub fn new(action: &Action) -> Self {
        RotationEntry {
            action_id: action.id,
            ogcd: action.ogcd,
            ..Default::default()
        }
    }

    pub fn with_condition(mut self, condition: Arc<dyn Check + Send + Sync>) -> Self {
        self.conditions.push(condition);
        self
    }
}

#[derive(Default)]
pub struct Rotation(Vec<RotationEntry>);

impl Rotation {
    pub fn add(&mut self, entry: RotationEntry) {
        self.0.push(entry);
    }

    pub fn get_next_action_id(
        &self,
        sim_time: SimTime,
        recast_expirations: &RecastExpirations,
        active_combos: &ActiveCombos,
    ) -> Option<u32> {
        for entry in &self.0 {
            // TODO: Check conditions, eg. buffs/combos/etc. recast_timers probably becomes cast_validator or some such
            if recast_expirations.check_ready(entry.action_id, entry.ogcd, sim_time)
                && entry.conditions.iter().all(|e| e.check(active_combos))
            {
                return Some(entry.action_id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::apply::ApplyCombo;
    #[test]
    fn get_next_action() -> std::result::Result<(), String> {
        let mut rotation = Rotation::default();
        rotation.add(RotationEntry {
            action_id: 0,
            ..Default::default()
        });
        rotation.add(RotationEntry {
            action_id: 1,
            ..Default::default()
        });

        let recast_expirations = RecastExpirations::default();
        let active_combos = ActiveCombos::default();
        let id = rotation.get_next_action_id(10, &recast_expirations, &active_combos);
        assert_eq!(0, id.unwrap());
        Ok(())
    }

    #[test]
    fn get_next_action_first_not_ready() -> std::result::Result<(), String> {
        let mut rotation = Rotation::default();
        rotation.add(RotationEntry {
            action_id: 0,
            ..Default::default()
        });
        rotation.add(RotationEntry {
            action_id: 1,
            ..Default::default()
        });

        let sim_time = 10;
        let mut recast_expirations = RecastExpirations::default();
        recast_expirations.set(0, sim_time + 1);

        let active_combos = ActiveCombos::default();
        let id = rotation.get_next_action_id(sim_time, &recast_expirations, &active_combos);
        assert_eq!(1, id.unwrap());
        Ok(())
    }

    #[test]
    fn get_next_action_first_failed_condition() {
        let vorpal_thrust = Action {
            id: 2,
            name: "Vorpal Thrust",
            ..Default::default()
        };
        let true_thrust = Action {
            id: 1,
            name: "True Thrust",
            results: vec![Arc::new(ApplyCombo(1))],
            ..Default::default()
        };
        let mut rotation = Rotation::default();
        rotation.add(RotationEntry::new(&vorpal_thrust).with_condition(Arc::new(CheckCombo(1))));
        rotation.add(RotationEntry::new(&true_thrust));

        let recast_expirations = RecastExpirations::default();
        let mut active_combos = ActiveCombos::default();
        let id = rotation.get_next_action_id(0, &recast_expirations, &active_combos);
        assert_eq!(1, id.unwrap());
        active_combos.add_action(1);
        let id = rotation.get_next_action_id(0, &recast_expirations, &active_combos);
        assert_eq!(2, id.unwrap());
    }
}
