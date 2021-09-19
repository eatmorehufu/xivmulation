use crate::sim::SimTime;
use std::collections::HashMap;

#[derive(Default)]
pub struct RecastExpirations {
    actions: HashMap<u32, SimTime>,
    gcd_expiration: SimTime,
}

impl RecastExpirations {
    pub fn check_ready(&self, action_id: u32, ogcd: bool, sim_time: SimTime) -> bool {
        let ready = (ogcd || self.check_gcd_ready(sim_time))
            && match self.actions.get(&action_id) {
                Some(expiration) => *expiration <= sim_time,
                None => true,
            };
        ready
    }

    pub fn set(&mut self, action_id: u32, expiration: SimTime) {
        self.actions.insert(action_id, expiration);
    }

    pub fn check_gcd_ready(&self, sim_time: SimTime) -> bool {
        self.gcd_expiration <= sim_time
    }

    pub fn set_gcd(&mut self, expiration: SimTime) {
        self.gcd_expiration = expiration;
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
        ogcd: bool,
        gcd_expiration: SimTime,
    }

    macro_rules! test_check_ready {
        ($test_name:ident, $test_data:expr, $expected:expr) => {
            #[test]
            fn $test_name() {
                let data = $test_data;
                let mut recast_expirations = RecastExpirations {
                    gcd_expiration: data.gcd_expiration,
                    ..Default::default()
                };
                recast_expirations.set(data.action_id, data.expiration);
                assert_eq!(
                    $expected,
                    recast_expirations.check_ready(data.action_id, data.ogcd, data.sim_time)
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
    test_check_ready!(
        check_ready_gcd,
        TestCheckReadyData {
            expiration: 10,
            sim_time: 10,
            gcd_expiration: 11,
            ogcd: false,
            ..Default::default()
        },
        false
    );
    test_check_ready!(
        check_ready_gcd_with_ogcd,
        TestCheckReadyData {
            expiration: 10,
            sim_time: 10,
            gcd_expiration: 11,
            ogcd: true,
            ..Default::default()
        },
        true
    );

    #[test]
    fn check_ready_no_recast_with_gcd() {
        let recast_expirations = RecastExpirations {
            gcd_expiration: 10,
            ..Default::default()
        };
        assert_eq!(false, recast_expirations.check_ready(0, false, 9));
    }
}
