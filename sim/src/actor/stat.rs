use std::collections::HashMap;
#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Stat {
    // primary
    Strength,
    Dexterity,
    Vitality,
    Intelligence,
    Mind,
    // offensive
    CriticalHit,
    Determination,
    DirectHit,
    // defensive
    Defense,
    MagicDefense,
    // other
    SkillSpeed,
    Piety,
    Tenacity,
}

#[derive(Default)]
pub struct Stats(HashMap<Stat, i32>);

impl Stats {
    pub fn add(&mut self, stat: Stat, amount: i32) {
        match self.0.get_mut(&stat) {
            Some(value) => *value += amount,
            None => {
                self.0.insert(stat, amount);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let mut stats = Stats::default();
        stats.add(Stat::CriticalHit, 10);
        assert_eq!(&10, &stats.0[&Stat::CriticalHit]);
    }
}
