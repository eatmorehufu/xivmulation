use super::skill::Skill;

// Should this be the same job struct as in the sim?
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Job {
    pub name: String,
    pub skills: Vec<Skill>,
}