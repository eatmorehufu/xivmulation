use super::recast_expirations::RecastExpirations;
use super::Action;
use crate::sim::SimTime;

#[derive(Default)]
pub struct RotationEntry {
    pub action_id: u32,
    pub ogcd: bool,
}

impl RotationEntry {
    pub fn new(action: &Action) -> Self {
        RotationEntry {
            action_id: action.id,
            ogcd: action.ogcd,
        }
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
    ) -> Option<u32> {
        for entry in &self.0 {
            // TODO: Check conditions, eg. buffs/combos/etc. recast_timers probably becomes cast_validator or some such
            if recast_expirations.check_ready(entry.action_id, entry.ogcd, sim_time) {
                return Some(entry.action_id);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let id = rotation.get_next_action_id(10, &recast_expirations);
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

        let id = rotation.get_next_action_id(sim_time, &recast_expirations);
        assert_eq!(1, id.unwrap());
        Ok(())
    }
}
