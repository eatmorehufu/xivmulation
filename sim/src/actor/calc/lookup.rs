#[allow(dead_code)]
pub enum LevelColumn {
    MP,
    MAIN,
    SUB,
    DIV,
    HP,
    ELMT,
    THREAT,
}

// Assume level 80
// https://www.akhmorning.com/allagan-studies/modifiers/
#[allow(dead_code)]
pub fn level_modifiers(column: LevelColumn) -> i32 {
    match column {
        LevelColumn::MP => 10000,
        LevelColumn::MAIN => 340,
        LevelColumn::SUB => 380,
        LevelColumn::DIV => 3300,
        LevelColumn::HP => 4400,
        LevelColumn::ELMT => 0, // ??? on akhmorning. https://www.akhmorning.com/allagan-studies/modifiers/levelmods/
        LevelColumn::THREAT => 569,
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Job {
    GLA,
    PGL,
    MRD,
    LNC,
    ARC,
    CNJ,
    THM,
    PLD,
    MNK,
    WAR,
    DRG,
    BRD,
    WHM,
    BLM,
    ACN,
    SMN,
    SCH,
    ROG,
    NIN,
    MCH,
    DRK,
    AST,
    SAM,
    RDM,
    BLU,
    GNB,
    DNC,
    None,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Attribute {
    HP,
    MP,
    STR,
    VIT,
    DEX,
    INT,
    MND,
}

impl Job {
    pub fn primary_attribute(self) -> Attribute {
        match self {
            Job::LNC | Job::PGL | Job::DRG | Job::MNK | Job::SAM => Attribute::STR,
            Job::ARC | Job::ROG | Job::BRD | Job::NIN | Job::MCH | Job::DNC => Attribute::DEX,
            Job::GLA | Job::MRD | Job::PLD | Job::WAR | Job::DRK | Job::GNB => Attribute::VIT,
            Job::THM | Job::ACN | Job::BLM | Job::SMN | Job::RDM | Job::BLU => Attribute::INT,
            Job::CNJ | Job::WHM | Job::SCH | Job::AST => Attribute::MND,
            _ => panic!("Tried to get primary attribute of unknown job: {:?}", self),
        }
    }

    pub fn is_tank(self) -> bool {
        match self {
            Job::GLA | Job::MRD | Job::PLD | Job::WAR | Job::DRK | Job::GNB => true,
            _ => false,
        }
    }
}

macro_rules! job_stat_match {
    ($attribute:expr, $hp:expr, $mp:expr, $str:expr, $vit:expr, $dex:expr, $int:expr, $mnd:expr) => {
        match $attribute {
            Attribute::HP => $hp,
            Attribute::MP => $mp,
            Attribute::STR => $str,
            Attribute::VIT => $vit,
            Attribute::DEX => $dex,
            Attribute::INT => $int,
            Attribute::MND => $mnd,
        }
    };
}

// TODO: Maybe this should be a CSV or something, but this works for now.
// https://www.akhmorning.com/allagan-studies/modifiers/
pub fn job_modifiers(job: Job, attribute: Attribute) -> i32 {
    match job {
        Job::GLA => job_stat_match!(attribute, 110, 49, 95, 100, 90, 50, 95),
        Job::PGL => job_stat_match!(attribute, 105, 34, 100, 95, 100, 45, 85),
        Job::MRD => job_stat_match!(attribute, 115, 28, 100, 100, 90, 30, 50),
        Job::LNC => job_stat_match!(attribute, 110, 39, 105, 100, 95, 40, 60),
        Job::ARC => job_stat_match!(attribute, 100, 69, 85, 95, 105, 80, 75),
        Job::CNJ => job_stat_match!(attribute, 100, 117, 50, 95, 100, 100, 105),
        Job::THM => job_stat_match!(attribute, 100, 123, 40, 95, 95, 105, 70),
        Job::PLD => job_stat_match!(attribute, 120, 59, 100, 110, 95, 60, 100),
        Job::MNK => job_stat_match!(attribute, 110, 43, 110, 100, 105, 50, 90),
        Job::WAR => job_stat_match!(attribute, 125, 38, 105, 110, 95, 40, 55),
        Job::DRG => job_stat_match!(attribute, 115, 49, 115, 105, 100, 45, 65),
        Job::BRD => job_stat_match!(attribute, 105, 79, 90, 100, 115, 85, 80),
        Job::WHM => job_stat_match!(attribute, 105, 124, 55, 100, 105, 105, 115),
        Job::BLM => job_stat_match!(attribute, 105, 129, 45, 100, 100, 115, 75),
        Job::ACN => job_stat_match!(attribute, 100, 110, 85, 95, 95, 105, 75),
        Job::SMN => job_stat_match!(attribute, 105, 111, 90, 100, 100, 115, 80),
        Job::SCH => job_stat_match!(attribute, 105, 119, 90, 100, 100, 105, 115),
        Job::ROG => job_stat_match!(attribute, 103, 38, 80, 95, 100, 60, 70),
        Job::NIN => job_stat_match!(attribute, 108, 48, 85, 100, 110, 65, 75),
        Job::MCH => job_stat_match!(attribute, 105, 79, 85, 100, 115, 80, 85),
        Job::DRK => job_stat_match!(attribute, 120, 79, 105, 110, 95, 60, 40),
        Job::AST => job_stat_match!(attribute, 105, 124, 50, 100, 100, 105, 115),
        Job::SAM => job_stat_match!(attribute, 109, 40, 112, 100, 108, 60, 50),
        Job::RDM => job_stat_match!(attribute, 105, 120, 55, 100, 105, 115, 110),
        Job::BLU => job_stat_match!(attribute, 105, 120, 70, 100, 110, 115, 105),
        Job::GNB => job_stat_match!(attribute, 120, 59, 100, 110, 95, 60, 100),
        Job::DNC => job_stat_match!(attribute, 105, 79, 90, 100, 115, 85, 80),
        _ => panic!("Tried to get base stats of unknown job: {:?}", job),
    }
}
