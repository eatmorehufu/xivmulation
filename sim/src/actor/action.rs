use super::appliable::Appliable;
use super::Actor;
use crate::sim::SimTime;

#[derive(Default)]
pub struct Action {
    pub id: u32,
    pub name: String,
    pub ogcd: bool,
    // recast_duration is the duration of time in ms till the action can be performed again.
    pub recast_duration: SimTime,
    // recast_expiration is the simulation time after which this action may be performed again.
    pub recast_expiration: SimTime,
    pub effects: Vec<Box<dyn Appliable + Send + Sync>>,
}

impl Action {
    pub fn ready(&self, simulation_time: SimTime, gcd_ready: bool) -> bool {
        println!("{}, {}, {}", simulation_time, self.ogcd, gcd_ready);
        simulation_time >= self.recast_expiration && (self.ogcd || gcd_ready)
    }

    pub fn start_recast(&mut self, sim_time: SimTime) {
        self.recast_expiration = sim_time + self.recast_duration;
    }
}

impl Appliable for Action {
    fn apply(&self, source: &Actor, target: &mut Actor) {
        for effect in &self.effects {
            effect.apply(source, target);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::appliable::{Appliable, DoDamage};

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

    #[test]
    fn do_damage() -> std::result::Result<(), String> {
        let action = Action {
            id: 1,
            effects: vec![Box::new(DoDamage { potency: 100 })],
            ..Default::default()
        };
        let mut source = Actor::default();
        let mut target = Actor::default();
        action.apply(&mut source, &mut target);
        assert_eq!(100, target.damage);
        Ok(())
    }

    #[test]
    fn start_recast() {
        let mut action = Action {
            id: 1,
            recast_duration: 10000,
            ..Default::default()
        };
        action.start_recast(10);
        assert_eq!(10010, action.recast_expiration);
    }
}
