pub mod lookup;
use super::stat::{Stat, Stats};
use crate::sim::SimState;

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
    multipliers: Vec<f32>,
) -> i32 {
    let wd = match attack_type {
        AttackType::PHYSICAL => stats.get(Stat::PhysicalWeaponDamage),
        AttackType::MAGIC => stats.get(Stat::MagicWeaponDamage),
    };
    let traitt = 48; // Assume level 80, static +48

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
        * traitt)
        / 100;
    // D3 = ⌊ D2 × CRIT? ⌋ /1000 ⌋ × DH? ⌋ /100 ⌋
    let d3 = (((d2 * crit) / 1000) * dh) / 100;
    // D = ⌊ D3 × rand[95,105] ⌋ /100 ⌋ × buff_1 ⌋ × buff_2 ⌋
    let d = d3 * sim.random_from_range(95, 106) / 100;

    // TODO: make sure int to float conversions aren't causing problems.
    multipliers
        .iter()
        .fold(d as f32, |total, multiplier| total * *multiplier) as i32
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
    130 * (det - lookup::level_modifiers(lookup::LevelColumn::SUB))
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
pub fn is_crit(sim: &SimState, chr: i32) -> bool {
    // p(CHR) = ⌊ 200 · ( CHR - LevelModLv, SUB )/ LevelModLv, DIV + 50 ⌋ / 10
    let probability: f32 = (200.0
        * ((chr - lookup::level_modifiers(lookup::LevelColumn::MAIN)) as f32)
        / (lookup::level_modifiers(lookup::LevelColumn::DIV) as f32)
        + 50.0)
        / 10.0;
    // TODO: Verify what type of value we're getting from probability and sim.random(). Is it 0.0 - 1.0?
    sim.random() < probability
}

// F(CRIT)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/functions/#critical-hit-damage-fcrit
pub fn critical_hit(sim: &SimState, crit: i32) -> i32 {
    if !is_crit(sim, crit) {
        return 1000;
    }

    // f(CRIT) = ⌊ 200 · ( CRIT - LevelModLv, SUB )/ LevelModLv, DIV + 1400 ⌋
    200 * (crit - lookup::level_modifiers(lookup::LevelColumn::SUB))
        / lookup::level_modifiers(lookup::LevelColumn::DIV)
        + 1400
}

// P(DHR)
// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/parameters/#pdhr
pub fn is_direct(sim: &SimState, dhr: i32) -> bool {
    // p(DHR) = ⌊ 550 · ( DHR - LevelModLv, SUB )/ LevelModLv, DIV ⌋ / 10
    let probability: f32 = 550.0
        * (dhr as f32 - (lookup::level_modifiers(lookup::LevelColumn::MAIN)) as f32)
        / (lookup::level_modifiers(lookup::LevelColumn::DIV) as f32)
        / 10.0;
    sim.random() < probability
}

pub fn direct_hit(sim: &SimState, crit: i32) -> i32 {
    if is_direct(sim, crit) {
        125
    } else {
        100
    }
}
