pub type SimTime = u64;

pub const TICKS_PER_SECOND: SimTime = 1;
pub const MS_PER_TICK: SimTime = 1000 / TICKS_PER_SECOND;

#[derive(Default)]
pub struct SimState {
    milliseconds: SimTime,
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
