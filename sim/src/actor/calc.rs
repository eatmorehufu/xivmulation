// https://www.akhmorning.com/allagan-studies/how-to-be-a-math-wizard/shadowbringers/damage-and-healing/#direct-damage-d
pub fn damage(potency: i32, crit: i32) -> i32 {
    // TODO: lol fix this math. just a demonstration
    if crit == 100 {
        potency * 2
    } else {
        potency
    }
}

pub fn crit(crit: i32) -> i32 {
    // https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
    // TODO: pull all this data into lookup table.
    let level_mod_sub = 380;
    let level_mod_div = 3300;
    200 * (crit - level_mod_sub) / level_mod_div + 1400
}
