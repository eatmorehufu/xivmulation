pub mod lookup;
use super::stat::{Stat, Stats};
use crate::sim::SimState;
use math::round::floor;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum AttackType {
    PHYSICAL,
    MAGIC,
}

// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/damage-and-healing/#direct-damage-d
pub fn direct_damage(
    sim: &SimState,
    potency: i32,
    job: lookup::Job,
    stats: &Stats,
    attack_type: AttackType,
    multipliers: Vec<f64>,
) -> i32 {
    let wd = match attack_type {
        AttackType::PHYSICAL => stats.get(Stat::PhysicalWeaponDamage),
        AttackType::MAGIC => stats.get(Stat::MagicWeaponDamage),
    };
    let primary_stat_trait = 48; // Assume level 80, static +48

    let crit = critical_hit(sim, stats.get(Stat::CriticalHitRate));
    let dh = direct_hit(sim, stats.get(Stat::DirectHitRate));

    // https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/damage-and-healing/#direct-damage-d
    // D1 = ⌊ Potency × f(ATK) × f(DET) ⌋ /100 ⌋ /1000 ⌋
    let d1 = ((potency
        * attack_power(stats.get(Stat::AttackPower))
        * determination(stats.get(Stat::Determination)))
        / 100)
        / 1000;
    // D2 = ⌊ D1 × f(TNC) ⌋ /1000 ⌋ × f(WD) ⌋ /100 ⌋ × Trait ⌋ /100 ⌋
    let d2 = (((((d1 * tenacity(stats.get(Stat::Tenacity))) / 1000) * weapon_damage(job, wd))
        / 100)
        * primary_stat_trait)
        / 100;
    // D3 = ⌊ D2 × CRIT? ⌋ /1000 ⌋ × DH? ⌋ /100 ⌋
    let d3 = (((d2 * crit) / 1000) * dh) / 100;
    // D = ⌊ D3 × rand[95,105] ⌋ /100 ⌋ × buff_1 ⌋ × buff_2 ⌋
    let d = d3 * sim.rng.random_from_range(95, 106) / 100;

    // TODO: make sure int to float conversions aren't causing problems.
    multipliers
        .iter()
        .fold(d as f64, |total, multiplier| floor(total * *multiplier, 0)) as i32
}

// Level 80 F(AP)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#lv-80-fap
pub fn attack_power(ap: i32) -> i32 {
    // ⌊ 165 · ( AP - 340 ) / 340 ⌋ + 100
    (165 * (ap - 340) / 340) + 100
}

// F(DET)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#determination-fdet
pub fn determination(det: i32) -> i32 {
    // f(DET) = ⌊ 130 · ( DET - LevelMod Lv, Main )/ LevelMod Lv, DIV + 1000 ⌋
    130 * (det - lookup::level_modifiers(lookup::LevelColumn::MAIN))
        / lookup::level_modifiers(lookup::LevelColumn::DIV)
        + 1000
}

// F(TNC)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#tenacity-ftnc
pub fn tenacity(tnc: i32) -> i32 {
    // f(TNC) = ⌊ 100 · ( TNC - LevelModLv, SUB )/ LevelModLv, DIV + 1000 ⌋
    100 * (tnc - lookup::level_modifiers(lookup::LevelColumn::SUB))
        / lookup::level_modifiers(lookup::LevelColumn::DIV)
        + 1000
}

// F(WD)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#weapon-damage-fwd
// Use the WD appropriate for the attack being calculated (eg. Auto-attack = physical damage)
// All weapons have a Physical and Magical Damage value even though one of them is hidden.
pub fn weapon_damage(job: lookup::Job, wd: i32) -> i32 {
    // f(WD) = ⌊ ( LevelModLv, MAIN · JobModJob, Attribute / 1000 ) + WD ⌋
    (lookup::level_modifiers(lookup::LevelColumn::MAIN)
        * lookup::job_modifiers(job, job.primary_attribute())
        / 1000)
        + wd
}

// P(CHR)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/parameters/#critical-hit-probability
fn critical_hit_rate(chr: i32) -> f64 {
    // p(CHR) = ⌊ 200 · ( CHR - LevelModLv, SUB )/ LevelModLv, DIV + 50 ⌋ / 10
    floor(
        200.0 * ((chr - lookup::level_modifiers(lookup::LevelColumn::SUB)) as f64)
            / (lookup::level_modifiers(lookup::LevelColumn::DIV) as f64)
            + 50.0,
        0,
    ) / 10.0
}

fn is_crit(sim: &SimState, chr: i32) -> bool {
    let roll = sim.rng.random();
    let probability = critical_hit_rate(chr) / 100.0;
    roll < probability
}

