mod action;
mod appliable;
mod status_effect;
use crate::sim::SimTime;
pub use action::Action;
pub use appliable::Appliable;
pub use status_effect::StatusEffect;

use std::collections::HashMap;

// TODO: not everything should be pub, but I'm being lazy right now...
#[derive(Default)]
pub struct Actor {
    pub id: u32,
    pub job: String,
    pub damage: u64,
    // gcd_duration is the number of milliseconds a global cooldown lasts.
    pub gcd_duration: SimTime,
    // gcd_expiration is the simulation timestamp that the gcd expires.
    pub gcd_expiration: SimTime,
    pub actions: HashMap<u32, Action>,
    pub rotation: Vec<RotationEntry>,
    pub status_effects: Vec<StatusEffect>,
}

// Enemy indicates that an actor is the target for the simulated actor.
#[derive(Default)]
pub struct Target {}

impl Actor {
    pub fn gcd_ready(&self, sim_time: SimTime) -> bool {
        self.gcd_expiration <= sim_time
    }

    pub fn get_next_action(&self, sim_time: SimTime) -> Option<&Action> {
        for entry in &self.rotation {
            if self.can_use(entry.action_id, sim_time) {
                match self.actions.get(&entry.action_id) {
                    Some(action) => return Some(action),
                    None => panic!(
                        // TODO: make actor and action to string methods
                        "Encountered a rotation action (id: {}) that this actor does not have.",
                        &entry.action_id
                    ),
                };
            }
        }
        None
    }

    pub fn can_use(&self, action_id: u32, sim_time: SimTime) -> bool {
        // TODO: add animation lock
        match self.actions.get(&action_id) {
            Some(action) => action.ready(sim_time, self.gcd_ready(sim_time)),
            None => false,
        }
    }

    pub fn receive_damage(&mut self, damage: u64) {
        self.damage += damage;
    }

    pub fn start_gcd(&mut self, sim_time: SimTime) {
        self.gcd_expiration = sim_time + self.gcd_duration;
    }

    pub fn receive_status_effect(&mut self, status_effect: StatusEffect) {
        self.status_effects.push(status_effect);
    }

    pub fn start_recast(&mut self, sim_time: SimTime, action_id: u32) {
        match self.actions.get_mut(&action_id) {
            Some(action) => action.start_recast(sim_time),
            None => (),
        }
    }
}

#[derive(Default)]
pub struct RotationEntry {
    pub action_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_gcd_ready {
        ($test_name:ident, $sim_time:expr, $gcd_expiration:expr, $expected:expr) => {
            #[test]
            fn $test_name() -> std::result::Result<(), String> {
                let mut actor = Actor::default();
                actor.gcd_expiration = $gcd_expiration;
                assert_eq!($expected, actor.gcd_ready($sim_time));
                Ok(())
            }
        };
    }
    test_gcd_ready!(ready, 1, 1, true);
    test_gcd_ready!(ready2, 2, 1, true);
    test_gcd_ready!(not_ready, 0, 1, false);

    #[test]
    fn get_next_action() -> std::result::Result<(), String> {
        let mut actor = Actor::default();
        actor.actions.insert(
            0,
            Action {
                id: 0,
                ..Default::default()
            },
        );
        actor.actions.insert(
            1,
            Action {
                id: 1,
                ..Default::default()
            },
        );
        actor.rotation.push(RotationEntry { action_id: 1 });
        actor.rotation.push(RotationEntry { action_id: 0 });

        let next_action1 = actor.get_next_action(10);
        assert_eq!(1, next_action1.unwrap().id);
        Ok(())
    }

    #[test]
    fn get_next_action_first_not_ready() -> std::result::Result<(), String> {
        let mut actor = Actor::default();
        actor.actions.insert(
            0,
            Action {
                id: 0,
                ..Default::default()
            },
        );
        actor.actions.insert(
            1,
            Action {
                id: 1,
                recast_expiration: 10,
                ..Default::default()
            },
        );
        actor.rotation.push(RotationEntry { action_id: 1 });
        actor.rotation.push(RotationEntry { action_id: 0 });

        let next_action = actor.get_next_action(5);
        assert_eq!(0, next_action.unwrap().id);
        Ok(())
    }

    macro_rules! test_can_use {
        ($test_name:ident, $id:expr, $expected:expr) => {
            #[test]
            fn $test_name() -> std::result::Result<(), String> {
                let mut actor = Actor::default();
                actor.actions.insert(
                    1,
                    Action {
                        id: 1,
                        ..Default::default()
                    },
                );
                assert_eq!($expected, actor.can_use($id, 0));
                Ok(())
            }
        };
    }

    test_can_use!(can_use, 1, true);
    test_can_use!(can_not_use_action_it_doesnt_have, 2, false);

    #[test]
    fn can_use_while_on_gcd() -> std::result::Result<(), String> {
        let mut actor = Actor {
            gcd_expiration: 1,
            ..Default::default()
        };
        actor.actions.insert(
            1,
            Action {
                id: 1,
                ogcd: false,
                ..Default::default()
            },
        );
        actor.actions.insert(
            2,
            Action {
                id: 2,
                ogcd: true,
                ..Default::default()
            },
        );

        assert_eq!(false, actor.can_use(1, 0));
        assert_eq!(true, actor.can_use(2, 0));
        Ok(())
    }

    #[test]
    fn receive_damage() -> std::result::Result<(), String> {
        let mut actor = Actor::default();
        actor.receive_damage(10);
        assert_eq!(10, actor.damage);
        Ok(())
    }

    #[test]
    fn start_gcd() {
        let mut actor = Actor {
            gcd_duration: 25000,
            ..Actor::default()
        };
        actor.start_gcd(10);
        assert_eq!(25010, actor.gcd_expiration)
    }

    #[test]
    fn receive_status_effect() -> std::result::Result<(), String> {
        let mut actor = Actor::default();
        let status_effect = StatusEffect::default();
        assert_eq!(0, actor.status_effects.len());
        actor.receive_status_effect(status_effect);
        assert_eq!(1, actor.status_effects.len());
        Ok(())
    }
}
