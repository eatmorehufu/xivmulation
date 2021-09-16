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
    CriticalHitRate,
    Determination,
    DirectHitRate,
    // defensive
    Defense,
    MagicDefense,
    // physical properties
    AttackPower,
    SkillSpeed,
    // mental properties
    AttackMagicPotency,
    HealingMagicPotency,
    SpellSpeed,
    // role
    Piety,
    Tenacity,
    // other
    PhysicalWeaponDamage,
    MagicWeaponDamage,
}

#[derive(Default)]
pub struct Stats {
    delta: HashMap<Stat, i32>,
    base: HashMap<Stat, i32>,
}

impl Stats {
    pub fn get(&self, stat: Stat) -> i32 {
        self.delta.get(&stat).unwrap_or(&0) + self.base.get(&stat).unwrap_or(&0)
    }

    pub fn add(&mut self, stat: Stat, amount: i32) {
        match self.delta.get_mut(&stat) {
            Some(value) => *value += amount,
            None => {
                self.delta.insert(stat, amount);
            }
        }
    }

    pub fn reset(&mut self) {
        self.delta.clear();
    }

    pub fn set_base(&mut self, stat: Stat, amount: i32) {
        self.base.insert(stat, amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut stats = Stats::default();
        stats.add(Stat::CriticalHitRate, 10);
        assert_eq!(10, stats.get(Stat::CriticalHitRate));
        stats.set_base(Stat::CriticalHitRate, 5);
        assert_eq!(15, stats.get(Stat::CriticalHitRate));
    }

    #[test]
    fn add() {
        let mut stats = Stats::default();
        stats.add(Stat::CriticalHitRate, 10);
        assert_eq!(10, stats.get(Stat::CriticalHitRate));
    }

    #[test]
    fn set_base() {
        let mut stats = Stats::default();
        stats.set_base(Stat::CriticalHitRate, 10);
        assert_eq!(10, stats.base[&Stat::CriticalHitRate]);
        assert_eq!(false, stats.delta.contains_key(&Stat::CriticalHitRate));
        assert_eq!(10, stats.get(Stat::CriticalHitRate));
    }

    #[test]
    fn reset() {
        let mut stats = Stats::default();
        stats.set_base(Stat::CriticalHitRate, 10);
        stats.add(Stat::CriticalHitRate, 5);
        assert_eq!(15, stats.get(Stat::CriticalHitRate));
        stats.reset();
        assert_eq!(10, stats.get(Stat::CriticalHitRate));
    }
}