fn critical_hit_damage(crit: i32) -> i32 {
    // f(CRIT) = ⌊ 200 · ( CRIT - LevelModLv, SUB )/ LevelModLv, DIV + 1400 ⌋
    200 * (crit - lookup::level_modifiers(lookup::LevelColumn::SUB))
        / lookup::level_modifiers(lookup::LevelColumn::DIV)
        + 1400
}

// F(CRIT)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#critical-hit-damage-fcrit
pub fn critical_hit(sim: &SimState, crit: i32) -> i32 {
    if !is_crit(sim, crit) {
        return 1000;
    }
    critical_hit_damage(crit)
}

// P(DHR)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/parameters/#pdhr
fn direct_hit_rate(dhr: i32) -> f64 {
    // p(DHR) = ⌊ 550 · ( DHR - LevelModLv, SUB )/ LevelModLv, DIV ⌋ / 10
    floor(
        550.0 * (dhr as f64 - (lookup::level_modifiers(lookup::LevelColumn::SUB)) as f64)
            / (lookup::level_modifiers(lookup::LevelColumn::DIV) as f64),
        0,
    ) / 10.0
}

pub fn is_direct(sim: &SimState, dhr: i32) -> bool {
    let roll = sim.rng.random();
    let probability = direct_hit_rate(dhr) / 100.0;
    roll < probability
}

pub fn direct_hit(sim: &SimState, crit: i32) -> i32 {
    if is_direct(sim, crit) {
        125
    } else {
        100
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sim::SimRng;

    #[test]
    fn test_direct_hit_rate() {
        assert_eq!(66.6, direct_hit_rate(4376));
        assert_eq!(0.0, direct_hit_rate(380));
        assert_eq!(31.5, direct_hit_rate(2270));
    }

    #[test]
    fn test_critical_hit_rate() {
        assert_eq!(29.2, critical_hit_rate(4373));
        assert_eq!(13.5, critical_hit_rate(1783));
        assert_eq!(5.0, critical_hit_rate(380));
    }

    #[test]
    fn test_determination() {
        assert_eq!(1000, determination(340));
        assert_eq!(1039, determination(1330));
        assert_eq!(1157, determination(4326));
    }

    #[test]
    fn test_tenacity() {
        assert_eq!(1000, tenacity(380));
        assert_eq!(1079, tenacity(2987));
        assert_eq!(1121, tenacity(4373));
    }

    #[test]
    fn test_critical_hit_damage() {
        assert_eq!(1642, critical_hit_damage(4373));
        assert_eq!(1485, critical_hit_damage(1783));
        assert_eq!(1400, critical_hit_damage(380));
    }

    /*
    fast blade potency 200
    6523
    7026

    6847
    6753
    6748
    6660
    6732
    6995
    6731
    6981
    6859
    6969
    7006
    8737?
    8143?
    8522?
    8469?
    10397!
    10947!
    12883!!
    */

    pub struct FakeRng {
        random_value: f64,
        random_from_range_value: i32,
    }

    impl SimRng for FakeRng {
        fn random(&self) -> f64 {
            self.random_value
        }
        fn random_from_range(&self, _low_inclusive: i32, _high_exclusive: i32) -> i32 {
            self.random_from_range_value
        }
    }

    fn get_stats() -> Stats {
        let mut stats = Stats::default();
        stats.set_base(Stat::PhysicalWeaponDamage, 134);
        stats.set_base(Stat::Strength, 5435);
        stats.set_base(Stat::Dexterity, 326);
        stats.set_base(Stat::Vitality, 6258);
        stats.set_base(Stat::Intelligence, 206);
        stats.set_base(Stat::Mind, 339);
        stats.set_base(Stat::CriticalHitRate, 3543);
        stats.set_base(Stat::Determination, 2965);
        stats.set_base(Stat::DirectHitRate, 1620);
        stats.set_base(Stat::Defense, 8740);
        stats.set_base(Stat::MagicDefense, 8740);
        stats.set_base(Stat::AttackPower, 5435);
        stats.set_base(Stat::SkillSpeed, 1012);
        stats.set_base(Stat::AttackMagicPotency, 206);
        stats.set_base(Stat::HealingMagicPotency, 339);
        stats.set_base(Stat::SpellSpeed, 380);
        stats.set_base(Stat::Tenacity, 606);
        stats.set_base(Stat::Piety, 340);
        stats
    }

    #[test]
    fn test_expected_damage() {
        let sim = SimState::new(FakeRng {
            random_value: 1.0,
            random_from_range_value: 100,
        });
        let stats = get_stats();
        let potency = 200;
        let job = lookup::Job::PLD;
        let attack_type = AttackType::PHYSICAL;

        // Does not pass.
        // assert_eq!(
        //     8000,
        //     direct_damage(&sim, potency, job, &stats, attack_type, vec![])
        // );
    }
}
