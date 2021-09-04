pub type SimTime = u64;

pub const TICKS_PER_SECOND: SimTime = 20;
pub const MS_PER_TICK: SimTime = 1000 / TICKS_PER_SECOND;

#[derive(Default)]
pub struct SimState {
    milliseconds: SimTime,
}

impl SimState {
    pub fn tick(&mut self) {
        self.milliseconds += MS_PER_TICK;
    }

    pub fn milliseconds(&self) -> SimTime {
        self.milliseconds
    }
}
