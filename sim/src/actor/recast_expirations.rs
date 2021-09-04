use crate::sim::SimTime;
use std::collections::HashMap;

#[derive(Default)]
pub struct RecastExpirations(HashMap<u32, SimTime>);

impl RecastExpirations {
    pub fn check_ready(&self, action_id: u32, sim_time: SimTime) -> bool {
        let RecastExpirations(timers) = self;
        match timers.get(&action_id) {
            Some(expiration) => *expiration <= sim_time,
            None => true,
        }
    }

    pub fn set(&mut self, action_id: u32, expiration: SimTime) {
        self.0.insert(action_id, expiration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct TestCheckReadyData {
        action_id: u32,
        sim_time: SimTime,
        expiration: SimTime,
    }

    macro_rules! test_check_ready {
        ($test_name:ident, $test_data:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let data = $test_data;
                let mut recast_expirations = RecastExpirations::default();
                recast_expirations.set(data.action_id, data.expiration);
                assert_eq!(
                    $expected,
                    recast_expirations.check_ready(data.action_id, data.sim_time)
                );
            }
        };
    }

    test_check_ready!(
        check_ready,
        TestCheckReadyData {
            ..Default::default()
        },
        true
    );
    test_check_ready!(
        check_ready_false,
        TestCheckReadyData {
            expiration: 10,
            sim_time: 9,
            ..Default::default()
        },
        false
    );
    test_check_ready!(
        check_ready_true_greater_sim_time,
        TestCheckReadyData {
            expiration: 10,
            sim_time: 11,
            ..Default::default()
        },
        true
    );
    test_check_ready!(
        check_ready_true_equal_sim_time,
        TestCheckReadyData {
            expiration: 10,
            sim_time: 10,
            ..Default::default()
        },
        true
    );
}
