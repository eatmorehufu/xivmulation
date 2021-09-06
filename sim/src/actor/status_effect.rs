use super::Apply;
use crate::sim::SimTime;
use std::sync::Arc;

// TODO: Maybe a time ordered heap would be faster. Benchmark when we have more functionality.
#[derive(Default)]
pub struct StatusEffects(Vec<StatusEffect>);

impl StatusEffects {
    pub fn add(&mut self, status_effect: StatusEffect) {
        self.0.push(status_effect);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
// Represents an applied status with an expiration
pub struct StatusEffect {
    // expiration is the simulation timestamp when this status effect should be removed.
    pub expiration: SimTime,
    pub status: Status,
}

impl StatusEffect {
    pub fn new(status: Status, sim_time: SimTime) -> StatusEffect {
        return StatusEffect {
            expiration: sim_time + status.duration,
            status: status,
        };
    }
}

#[derive(Default, Clone)]
pub struct Status {
    pub duration: SimTime,
    pub effects: Vec<Arc<dyn Apply + Send + Sync>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_effect_new() {
        let status_effect = StatusEffect::new(Status::default(), 10);
        assert_eq!(10, status_effect.expiration);
    }

    #[test]
    fn status_effect_new2() {
        let status_effect = StatusEffect::new(
            Status {
                duration: 1000,
                ..Status::default()
            },
            10,
        );
        assert_eq!(1010, status_effect.expiration);
    }

    #[test]
    fn receive_status_effect() -> std::result::Result<(), String> {
        let mut effects = StatusEffects::default();
        let status_effect = StatusEffect::default();
        assert_eq!(0, effects.len());
        effects.add(status_effect);
        assert_eq!(1, effects.len());
        Ok(())
    }
}
