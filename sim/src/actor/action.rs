use super::apply::Apply;
use super::Actor;
use crate::sim::SimState;
use crate::sim::SimTime;
use std::collections::HashMap;

#[derive(Default)]
pub struct Actions(HashMap<u32, Action>);

impl Actions {
    pub fn add(&mut self, action: Action) {
        self.0.insert(action.id, action);
    }
}

#[derive(Default)]
pub struct Action {
    pub id: u32,
    pub name: String,
    pub target_self: bool,
    pub ogcd: bool,
    // recast_duration is the duration of time in ms till the action can be performed again.
    pub recast_duration: SimTime,
    // recast_expiration is the simulation time after which this action may be performed again.
    pub recast_expiration: SimTime,
    pub results: Vec<Box<dyn Apply + Send + Sync>>,
}

impl Action {
    pub fn ready(&self, simulation_time: SimTime, gcd_ready: bool) -> bool {
        println!("{}, {}, {}", simulation_time, self.ogcd, gcd_ready);
        simulation_time >= self.recast_expiration && (self.ogcd || gcd_ready)
    }

    pub fn start_recast(&mut self, sim_time: SimTime) {
        self.recast_expiration = sim_time + self.recast_duration;
    }

    pub fn perform(&self, sim_state: &SimState, source: &mut Actor, target: &mut Actor) {
        for result in &self.results {
            if self.target_self {
                result.apply(sim_state, source);
            } else {
                result.apply(sim_state, target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::apply::{Apply, DoDamage};

    #[derive(Default)]
    struct TestReadyData {
        sim_time: SimTime,
        recast_expiration: SimTime,
        gcd_ready: bool,
        action_ogcd: bool,
    }

    macro_rules! test_ready {
        ($test_name:ident, $test_ready_data:expr,  $expected:expr) => {
            #[test]
            fn $test_name() -> std::result::Result<(), String> {
                let d = $test_ready_data;
                let action = Action {
                    recast_expiration: d.recast_expiration,
                    ogcd: d.action_ogcd,
                    ..Default::default()
                };

                assert_eq!($expected, action.ready(d.sim_time, d.gcd_ready));
                Ok(())
            }
        };
    }
    test_ready!(
        gcd_ready,
        TestReadyData {
            gcd_ready: true,
            ..Default::default()
        },
        true
    );
    test_ready!(
        gcd_not_ready,
        TestReadyData {
            gcd_ready: false,
            ..Default::default()
        },
        false
    );
    test_ready!(
        recast_ready,
        TestReadyData {
            recast_expiration: 10,
            sim_time: 10,
            gcd_ready: true,
            ..Default::default()
        },
        true
    );
    test_ready!(
        recast_not_ready,
        TestReadyData {
            recast_expiration: 10,
            sim_time: 9,
            ..Default::default()
        },
        false
    );
    test_ready!(
        ready_ogcd_gcd_ready,
        TestReadyData {
            gcd_ready: true,
            action_ogcd: true,
            ..Default::default()
        },
        true
    );
    test_ready!(
        ready_ogcd_gcd_not_ready,
        TestReadyData {
            gcd_ready: false,
            action_ogcd: true,
            ..Default::default()
        },
        true
    );
}
