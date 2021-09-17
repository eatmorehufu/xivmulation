use rand::{random, Rng};
use std::sync::Arc;

pub type SimTime = u64;

pub const TICKS_PER_SECOND: SimTime = 1;
pub const MS_PER_TICK: SimTime = 1000 / TICKS_PER_SECOND;

pub trait SimRng {
    fn random(&self) -> f64;
    fn random_from_range(&self, low_inclusive: i64, high_exclusive: i64) -> i64;
}

struct RealRng {}

impl SimRng for RealRng {
    fn random(&self) -> f64 {
        random::<f64>()
    }

    fn random_from_range(&self, low_inclusive: i64, high_exclusive: i64) -> i64 {
        rand::thread_rng().gen_range(low_inclusive..high_exclusive)
    }
}

pub struct SimState {
    milliseconds: SimTime,
    pub rng: Arc<dyn SimRng + Sync + Send>,
}

impl SimState {
    pub fn new<T: SimRng + Sync + Send + 'static>(rng: T) -> Self {
        SimState {
            milliseconds: 0,
            rng: Arc::<T>::new(rng),
        }
    }
}

impl Default for SimState {
    fn default() -> Self {
        SimState {
            milliseconds: 0,
            rng: Arc::<RealRng>::new(RealRng {}),
        }
    }
}

impl SimState {
    pub fn tick(&mut self) -> SimTime {
        self.milliseconds += MS_PER_TICK;
        self.milliseconds
    }

    pub fn now(&self) -> SimTime {
        self.milliseconds
    }
}
